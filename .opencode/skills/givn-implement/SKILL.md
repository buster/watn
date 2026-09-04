---
name: givn-implement
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time.
---

# givn-implement

Implement change `<change-id>` using scenario-by-scenario TDD.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Design: `givn/changes/<change-id>/design.md`
- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`

## Before starting

1. Run `givn instructions --change <change-id>` to confirm what to do
   next — do not guess or explore files to determine next steps.
2. Check progress: `givn status --change <change-id>`
3. `givn lint` to confirm the spec is well-formed (no tests run).
4. Read the `givn-steps` and `givn-dev-principles` skills in full
   (RED/GREEN/REFACTOR/COMMIT loop, step reuse, never-empty-body rule,
   lint-vs-test distinction, engineering principles).
5. Confirm the setup task is checked AND has pasted proof-of-strictness
   output showing non-zero exit. Missing/zero → do the proof now — every
   later GREEN depends on it.

## Rule: never claim PASS without re-checking

A whole change has been checked off complete before, every task claiming
"RED failed, GREEN passed," while every step was an empty stub (`{}`) and
the runner never actually failed (cucumber-jvm's JUnit Platform Suite mode
treats an empty body as PASS, not pending). Before writing "PASSES":
- Grep the file yourself for `{}`, bare `pass`/`return` — confirm every
  stub was replaced.
- Confirm production code was actually created/modified (list files),
  unless a genuine pure step-reuse case.
- Confirm the captured output shows a zero exit for a targeted run of THIS
  scenario, not an inference from the whole suite.

## The loop

Non-@e2e scenarios first, then @e2e. Every phase runs the runner **targeted
at this scenario only** and captures output as evidence.

### Non-@e2e scenarios

#### RED
1. Remove `@wip` from this scenario only.
2. Unimplemented steps use the not-implemented stub from design.md — never
   an empty body.
3. Run `./run-tests.sh`, targeting this scenario (single-scenario
   command from design.md). Capture output. Must exit non-zero. Exit 0 =
   step is empty or runner isn't strict — STOP, fix before writing code.

#### GREEN
4. Replace every stub with a real assertion. Write minimum production code.
   List every file created/modified — empty list on a non-reuse scenario
   means investigate before calling this GREEN.
5. Run targeting this scenario → zero exit. Capture output.

5a. Confirm the new code is covered. The `./run-tests.sh` output
    contains coverage data when the runner is configured with coverage
    instrumentation. Extract the summary: every production file
    created/modified in this scenario must show non-zero coverage. If
    coverage data is absent from the output, state this explicitly as
    an unmeasured gap. If new code shows uncovered regions, this is a
    RED condition — fix step assertions or add missing step definitions,
    then re-run GREEN → COVERAGE.

#### REFACTOR
6. Clean up, no behaviour change.
7. Re-run targeting this scenario → still zero exit. Capture output.

#### COMMIT
8. One commit for RED+GREEN+REFACTOR, message references the scenario title
   (`feat(<capability>): <title>`). Record the hash.

#### Done
9. Paste evidence, files touched, and commit hash into the task in
   `tasks.md`. Only then check it: `- [ ]` → `- [x]`. Do this
   **immediately** — before moving to the next scenario or spawning
   another subagent. tasks.md must always reflect true state: an
   unchecked box means the scenario is not complete. Never leave boxes
   unchecked and batch-check them later. Using `sed` or any
   text-replacement tool to mass-check boxes is prohibited — each box
   is checked individually, with evidence and commit hash filled in.
10. Move to the next scenario.

### @e2e smoke tests (after all non-@e2e are GREEN)

**Rule: @e2e tags are immutable.** Once a scenario is tagged `@e2e` in a
delta `.feature` file, the tag MUST NOT be removed. If you cannot run e2e
tests, configure `verify.e2e_command` in `givn/config.yaml` — never remove
`@e2e` tags as a workaround. Removing `@e2e` tags to bypass the verify-e2e
gate is a procedure violation and is independently detected by `givn check
review` and `givn lint`.

Before the first @e2e scenario: bring up the local environment from
design.md's Local Runnability section (local run command, all
dependencies, all digital twins, isolated network). Confirm clean startup.

Same loop, `verify.e2e_command`, and the separate step location named in
design.md. Before implementing E2E steps, read the canonical policy with
`givn instructions specs --change <change-id>`; GREEN applies its resolved
action scope and real-interface rules while setting up the design's
infrastructure.
After GREEN: confirm coverage via `./run-tests.sh` output. If the
e2e runner does not produce coverage data, state this explicitly —
@e2e coverage must still be estimated and classified in review.

The canonical specs policy owns primary assertions and real-interface fidelity.
If an interface obstacle appears, apply the concrete fix named in design.md;
do not substitute a repository-only or alternate-interface shortcut.

Commit message: `test(e2e): <title>`.

## When all tasks are checked

Final independent sanity check — do not skip because scenarios were
already verified individually:
1. Run the FULL suite (both runners, not scenario-targeted). Confirm zero exit.
2. Grep every step definition file touched for empty/no-op bodies. Must be 0.
3. Confirm commit count matches scenario task count.
4. Confirm coverage. Re-run `./run-tests.sh` and extract the coverage
   summary. If the runner is coverage-instrumented, every new/modified
   production file must show non-zero line/branch coverage. If coverage
   data is absent from the output, note this as an unmeasured gap — the
   review step will need to classify uncovered regions manually.
4. Read the canonical specs policy and cross-reference its normalized
   inventory against completed E2E tasks and real-interface assertions.

Run `givn status --change <change-id>` to confirm progress. Proceed to review.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
