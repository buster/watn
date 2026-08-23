---
name: givn-tasks
description: Break down implementation into a scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change.
---

# givn-tasks

Write the implementation task breakdown for change `<change-id>`.

## Context

- Tasks file: `givn/changes/<change-id>/tasks.md`
- Instructions: run `givn instructions tasks --change <change-id>`
- Design (HOW): `givn/changes/<change-id>/design.md`
- Test runner: `./run-tests.sh`

## Verify Interaction Coverage before writing tasks (mandatory)

Before writing any task, cross-reference the spec's User Interaction
Inventory against the design's Interaction Coverage Matrix:

1. Read the `# User Interaction Inventory:` comment block at the top of
   every `.feature` file in `givn/specs/`.
2. Read the Interactive Coverage Matrix in `givn/changes/<CHANGE_ID>/design.md`.
3. Confirm every inventory entry has a corresponding row in the matrix.
4. Confirm every matrix row names a valid driving mechanism (real browser
   driver for Web UI; real HTTP client for HTTP API; real subprocess for
   CLI). For a Web UI, the mechanism must read like "Playwright: click X" or
   "Selenium: fill Y" — not "reqwest" or "HTTP client".
5. Confirm every matrix row's @e2e scenario title appears in the `.feature`
   file as a `@e2e` scenario.

Any gap is a finding: report it to the user before proceeding. Do not write
task entries for @e2e scenarios that do not exist in the spec yet.

## Strict mode must be proven first (mandatory setup task)

Empty/unimplemented steps are indistinguishable from a PASS in most
Cucumber runners — this has checked off a whole change complete with every
step an empty stub and the runner reporting 100% GREEN. The setup task MUST:

- Configure the runner's strict-mode flag/config from design.md.
- **Proof-of-strictness**: write a step using the not-implemented stub (see
  design.md's Strict Mode section), run the runner, confirm NON-ZERO exit.
  Paste command + output. Exit 0 → fix the config and re-prove.
- Create step skeletons **one file per capability** (design.md's Step
  Definitions table) — never one file for everything.

## The TDD loop (one scenario at a time, ending in a commit)

Non-@e2e scenarios first:

```
For each non-@e2e scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions — unimplemented steps use the
              not-implemented stub from design.md, NEVER an empty body.
            Run: ./run-tests.sh, targeting this scenario only (use the
              single-scenario run command from design.md) → MUST FAIL with
              non-zero exit. Paste the output as evidence.
  GREEN:    Replace stubs with real assertions. Write minimum production
              code. List every production file created/modified — empty
              list on a non-reuse scenario means investigate before
              claiming GREEN.
            Run: ./run-tests.sh, targeting this scenario only → PASSES.
              Paste the output as evidence.
  REFACTOR: Clean up without changing behaviour.
            Run: ./run-tests.sh, targeting this scenario only → still
              PASSES. Paste the output as evidence.
  COMMIT:   One atomic commit for RED+GREEN+REFACTOR, message references the
              scenario title. Record the commit hash.
Then move to the next scenario.
```

Before the first @e2e scenario, bring up the local environment (design.md's
Local Runnability section: local run command, all dependencies, all
digital twins, isolated network) and confirm clean startup. Set
`verify.e2e_command` to the exact command design.md names, including its
tag-filter mechanism (`-- --tags @e2e`, `CUCUMBER_FILTER_TAGS`, etc.) — it
must NEVER be identical to `verify.command`; prove it by running both and
recording that the e2e scenario count is strictly smaller (or state
explicitly that every scenario is `@e2e`).

Then @e2e smoke tests (after all non-@e2e scenarios are GREEN):

**Rule: @e2e tags are immutable.** Once a scenario is tagged `@e2e` in a
delta `.feature` file, the tag MUST NOT be removed. If the verify-e2e gate
fails, configure `verify.e2e_command` — never remove the `@e2e` tag. Tag
removal is independently detected by `givn check review` and `givn lint
--change <change-id>`.

```
For each @e2e scenario:
  RED:      Remove @wip. Write e2e step definitions — unimplemented steps
              use the not-implemented stub, never an empty body.
            Run: verify.e2e_command, targeting this scenario only →
              MUST FAIL. Paste evidence.
  GREEN:    Read `givn instructions specs --change <change-id>` and the
              reviewed design's interface section. Set up the named
              infrastructure and assert through the real interface as the
              canonical policy requires. List files touched.
            Run: verify.e2e_command, targeting this scenario only →
              scenario PASSES. Paste evidence.
  REFACTOR: Clean up e2e code. Runner still PASSES. Paste evidence.
  COMMIT:   One atomic commit, message references the scenario title.
              Record the commit hash.
Then move to the next @e2e scenario.
```

## Important

- `givn lint` is a STATIC Gherkin check only. It does NOT run tests.
- A step definition body is NEVER empty (`{}`, bare `pass`, bare `return`).
  Unimplemented steps always use the not-implemented stub from design.md.
- GREEN for non-@e2e is confirmed by `./run-tests.sh` only, and only
  once strict mode has been proven in the setup task.
- GREEN for @e2e is confirmed by `verify.e2e_command` only, likewise proven.
- Never conflate lint, verify.command, or verify.e2e_command.
- **Never silently deviate from design.md.** If the exact command, file
  layout, or framework design.md names turns out inconvenient (e.g.
  reusing one step file design.md split into two, or pointing
  `verify.e2e_command` at the same string as `verify.command`), STOP —
  do not implement the shortcut. Update design.md with the corrected
  decision, re-run design-review (and reassess arc42 chapters if
  structural), then resume. A shortcut that never touches design.md is an
  unreviewed design decision made by the wrong artifact, and review.md's
  fabrication audit is built to catch it.
- Every scenario task ends in a COMMIT sub-task with a recorded hash. A
  checked task with no commit is not done.
- Every RED/GREEN/REFACTOR sub-task has a place to paste captured runner
  output — describing what should happen is not the same as recording what
  did happen.
- **tasks.md must always be up-to-date.** Check each scenario's box
  immediately after completing it — before starting the next one. An
  unchecked box means the scenario is incomplete. Never batch-check all
  boxes at the end. Using `sed` or any text-replacement tool to
  mass-check boxes without filling in evidence and commit hashes is
  prohibited — each box is checked individually as each scenario is done.
- Read the canonical specs instruction before evaluating E2E action scope,
  primary assertions, or driver fidelity. A scenario that violates it is not
  done, regardless of the runner result.
- Track progress: `givn status --change <change-id>`

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
