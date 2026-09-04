---
description: (Re-)write the scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change
---

Write (or re-write) the implementation task breakdown for a givn change.

For starting a new change, use `/givn-propose` instead. Use this command to
redo or extend the task list for an existing change.

**Input**: Optionally specify a change ID after `/givn-tasks`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Announce: "Using change: <id>".

Check the `artifacts` array. If `design-review` appears in the sequence and
is not `"status": "done"`, stop: "Design-review must be complete before
tasks. Run `/givn-design-review <id>`." Do not write tasks.md around a
missing or skipped design-review.

### 2. Get tasks instructions

```sh
givn instructions tasks --change <id>
```

When `--json` is supplied, parse `id`, `generates`, `requires`, and
`instruction`. Use `generates` for the output path and `requires` for
prerequisite artifacts; do not copy instruction prose into the file. Without
`--json`, the command emits the resolved instruction text only.

### 3. Read the inputs

`givn/changes/<id>/design.md` (step def locations, strict-mode config, stub
pattern), `.../specs/<group>/<capability>.feature`, `.../proposal.md`.

### 4. Write the task list

Write `givn/changes/<id>/tasks.md` as a checkboxed breakdown.

**Setup task (first, always). Not done until proof-of-strictness output
shows non-zero exit** — a silently-passing runner makes every later GREEN
meaningless.

```markdown
- [ ] Setup: configure test infrastructure, strict mode, and verify.command
      - Install/configure the runner from design.md. Step definitions ONE
        FILE PER CAPABILITY — never one file for the whole change.
      - Configure the runner's strict-mode flag from design.md.
      - PROOF OF STRICTNESS (mandatory): write one step using the
        not-implemented stub from design.md. Run the runner. Confirm
        NON-ZERO exit. Paste command + output. If exit 0, fix strict-mode
        config and re-prove before continuing.
      - Set `verify.command` in `givn/config.yaml`.
      - Run: <verify.command> → must exit 0 (proof step removed/fixed,
        real scenarios still @wip).
```

**One task per non-@e2e scenario, ending in a commit.** Embed the design
constraints relevant to that scenario — the executing agent may not read
design.md, so everything needed must be in the task.

```markdown
- [ ] <capability>: <Scenario title>
      Design constraints:
        - <architectural decisions from design.md that apply here — layer/
          module, protocol/data format, library, response shape. Specific
          enough that no ambiguous shortcut exists.>
      RED:      Remove @wip. Unimplemented steps use the not-implemented
                stub — never an empty body. Run <verify.command>, targeting
                this scenario only (single-scenario command from design.md)
                → MUST FAIL. Paste output.
      GREEN:    Replace stubs with real assertions. Write minimum production
                code per the design constraints. List every production file
                created/modified — empty list on a non-reuse scenario means
                stop and investigate. Run targeting this scenario → PASSES.
                Paste output.
      REFACTOR: Clean up. Re-run targeting this scenario → still PASSES.
                Paste output.
      COMMIT:   One commit for RED+GREEN+REFACTOR, message references the
                scenario title, e.g. "feat(<capability>): <title>". Record
                the hash.
```

**After all non-@e2e scenarios: e2e setup + one task per @e2e scenario.**

Read `givn instructions specs --change <id>` for the canonical normalized
action scope and real-interface policy. This command defines task mechanics,
not a second E2E policy.

```markdown
- [ ] Setup: configure e2e test infrastructure, strict mode, and verify.e2e_command
      - Install the e2e framework from design.md (Playwright, Testcontainers, etc.)
      - Create e2e step skeleton in the exact location(s) design.md names,
        one file per capability, separate from non-@e2e steps.
      - Configure and PROVE strict mode for the e2e runner (same procedure
        as main setup). Paste command + non-zero-exit output.
      - Set `verify.e2e_command` to the exact command design.md names,
        including its tag-filter flag/env var. It must NEVER be identical
        to `verify.command` — prove isolation: run both, paste both
        scenario counts, e2e count MUST be strictly smaller (or state
        explicitly every scenario is `@e2e`).
      - Document local environment start/stop if needed.
      - Run: <verify.e2e_command> → must exit 0.

- [ ] @e2e <capability>: <E2E Scenario title>
      Design constraints:
        - <e2e infrastructure decisions from design.md — browser driver,
          server startup, test DB, e2e step location.>
      RED:      Remove @wip. Unimplemented steps use the stub. Run
                <verify.e2e_command>, targeting this scenario only → MUST
                FAIL. Paste evidence.
      GREEN:    Set up test infrastructure. Replace stubs with real
                assertions driving the actual interface. List files
                touched. Run targeting this scenario → PASSES. Paste evidence.
      REFACTOR: Clean up. Re-run → still PASSES. Paste evidence.
      COMMIT:   One commit, message references the scenario title. Record
                the hash.
```

**Rules:**
- Setup task is not done until proof-of-strictness output (non-zero exit)
  is pasted in.
- One scenario = one full RED/GREEN/REFACTOR/COMMIT cycle.
- Every scenario task has a populated Design constraints block — never
  empty or generic.
- Step body is never empty; unimplemented steps use the stub from design.md.
- Every RED/GREEN/REFACTOR line requires pasted output, not a description.
- GREEN requires a list of production files touched.
- Every scenario ends in a COMMIT with a recorded hash.
- Order: non-@e2e scenarios (by dependency), then e2e setup, then @e2e.
- `givn lint` is static only; GREEN comes from the runners, after
  strictness is proven.
- **Never silently deviate from design.md.** If a command, file layout, or
  framework design.md names turns out inconvenient during implementation
  (e.g. one shared step file instead of the two design.md split, or
  `verify.e2e_command` == `verify.command` because a real tag filter is
  more work), STOP — do not implement the shortcut as a task. Update
  design.md with the corrected decision and re-run design-review before
  resuming.
  When the design.md fix is structural, reassess the affected arc42 chapters
  before resuming.
  review.md's fabrication
  audit diffs the built system against design.md's stated decisions
  specifically to catch an unreviewed shortcut like this.

### 5. Check status

```sh
givn status --change <id>
```

### 6. Show next step

```sh
givn status --change <id> --json
```

Read `next_required.id`. If `review` (or `all_required_done`), report:
"Ready to implement. Run `/givn-implement`." Otherwise report the next
planning artifact.

---

## Output

```
## Tasks written: <id>

**Tasks written:** givn/changes/<id>/tasks.md
**Total tasks:** N (one per scenario)

**Next:** Run `/givn-implement` to start the TDD loop (or the next planning
artifact if the manifest defines one after `tasks`).
```

---

## Guardrails

- One task per scenario — never collapse multiple scenarios into one.
- Read design.md before writing tasks; copy its strict-mode config and stub
  pattern into the setup task verbatim.
- A task with no Design constraints, or placeholder text, is incomplete.
- The setup task's proof-of-strictness step is mandatory — it is the single
  control preventing a change from being fabricated with empty stubs.
- Every scenario task needs pasted-output fields for RED/GREEN/REFACTOR and
  a files-touched list for GREEN — not just a description of what should happen.
- Tasks are checkboxes (`- [ ]`) for `givn status --change` to track.
- Verify the file exists after writing.
