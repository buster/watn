# watn Improvement Handoff Plan

Handoff snapshot: 2026-08-10

This file is the working handoff for the next agent. It describes the current
repository state, the decisions already made, the exact verification commands,
and the remaining implementation work.

## Current Repository State

- Repository: `/home/sebastian/projects/watn`
- Branch: `main`
- Worktree before this handoff edit: clean
- Current worktree: only the intentional uncommitted `PLAN.md` update
- Tracking branch: `origin/main`
- Local branch state: 6 commits ahead of `origin/main`
- Push state: the six commits have not been pushed
- Active givn change: `isolate-test-transport`
- Active change status: 24/25 tasks complete
- Active change review artifact: not yet created
- Active change archive state: not archived
- Current package version: `0.1.2` in `Cargo.toml`
- Current CLI version: still hardcoded as `0.1.0` in `src/main.rs`

Do not push unless explicitly requested. Do not amend the existing commits.

## Required Session Start

Run this before exploring or editing:

```text
givn instructions
```

For the active change, run:

```text
givn status --change isolate-test-transport
```

The project uses givn. The required lifecycle is:

```text
new -> propose -> spec -> design -> design-review -> tasks -> implement -> review -> archive
```

Keep exactly one active change. Complete one scenario at a time. Use RED,
GREEN, REFACTOR, and one atomic scenario commit. Record the commit hash in
`tasks.md` immediately after the scenario commit. Do not batch-check task
boxes.

## Existing Commits

The current local branch contains these commits for the first change:

| Commit | Meaning | Status |
|---|---|---|
| `e0dd980` | Provider readiness ignores test routing setting | Current scenario commit |
| `0554516` | Initial release-scoped transport scenario | Superseded by `93b0343`; do not treat its release matrix as current design |
| `16692a8` | Debug test-support routing uses isolated provider | Current scenario commit |
| `93b0343` | Corrected normal scenario to debug-only and shared-cache binaries | Current scenario commit |
| `2df8266` | Missing and whitespace override fallback | Current scenario commit |
| `3d7282a` | Clippy cleanup and final verification evidence | Current follow-up commit |

The relevant current implementation is represented by `93b0343`, `2df8266`,
and `3d7282a`. Commit `0554516` remains in history but its release-focused
scenario was deliberately replaced after the user requested an efficient
debug-first implementation.

## Baseline Contracts

These contracts must remain true unless a later change explicitly changes them
through a reviewed proposal and specification:

- Commands and generated command content go to stdout.
- Metadata, prompts, spinners, setup guidance, and diagnostics go to stderr.
- LiteLLM is a model-discovery service only. It must never replace the active
  chat-completion provider.
- `watn models` changes model tiers without replacing the active provider.
- `watn setup` and implicit first-use setup save a valid provider before the
  first model-catalog request.
- An absent thinking-tier reasoning value retains the existing `high` default.
- Empty and unknown persisted reasoning values disable reasoning.
- The test endpoint seam is not available in normal or release-profile builds.
- A release binary is currently dynamically linked. Do not claim universal
  static deployment without producing and verifying static artifacts.
- Saved literal credentials and exact environment references are authoritative.
- A missing saved environment reference is an authentication error and does
  not fall through to another environment variable.

## Change 1: Isolate Test Transport

### Status

Production and debug test coverage are implemented. The active change is not
archived because one verification task remains unchecked and `review.md` has
not been written.

The remaining unchecked task is the repository-wide formatting command:

```text
cargo fmt --all -- --check
```

It currently fails because of pre-existing formatting drift in unrelated test
step files. The current change did not mass-format those files because doing so
would add unrelated formatting noise. The other final checks pass.

The release-profile runtime test is intentionally deferred to
`release-truth-and-repository-cleanup`. The production source guard is already
strong enough to compile the override branch only for
`test-support + debug_assertions`.

### Implemented Production Changes

- `Cargo.toml` declares:

  ```toml
  [features]
  default = []
  test-support = []
  ```

- `src/provider/transport.rs` reads `WATN_TEST_ENDPOINT_OVERRIDE` only under:

  ```rust
  #[cfg(all(feature = "test-support", debug_assertions))]
  ```

- The negated compile-time branch returns the configured endpoint for normal
  debug builds and all release-profile builds.
- URL construction remains separate from configuration loading, persistence,
  readiness, and endpoint display.
- Unit coverage exists for the debug test-support override, whitespace fallback,
  and default-feature override isolation.

### Implemented Test Harness Changes

- `tests/steps/transport_steps.rs` owns a concrete `TransportState`.
- Configured, competing, and isolated loopback provider twins are separate
  `httpmock` servers.
- Provider fixtures include a reachable `/v1` endpoint, an exact API key, and a
  default model so requests reach the provider path under test.
- Chat mocks match exact method, path, and Authorization header.
- Assertions verify response source, exact request counts, configured endpoint
  persistence, and absence of the isolated endpoint from TOML.
- `tests/steps/mod.rs::binary_from_env` requires an explicit binary path. It
  does not fall back to `target/debug/watn`.
- Existing child-process setup uses `WATN_TEST_SUPPORT_DEBUG_BIN`.
- Transport scenarios use these explicit variables:

  ```text
  WATN_DEFAULT_DEBUG_BIN
  WATN_TEST_SUPPORT_DEBUG_BIN
  ```

### Current Debug Bootstrap

The current `givn/commands.yaml` verify commands use one shared Cargo target
cache and copy the two resulting debug binaries to unique temporary paths:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) &&
trap 'rm -rf "$root"' EXIT &&
cargo build --bin watn &&
cp target/debug/watn "$root/default-debug" &&
cargo build --features test-support --bin watn &&
cp target/debug/watn "$root/test-support-debug" &&
WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
```

The E2E command is the same bootstrap with:

```text
--tags '@e2e and not @wip'
```

Do not reintroduce one `--target-dir` per feature/profile combination. That
caused every dependency to be rebuilt repeatedly and consumed approximately
16 GB in temporary directories. Temporary copy paths are sufficient to avoid
stale or overwritten child binaries.

### Current Scenarios

The active delta is:

```text
givn/changes/isolate-test-transport/specs/transport/transport.feature
```

It contains four scenarios:

- `Normal debug requests ignore test routing settings`
  - Runs only the default-feature debug binary with a non-empty override.
  - Requires the configured twin to receive the request.
  - Requires the competing twin to receive zero requests.
- `Test-support requests use isolated routing without changing saved configuration`
  - Runs the debug test-support binary with an isolated endpoint override.
  - Requires the isolated response and exact Authorization header.
  - Requires the configured endpoint to remain in persisted TOML.
- `Missing or whitespace test overrides fall back to the configured provider`
  - Runs the debug test-support binary twice in one parser-safe scenario.
  - The first child has no override.
  - The second child has a whitespace override.
  - Both must use the configured provider; aggregate configured hits must equal
    two and competing hits must equal zero.
- `Provider readiness ignores the test routing setting`
  - Is a non-E2E local readiness contract.
  - It proves readiness remains true and makes no network request.

The custom Cucumber parser in this repository does not substitute Scenario
Outline example placeholders reliably. Do not change the fallback scenario
back to a Scenario Outline. The explicit two-child invocation is intentional.

### First Change Verification Evidence

- `givn lint --change isolate-test-transport`: clean, 1 file checked, 0
  findings.
- Debug non-E2E verify command: 9 features, 44 scenarios, 240 steps passed.
- Debug E2E verify command: 11 features, 42 scenarios, 267 steps passed.
- `cargo test --all-targets --features test-support` with explicit copied
  binary paths: 15 unit tests and 86 scenarios passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo check --all-targets`: passed.
- `cargo test --doc`: passed, no doctests.
- `cargo build --release`: passed once as a compile check.
- `git diff --check`: passed.
- `cargo fmt --all -- --check`: failed on pre-existing repository-wide drift.

### Finish The First Change

The next agent should:

1. Run `givn instructions review --change isolate-test-transport`.
2. Run the required fabrication and coverage review. Do not claim coverage
   validity from the current stale coverage commands without fixing their
   explicit binary environment first.
3. Decide whether the pre-existing rustfmt drift is fixed globally or recorded
   as a known repository blocker. Do not silently format unrelated files.
4. If review passes, run the archive workflow:

   ```text
   givn archive --change isolate-test-transport
   ```

5. Confirm the archive preserves the scenario commits and the final `tasks.md`
   evidence.

### First Change Scope Corrections

The original review report grouped two items under transport isolation that are
now intentionally owned by the next change:

- Stale-search concurrency belongs to model discovery and setup correctness.
- Exact LiteLLM model-discovery routing belongs to model discovery and setup
  correctness. The permanent provider scenario currently has a step that only
  checks mock hit count and ignores the expected URL; that remains to be fixed.

Do not expand the first change again to include those items.

## Change 2: Model Discovery and Setup Correctness

Create this as the next active givn change only after the first change is
reviewed and archived:

```text
givn new model-discovery-and-setup-correctness
```

Use the full givn workflow. The scope covers report findings 1, 4, 5, 6, and 8,
plus the stale-search test defect that was explicitly deferred from change 1.

### Workstream A: Credential Source Preservation

Current defect:

- `src/models/mod.rs::run_models_result()` resolves an API key and discards
  the result before entering the TTY wizard.
- The built-in OpenRouter provider in `src/config/mod.rs` has `api_key: None`.
- `src/setup.rs::SetupWizard::from_config()` initializes an empty configuration
  credential.
- `load_catalog()` then rejects the empty credential.

Required behavior:

- With no saved OpenRouter key and `OPENROUTER_API_KEY` set, TTY
  `watn models` must discover models successfully.
- The wizard must treat the credential as environment-backed, not literal.
- The secret must not appear in terminal output or persisted TOML.
- Confirming that source persists `${OPENROUTER_API_KEY}`.
- A saved literal key remains authoritative over environment fallback.
- A saved exact environment reference remains authoritative and reports a
  missing variable as an authentication error.

Likely implementation direction:

- Preserve a credential source representation through `run_models_result()` and
  `SetupWizard` initialization.
- Distinguish `None`, literal, and complete `${VARIABLE}` references.
- Use `config::get_provider_api_key()` only for the outbound discovery secret;
  retain the source representation for persistence and UI state.
- Avoid displaying resolved secrets.

Required scenarios:

- TTY model setup with implicit built-in OpenRouter and `OPENROUTER_API_KEY`.
- Environment-backed confirmation persists the reference, not the value.
- Missing saved environment reference fails without a request.

### Workstream B: Catalog Source Resolution

Current defect:

- `Config.litellm` is parsed but production model discovery ignores it.
- `run_models_result()` uses the active provider endpoint for catalog requests.
- The permanent LiteLLM scenario in `givn/specs/providers/providers.feature`
  only checks that some mock was hit; `tests/steps/ask_steps.rs` ignores the
  expected URL.

Required behavior:

- When `[litellm]` exists, `/models`, paginated catalog, and search requests use
  the LiteLLM endpoint.
- A LiteLLM key is optional. No Authorization header is sent when absent.
- A configured LiteLLM environment reference expands at request time.
- Without `[litellm]`, discovery falls back to the selected provider endpoint.
- Chat completions always use the selected provider endpoint, never LiteLLM.
- The active provider draft and catalog source remain separate.
- Exact URL, method, path, query, and Authorization assertions must be used.

Likely implementation direction:

- Add one catalog-source resolver used by non-TTY model setup and the shared
  wizard.
- Pass endpoint and optional credential explicitly into model-list functions.
- Keep `LiteLLMConfig` as a production-consumed configuration type.
- Update `should_query_models_at()` to assert the supplied URL rather than only
  checking `mock.hits() > 0`.
- Add exact model search and pagination URL assertions where applicable.

Required scenarios:

- LiteLLM configured with a key uses exact `/models` URL and Authorization.
- LiteLLM configured without a key uses exact `/models` URL without auth.
- Provider discovery is used when LiteLLM is absent.
- Chat requests remain on the active provider when LiteLLM is configured.
- Search and pagination use the correct catalog source.

### Workstream C: Partial Save Through The Real Wizard

Current defect:

- Documentation claims the provider is saved before model discovery.
- `apply_result()` currently persists only after the wizard returns a complete
  result.
- Catalog failure in `move_next()` returns to the API-key page but does not save
  the confirmed provider.
- The current catalog-failure step bypasses the real wizard by calling
  `save_provider_draft()` directly.

Required behavior:

- Provider setup saves only after valid credential confirmation.
- Full setup and implicit onboarding save the provider before the first catalog
  request.
- Catalog failure leaves the provider persisted and tiers unchanged.
- Cancellation before credential confirmation does not write.
- Cancellation after credential confirmation preserves the provider.
- No original chat request is sent after setup failure or cancellation.
- `watn models` changes tiers without replacing provider or LiteLLM settings.

Required test correction:

- Drive catalog failure through the actual unified wizard or a reviewed seam
  that exercises the same persistence boundary.
- Do not use a direct `save_provider_draft()` call as the primary simulation of
  automatic onboarding.
- Assert terminal/CLI behavior first and config persistence second.

### Workstream D: Reasoning Defaults And Persistence

Current defects:

- Non-TTY `watn models` creates three empty reasoning strings and overwrites
  existing reasoning configuration.
- `ModelReasoning.default_enabled` is parsed but ignored.
- `TierReasoning::effort()` forwards arbitrary values such as `bogus`.
- Reasoning metadata behavior is missing tests for `minimal`, mandatory,
  disabled defaults, and unknown strengths.

Required policy:

- Valid strengths are `off`, `low`, `minimal`, `medium`, and `high`.
- Empty and unknown persisted values resolve to no reasoning.
- A non-mandatory model with `default_enabled = false` defaults to `off`.
- A mandatory model cannot select `off`.
- A valid `default_effort` is preferred when enabled and supported.
- Otherwise select the first valid supported effort.
- Unknown supported efforts are ignored.
- Non-TTY model assignment preserves existing reasoning unless a valid model
  default replaces it.
- No empty reasoning strings are serialized.

Likely implementation direction:

- Centralize parsing and resolution in a small pure policy function.
- Reuse the policy in TTY wizard synchronization and non-TTY selection.
- Keep the thinking-tier absent-value compatibility default explicit.
- Add unit tests for policy boundaries plus Gherkin scenarios for observable
  request bodies and persisted TOML.

Required scenarios:

- Disabled default selects `off` even when a default effort is present.
- Mandatory reasoning excludes `off`.
- `minimal` is persisted and sent.
- Unknown configured strength sends no reasoning.
- Non-TTY model selection never writes empty strings.
- Existing reasoning survives model selection when no valid replacement exists.

### Workstream E: Stale Search Concurrency

Current defect:

- `tests/steps/ask_steps.rs` waits for the slow search before starting the fast
  search.
- `search_query_delays` is populated but does not control actual overlap.
- The current scenario can pass even if the generation guard is removed.

Required behavior:

- Start the slow query and fast query before either result is fully applied.
- The fast/newest result becomes visible.
- A late slow/older result cannot replace it.
- The assertion checks exact final IDs: includes the fast result and excludes
  the stale result.
- Search workers are cleaned up before scenario exit.

Likely implementation direction:

- Add a deterministic barrier or channel to the test twin.
- Dispatch both real search operations concurrently.
- Apply results through the same generation guard as production.
- Remove `search_query_delays` if the corrected test no longer needs it.

## Change 3: Incremental SSE Rendering

Create after change 2 is archived:

```text
givn new incremental-sse-rendering
```

This covers report finding 2, usage-only parsing from finding 7, and the output
and spinner coverage gaps.

### Provider API

Current defect:

- `src/provider/openai_compat.rs` calls `response.bytes()` and buffers the full
  response before parsing.
- `src/provider/mod.rs::Provider` returns only a final `StreamingResponse`.
- `src/main.rs` prints only after the provider returns.

Required behavior:

- Parse SSE incrementally from the blocking response reader.
- Emit content and reasoning events as they arrive.
- Accumulate the same content for final metadata, verbose output, and `-x`.
- Return final model, usage, elapsed time, accumulated content, and reasoning.
- Propagate mid-stream transport errors after preserving any already-visible
  output and cleaning the spinner.

Recommended design:

- Keep `reqwest::blocking`; do not add async solely for streaming.
- Use a synchronous event sink or callback owned by the single CLI consumer.
- Parse complete SSE lines/events with a buffered reader.
- Do not introduce a worker channel unless it is required by a concrete
  concurrency need.

### SSE Parsing Rules

- Handle `data:` lines and `[DONE]`.
- Ignore blank and non-data lines.
- Tolerate malformed nonessential JSON events without crashing the whole stream.
- Extract `content` and `reasoning` from choice deltas.
- Extract `usage` from the top-level event even when `choices` is empty.
- Extract the response model from the top-level event independently of choices.
- Measure elapsed time from the first received stream event.
- Preserve correct cost and tok/s when usage appears in a final usage-only event.

### CLI Output Rules

- Start the spinner before request execution.
- Clear or stop the spinner when the first content token arrives.
- Flush content to stdout immediately.
- Print final metadata only after stream completion.
- Never print the complete command a second time after incremental output.
- Keep reasoning on stderr and only print it under `-v`.
- Prompt for `-x` only after the complete command is received.

### Required Tests

- A local provider flushes one SSE event, waits, and then flushes the final
  event. The test observes the first token before the delayed response ends.
- Usage-only final event produces non-zero cost/tok-s values when configured.
- Reasoning and content are emitted separately.
- `[DONE]` terminates cleanly.
- Partial network reads are parsed correctly.
- Malformed nonessential SSE lines are tolerated.
- Mid-stream failure returns a non-zero status and cleans the spinner.
- Spinner startup, worker lifecycle, cleanup, and Drop are covered where
  observable.
- Raw TTY confirmation is tested separately from piped stdin confirmation.

## Change 4: Release Truth And Repository Cleanup

Create after change 3 is archived:

```text
givn new release-truth-and-repository-cleanup
```

This covers findings 9 and 10, documentation drift, coverage command repair,
and remaining dead-code candidates.

### Version

- Replace the hardcoded `0.1.0` in `src/main.rs` with Cargo package metadata.
- Make the version scenario assert the package version from `Cargo.toml` or
  equivalent runtime metadata.
- Do not bump `0.1.2` unless a release is explicitly being prepared.

### Deployment Truth

- Current `cargo build --release` output is dynamically linked.
- Update `docs/arc42/07-deployment-view.md` to state target-dependent runtime
  library requirements.
- Add release verification using `file` and `ldd`.
- If static artifacts become a requirement, make that a separate release
  engineering decision involving musl, TLS, compression, and CI artifact
  verification.

### Documentation Reconciliation

Update the active Arc42 and README claims for:

- Incremental versus buffered streaming.
- LiteLLM discovery-only behavior.
- Actual shared setup wizard behavior.
- Actual PTY helper names.
- Ctrl-R rather than plain `r` for reasoning focus.
- Four reasoning strengths plus `minimal`.
- stdout command output and stderr metadata/prompt behavior.
- Config-only XDG storage rather than an unimplemented data directory.
- Debug-only test-support routing and deferred release verification.
- Historical status of archived Arc42 snapshots.
- One authoritative coverage command and current measured values.

### Coverage Commands

The current `coverage` commands in `givn/commands.yaml` still invoke
`cargo llvm-cov` without exporting the explicit binary copy paths now required
by `tests/steps/mod.rs`. Fix this before relying on coverage:

- Build instrumented default and test-support debug copies in the shared target
  cache.
- Export `WATN_DEFAULT_DEBUG_BIN` and `WATN_TEST_SUPPORT_DEBUG_BIN` to those
  copies during the Cucumber run.
- Use collision-safe `LLVM_PROFILE_FILE` paths for the runner and child CLI.
- Confirm a known production path has non-zero coverage.
- Do not report stable-toolchain branch coverage as meaningful when it emits a
  `0/0` denominator.

### Dead Code And Hygiene

- Keep `LiteLLMConfig` after adding its production consumer.
- Keep `ModelReasoning.default_enabled` after adding its behavior.
- Remove provider setup result wrappers only after confirming there are no
  external library consumers. This repository is currently structured as a
  binary application, but public modules exist in `src/lib.rs`.
- Remove the unused `_config` parameter from `build_registry()`.
- Reassess whether `ProviderRegistry` is useful for one active provider.
- Remove write-only fields from `WatnWorld` after their scenarios are corrected.
- Remove obsolete helper names and archived documentation claims.
- Decide separately whether to format the entire repository. Avoid mixing a
  repository-wide rustfmt rewrite into behavioral commits.

## Verification Commands

### Debug Change Commands

Use the current shared-cache bootstrap. Do not use four isolated target dirs:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) &&
trap 'rm -rf "$root"' EXIT &&
cargo build --bin watn &&
cp target/debug/watn "$root/default-debug" &&
cargo build --features test-support --bin watn &&
cp target/debug/watn "$root/test-support-debug" &&
WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
```

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) &&
trap 'rm -rf "$root"' EXIT &&
cargo build --bin watn &&
cp target/debug/watn "$root/default-debug" &&
cargo build --features test-support --bin watn &&
cp target/debug/watn "$root/test-support-debug" &&
WATN_DEFAULT_DEBUG_BIN="$root/default-debug" \
WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
```

For `cargo test --all-targets`, export the same two binary paths. Running it
without those variables now fails intentionally because stale binary discovery
is prohibited.

### Static Checks

```text
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test --doc
cargo build --release
git diff --check
```

Run `cargo fmt --all -- --check` and classify the result. Current known result:
it fails on unrelated pre-existing test formatting drift.

### Release Verification Later

The next release-focused change should run once, not as part of every debug
scenario:

```text
cargo build --release
cargo build --release --features test-support
file target/release/watn
ldd target/release/watn
```

The release feature build must still ignore `WATN_TEST_ENDPOINT_OVERRIDE` due
to `debug_assertions` being false.

## Handoff Rules

- Read `givn instructions` before acting.
- Inspect `givn status --change <id>` before editing an active change.
- Preserve user changes and never reset or checkout unrelated work.
- Use `apply_patch` for manual edits.
- Use one active change at a time.
- Use one scenario commit for RED, GREEN, and REFACTOR.
- Do not amend existing commits.
- Do not push unless the user explicitly requests it.
- Keep secrets out of diagnostics, persisted test output, and commits.
- Prefer the smallest correct implementation. Do not add compatibility layers
  without a concrete persisted-data or external-consumer requirement.
