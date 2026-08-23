---
description: Execute the characterization loop for a reverse-engineering change — RED/GREEN/REFACTOR with step reuse, testability seams, reverse review
---

Execute the characterization loop for a givn reverse-engineering change.
RED/GREEN/REFACTOR per scenario: step definitions assert **observed**
behaviour, production code is refactored only for testability seams.

**Input**: Optionally specify a change ID after `/givn-characterize`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Role: orchestrator

Two phases:
1. **Characterization loop** — per task, a subagent writes tests matching
   observed behaviour, confirms GREEN, applies testability seams if needed.
2. **Reverse review epilogue** — after all tasks are checked, a review
   subagent runs the coverage review with reverse-mode rules (no dead-code
   deletion, bug flagging, behaviour-preservation invariant).

---

## Steps

### 1. Resolve the change

```sh
givn status --json
```

Identify from `active[0].id` (or ask if multiple). Announce: "Using change: <id>".

### 2. Check current progress

```sh
givn status --change <id>
```

If all tasks are already checked, suggest restarting.

### 3. Validate the spec

```sh
givn lint --change <id>
```

Exit 0 or 2 = proceed. Exit 1 = parse error, fix first.

### 4. Collect context for subagents

`tasks.md` (each task's "Seam" block if needed), `specs/<capability>/
<capability>.feature` (Feature: description has the black-box boundary and
suspected bugs), `verify.command`/`verify.e2e_command`.

---

## Phase 1: Characterization loop

### 5. Loop: one characterization subagent per task

For each unchecked task, in order:

#### 5a. Spawn a characterization subagent

Pass a self-contained prompt:

```
You are a characterization agent executing a single task for the givn
reverse-engineering change '<id>'.

## Your task
<paste the task line verbatim, including the Seam block if present>

## Scenario to characterize
<paste the relevant scenario(s)>

## Context
- Verify command: <verify.command>
- Step definitions location: <one file per capability, never a single file>
- Single-scenario run command: <if known>
- Not-implemented stub for this language: <e.g. Java PendingException,
  Python NotImplementedError, Rust unimplemented!()>

## Rule: step body is never empty
`{}`, bare `pass`/`return` = indistinguishable from PASS. This has caused a
whole change to be checked off complete while nothing was characterized.
Every step without a real assertion probing observed behaviour uses the
not-implemented stub.

## Before any code
Read the givn-steps skill (`.agents/skills/givn-steps/SKILL.md` or your
harness's equivalent) in full.

## Characterization overlay on givn-steps

### GREEN
- Implement real assertions probing OBSERVED behaviour — read the actual
  code, never guess from documentation.
- Do NOT change production code. You are matching reality, not driving new code.
- All steps reused and already GREEN → confirm assertions match, check off,
  no code changes.
- Verify FAILS → diagnose in order:
  1. **Infrastructure** (most common): wiring, config, imports, build
     errors, missing deps. Fix infrastructure only. Re-run.
  2. **Spec/implementation mismatch**: infra is correct, test runs,
     assertions fail because code doesn't match the spec inferred from
     docs/code. Re-read the actual code and the source documentation.
     Root cause: (a) documentation was wrong, or (b) code is
     ambiguous/misleading (including bugs — characterize AS-IS, do not fix).
     - (a): update tasks.md, the scenario, and the wrong documentation.
       Re-run. No code comment needed.
     - (b): update the scenario to assert actual behaviour. Add a comment
       (language's syntax) starting `TODO: REVERSE-ENGINEERING NOTE: <what
       was misleading> — actual behaviour: <Z>. Code smell, refactor
       candidate.` Do not change code behaviour. Re-run. Flag as code smell.
  - Never: change code to match the spec, fix bugs found here, remove
    REVERSE-ENGINEERING NOTE markers.

### REFACTOR
- Only if the task's Seam block calls for it. Allowed: extract interface,
  DI, make public, test-only constructor, extract pure function. Not
  allowed: rename, dedup, simplify, restructure, fix bugs, change error
  messages/return types/control flow.
- Re-run targeting this scenario. Capture output. If red: revert entirely —
  the test was correct, the refactor broke behaviour.
- No seam needed → skip.

## If the black-box boundary proves untestable
Do not halt the loop. Comment out the scenario (Gherkin `#`), explain why
the boundary is untestable, check off the task as "untestable — follow-up
needed," continue. Report all boundary failures at the end.

## Done
- One commit for RED+GREEN+REFACTOR: "characterize(<capability>): <title>".
  Record the hash.
- Paste RED/GREEN(/REFACTOR) evidence and the commit hash into the task in
  tasks.md. Only then check it off.
- Report back: RED result (failed as expected / reused → immediate GREEN)
  with output, GREEN result with output and root cause if diagnosed,
  boundary failure details if any, REFACTOR result, commit hash, any
  REVERSE-ENGINEERING NOTE markers added, suspected bugs, issues/decisions.
```

#### 5b. Wait for the subagent

Must complete the cycle and check off the task. A reported blocker
(unresolvable mismatch) goes to the user.

#### 5c. Independently verify — do not trust the self-report

Before moving on, the orchestrator itself re-checks:

1. Re-run the scenario yourself, targeted. Confirm it passes now.
2. Confirm a commit exists with real assertion code, not an empty body.
3. Grep the touched step file(s) for empty bodies. Any match = reject.
4. Confirm tasks.md has pasted evidence and a commit hash, not just a
   checked box.

Any failure: uncheck the task, re-spawn with the specific failure named.

```sh
givn status --change <id>
givn lint --change <id>
```

Exit 0 or 2 = OK (1 = parse error from a spec correction, surface to user).
Move to the next task.

**Do not use `givn check <artifact>` during this loop** — it fires the
verify hook and treats failure as a gate violation. Verify failures during
characterization are expected and handled by the subagent. Run
`verify.command` directly. `givn check` is only for the review epilogue.

### 6. On loop completion

```sh
givn status --change <id>
```

All tasks checked → proceed to Phase 2.

---

## Phase 2: Reverse review epilogue

### 7. Collect review instructions

Read `assets/instructions/review-reverse.md` in full (passed to the review
subagent).

```sh
givn instructions review --change <id>
```

### 8. Spawn a review subagent

```
You are a review agent completing the reverse-engineering coverage review
for givn change '<id>'. Produce review.md at
givn/changes/<id>/review.md, final line REVIEW: PASS — only once all
sign-off conditions are met.

## Before any work
Read the givn-review skill in full (coverage gate, gap classification,
sign-off).

## Reverse-mode overlay
<paste full content of assets/instructions/review-reverse.md>

Overlays gap classification: dead code → flag @givn.removed, do NOT delete.
Missing test → flag @givn.added, do NOT add during review. Hard to test →
justification. Behaviour-preservation: all characterization tests still
GREEN after refactoring.

## Verify command
<verify.command>

## Steps

### 0. Fabrication audit (mandatory, first)
A characterization change can also reach review with empty stubs left
behind. Grep every step file for empty/no-op bodies (fabricated
characterization). Cross-check every checked task has a commit with real
assertion code. Any finding = unfinished — reopen and redo it for real.
Record the result even when clean.

### 1. Confirm all tests pass
Run <verify.command>. GREEN, no @wip. Failures: infrastructure (fix
plumbing only) or spec/implementation mismatch (correct the spec, never
the code).

### 2. Measure and record coverage
Percentage + raw tool output into review.md.

### 3-4. Classify and resolve gaps (reverse-mode)
Dead code → flag @givn.removed, do NOT delete. Missing test → flag
@givn.added, do NOT add. Hard to test → justification.

### 5. Behaviour-preservation invariant
Re-run <verify.command>. All characterization tests still GREEN. Any that
went RED during the change = blocker, flag it.

### 6. Compile mandatory reporting sections
Spec corrections, documentation corrections, code smell markers (TODO:
REVERSE-ENGINEERING NOTE, file/line), suspected bugs (@givn.modified
follow-up), coverage gap table, test suite output, sign-off checklist.

### 7. Write review.md
All mandatory sections. Final line: REVIEW: PASS.

### 8. Run the review hook
`givn check review --change <id>`. Fix failures before reporting back.

## Report back with
Coverage percentage, gaps by bucket, spec/documentation corrections count,
code smell markers count, suspected bugs count, boundary failures count,
final test result, path written, confirmation of REVIEW: PASS.
```

### 9. Wait for the review subagent

Must finish gap resolution, write `review.md`, run `givn check review`
first. A reported blocker goes to the user.

### 10. Show final status

```sh
givn status --change <id>
```

Display: "Run `/givn-archive` to archive this change and create the
baseline spec."

---

## Output during characterization

```
## Characterizing: <id>

Task 3/7: <capability>: <Scenario title>
  → Spawning characterization subagent...
  RED:      FAILED as expected / all steps reused → immediate GREEN
  GREEN:    PASSED (on first try / after diagnosis: <root cause>)
  REFACTOR: APPLIED / NOT NEEDED / REVERTED
  [x] task checked off

Task 4/7: ...
```

## Output on completion

```
## Characterization complete: <id>

**Progress:** 7/7 tasks complete

**Spec corrections:** N
**Documentation corrections:** N
**Code smell markers:** N
**Suspected bugs flagged:** N
**Boundary failures:** N

**Review:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change and create the baseline spec.
```

## Output on pause

```
## Characterization paused: <id>

**Progress:** 4/7 tasks complete

**Blocker:** <description>

**Waiting for:** <what is needed to continue>
```

---

## Guardrails

- Orchestrator manages the loop and delegates; it does not write code.
- One subagent per task (one characterization cycle).
- Subagents read the givn-steps skill before writing step definitions.
- Step body is never empty; unimplemented steps always use the
  not-implemented stub.
- Independent verification (5c) is mandatory for every task, never skipped.
- Every task requires a commit before the box is checked.
- GREEN must genuinely pass; diagnose infrastructure vs mismatch before
  proceeding, never change production code to force a pass.
- Do not spawn the next subagent until the previous task is verified and checked off.
- Remove `@wip` from one scenario at a time — state this to each subagent.
- Never use `givn check <artifact>` during the loop; run `verify.command`
  directly. `givn check` is only for the review epilogue.
- Ambiguous task → ask the user.
- Never batch multiple scenarios into one subagent.
- REFACTOR is testability seams only, per the task's Seam block — never
  general cleanup. A refactor that goes red gets reverted, not fixed around.
- Suspected bugs are flagged, never fixed. REVERSE-ENGINEERING NOTE markers
  are never removed during characterization.
- `givn lint` is static only; GREEN comes from the runners.
- Archive hard-blocks if `REVIEW: PASS` is absent or hooks fail.
