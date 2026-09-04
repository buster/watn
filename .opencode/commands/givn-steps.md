---
description: Shared step-writing mechanics for Gherkin scenarios — RED/GREEN/REFACTOR with step reuse. Reference skill for givn-implement and givn-characterize.
---

Shared step-writing mechanics for Gherkin `.feature` scenarios. Reference
skill, not a standalone workflow — read by `/givn-implement` and
`/givn-characterize` subagents before writing step definitions.

This command prints the reference only. To write steps as part of a change:
`/givn-implement` (normal changes) or `/givn-characterize` (reverse changes).

---

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is
the contract; step definitions are shared glue the runner binds to scenarios.
Never maintain a parallel hand-written test file.

## Step reuse

Check for a matching step definition before writing a new one. Reused: keep
as-is, do not modify — its assertions are already proven. New: write fresh.

## Rule: step body is never empty

`{}`, bare `pass`, bare `return`, whitespace-only — all indistinguishable
from PASS in most Cucumber runners. Banned at every phase, not just RED.
Unimplemented steps use:

| Language / framework | Stub |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO");` |
| Python (`behave`) | `raise NotImplementedError("TODO")` |
| JS/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO")` |

Exempt only once a step has a real assertion (GREEN) or performs a real
reused action. Before trusting any PASS, confirm the runner is strict (see
design.md's Strict Mode section, proven in tasks.md's setup task).

## Canonical E2E policy

Read `givn instructions specs --change <id>` for the normalized action scope,
real-interface assertions, and driver-fidelity rules. This command owns only
step-definition mechanics and scenario targeting; it does not restate policy.

## Black-Box-First

For every internal step or test retained, record which case this test covers
that the E2E does not. Otherwise remove the duplicate and extend the real-
interface scenario.

## RED → GREEN → REFACTOR → COMMIT (one scenario at a time)

Every phase targets the runner **at this scenario only** (name/line/tag —
never infer from whole-suite output) and captures output verbatim as evidence.

### RED

1. Remove `@wip` from this scenario only.
2. New steps use the stub above; reused steps kept as-is.
3. Run targeting this scenario. Non-@e2e: `verify.command`. @e2e:
   `verify.e2e_command`. Capture output.
   - Non-zero exit → RED confirmed, proceed to GREEN.
   - Zero exit, all steps reused → legitimate immediate GREEN (record which
     steps were reused).
   - Zero exit with a new step → step is empty or runner isn't strict. STOP,
     fix before proceeding.

### GREEN

4. Replace stubs with real assertions (concrete values, not "it ran").
   - Normal TDD: write minimum production code. List every file
     created/modified — empty list on a non-reuse scenario is a red flag.
   - Characterization: probe observed behaviour, no production code
     changes. On failure, run the mismatch diagnostic.
   - E2e: set up test infrastructure, write glue driving the real interface.
5. Run targeting this scenario → zero exit. Capture output.

### REFACTOR

6. Normal TDD: general cleanup. Characterization: testability seams only —
   revert if red. E2e: clean up infrastructure code.
7. Re-run targeting this scenario → still zero exit. Capture output.

### COMMIT

8. One commit for RED+GREEN+REFACTOR, message references the scenario title
   (`feat(<capability>): <title>`, `characterize(<capability>): <title>`,
   or `test(e2e): <title>`). Record the hash. No commit, or a commit
   touching only the spec, means the work is not done.

### Done

9. Paste RED/GREEN/REFACTOR evidence, files touched, and commit hash into
   the task in `tasks.md`. Only then check it: `- [ ]` → `- [x]`.
10. Move to the next scenario.

## Distinctions

`givn lint`: static check, no tests. `verify.command`: GREEN for non-@e2e.
`verify.e2e_command`: GREEN for @e2e. Never conflate the three.

## Guardrails

- One scenario at a time.
- Step definitions: shared glue, one file per capability, never a
  hand-maintained parallel test file.
- Reuse existing steps; never duplicate.
- Every run targets the single scenario; output is captured, not described.
- `@e2e` GREEN requires a real-interface assertion. Repository-only
  assertions never satisfy an `@e2e` scenario, no matter how the
  interface's obstacle is explained.
