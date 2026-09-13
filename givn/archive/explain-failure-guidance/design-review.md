# Design Review: explain-failure-guidance

## Decision record

A fresh grilling review of the complete planning layer found twelve defects
(F1-F12). The human chose **option A — full hardening**: apply every correction
to the artifacts rather than descope or defer any finding. This hardening pass
edited the delta spec, `design.md`, `givn/specs/use-shell/usecase.md`, the
affected arc42 chapters, and added the missing delta scenario. It did not write
`tasks.md` and did not modify production code; the design specifies the
production changes that implementation will make.

## Findings and resolutions

| # | Finding | Resolution |
|---|---|---|
| F1 | The explicit-selection scenarios used a subcommand-local `watn explain --provider ...` form that does not match the working top-level flag convention (`watn -x explain`). | Reworded the delta steps to `watn --provider missing explain '<cmd>'` and `watn --provider openrouter explain '<cmd>'`; updated design behaviour rows 9-11 and the CLI behaviour bullet. No CLI surface expansion. |
| F2 | The quick-setup scenario reused six interaction steps registered `#[when]` but typed `Then` after the first `Then`, so they could never match. | Inserted an explicit `When I accept the suggested endpoint` after the announcement `Then`, making the following `And` lines `When`-typed, matching `givn/specs/configure-interactive/quicksetup.feature:70-73`. |
| F3 | No scenario completed the setup wizard on the config-exists branch, so the rerun-hint and exit-0 promises were untested. | Added `@givn.added @wip` "An existing configuration without a usable model completes the setup wizard", asserting the rerun hint, exit 0, no card, and no provider request. The design step table names the reused catalog fixture (`tests/steps/streamlined_setup_steps.rs`) and wizard steps (`tests/steps/setup_wizard_steps.rs`). |
| F4 | Ctrl+C during the explanation fetch was specified only as "existing interrupt contract"; the post-fetch interrupt check and no-card guarantee were absent. | Design now states that `run_explain_command` re-checks the shared interrupt flag (and/or `Err(Error::Interrupted)`) after the fetch and before opening the card, exits 130, and opens no card. Added `@givn.added @wip` "Interrupting the explanation request opens no card" and named the hanging-provider pattern (`HangServer`, `tests/steps/cancel_completion_steps.rs`). |
| F5 | The proposal promised a missing-model explicit-selection error, but no scenario covered it. | Added `@givn.added @wip` "An explicit provider without a resolvable model reports its error" for `watn --provider custom explain` against `[providers.custom]` without `default_model`, asserting the configuration error and exit 1. Design row 11 and the step table name the `no default model configured` error and the fixture. The proposal promise is kept. |
| F6 | The failure table mapped network failures to exit 3, but no scenario asserted it; only the 401 exit-2 path existed. | Added `@givn.added @wip` "A network failure reports the mapped exit status" (exit 3) alongside the existing 401 exit-2 scenario. The design step table names the closed-port mechanism (bind an ephemeral listener, drop it, configure that port). |
| F7 | "An explanation that does not cover the command is not trusted" did not assert success, so the exit-0 promise was untested. | Added `And watn should exit successfully` to the delta scenario. |
| F8 | "An unconfigured machine starts quick setup instead of explaining" did not assert that no card opens. | Added `And no explanation card should open` to the delta scenario. |
| F9 | The runner executes permanent and delta specs together and drops only `@givn.removed` titles, so stale permanent scenarios would fail on the new readiness gate; the design never said how they are updated. | Added "Permanent-spec lockstep" to `design.md`, naming the eight permanent scenarios that must be re-based to a ready provider. Added the missing eighth scenario ("The explanation card opens even when the review surface is disabled") to the delta as `@givn.modified`, so the archive merge carries all eight. |
| F10 | The reused-step list omitted three steps used by the modified scenarios. | Completed the list with `watn should exit successfully` (`tests/steps/interactive_shell_shortcut_e2e_steps.rs`), `the command-output channel should contain no command` (`tests/steps/explain_command_steps.rs`), and `the file "..." should not exist` (`tests/steps/preserve_ctrl_w_requests_steps.rs`). |
| F11 | `ReviewResponseError`'s `Display` names a "candidate", an anti-term for an Explained command, and would leak into the `explain response was not usable: <reason>` diagnostic. | Added "Explain-domain diagnostic wording" to `design.md`: `ReviewResponseError::explain_reason()` maps every variant to explain-domain text, and no explain diagnostic may contain `candidate` or `review response`. The review path keeps `Display` unchanged. |
| F12 | Two inaccurate statements: "Exit status 12 is applied after the card closes" used a row number as a status, and the shell-target isolation credit went to `$XDG_CONFIG_HOME` instead of the PTY helper's temporary `HOME`. | Reworded the rules to refer to the request-failure table row and the unusable-response row. Reworded the obstacle to credit the PTY helper's temporary `HOME` (and `$XDG_CONFIG_HOME`) in `tests/steps/mod.rs`. |

## Branch verdicts

| Branch | Verdict |
|---|---|
| Scope | PASS after hardening — the delta covers all five proposal promises; F3, F5, F6, and F9 closed the coverage gaps. No scope was added beyond the proposal. |
| Tech choices | PASS — no stack change; the F11 mapping is a pure function on the existing error type; no new dependency. |
| Missing scenarios | PASS after F3 (wizard completion), F4 (interrupt), F5 (missing model), F6 (network exit), and F9 (persisted-surface re-base). |
| Testability | PASS — every new scenario drives the real binary in a PTY and asserts concrete exit statuses, stderr diagnostics, and card absence; each can fail in RED via the `unimplemented!()` step bodies. |
| E2E fidelity | PASS — no `@e2e` tag was removed or added; the single real-interface action (explain an existing command in the review card) is unchanged and the new scenarios are regular PTY scenarios. |
| Visual design contract | PASS — the `## Visual Design Contract` section exists with `Interface: terminal`; no new screen or motion; the new diagnostics are plain stderr text. |
| Interaction coverage | PASS — the only inventory entry maps to the unchanged `@e2e` row; the unready-model delegation remains a precondition variant with no new inventory entry. |
| Risk | PASS — the configuration-writing side effect is recorded in R-086 and the changed failure exit status in R-087; the interrupt and network paths add no new write boundary. |
| Use-case and Persona context | PASS — `use-shell` owns `explain-command`; `terminal-developer--interactive` is the only confirmed Persona, used as a review lens and never as a Gherkin actor. |
| ADR qualification | PASS — the readiness-delegation candidate remains `NOT_QUALIFIED` with `CANONICAL_ARTIFACT` routed to `design.md`; the existing-ADR check found no duplicate. |
| Architecture documentation (arc42) | PASS — the 12-row impact assessment is complete; chapters 01, 03, 04, 05, 06, 08, 10, 11, and 12 carry the change; chapter 09 is correctly not affected. F5 and F4 consistency was added to QS-079 and the runtime/crosscutting text. |
| Ubiquitous language | PASS — "Explained command", "Explanation request", and "Setup guidance" are in `docs/arc42/12-glossary.md`; F11 keeps the "candidate" anti-term out of explain diagnostics. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/explain-failure-guidance/specs/use-shell/explain-command.feature` | Added four `@givn.added @wip` scenarios (wizard completion, interrupt, network exit, missing model); re-based the persisted-review-surface scenario as `@givn.modified`; reworded explicit-selection steps to top-level flags; fixed the quick-setup step typing; added the F7 and F8 assertions. Scenario count 13 → 18 (modified 10 → 11, added 3 → 7). |
| `givn/changes/explain-failure-guidance/design.md` | Updated behaviour rows 9-11 and 13; reworded the two inaccurate rule statements; added the interrupt branch to the runtime sequence; specified the post-fetch interrupt check; added the explain-domain wording map; updated the explicit-selection decisions; added the permanent-spec lockstep section naming the eight scenarios; completed the reused-step list and step responsibilities; credited the PTY helper's temporary `HOME`. |
| `givn/specs/use-shell/usecase.md` | Added one single-line extension bullet: an interrupted explanation request opens no card and exits 130. |
| `docs/arc42/10-quality-requirements.md` | QS-079 now names the missing-model (exit 1) explicit-selection error. |
| `docs/arc42/06-runtime-view.md` | Prose names the missing-model error; the sequence diagram gains the Ctrl+C interrupt branch. |
| `docs/arc42/08-crosscutting-concepts.md` | Readiness prose names the missing-model error. |

## Lint

`givn lint --change explain-failure-guidance` exits **2** (advisory `@wip`
findings only; no syntax or delta-tag errors). Exit 0 or 2 is the required
range.

## Sign-off

- [x] All twelve findings resolved by artifact edits, not by descoping.
- [x] Every branch walked and recorded above.
- [x] `design.md` reflects every decision reached.
- [x] `@givn.modified` titles match the permanent spec verbatim; no `@givn.added` title exists in a permanent spec.
- [x] No `@e2e` tag was removed or added; every scenario is `@wip`.
- [x] `givn lint --change explain-failure-guidance` exits 2 (within the required 0-or-2 range).
- [x] `tasks.md` was not written.

DESIGN-REVIEW: PASS
