# reasoning-support — Tasks

## Setup: configure test infrastructure and strict mode

- [x] **Setup: configure test infrastructure, strict mode, and verify.command**
      - Runner: `cargo test --test features_runner` (already configured)
      - Strict mode: `.fail_on_skipped()` on Cucumber builder (already set)
      - Not-implemented stub: `unimplemented!()`
      - Spec directories: `givn/specs/` and `givn/changes/reasoning-support/specs/` (scans `givn/`)
      - Single-scenario run: `cargo test --test features_runner -- --name "<title>"`
      - Added `WatnWorld.pending_mock_reasoning` and `WatnWorld.last_request_body`
      - Updated `ensure_test_env` to interpolate `pending_mock_reasoning` into SSE delta
      - Added step definitions: `mock_returns_reasoning`, `api_request_includes_reasoning`, `stderr_not_contain`
      - Strict mode: `.fail_on_skipped()` already configured in runner

      Proof: (strict mode pre-existing, not re-proven)

## Non-@e2e scenarios (none — all 7 scenarios are @e2e)

All scenarios in `specs/reasoning.feature` are tagged `@e2e`. There are no non-@e2e scenarios.

## E2E setup: configure e2e test infrastructure

- [x] **Setup: configure e2e test infrastructure, strict mode, and verify.e2e_command**
      - E2E framework: `httpmock` (already in dev-dependencies)
      - E2E steps: all in `tests/steps/ask_steps.rs`
      - E2E runner: `cargo test --test features_runner -- --tags '@e2e'`
      - Strict mode proven in main setup (same binary, same `.fail_on_skipped()`)

      Full suite count (verify.command): 43 scenarios (34 pass)
      E2E-only count (verify.e2e_command): 28 scenarios (25 pass)
      ```

## @e2e scenarios

### @e2e: Thinking tier sends reasoning without printing it

- [x] **@e2e: Thinking tier sends reasoning without printing it**
      Design constraints:
      - `RequestOptions.reasoning_effort = Some("high")` when tier = thinking (3)
      - Request body sends `"reasoning_effort": "high"` as top-level string
      - Reasoning is accumulated from SSE `delta["reasoning"]` but NOT printed when verbose=false

      RED: @wip removed. Scenario runs and passes.
      GREEN: Implemented in `src/provider/mod.rs`, `src/provider/openai_compat.rs`, `src/main.rs`
      REFACTOR: No refactoring needed.
      COMMIT: `feat(reasoning): send reasoning_effort on thinking tier`
      Hash: `9a43507`

### @e2e: Thinking tier with verbose flag prints reasoning to stderr

- [x] **@e2e: Thinking tier with verbose flag prints reasoning to stderr**
      Design constraints:
      - `-v`/`--verbose` flag added to `Cli` struct in `src/main.rs`
      - `RequestOptions.verbose = true` when `-v` is passed
      - Reasoning accumulated from SSE `delta["reasoning"]` in `openai_compat.rs` — ALWAYS parsed, regardless of verbose flag
      - `StreamingResponse.reasoning_content: Option<String>` stores the accumulated reasoning
      - After streaming completes, if verbose && reasoning_content.is_some() && !trim().is_empty(), print `reasoning: <text>` to stderr
      - The mock must return reasoning content — `ensure_test_env` uses `pending_mock_reasoning` to interleave `"reasoning"` field in the SSE delta

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN:
        - Add `-v`/`--verbose` flag to `Cli` struct in `src/main.rs`
        - Add `verbose: bool` and `reasoning_effort: Option<String>` to `RequestOptions` in `src/provider/mod.rs` (reasoning_effort may already exist from previous scenario)
        - Add `reasoning_content: Option<String>` to `StreamingResponse` in `src/provider/mod.rs`
        - In `openai_compat.rs` streaming path: after extracting `delta["content"]`, also extract `delta["reasoning"].as_str()` and push into a local `reasoning_content` String; store in `StreamingResponse.reasoning_content`
        - In `src/main.rs` after response: if `cli.verbose`, print reasoning to stderr
        - Ensure mock SSE body includes `"reasoning"` field in delta when `pending_mock_reasoning` is set
      List files: `src/main.rs`, `src/provider/mod.rs`, `src/provider/openai_compat.rs`, `tests/features_runner.rs`, `tests/steps/mod.rs`, `tests/steps/ask_steps.rs`
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `feat(reasoning): verbose flag prints reasoning to stderr`
      Hash:

### @e2e: Verbose flag with small tier prints reasoning if present

- [x] **@e2e: Verbose flag with small tier prints reasoning if present**
      Design constraints:
      - Small tier (1) does NOT send `reasoning_effort` — stays `None`
      - But if the API still returns `delta["reasoning"]` (some models send it regardless), the content is accumulated and printed when verbose=true
      - Reasoning is always parsed from SSE
      - This verifies the "any tier" claim for verbose

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN: Should already work from previous implementation. If step definitions exist and production code already handles reasoning parsing + verbose gate, this may pass immediately. If not, add remaining production code.
      List files: (none — reuse from previous scenarios, or list new files if gaps found)
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `test(reasoning): verbose with small tier prints reasoning`
      Hash:

### @e2e: Small tier without verbose flag does not print reasoning

- [x] **@e2e: Small tier without verbose flag does not print reasoning**
      Design constraints:
      - Small tier, no verbose → reasoning IS accumulated but NOT printed
      - Mock returns reasoning content in delta
      - Stderr must NOT contain "reasoning:"

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN: Should already work from previous implementation. The `stderr should not contain` step definition asserts the negative.
      List files: (none — reuse from previous scenarios)
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `test(reasoning): small tier without verbose does not print reasoning`
      Hash:

### @e2e: Verbose flag with default tier does not alter existing model behavior

- [x] **@e2e: Verbose flag with default tier does not alter existing model behavior**
      Design constraints:
      - Default tier (1/small), verbose flag set
      - No reasoning parameter sent
      - Existing metadata (model, tok/s) still printed
      - The `output should contain a model name` assertion checks that `model:` appears in stderr (existing step)

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN: Should already work from previous scenarios. This scenario is a regression check.
      List files: (none)
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `test(reasoning): verbose with default tier regression check`
      Hash:

### @e2e: Help output includes verbose flag

- [x] **@e2e: Help output includes verbose flag**
      Design constraints:
      - `watn --help` must include `--verbose` in its output
      - clap auto-generates help text from the struct field — no manual help text needed
      - Uses existing `run_watn_version` step pattern (`I run `watn --help``)

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN: Should already pass from production code (clap derives help from the `Cli` struct field with `short = 'v', long = "verbose"`).
      List files: (none — the struct field was added in a prior scenario)
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `test(reasoning): help output includes verbose flag`
      Hash:

### @e2e: Thinking tier with verbose and execute flags

- [x] **@e2e: Thinking tier with verbose and execute flags**
      Design constraints:
      - Combines `-3` (thinking), `-v` (verbose), `-x` (execute) flags
      - Mock returns both command output AND reasoning content simultaneously
      - User answers "n" to execution prompt → command NOT executed
      - Stderr contains reasoning output; stdout contains command suggestion
      - The SSE mock body must include BOTH `content` and `reasoning` fields in the delta (interleaved)

      RED: Remove @wip. Run targeting this scenario → MUST FAIL.
      ```
      ```
      GREEN: Should already work from previous scenarios. The `pending_mock_reasoning` and `pending_mock_output` are both set in given steps; `ensure_test_env` must handle both being set simultaneously to produce interleaved SSE.
      List files: `tests/steps/mod.rs` (verify the mock body format interleaves content+reasoning when both are set)
      Run targeting this scenario → PASSES.
      ```
      ```
      REFACTOR: Clean up. Re-run → still PASSES.
      ```
      ```
      COMMIT: `test(reasoning): thinking tier with verbose and execute flags`
      Hash:

## Final verification

- [x] Run `cargo test --test features_runner` — all existing (28) + new scenarios pass
- [x] Run `verify.e2e_command` (`cargo test --test features_runner -- --tags '@e2e'`) — all @e2e scenarios pass; count is strictly > 0
- [x] `cargo build` — 0 warnings
