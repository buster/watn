# Setup Refactoring Tasks

## Setup

- [x] Reuse the configured `cucumber-rs` runner in `tests/features_runner.rs`.
- [x] Confirm strict mode is enabled with `.fail_on_skipped()` and
  `.max_concurrent_scenarios(1)`.
- [x] Confirm `verify.command` runs `.feature` files from `givn/specs/` and the
  active change with `not @wip and not @e2e`.
- [x] Confirm `verify.e2e_command` is a strict subset using
  `@e2e and not @wip`.
- [x] Existing strictness proof: the runner currently rejects undefined or
  pending steps through `fail_on_skipped`; no empty step body is accepted.

The existing runner configuration is the exact invocation in
`givn/commands.yaml`. The current normal run reports 91 scenarios and 526
steps; the E2E filter reports 36 scenarios and 209 steps. The E2E count is
strictly smaller and excludes all non-E2E scenarios. The new change's remaining
terminal-interaction scenarios remain `@wip` until their strict bindings are
added.

## Non-E2E Scenarios

### Non-interactive first use requires explicit setup even with a detected credential

- [ ] RED: remove only this scenario's `@wip`; run the named scenario and capture the strict failure.
- [ ] GREEN: add real subprocess assertions for status 1, `watn setup` stderr guidance, empty stdout, no TUI/config/catalog/chat side effects. Production files: `src/config/mod.rs`, `src/main.rs`, and setup guidance tests.
- [ ] REFACTOR: rerun the named scenario and preserve the no-side-effect contract.
- [ ] COMMIT: not yet committed.

### The unified setup command replaces focused commands and selection overrides

- [ ] RED: target this scenario only and capture parser/completion failure before implementation.
- [ ] GREEN: assert removed commands/options are rejected and retained tier selectors remain valid. Production files: `src/main.rs`.
- [ ] REFACTOR: rerun the named scenario and inspect generated completion output.
- [ ] COMMIT: not yet committed.

### Removed environment selection variables do not override persisted configuration

- [ ] RED: target this scenario only and capture the old overlay behavior.
- [ ] GREEN: assert `WATN_PROVIDER` and `WATN_MODEL` do not alter persisted provider/model selection. Production files: `src/config/mod.rs`, `src/config/env.rs`, `src/main.rs`.
- [ ] REFACTOR: rerun with both variables present and inspect the exact request target/model.
- [ ] COMMIT: not yet committed.

## E2E Scenarios

### Interactive first use reviews a detected credential before saving

- [ ] RED: target the scenario in the E2E runner and capture undefined-step failure.
- [ ] GREEN: drive the PTY through all four topics and assert the variable-only credential, one Finish save, stderr retry guidance, empty stdout, and zero original requests. Production files: `src/config/mod.rs`, `src/setup.rs`, `src/main.rs`.
- [ ] REFACTOR: rerun the PTY scenario and audit output for the resolved secret.
- [ ] COMMIT: not yet committed.

### First use without a credential shows a missing recommendation

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert the OpenRouter recommended endpoint/variable, missing state, blocked Finish, and absent config before Finish. Production files: `src/setup.rs`, `src/config/mod.rs`.
- [ ] REFACTOR: rerun at the narrow layout.
- [ ] COMMIT: not yet committed.

### Multiple discovered credentials require an explicit selection

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert deterministic separate choices, no implicit selection, and no secret values. Production files: `src/config/env.rs`, `src/setup.rs`.
- [ ] REFACTOR: rerun with allowlist-order permutations.
- [ ] COMMIT: not yet committed.

### A deliberately named credential variable persists only its reference

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert Custom setup stores `${CUSTOM_LLM_TOKEN}` and never the resolved value. Production files: `src/setup.rs`, `src/config/mod.rs`.
- [ ] REFACTOR: rerun with a literal credential path to confirm masking.
- [ ] COMMIT: not yet committed.

### A legacy commented template is existing configuration

- [ ] RED: target the scenario and capture the old auto-init behavior.
- [ ] GREEN: assert the existing comment-only file bypasses first-run setup and the implicit request is sent. Production files: `src/config/mod.rs`, `src/main.rs`.
- [ ] REFACTOR: rerun with a physically absent path and compare both branches.
- [ ] COMMIT: not yet committed.

### Contextual help remains visible at every supported width

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert all four help sections beside settings at 120 columns and below settings at 80 columns. Production files: `src/setup.rs`.
- [ ] REFACTOR: rerun buffer and PTY layout checks.
- [ ] COMMIT: not yet committed.

### Model roles are reviewed together after a provider change

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert three visible role rows, `Needs attention` invalidation, and explicit re-review before Finish. Production files: `src/setup.rs`.
- [ ] REFACTOR: rerun with provider endpoint reverted and retained roles.
- [ ] COMMIT: not yet committed.

### Manual roles may finish with an unverified catalog warning

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert manual IDs, warning, Finish availability, and persisted `off` reasoning. Production files: `src/setup.rs`, `src/config/mod.rs`.
- [ ] REFACTOR: rerun with authentication, transport, and empty-catalog failures.
- [ ] COMMIT: not yet committed.

### Review is the only configuration commit boundary

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert no file before Finish, complete Review summary, and no file after discard. Production files: `src/config/mod.rs`, `src/setup.rs`.
- [ ] REFACTOR: count writes through an isolated filesystem seam.
- [ ] COMMIT: not yet committed.

### Cancelling an existing setup keeps its configuration byte-for-byte unchanged

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert loaded origins, edit/cancel behavior, unchanged bytes, and zero request side effects. Production files: `src/setup.rs`.
- [ ] REFACTOR: rerun with literal and environment-backed saved credentials.
- [ ] COMMIT: not yet committed.

### Finish reconciles shell marker blocks without persisting shell state in TOML

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert marker-derived selections, removal/install, unrelated byte preservation, and no TOML shell fields. Production files: `src/shell_shortcut.rs`, `src/shell_completion.rs`, `src/setup.rs`.
- [ ] REFACTOR: rerun with malformed and missing targets.
- [ ] COMMIT: not yet committed.

### Shell failure reports partial completion after configuration commits

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert config commit, successful target retention, failed target diagnostics/retry guidance, and non-zero status. Production files: `src/setup.rs`, `src/shell_shortcut.rs`, `src/main.rs`.
- [ ] REFACTOR: rerun with failure ordering reversed.
- [ ] COMMIT: not yet committed.

### OpenAI setup uses the explicit identity and credential mapping

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert the OpenAI endpoint, `OPENAI_API_KEY` provenance, persisted `openai` provider, and absent secret. Production files: `src/provider/setup.rs`, `src/config/env.rs`, `src/setup.rs`.
- [ ] REFACTOR: rerun with `WATN_OPENAI_API_KEY` and endpoint edits.
- [ ] COMMIT: not yet committed.

### Finish preserves supported configuration outside the setup draft

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert defaults, provider `default_model`, pricing, LiteLLM, and unrelated providers survive Finish. Production files: `src/config/mod.rs`, `src/setup.rs`.
- [ ] REFACTOR: rerun after a provider replacement.
- [ ] COMMIT: not yet committed.

### Setup catalog discovery honors the configured LiteLLM source

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert exact LiteLLM `/models` routing, no chat-provider catalog request, and separate Review labels. Production files: `src/setup.rs`, `src/models/list.rs`.
- [ ] REFACTOR: rerun with optional and environment-backed LiteLLM credentials.
- [ ] COMMIT: not yet committed.

### Ctrl-C during catalog discovery discards the setup draft

- [ ] RED: target the scenario and capture strict failure.
- [ ] GREEN: assert status 130, terminal restoration, no config, and no shell-file mutation. Production files: `src/setup.rs`.
- [ ] REFACTOR: rerun with delayed success and delayed failure responses.
- [ ] COMMIT: not yet committed.

## Full Verification

- [x] `verify.command` passes the current non-E2E suite: 91 scenarios and 526 steps.
- [x] `verify.e2e_command` passes the current E2E suite: 36 scenarios and 209 steps; active change E2E scenarios remain WIP.
- [x] `cargo fmt --all -- --check` passes.
- [x] `cargo check --locked --all-targets` passes.
- [x] `cargo clippy --locked --all-targets -- -D warnings` passes.
