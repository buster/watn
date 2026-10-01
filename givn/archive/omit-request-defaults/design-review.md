# Design review: omit-request-defaults

## Method

Phase 1 grilling ran in a fresh context: a subagent with no part in writing the
proposal, the specs, or the design read all planning artifacts, the permanent
neighbours, the source, the runner, and the glossary, verified the claimed
mechanisms against the code, and returned findings and a ranked question list.
Phase 2 hardening was applied by the orchestrator from the answers.

## Findings (settled by the grilling agent, with evidence)

| # | Finding | Disposition |
|---|---|---|
| 1 | The design promised a transcript-recording Then that the scenario did not contain, so the evidence step would be dead code (`design.md` vs `specs/fragments/ask.feature`) | Fixed: design-review Q1 demoted the scenario to a regular scenario, which carries no transcript obligation; the transcript step and evidence path are removed from the design |
| 2 | The rendered reference claimed the transcript would show the metadata line, which is written to stderr while `world.output` is stdout only (`tests/steps/mod.rs:593-594`, `src/output/render.rs:72-86`) | Fixed: the Visual Design Contract now records no rendered reference and no capture |
| 3 | The design never required removing `@wip` in the implementing revision, so the only behavioural proof could stay filtered from both suites (`run-tests.sh:25`, `:28`) | Fixed: the design states `@wip` is removed in the same commit as the step and `givn lint --change` must exit 0 before review |
| 4 | The ADR verdict named four canonical artifacts where the procedure allows exactly one (`arc42.md`) | Fixed: `canonical_artifact` is now the ask capability's Gherkin scenario; the chapter edits remain recorded as arc42 impact |
| 5 | The runner citation `tests/features_runner.rs:169` points at the `Vec` declaration, not the collection (`:171`) | Fixed |
| 6 | Blocking-mock registration was described for one mock-setup branch while `ensure_test_env` has two answering-mock branches (`tests/steps/mod.rs:281-295`, `:427-441`) | Fixed: the design requires the registration in both branches |
| 7 | `@wip` is present on the only new scenario | Expected until implementation; recorded in the design and tasks, not a blocker |

## Questions and answers

1. **Is the rejecting-provider ask a new end-to-end interaction or an error
   variant of the existing ask action?** Answered: an error variant. The
   canonical policy classifies error cases and variants as regular scenarios,
   the `ask` capability already carries ten `@e2e` scenarios in the permanent
   corpus, and the scenario still drives the real CLI subprocess through
   `run_binary_with_state` under `verify.command`. No fragment inventory row
   and no transcript are added.
2. **Should removing the forced parameters also remove the two fields from the
   public `RequestOptions` struct?** Answered by the operator: yes, remove
   them. Every `RequestOptions` literal must drop the fields, so no future call
   path can resend a rejected parameter. This is a source-breaking change to
   the published crate's Rust API; the implementing revision is marked
   breaking (`feat(ask)!:` with a `BREAKING CHANGE` footer), and the 0.x minor
   bump covers it per Cargo convention.

## RED-test audit

One scenario, and it can fail in RED against today's code: the blocking twins
return HTTP 400 for bodies containing `temperature`, `max_tokens`, or
`max_completion_tokens`; today `openai_compat.rs:58-59` always sends the first
two, so the binary exits non-zero and `the exit status should be 0` fails. The
scenario is run with
`./run-tests.sh --name "Ask succeeds against a provider that rejects generation parameters"`
before any production edit, and the observed failure is recorded in `tasks.md`.

## Deferrals

None. Nothing mandatory was deferred, and no `@e2e` tag was removed or
weakened; the permanent `ask` `@e2e` scenarios are untouched.

## Verification

- `givn lint --change omit-request-defaults` exits 0/2 with the single expected
  `@wip` finding on the unauthored scenario; the finding clears when the step
  lands and `@wip` is removed.
- Independent 12-row arc42 derivation by the grilling agent matches `arc42.md`
  row for row; chapters 05, 08, 11, and 12 carry real content, and no MADR is
  required because no ADR candidate qualifies.
- ADR qualification independently re-checked: NOT_QUALIFIED, routed to the ask
  capability's Gherkin scenario.
- All five `RequestOptions` literals in the repository were enumerated and the
  field removal compile-breaks each one (`src/main.rs:496`, `:962`, `:1247`;
  `tests/steps/interactive_shell_shortcut_steps.rs:3173`, `:3374`).

DESIGN-REVIEW: PASS