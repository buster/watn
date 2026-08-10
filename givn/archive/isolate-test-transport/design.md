# Design: Isolate Test Transport

## Scope and decisions

- Keep the existing blocking HTTP client and OpenAI-compatible request shapes.
- Add an opt-in `test-support` Cargo feature with no default activation.
- Make the endpoint override available only to debug binaries compiled with
  `test-support`.
- Keep URL builders pure: they receive an already resolved endpoint and never
  read process-global environment state.
- Keep the existing loopback `httpmock` digital twins and CLI subprocess
  harness. The transport scenarios use ordinary subprocesses, not direct
  provider calls.
- Do not change provider-selection precedence, persisted TOML shape, or the
  external provider protocol.
- Do not fix the stale-search false-green behavior in this change. It is a
  model-discovery concurrency defect, not a transport-isolation behavior, and
  is deferred to `model-discovery-and-setup-correctness`. This change must not
  retain a stale-search implementation obligation or alter
  `search_query_delays`.

The compile-time boundary is intentionally stronger than a feature-only
boundary. A release profile with `test-support` enabled is still a release
binary and must not be able to use `WATN_TEST_ENDPOINT_OVERRIDE`. This change
exercises the debug binaries first; release-profile smoke verification is
deferred to `release-truth-and-repository-cleanup`.

## Compile-time transport boundary

### Cargo feature

`Cargo.toml` adds a feature with no default activation:

```toml
[features]
default = []
test-support = []
```

The endpoint resolver has exactly two compile-time branches. The override
branch is compiled only under:

```rust
#[cfg(all(feature = "test-support", debug_assertions))]
```

The configured-endpoint branch is compiled under the equivalent negation:

```rust
#[cfg(not(all(feature = "test-support", debug_assertions)))]
```

Required properties:

- A default-feature debug binary uses the configured endpoint.
- A `test-support` debug binary uses a non-empty, non-whitespace override only
  for outbound request construction.
- A default-feature release binary never reads
  `WATN_TEST_ENDPOINT_OVERRIDE`.
- A `test-support` release binary also never reads
  `WATN_TEST_ENDPOINT_OVERRIDE`; `debug_assertions` is false in the release
  profile, so it compiles the configured-endpoint branch.
- Missing and whitespace-only overrides resolve to the configured endpoint in
  the debug test-support branch.
- Configuration loading, provider readiness, provider persistence, and
  endpoint display use the configured endpoint, never the effective request
  endpoint.
- No production module may add a second lookup of
  `WATN_TEST_ENDPOINT_OVERRIDE`.

The four URL builders remain pure after resolution. They must construct only
from the effective endpoint passed to them. If the effective endpoint is
`<server-base>/v1`, the resulting full URLs are:

- `chat_completions_url`: `<effective>/chat/completions`.
- `models_url`: `<effective>/models`.
- `models_search_url`: `<effective>/models?search=<query>` with the existing
  query encoding contract.
- `models_page_url`: `<effective>/models?page=<page>&limit=<limit>`.

The request-construction boundary resolves the endpoint once, then passes the
result to the pure URL builder. Readiness calls configuration resolution only;
it does not construct an HTTP URL and does not start a network request.

## Build and binary matrix

The harness must receive explicit binary paths. It must never discover a
fallback such as `target/debug/watn`, reuse a stale package binary, or build
inside a scenario. Debug verification uses Cargo's shared default target
cache, then copies each debug executable to a unique temporary path. This
keeps the two child binaries distinct without recompiling all dependencies in
four isolated target directories.

| Binary path key | Build and copy sequence | Scenario use |
|---|---|---|
| `WATN_DEFAULT_DEBUG_BIN` | `cargo build --bin watn && cp target/debug/watn <root>/default-debug` | configured-endpoint control |
| `WATN_TEST_SUPPORT_DEBUG_BIN` | `cargo build --features test-support --bin watn && cp target/debug/watn <root>/test-support-debug` | isolated routing and fallback |

The bootstrap creates one temporary copy directory before Cucumber starts,
runs the two build commands sequentially, and exports both absolute copy paths.
The second build reuses Cargo's dependency cache. Missing variables or copy
files are bootstrap errors before any scenario runs. Scenario steps select a
path by key; they do not discover binaries from the filesystem.

The two copied debug variants are a bootstrap concern, not a requirement that
each scenario invoke both. The normal scenario invokes only
`WATN_DEFAULT_DEBUG_BIN`; the isolated-routing and fallback scenarios select
`WATN_TEST_SUPPORT_DEBUG_BIN` where the override behavior is intended.

The release guard remains in production code through the negated
`cfg(all(feature = "test-support", debug_assertions))` branch. Building and
running release variants is deliberately not part of this debug-focused
change; the later release change will verify that guard with the same copy
pattern.

## Transport-specific test state

Add a concrete `TransportState` owned by `transport_steps.rs`; do not condition
new behavior on optional fields in the shared `WatnWorld` or reuse the shared
single-server mock state. The shared world contains one transport state value
initialized for every scenario. Its fields are:

```text
TransportState {
    configured_server: local server handle,
    configured_endpoint: exact <server-base>/v1 URL,
    configured_chat_mock_ids: per-binary mock ids,
    competing_server: local server handle,
    competing_endpoint: exact <server-base>/v1 URL,
    competing_chat_mock_id: mock id,
    isolated_server: optional local server handle,
    isolated_endpoint: optional exact <server-base>/v1 URL,
    isolated_chat_mock_id: optional mock id,
    configured_endpoint_before: exact config value,
    persisted_config_path: temporary config path,
    expected_api_key: "sk-configured",
    default_model: "test-model",
    override_state: missing | whitespace | competing | isolated,
    binary_paths: explicit build-matrix paths,
    invocations: per-binary exit/stdout/stderr records,
    readiness_before_request: optional bool,
}
```

Every provider twin binds to `127.0.0.1` on an ephemeral port. The configured,
competing, and isolated servers are separate server instances, so a hit on one
cannot satisfy a mock on another. Their configuration endpoints include the
`/v1` path and are reachable during the scenario. Every CLI fixture writes:

```toml
[defaults]
provider = "configured"

[providers.configured]
endpoint = "<configured-loopback-base>/v1"
api_key = "sk-configured"
default_model = "test-model"
```

The state records the exact configured URL before the child starts and reads
the raw TOML after the child exits. It never prints the API key in diagnostics.
Server handles, mock ids, config path, and child results are cleaned up after
each scenario.

## Mock and assertion contract

No transport mock is a catch-all. Each relevant mock matches the method, exact
path, and expected Authorization header. The step implementation also records
the full expected URL as `<captured-server-endpoint><path>` and asserts it
against the captured outgoing request record. A server identity plus a positive
mock hit is not sufficient evidence of the full URL.

For a chat request, all of these assertions are mandatory:

| Assertion | Required value |
|---|---|
| Full URL | exact configured, competing, or isolated endpoint plus `/v1/chat/completions` |
| Method and path | `POST /v1/chat/completions` exactly |
| Expected request count | exactly `1` for each invoked child and expected server |
| Competing request count | exactly `0` |
| Authorization | exactly `Bearer sk-configured` |
| Response evidence | child output contains the response body from the expected twin and not the competing response |
| Persisted endpoint | exact configured loopback URL, unchanged after the child |
| Persisted override | the override URL is absent from raw TOML |

For the normal debug scenario, only `WATN_DEFAULT_DEBUG_BIN` is invoked with a
non-empty override. It must produce one configured-server hit and zero
competing-server hits. This proves that a normal debug binary cannot be
redirected. The `WATN_TEST_SUPPORT_DEBUG_BIN` copy is intentionally not invoked
in this scenario; its override-honoring behavior is covered by the dedicated
isolated-routing scenario.

For the test-support debug scenario, the isolated server must receive exactly
one request and the configured server exactly zero. For the missing and
whitespace cases, the configured server must receive exactly one request and
the competing server exactly zero. The persisted endpoint is checked in every
subprocess scenario, not only in the isolated-routing scenario.

The existing provider and model-discovery transport steps that are touched by
this change use the same contract. Model requests assert exact paths and
headers: `GET /v1/models`, `GET /v1/models?search=<query>`, or
`GET /v1/models?page=<page>&limit=<limit>` relative to the configured `/v1`
endpoint, with an exact `Bearer` header when a credential is configured. A
positive hit count alone is never an assertion.

## Scenario mechanics

### Normal debug requests

The scenario starts configured and competing loopback twins, writes the
configured endpoint and `default_model = "test-model"`, sets the override to
the competing endpoint, and invokes the default-feature debug copy from the
build matrix. Readiness must complete without contacting either twin. The child
must send its chat request to the configured full URL, return
`configured-response`, and exit successfully. The competing server must remain
at zero hits.

### Debug isolated routing

The scenario starts configured and isolated loopback twins, writes the
configured endpoint and key/default model, sets the override to the isolated
endpoint, and invokes `WATN_TEST_SUPPORT_DEBUG_BIN`. The isolated full URL,
path, count, and Authorization header must match exactly. The configured twin
must remain at zero hits. The raw TOML must still contain exactly the configured
endpoint and must not contain the isolated endpoint.

### Missing and whitespace fallback

The scenario runs two explicit child invocations in one Gherkin scenario: one
with the override removed from the child environment and one with its value
set to whitespace. Both invocations use `WATN_TEST_SUPPORT_DEBUG_BIN`. Each
must use the configured full URL exactly, send one authorized request, and
return `configured-response`; the aggregate configured count is two and the
competing server remains at zero hits. The configured endpoint remains
unchanged in TOML.

### Readiness contract

The readiness scenario is deliberately non-E2E because readiness is a local
configuration predicate, not a user interaction. It constructs a `Config` with
the reachable configured loopback endpoint and complete credential/default
model, sets a separate local competing endpoint as the override, calls the
public readiness predicate, and asserts `true`. It then asserts zero requests
on both servers and exact equality of the configured endpoint in the provider
record. No HTTP client or URL builder is invoked by this scenario.

## E2E interface and strictness

The capability interface is the CLI. The three `@e2e` scenarios use real watn
subprocesses and temporary XDG configuration, not direct function calls. The
readiness contract is a non-`@e2e` API-level scenario and is driven through the
public configuration predicate.

The runner uses `.fail_on_skipped()` and serial scenario execution. New steps
must set up real local twins and perform real assertions; no empty or
panic-only step body is acceptable. The `@wip` tags remain until implementation
and strict verification are complete. No existing `@e2e` tag is removed.

## Test commands

Build the debug copies before running the Cucumber commands. The path variables
below are illustrative names for the explicit paths defined in the matrix:

```text
WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'

WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
```

Single-scenario commands use the same two path variables and select one
scenario by name:

```text
WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --name "Normal debug requests ignore test routing settings"

WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --name "Test-support requests use isolated routing without changing saved configuration"

WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --name "Missing or whitespace test overrides fall back to the configured provider"

WATN_DEFAULT_DEBUG_BIN=<root>/default-debug \
WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug \
cargo test --test features_runner --features test-support -- --name "Provider readiness ignores the test routing setting"
```

The debug copy bootstrap is:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX)
cargo build --bin watn
cp target/debug/watn "$root/default-debug"
cargo build --features test-support --bin watn
cp target/debug/watn "$root/test-support-debug"
```

## Interaction Coverage Matrix

The feature inventory has exactly one `@e2e` scenario for each listed user
interaction. The readiness contract is intentionally absent from this matrix
because it is not an E2E user interaction.

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run a normal debug watn request while a non-empty test routing setting is present | Normal debug requests ignore test routing settings | CLI | Run exactly one explicit default-feature debug subprocess copy against separate configured and competing loopback twins; do not invoke the test-support copy in this scenario; inspect output, exact URL/path, counts, Authorization, and TOML |
| run a test-support debug watn request through an isolated local provider twin | Test-support requests use isolated routing without changing saved configuration | CLI | Run `WATN_TEST_SUPPORT_DEBUG_BIN` as a subprocess with temporary XDG config and an isolated loopback override; inspect the isolated/configured twins and persisted TOML |
| run a test-support debug watn request with a missing or whitespace override and fall back to the configured local provider | Missing or whitespace test overrides fall back to the configured provider | CLI | Run the explicit test-support debug subprocess twice in one scenario, once with the variable removed and once with whitespace; inspect exact configured/competing routes, counts, Authorization, output, and TOML |

## Architecture impact

### `Cargo.toml`

Add only the no-dependency `test-support` feature described above. The feature
is not a release capability; its debug-only endpoint branch is enforced by
`cfg(all(feature = "test-support", debug_assertions))`.

### `src/provider/transport.rs`

Keep one endpoint-resolution boundary. The implementation may use a helper,
but it must have the exact compile-time guard above and must return the
configured string in every release profile. The environment variable is never
part of `Config`, `ProviderConfig`, readiness state, or serialization.

### Request paths

The provider and model request paths resolve the endpoint only at outbound HTTP
construction, then pass it to pure URL builders. No caller may resolve the
override during config load, readiness, persistence, or display.

### Test harness

Add transport-specific steps and the explicit binary-path bootstrap. Do not
reuse the shared conditional mock fields for this state. The harness owns the
loopback twin endpoints, exact mock ids, expected key, default model, binary
matrix, child results, and persisted-config snapshot.

## No data model change

The persisted TOML format is unchanged. The endpoint override is a debug-only
test capability and is never represented in `Config`, `ProviderConfig`, or
readiness state. The stale-search state and implementation are not changed in
this capability.
