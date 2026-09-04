---
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time
---

Implement a givn change using scenario-by-scenario TDD.

**Input**: Optionally specify a change ID after `/givn-implement`. If omitted,
infer from conversation context, or auto-select if only one active change exists.
If ambiguous, list changes and ask.

---

## Role: orchestrator

Spawns one **coding subagent** per task for the complete RED/GREEN/REFACTOR
cycle. Does not write code directly.

**Never trust subagent self-reports.** A "PASSED" claim is not proof — a
subagent has previously checked off every task with "PASSED" while every
step was an empty stub and the runner silently reported GREEN. Step 5c
(independent verification) is mandatory for every task, not a spot-check.

---

## Steps

### 1. Resolve the change

```sh
givn status --json
```

Identify the active change from `active[0].id` (or ask if multiple).
Announce: "Using change: <id>".

### 2. Check current progress

```sh
givn status --change <id>
```

If all tasks are already checked, suggest `/givn-review`. Otherwise continue.

### 3. Validate the spec

```sh
givn lint --change <id>
```

Exit 0 or 2 = proceed (2 means @wip present — expected). Exit 1 = parse
error, fix first.

### 4. Collect context for subagents

Read once, pass to every subagent in full:
- `givn/changes/<id>/tasks.md` — task list (each task has a "Design
  constraints" block; add it from design.md if missing)
- `givn/changes/<id>/specs/<group>/<capability>.feature` — scenarios
- `givn/changes/<id>/design.md` — full document, including Strict Mode
  section (stub pattern) and single-scenario run command
- `givn/config.yaml` — `verify.command` and `verify.e2e_command`

### 4a. Confirm proof-of-strictness before looping

Before spawning any scenario subagent, confirm the setup task is checked
AND has pasted proof-of-strictness output showing non-zero exit. If
missing or zero, do the proof now — every scenario's RED depends on it.

### 5. Loop: one coding subagent per task

For each unchecked task, in order:

#### 5a. Spawn a coding subagent

Pass a self-contained prompt:

```
You are a coding agent executing a single TDD task for givn change '<id>'.

## Your task
<paste the task line verbatim, including Design constraints>

## Scenario to implement
<paste the relevant scenario(s)>

## Context
- Verify command (non-@e2e / @e2e): <verify.command> / <verify.e2e_command>
- Single-scenario run command: <from design.md — use for every RED/GREEN/
  REFACTOR run, never infer from whole-suite output>
- Step definitions location: <from design.md — one file per capability>
- E2E step definitions location: <from design.md>
- Not-implemented stub for this language: <from design.md>

## Rule: step body is never empty
`{}`, bare `pass`/`return`, whitespace-only = indistinguishable from PASS.
Every step without a real assertion uses the stub above, at every phase.

## Before any code
Read the givn-steps skill (`.agents/skills/givn-steps/SKILL.md` or your
harness's equivalent path) in full. Read givn/changes/<id>/design.md in
full — the Design constraints block is a summary, design.md is authoritative.
Do not substitute a simpler approach that happens to pass the test.

## RED
1. Remove `@wip` from this scenario only.
2. New steps use the not-implemented stub. Reused steps kept as-is.
3. Run, targeting this scenario via the single-scenario command. Capture
   output verbatim. Must exit non-zero. Exit 0 = runner not strict or a
   step is empty — STOP, fix before writing production code.

## GREEN
4. Replace stubs with real assertions. Write minimum production code
   following the design constraints and design.md exactly. "Minimum" means
   fewest lines, not a simpler architecture than designed.
5. List every production file created/modified. Empty list on a non-reuse
   scenario — STOP, a scenario with no assertions and no code is not GREEN.
6. Run targeting this scenario → zero exit. Capture output verbatim.

## REFACTOR
7. Clean up, no behaviour change.
8. Re-run targeting this scenario → still zero exit. Capture output verbatim.

## @e2e scenarios: same RED/GREEN/REFACTOR, with:
- e2e step location and e2e runner (`<verify.e2e_command>`) from design.md
- GREEN infrastructure and assertions applying the canonical policy from
  `givn instructions specs --change <id>` and the concrete design decision
- any real-interface obstacle resolved exactly as named in design.md; never
  silently substitute a weaker interface

## Done
9. One commit for RED+GREEN+REFACTOR: "feat(<capability>): <title>" (or
   "test(e2e): <title>"). Record the hash.
10. Paste RED/GREEN/REFACTOR evidence, files touched, and commit hash into
    the task in tasks.md. Only then check it: `- [ ]` → `- [x]`. Do this
    **immediately** — the box must be checked before the subagent reports
    back. Never leave the box unchecked and assume the orchestrator will
    handle it. tasks.md must always reflect true state: unchecked means
    not done. Using `sed` or any text-replacement to mass-check boxes is
    prohibited.
11. Report back: RED/GREEN/REFACTOR output verbatim, files touched, commit
    hash, any issues.
```

#### 5b. Wait for the subagent

Must complete RED/GREEN/REFACTOR, commit, and check off the task before
the orchestrator proceeds. A reported blocker goes to the user.

#### 5c. Independently verify — do not trust the self-report

Before moving on, the orchestrator itself (not another subagent) checks:

1. Re-run the scenario yourself, targeted. Confirm zero exit now.
2. Check the commit: `git show --stat <hash>`. Exists, and touches
   production source (not only spec/stub), unless a pure step-reuse case.
3. Grep the step file(s) touched for empty bodies (`{}`, bare
   `pass`/`return`). Any match = reject.
4. Confirm tasks.md has pasted evidence and the commit hash, not just a
   checked box.
5. **For `@e2e` tasks only:** read the step definition(s) for this
   scenario's Then steps. Confirm at least one asserts on the real
   interface's output (page content, HTTP response, redirect, CLI output) —
   not exclusively on repository/database state. A `@e2e` GREEN with
   Then-step assertions that only touch a repository is a downgraded
   scenario — reject it, regardless of what the subagent reported.
6. **For `@e2e` tasks on a browser-UI capability:** also read the step
   definition's implementation, not just its assertion. If it sends an
   HTTP request or calls `fetch()`/`XMLHttpRequest` from the page's JS
   context instead of driving the browser, reject it even if the assertion
   checks page content — the interaction was never real.

Any failure: uncheck the task, do not proceed, re-spawn with the specific
failure named (e.g. "`FooSteps.java:42` has an empty body — redo it", or
"this @e2e scenario's Then steps only assert repository state — assert on
the real interface's response, and if there's an obstacle, name it so
design.md can be fixed first").

Only if all checks pass:

```sh
givn status --change <id>
```

Confirm checked off. Move to the next task.

### 6. On completion

```sh
givn status --change <id>
```

If all tasks checked, run a final independent sanity check (not a repeat of
5c — whole-suite this time):

1. Run the FULL suite (both runners, not scenario-targeted). Confirm zero exit.
2. Grep every step definition file for empty/no-op bodies. Must be 0.
3. Count commits made vs. scenario tasks. Fewer commits than tasks = a task
   was checked off without a commit — investigate.

```sh
givn status --change <id>
```

Suggest: "Run `/givn-review` to complete the coverage review."

---

## Output during implementation

```
## Implementing: <id>

Task 3/7: <capability>: <Scenario title>
  → Spawning coding subagent...
  RED:      FAILED as expected
  GREEN:    PASSED
  REFACTOR: PASSED
  [x] task checked off

Task 4/7: ...
```

## Output on completion

```
## Implementation complete: <id>

**Progress:** 7/7 tasks complete

**Next:** Run `/givn-review` to complete the coverage review.
```

## Output on pause

```
## Implementation paused: <id>

**Progress:** 4/7 tasks complete

**Blocker:** <description>

**Waiting for:** <what is needed to continue>
```

---

## Guardrails

- Orchestrator manages the loop and delegates; it does not write code.
- One subagent per task (one RED/GREEN/REFACTOR/COMMIT cycle).
- RED must genuinely fail. A subagent reporting it passed on RED gets
  rejected and re-spawned.
- Independent verification (5c) is mandatory for every task, never skipped.
- Step body is never empty. Found during verification → reject regardless
  of subagent claims.
- Every task requires a commit before its box is checked.
- Do not spawn the next subagent until the previous task is verified and checked off.
- tasks.md must always reflect true state. An unchecked box in tasks.md
  means the scenario is incomplete. Never use `sed` or text-replacement
  to mass-check boxes — each box must be checked individually after
  evidence and commit hash are filled in.
- Remove `@wip` from one scenario at a time — state this explicitly to each subagent.
- `givn lint` is static only; GREEN comes from the runners, only after
  strict mode is proven (step 4a).
- Ambiguous task → ask the user before spawning.
- Never batch multiple scenarios into one subagent.
- GREEN requires: test passes AND implementation matches the design AND
  independent verification confirms it. A subagent that deviated from the
  design gets rejected and re-spawned even if the test passes.
- A `@e2e` scenario whose Then steps assert only against a repository/
  database, with no assertion on the real interface's output, is a
  downgraded scenario — reject it (5c.5), regardless of subagent claims.
- A real-interface obstacle hit during implementation is never a reason to
  substitute a weaker assertion. If design.md has no fix for it, that is a
  design gap — surface it as a blocker, get design.md fixed, then continue.
  "I could not make the browser work so I checked the database" is not an
  acceptable resolution at any point in this loop.
