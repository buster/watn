# Design: normalize-fragment-e2e-inventory

## Summary

This change reclassifies 15 existing fragment scenarios from the e2e scope to the regular scope and records 11 distinct actions in the fragment interaction inventory. It adds no production code, no step definitions, and no scenario text. The mechanism is six delta feature files carrying `@givn.modified` scenarios without `@e2e`; archive replaces the permanent scenarios by title.

## Reclassification

Every retagged scenario is a variant, error case, or output-affecting flag combination of an action that keeps one `@e2e` scenario. The retagged scenario keeps its Given/When/Then text and runs in the regular suite.

| Capability | Retained `@e2e` action | Scenario moved to regular | Classification |
|---|---|---|---|
| ask | ask a question and read the generated command | Explicit tier -1 uses the small/fast model | output-affecting flag variant |
| ask | ask a question and read the generated command | Tier -2 uses the normal model | output-affecting flag variant |
| ask | ask a question and read the generated command | Tier -3 uses the thinking/reasoning model | output-affecting flag variant |
| ask | ask a question and read the generated command | Cost is displayed when pricing is configured | output variant |
| ask | ask a question and read the generated command | Tokens/second is displayed after response completes | output variant |
| ask | ask a question and read the generated command | Ask via stdin pipe | input-channel variant (D1) |
| ask | execute a generated command after Enter confirmation | Execute flag with explicit "y" confirmation | confirmation variant |
| ask | execute a generated command after Enter confirmation | Execute flag with "n" answer skips execution | refusal variant |
| cancel-running-completion | cancel a completion while waiting for streamed output | One Ctrl+C cancels a completion waiting for a connection | timing variant |
| config | configure model tiers in the config file | Environment variable overrides config file | precedence variant |
| config | configure model tiers in the config file | CLI flag overrides environment variable | precedence variant |
| incremental-sse-rendering | read command text before a delayed stream completes | Verbose streaming keeps reasoning on stderr and command text on stdout | verbose flag variant |
| incremental-sse-rendering | read command text before a delayed stream completes | A mid-stream failure preserves visible content and exits unsuccessfully | error case |
| incremental-sse-rendering | confirm on a raw terminal after the complete command arrives | Piped confirmation remains available after streamed output | input-channel variant (D1) |
| transport | run a test-support request against an isolated provider | Missing or whitespace test overrides fall back to the configured provider | fallback variant |

The retained e2e scenarios are:

| Capability | Retained scenario |
|---|---|
| ask | Ask with default tier returns a copy-pasteable command |
| ask | Execute flag prompts for confirmation |
| cancel-running-completion | One Ctrl+C cancels a completion waiting for streamed output |
| config | Configure model tiers in config file |
| config | Model pricing configured for cost display |
| incremental-sse-rendering | Command text appears before a delayed stream completes |
| incremental-sse-rendering | Raw terminal confirmation happens after the complete command arrives |
| release-truth | Version flag reports the package version |
| release-truth | Repeated revisions of one change yield one changelog entry |
| transport | Normal debug requests ignore test routing settings |
| transport | Test-support requests use isolated routing without changing saved configuration |

Expected counts: before archive the runner loads the permanent scenarios and the 15 delta copies, so the regular scope runs 290 scenarios and the e2e scope 92. After archive replaces the 15 permanent scenarios, the regular scope runs 290 and the e2e scope 77; the 367-scenario total is unchanged.

## Decisions

Decides D1: piped ask and piped confirmation are input-channel variants of their actions, not distinct actions — realized in the ask and incremental-sse-rendering delta files and in the 11-row fragment inventory.

## Test runner and strict mode

- `verify.command`: `./run-tests.sh`.
- `verify.e2e_command`: `./run-tests.sh --e2e`.
- Scope selection: `run-tests.sh` sets `GIVN_RESULT_SCOPE=regular` with the tag filter `not @wip and not @e2e`, and `GIVN_RESULT_SCOPE=e2e` with `@e2e and not @wip`.
- Single-scenario run command: `./run-tests.sh --name "<scenario title>"`.
- Runner: `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()` at `tests/features_runner.rs:211`; an undefined or pending step fails the build. The change introduces no step phrase, so the existing proof stands unchanged.

### Change verification adapter

`givn check tasks` sets `GIVN_FEATURES` to the change's `specs` directory, `GIVN_RESULT_SCOPE=change`, and `GIVN_RESULT_FILE`. The adapter must honor all three:

- `tests/features_runner.rs` collects from `GIVN_FEATURES` when it is set, instead of the permanent corpus plus every active change. The change-scope execution therefore runs exactly the 15 delta scenarios.
- `run-tests.sh` preserves an incoming `GIVN_RESULT_SCOPE=change` and selects the tag filter `not @wip`, so delta `@e2e` scenarios are included and the written result names the `change` scope. The regular and e2e branches keep their existing filters.
- The archive path is unchanged: archive sets `GIVN_FEATURES=givn/specs` and a regular or e2e scope, and `GIVN_ARCHIVE_ONLY` still prevents change-spec collection when the path is not supplied.

Verification: `givn check tasks --change normalize-fragment-e2e-inventory` must write `verification/tasks-verification.json` with 15 change-scope scenarios and exit 0. The regular and e2e suites must still report 290 and 92 scenarios before the archive merge.

## Step Definitions

None. Every modified scenario reuses an existing binding; no delta step file is created.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| ask a question and read the generated command | Ask with default tier returns a copy-pasteable command | CLI | `cucumber-rs` step definitions invoke the built `watn` binary as a subprocess and assert stdout |
| execute a generated command after Enter confirmation | Execute flag prompts for confirmation | CLI | PTY step definitions run `watn -x` and send Enter to the terminal |
| cancel a completion while waiting for streamed output | One Ctrl+C cancels a completion waiting for streamed output | CLI | PTY step definitions run `watn` against a holding twin and send Ctrl+C |
| configure model tiers in the config file | Configure model tiers in config file | CLI | step definitions write an isolated config file, run `watn`, and assert the model used by the local provider twin |
| configure model pricing for cost display | Model pricing configured for cost display | CLI | step definitions write pricing into the isolated config, run `watn`, and assert the printed cost |
| read command text before a delayed stream completes | Command text appears before a delayed stream completes | CLI | PTY step definitions run `watn` against a delayed streaming twin and read the terminal output |
| confirm on a raw terminal after the complete command arrives | Raw terminal confirmation happens after the complete command arrives | CLI | PTY step definitions run `watn -x`, read the streamed command, and send Enter |
| read the package version from the version flag | Version flag reports the package version | CLI | step definitions run the built release binary with `--version` and assert the output |
| read one changelog entry for repeated revisions of one change | Repeated revisions of one change yield one changelog entry | CLI | step definitions run the changelog generation over a fixture range and assert the emitted entry |
| run a normal debug request with test routing settings present | Normal debug requests ignore test routing settings | CLI | step definitions run the default debug binary against two local twins and assert which twin received the request |
| run a test-support request against an isolated provider | Test-support requests use isolated routing without changing saved configuration | CLI | step definitions run the test-support binary against the isolated twin and assert the request target and persisted configuration |

## Use-case traceability

| Fragment capability | Normalized action | E2E evidence |
|---|---|---|
| ask | ask a question | Ask with default tier returns a copy-pasteable command |
| ask | execute a generated command | Execute flag prompts for confirmation |
| cancel-running-completion | cancel a running completion | One Ctrl+C cancels a completion waiting for streamed output |
| config | configure model selection | Configure model tiers in config file |
| config | configure pricing | Model pricing configured for cost display |
| incremental-sse-rendering | read incremental output | Command text appears before a delayed stream completes |
| incremental-sse-rendering | confirm after streamed output | Raw terminal confirmation happens after the complete command arrives |
| release-truth | read the version | Version flag reports the package version |
| release-truth | generate the changelog | Repeated revisions of one change yield one changelog entry |
| transport | route a normal debug request | Normal debug requests ignore test routing settings |
| transport | route a test-support request | Test-support requests use isolated routing without changing saved configuration |

Persona impact: `terminal-developer--interactive` remains the review lens. The change alters test classification only; no rendered screen, terminal UI, or CLI output changes.

## Visual Design Contract

None. The change introduces or visibly changes no screen and no CLI output.

## External boundary cases

No external boundary is added or changed. The retagged mid-stream failure, transport fallback, cancellation timing, and confirmation variants remain in the regular suite, so their adversarial coverage is retained. The retained e2e scenarios keep the existing local provider twins, PTY sessions, and subprocess boundary.

## Local runnability

`./run-tests.sh` runs the whole suite locally. Provider interactions use in-process `httpmock` twins; terminal scenarios use a local PTY; no external service, container, or network access is required.

## Coverage process boundaries

No new production process is introduced. The existing coverage configuration is unchanged: `./measure-coverage.sh` instruments the `watn` binary and the `features_runner` test binary, and `./merge-coverages.sh` merges the non-e2e and e2e Cobertura reports. Measured population and exclusions stay as configured in `givn/commands.yaml`.

## Black-box-first

No internal test is added. Every scenario drives the built `watn` binary through its real CLI or PTY interface; the retagged scenarios remain black-box scenarios in the regular scope.

## Risks and verification

- The archive merge replaces scenarios by title. Post-archive verification must show `givn lint` clean, e2e 77, and regular 290; a count drift means a title mismatch or a missing delta.
- The runner loads permanent and delta specs before archive, so the e2e count stays 92 until the merge. Review records the pre-merge counts and treats the archive receipt as the final evidence.
- No scenario text is edited; a textual diff in any retagged scenario is a defect.
