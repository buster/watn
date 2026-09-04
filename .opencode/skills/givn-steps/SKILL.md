---
name: givn-steps
description: Shared step-writing mechanics for Gherkin scenarios — the RED/GREEN/REFACTOR loop with step reuse. Referenced by givn-implement and givn-characterize.
---

# givn-steps

Shared step-writing mechanics for Gherkin `.feature` scenarios. Reference
skill, not a standalone workflow — read by `/givn-implement` and
`/givn-characterize` subagents before writing step definitions.

## Context

- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`
- Change: `givn/changes/<change-id>/`

## Before starting

Before writing or executing any step definitions, confirm your orientation:

1. Run `givn instructions --change <change-id>` — this tells you exactly what
   to do next, derived from the project's current filesystem state. Do not
   guess, search files, or explore to determine next steps.
2. Run `givn status --change <change-id>` — this shows the full artifact
   checklist, task progress, and any pending task descriptions. Check state
   before acting.
3. The task you are about to implement: read its full entry in `tasks.md`,
   including what step definitions already exist, the commit status of previous
   tasks, and the Design constraints block. An unchecked box means the scenario
   is not done — tick it off immediately after completing each scenario, with
   evidence and commit hash filled in. Never batch-check boxes later.

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is
the contract; step definitions are shared glue the runner binds to
scenarios. Never maintain a parallel hand-written test file.

## Step definition locations

One file per capability — never a single file for the whole change. A
giant all-in-one step file is a red flag: it hides empty/no-op stubs.

- Rust (`cucumber-rs`): `tests/steps/<capability>.rs`.
- Python (`behave`): `features/steps/<capability>_steps.py`.
- Node (`cucumber-js`): `features/steps/<capability>.steps.js`.
- Java (`cucumber-jvm`): `src/test/java/.../steps/<Capability>Steps.java`.

`@e2e` scenarios use separate step definitions and the same one-file-per-
capability rule. Read the canonical E2E policy with
`givn instructions specs --change <change-id>`; do not restate its action
scope or interface policy here.

## Rule: `@e2e` tags are immutable — never remove them

Once a scenario is tagged `@e2e` in a delta `.feature` file, the tag MUST
NOT be removed. If the verify-e2e gate fails because no e2e runner is
configured, fix the configuration — never delete the `@e2e` tag. Removing
`@e2e` tags to bypass the gate is a procedure violation and is independently
detected at review time (`givn check review`) and lint time (`givn lint
--change <id>`). The procedure: set `verify.e2e_command` in
`givn/config.yaml`, implement the e2e test infrastructure, and make the
scenario pass.

## Canonical E2E policy

Read `givn instructions specs --change <change-id>` for the real-interface
assertion, driver-fidelity, and normalized action rules. This skill supplies
step-writing mechanics and applies the resolved design's concrete driver; it
does not define another policy.

## Black-Box-First policy

Prefer reused real-interface steps. For an internal step or test that remains,
record **which case this test covers that the E2E does not**; otherwise remove
the duplicate and extend the E2E scenario.

## Step reuse

Check for a matching step definition before writing a new one. Reused: keep
as-is — its assertions are already proven. New: write fresh.

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

## RED → GREEN → REFACTOR → COMMIT (one scenario at a time)

Every phase targets the runner **at this scenario only** (name/line/tag —
never infer from whole-suite output) and captures output verbatim as evidence.

### RED

1. Remove `@wip` from this scenario only.
2. New steps use the stub above; reused steps kept as-is.
3. Run targeting this scenario. Non-@e2e: `./run-tests.sh`. @e2e:
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
   - E2e: bring up the local environment (design.md's Local Runnability
     section) if not already running, write glue driving the real
     interface, and assert PRIMARILY on its output. If a real-interface
     obstacle appears, apply the fix design.md names for it; if none is
     named, STOP and report the blocker — never downgrade to a
     repository-only assertion.
5. Run targeting this scenario → zero exit. Capture output.

5a. Validate coverage measurement before evaluating gaps:

    - Instrument every production process used by the scenario.
    - Launch instrumented artifacts, not normal builds.
    - Use collision-safe output; flush; merge all profiles.
    - Verify one exercised production path is non-zero.
    - Reject runner-only, test-only, absent, or zero-production coverage as
      `COVERAGE MEASUREMENT: INVALID`; fix instrumentation and rerun.
    - Only then treat uncovered production code as RED. Capture output.

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
   the task in `tasks.md`. Only then check it: `- [ ]` → `- [x]`. Do this
   **immediately** — before moving to the next scenario. tasks.md must
   always reflect true state: an unchecked box means the scenario is not
   complete. Never leave boxes unchecked and batch-check them later.
   Using `sed` or any text-replacement tool to mass-check boxes is
   prohibited — each box is checked individually, with evidence and
   commit hash filled in.
10. Move to the next scenario.

## Distinctions

`givn lint`: static check, no tests. `./run-tests.sh`: GREEN for
non-@e2e, only once strict mode is proven. `verify.e2e_command`: GREEN for
@e2e. Never conflate the three.

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
