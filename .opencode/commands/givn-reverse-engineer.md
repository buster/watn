---
description: Reverse-engineer an existing capability into descriptive Gherkin specs — discovery, feature writing, characterization tasks
---

Reverse-engineer an existing codebase capability into **descriptive Gherkin
specs** that document what the system *does* today. No proposals, no designs —
just `.feature` files + characterization tasks that make them executable.

**Input**: The argument after `/givn-reverse-engineer` is the change ID
(kebab-case), OR a plain description of the capability to reverse-engineer
(e.g. "the gherkin merge engine"). If nothing is provided, ask.

---

## The reverse-engineering stance

**Descriptive, not prescriptive.** You are capturing what the system *does*
today, not what it *should* do. Every spec assertion is probed from the real
code. Suspected bugs are flagged, never fixed. Production code behaviour is
never changed — it is refactored only for testability seams during
characterization.

---

## Steps

### 1. Discovery phase — explore and propose a capability

Use the **Task tool** to spawn a discovery subagent (fall back to describing
the pattern in prose if the Task tool is not available to you).

Pass a self-contained prompt:

```
You are a discovery agent for a givn reverse-engineering change.

Your job: explore the codebase and propose up to 3 candidate capabilities to
reverse-engineer, ranked by suitability.

## What to do

1. Explore the codebase. Identify coherent capabilities — modules, command
   groups, or subsystems that have a clear black-box boundary (CLI surface,
   public API, or module boundary).

2. For each candidate capability, note:
   - Capability name (kebab-case)
   - Black-box boundary (how it will be exercised in tests)
   - Brief description of what it does
   - Estimated number of observable behaviours (scenarios)
   - Suspected bugs found during discovery (if any)

3. Rank candidates by suitability. Consider: clear boundary, tractable scope,
   high value. Return up to 3.

## Return format

Return a JSON object:

{
  "candidates": [
    {
      "capability": "<kebab-case name>",
      "boundary": "<CLI surface / public API / module boundary description>",
      "description": "<what it does>",
      "estimated_scenarios": <number>,
      "observable_behaviours": [
        "<brief description of each observable behaviour>"
      ],
      "suspected_bugs": [
        "<description of any suspected bugs found during discovery, or empty array>"
      ],
      "codebase_findings": [
        "<any architectural observations relevant to characterization>"
      ]
    }
  ]
}

Return only the JSON — no prose before or after it.
```

### 2. Present discovery results interactively

Wait for the discovery subagent to return.

Present all candidates to the user, ranked:

```
## Discovery complete

**Candidate 1 (recommended):** <capability>
  Boundary: <boundary>
  Description: <description>
  Estimated scenarios: <N>
  Observable behaviours: <list>
  Suspected bugs: <list or "none">

**Candidate 2:** <capability>
  ...

**Candidate 3:** <capability>
  ...
```

**Wait for the user to select one.** This is an interactive checkpoint.
The user may:
- Select a candidate.
- Adjust the boundary of a selected candidate.
- Add or remove observable behaviours.
- Flag additional suspected bugs.
- Reject all and ask for a different search.

Do NOT proceed until the user selects a capability. If the user wants a
different search, re-run the discovery subagent with the user's guidance.

### 3. Resolve the change ID

If the user gave a kebab-case ID, use it. Otherwise derive one from the
capability name (e.g. "gherkin merge engine" → `reverse-engineer-gherkin-merge`).
Confirm with the user if ambiguous.

### 4. Create the change (skipping proposal, design, design-review)

```sh
givn new <id> --skip proposal,design,design-review
```

This creates the change directory and writes `.givn-skip` so that `givn status`
shows those artifacts as ⊘ skipped (not pending). No proposal.md, design.md,
or design-review.md is scaffolded.

If the change already exists (user pre-created it), `givn new` will fail —
that's fine. Check if `givn/changes/<id>/` exists and has artifacts. If so,
proceed with the existing change rather than creating a new one.

### 5. Write the descriptive spec directly

Write `givn/changes/<id>/specs/<group>/<capability>.feature`:

- Tag the Feature: `@givn.delta @<capability>`.
- Tag each Scenario: `@givn.added @wip` (all scenarios are added — the
  permanent spec is empty for this capability).
- The **Feature: description block** (the free-text lines after the Feature
  line) must contain:
  - The **black-box boundary** (named explicitly: "The black-box boundary
    for this capability is: <CLI surface / public API / module boundary>").
  - **Suspected bugs** discovered during discovery, flagged as follow-up
    `@givn.modified` candidates. These are NOT addressed in this change.
- Scenarios assert **observed behaviour** in domain language
  (Given/When/Then about inputs, outputs, state visible to a user/client).
  The behaviour is probed from the real code, not imagined from requirements.
- Never reference internal calls, class names, function names, specific code
  structures, HTTP routes, step-definition mechanics, or framework details.
- One scenario = one observable behaviour.
- If a scenario describes buggy behaviour (the code does something wrong, but
  that is what it actually does), the scenario captures the buggy AS-IS
  behaviour. Do NOT write the "correct" behaviour if the code does something
  different.

**Read the ACTUAL CODEBASE to determine observed behaviour.** Do not guess
from documentation alone — documentation can be wrong.

After writing, validate:

```sh
givn lint --change <id>
```

Exit 0 or 2 = correct (exit 2 means @wip scenarios present — expected).
Exit 1 = parse error — fix before continuing.

### 6. Write tasks.md (characterization cycles)

Write `givn/changes/<id>/tasks.md` with:

**First task: test infrastructure setup (or verification), PROVEN strict**

- No runner configured: install one, configure its strict-mode flag, set
  `verify.command`, create the runner binary and step skeleton one file per
  capability.
- Runner exists: do not assume strict — run the proof below regardless.
- **Proof of strictness (mandatory)**: write one step using the
  not-implemented stub for the language. Run the runner, confirm NON-ZERO
  exit, paste command + output. Exit 0 = fix the strict-mode config before
  continuing — this exact defect has previously checked off a whole
  characterization complete with every step an empty stub.
- Confirm the runner exits 0 (or only @wip-related failures) before
  characterization begins. If `verify.e2e_command` is configured and the
  feature has `@e2e` scenarios, prove it strict too.

**Subsequent tasks: one characterization cycle per scenario, in .feature
order, each ending in a commit**

Each task is a `- [ ]` checkbox covering a full RED/GREEN/REFACTOR/COMMIT
cycle (see `givn-steps` skill for the shared loop mechanics). For each task:

- Include the scenario title.
- Include a "Seam" block if a testability seam is needed for this scenario
  (extract interface, DI, make public). If no seam is needed, omit the block.
  Seam constraints: allowed — extract interface, DI, make public, add a
  test-only constructor, extract a pure function from a side-effectful one.
  NOT allowed — rename, deduplicate, simplify logic, fix bugs, change error
  messages, change return types, change control flow.
- Include a place to paste the RED/GREEN/REFACTOR runner output (a
  description of what should happen is not evidence — pasted output is).
- Include a place for the commit hash. One atomic commit per scenario,
  message references the scenario title.
- Any step without a real assertion yet uses the not-implemented stub
  pattern — never an empty body.

**Task format example:**

```markdown
- [ ] **Scenario: Valid multi-word kebab-case id creates the change scaffold**
  - RED: write step definitions using the not-implemented stub for new
    steps. Run verify, targeting this scenario → must fail (or pass
    immediately if all steps reused). Paste output.
  - GREEN: replace stubs with real assertions of observed behaviour. Run
    verify, targeting this scenario → must pass. Paste output.
  - REFACTOR: no seam needed.
  - COMMIT: <commit hash>
- [ ] **Scenario: Id with uppercase letters is rejected**
  - RED: write step definitions asserting observed exit code and stderr,
    using the not-implemented stub for anything new. Paste output.
  - GREEN: replace with real assertions. Paste output.
  - Seam: none.
  - COMMIT: <commit hash>
```

After writing tasks:

```sh
givn status --change <id>
```

Confirm task tracking is live.

### 7. Show final status

```sh
givn status --change <id>
```

---

## Output

```
## Reverse-engineering change planned: <id>

**Capability:** <capability name>
**Black-box boundary:** <boundary>

**Spec written:**
- givn/changes/<id>/specs/<group>/<cap>.feature  (N scenarios, all @givn.added @wip)

**Tasks written:**
- givn/changes/<id>/tasks.md  (N+1 tasks: 1 setup + N characterization cycles)

**Suspected bugs flagged (NOT fixed):**
- <bug 1 or "none">

**Ready to characterize.** Run `/givn-characterize` to execute the
RED/GREEN/REFACTOR loop and make the specs executable.
```

---

## Guardrails

- Discovery is always interactive — never skip the user confirmation checkpoint.
- One capability per change. If the user wants multiple capabilities, suggest
  separate changes.
- No proposal.md, no design.md, no design-review.md — those are skipped via
  `--skip`. The black-box boundary and suspected bugs go in the Feature:
  description block of the .feature file.
- All scenarios must be `@givn.added @wip` — they are descriptive, against an
  empty permanent spec.
- Read the ACTUAL CODEBASE to determine observed behaviour — specs must not
  be guessed from documentation alone.
- Suspected bugs are flagged in the Feature: description block — never fixed.
- Production code is never changed during planning. Testability seams are
  documented per-task but applied during `/givn-characterize`.
- Tasks use `- [ ]` checkboxes for `givn status --change` to track.
- The setup task's proof-of-strictness sub-task is mandatory — omitting it
  is how a characterization gets checked off complete with empty stubs.
- Every scenario task needs a field for RED/GREEN/REFACTOR output and a
  commit hash, not just a description of intended steps.
- `givn lint --change <id>` after writing specs; fix parse errors (exit 1).
