# Design: model-discovery-and-setup-correctness

## Domain model

### Ubiquitous language

| Term | Meaning |
|---|---|
| Provider | The configured service that receives chat-completion requests. |
| Catalog source | The service from which model metadata is listed, paged, or searched. It is LiteLLM when configured, otherwise the active provider. |
| Credential source | The saved origin of a secret: a literal value, an exact environment-variable reference, or no source. |
| Provider draft | The endpoint and credential source confirmed by the user but not necessarily followed by complete model-tier setup. |
| Model tier | One of the small, normal, or thinking model assignments. |
| Reasoning strength | One of `off`, `low`, `minimal`, `medium`, or `high`; `off` means no reasoning request field. |
| Newest search | The search generation most recently entered by the user. Older generations are stale even if they finish later. |

### Aggregate boundaries

- **Configuration aggregate**, rooted by the loaded configuration: provider records, the active provider name, LiteLLM catalog settings, model tiers, and reasoning values are saved together when a complete configuration change is committed.
- **Setup draft boundary**, rooted by the interactive wizard: endpoint, credential source, model choices, and reasoning choices are transient until the credential-confirmation boundary or final tier save is crossed.
- **Catalog request boundary**, rooted by one catalog source: all list, page, and search requests in one discovery operation use one resolved endpoint and its credential policy.
- **Search session boundary**, rooted by one picker session: each search worker carries a generation and is joined or discarded before the scenario ends.

### Invariants

- A saved literal credential is used as-is and takes precedence over all environment fallbacks. Enforced by credential-source resolution.
- A saved exact environment reference is the only source used for that provider. A missing referenced variable is an authentication error before any request. Enforced by credential expansion.
- A resolved secret is never persisted as a replacement for its source and is never included in setup diagnostics. Enforced at wizard save and request construction.
- LiteLLM, when configured, is the only catalog source for model listing, pagination, and search. The selected provider remains the only chat source. Enforced by the catalog-source resolver and provider construction.
- A provider draft is saved only after credential validation, and that save happens before the first catalog request. Enforced by the wizard transition from credential confirmation to model pages.
- A catalog failure cannot roll back a provider already confirmed, and cannot alter model tiers. Enforced by separate provider-draft and tier-save operations.
- Only the five valid reasoning strengths are persisted or sent. Unknown and empty persisted values produce no reasoning request. Enforced by the reasoning policy and request-option construction.
- Mandatory reasoning never resolves to `off`. Enforced by the shared policy used by both interactive and non-interactive model selection.
- A search result can update the picker only when its generation is still current. Search workers are joined or dropped before the test boundary exits. Enforced by the generation guard and test seam.

## Technology decisions

| Concern | Choice | Rationale |
|---|---|---|
| Language and runtime | Existing Rust toolchain and Cargo package; use the repository's current lockfile versions | No new runtime or dependency is needed for the behavior. |
| HTTP | Existing `reqwest::blocking` client | HTTP remains blocking inside search workers; the picker remains event-driven at the UI boundary. |
| Configuration | Existing TOML/Serde types with raw credential-source strings | Literal values and exact `${VARIABLE}` references must remain distinguishable at persistence time. |
| Interactive setup | Existing ratatui wizard and PTY harness | The real user boundary is the terminal; provider confirmation and catalog failure must be driven through the same wizard. |
| Reasoning policy | Small pure resolver reused by wizard, non-TTY selection, and request configuration | One policy prevents the interactive and non-interactive paths from diverging. |
| Search concurrency | Existing blocking worker threads plus generation counters, deterministic test coordination | Preserve the current synchronous HTTP model and test the stale-result boundary without introducing async application code. |
| Test runner | Existing cucumber-rs runner in `tests/features_runner.rs` | It discovers permanent and change `.feature` files, runs one scenario at a time, and is the executable specification. |

No versioned dependency or service is added. Existing dependency versions are
read from `Cargo.toml` and `Cargo.lock` during implementation rather than
redeclared here.

## Architecture impact

### Credential and catalog resolution

Add a configuration-level catalog-source resolver that returns an endpoint and
the raw credential source. Its precedence is:

  1. `[litellm]` endpoint and optional key, if present.
  2. The selected provider endpoint and provider credential source.

The resolver does not replace the active provider. It also does not resolve a
  secret until the request is about to be sent. Literal values, complete
  `${UPPER_CASE_NAME}` references, and absent sources remain distinct. A
  provider with no saved key checks `WATN_<PROVIDER>_API_KEY` and then
  `WATN_API_KEY`; provider names are uppercased and non-alphanumeric
  characters become `_`. An explicitly saved source, including a missing
  environment reference, is never replaced by fallback discovery.

`fetch_models`, `fetch_models_page`, and `search_models` receive the resolved
endpoint and an optional already-resolved key. They share request construction
so path, query, and Authorization behavior cannot differ between list, page,
and search operations. `None` means no Authorization header, which is valid
only for an optional LiteLLM key or a provider whose request path explicitly
allows an absent key.

The chat path continues to resolve the selected provider separately. It never
uses the catalog source.

### Setup persistence boundary

The wizard retains the loaded configuration and entry point. For full setup or
provider setup, the explicit credential confirmation action performs these
actions in order:

1. Validate the endpoint and credential source syntax.
2. Resolve the credential once to prove that a usable secret exists.
3. Persist the provider draft, retaining its literal or environment-backed source.
4. Request the catalog and enter the model pages.

The wizard result carries an optional provider draft. Model-only entry points
carry no provider replacement, so `watn models` changes tiers without rewriting
the provider or LiteLLM sections. Tier persistence is a separate final step.
If catalog loading fails after step 3, the process reports the error while the
saved provider remains and tiers retain their previous values. Cancellation
before step 3 writes nothing; cancellation after step 3 cannot undo the
confirmed provider.

### Reasoning policy

Implement a pure resolver around `ReasoningStrength` and `ModelReasoning`:

- Parse only `off`, `low`, `minimal`, `medium`, and `high`.
- For a non-mandatory model with `default_enabled = false`, return `off`.
- For an enabled or mandatory model, filter supported efforts through the valid
  strength parser, prefer a valid supported `default_effort`, then use the
  first valid supported effort.
  - A mandatory model never accepts `off`; if malformed metadata supplies no
    usable effort, retain a valid non-off existing choice or return a typed
    reasoning-policy error. The resolver never invents an effort.
- If model metadata supplies no valid replacement, preserve an existing valid
  reasoning value; absent or invalid persisted reasoning resolves to no
  reasoning.

The thinking-tier absent-value compatibility rule remains explicit in
`TierReasoning::effort`: absent thinking reasoning returns `high`; empty,
`off`, and unknown values return no request effort. The same valid-strength
parser is used by request construction, wizard synchronization, and
non-TTY model assignment. No empty reasoning string is constructed or
serialized.

### Search generation seam

The production picker increments a generation before dispatching each search,
checks it before and after the blocking request, and applies only matching
messages. The change's test twin will coordinate two actual search workers with
channels: the older worker starts, the newer worker starts, the newer result is
allowed to complete and apply, then the older worker is released and joined.
The test asserts exact final IDs, including the newer entered result and
excluding the stale older result, even when completion order differs. It does
not rely on a sleep-only ordering or a write-only delay map.

## Data model changes

- Keep `ProviderConfig.api_key` as the raw persisted source string.
- Add a runtime-only catalog source value containing `endpoint` and optional raw
  credential source; it is never serialized.
- Extend the setup result so provider persistence is optional for model-only
  entry points.
- Keep `LiteLLMConfig` as a production-consumed configuration type.
- Keep `ModelReasoning.default_enabled` and use it in the shared policy.
- Remove no persisted fields in this change.

## Step definitions and test seams

The cucumber-rs registry is global, so each new capability gets a distinct
step module with no duplicate expression patterns:

- `tests/steps/credentials_steps.rs`: environment-backed wizard setup and exact
  credential-source assertions.
- `tests/steps/catalog_source_steps.rs`: separate provider/LiteLLM twins,
  exact request path/query/header assertions, and chat-source separation.
- `tests/steps/setup_persistence_steps.rs`: PTY save/cancel flows and
  persistence-boundary assertions.
- `tests/steps/reasoning_policy_steps.rs`: metadata fixtures, policy outcomes,
  and persisted-request assertions not already shared by the existing ask
  steps.
- `tests/steps/search_concurrency_steps.rs`: coordinated overlapping search
  workers and cleanup assertions.

Shared subprocess, PTY, environment, and mock helpers remain in
`tests/steps/mod.rs`; existing globally registered steps are reused where their
observable contract is already exact. New step modules are declared from that
module and contain no empty or pending bodies once their scenario is enabled.

## Local runnability and digital twins

The application is a single CLI. Manual local execution is `cargo run --
<question>` or `cargo run -- models`; no application server or database must be
started. The full feature verification command builds the two debug binaries,
then invokes the Cucumber runner.

The only external dependency is an OpenAI-compatible HTTP provider or LiteLLM.
Every scenario uses an `httpmock::MockServer` bound to loopback, with one twin
for provider chat/catalog traffic and a separate twin when source separation
must be proven. No live provider, network credential, or external service is
used.

Interactive scenarios use one persistent `portable-pty` session per scenario.
The session writes actual terminal keystrokes, captures terminal output for the
primary assertion, waits for child exit, joins the reader, and is cleaned up by
the world drop handler. Non-interactive scenarios use the real built binary
with piped stdin and captured stdout/stderr. The known obstacle is that raw
terminal input cannot be tested through a pipe; the PTY helper is the named fix.

## Runner and strict mode

`verify.command` in `givn/commands.yaml` is the existing shell command that
builds `watn` and its `test-support` binary, sets `WATN_DEFAULT_DEBUG_BIN` and
`WATN_TEST_SUPPORT_DEBUG_BIN`, and runs:

```text
cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
```

It executes both `givn/specs/**/*.feature` and the active change features via
the feature collector in `tests/features_runner.rs`. The E2E command uses the
same build wrapper and runs:

```text
cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
```

Strict mode is `Cucumber::fail_on_skipped()` in the runner. Undefined or
pending steps therefore fail the command. During RED, a new step body uses
`unimplemented!("<step contract>")`; it must be replaced before removing
`@wip`. The single-scenario form of the configured command is the same build
wrapper with the final filter changed to:

```text
-- --name '<scenario title>'
```

## E2E interaction coverage matrix

The real interface for every E2E scenario is the CLI, not an internal function.
PTY is used for interactive flows; the subprocess runner is used for piped
model assignment and chat requests. Terminal/stdout assertions are primary;
TOML and mock-request checks are secondary.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| start the interactive `watn models` command and confirm model tiers | Interactive model discovery preserves an OpenRouter environment credential | CLI | `portable-pty` starts `watn models`, sends model selections and Enter, then reads terminal output |
| run `watn <question>` through a configured provider | A literal saved credential is authoritative over environment fallback | CLI | Real debug subprocess runs `watn "hello"` with isolated XDG config and environment |
| run `watn models` to discover and assign model tiers | Configured LiteLLM is used for model catalog requests | CLI | Real subprocess runs `watn models` with piped tier selections against loopback HTTP twins |
| run `watn <question>` to send a chat request after catalog discovery | LiteLLM discovery does not replace the active chat provider | CLI | Real debug subprocess first discovers a model through LiteLLM, then runs `watn "hello"` and observes the generated stdout response while the chat twin records the provider endpoint |
| start `watn setup` in a terminal and confirm or cancel provider setup | Model catalog failure after provider setup preserves the provider and sends no request | CLI | `portable-pty` drives endpoint and credential confirmation through the actual setup wizard |
| run `watn models` with tier assignments | Assigning tiers does not replace the active provider or catalog settings | CLI | Real debug subprocess runs `watn models` and observes the confirmation output |
| run `watn <question>` with a selected model tier and observe the request | Minimal reasoning is persisted and sent | CLI | Real debug subprocess runs `watn -2 "summarise the changes"` and observes successful command output |
| type overlapping searches into the model picker and observe the final suggestions | The terminal model picker displays the newest entered catalog result | CLI | Real debug subprocess runs `watn models`, enters the older query then the newer query before either result is applied, and asserts exact final suggestions plus worker cleanup |

## Implementation order

Implement one scenario at a time in this dependency order:

1. Credential source preservation and exact credential assertions.
2. Catalog source resolution and exact URL/header assertions.
3. Provider confirmation save boundary and real wizard failure/cancellation flows.
4. Shared reasoning policy and non-TTY preservation.
5. Concurrent search test seam and corrected stale-result scenario.

Each scenario follows RED, GREEN, REFACTOR, then one atomic commit recorded in
`tasks.md`. No unrelated cleanup or documentation rewrite belongs in this
change.
