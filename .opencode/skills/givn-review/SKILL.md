---
name: givn-review
description: Complete the coverage review for a givn change — classify every gap, sign off, and confirm GREEN.
---

# givn-review

Complete the coverage review for change `<change-id>`.

## Context

- Review file: `givn/changes/<change-id>/review.md`
- Instructions: run `givn instructions review --change <change-id>`
- Test runner: `./run-tests.sh`

## Step 0: fabrication audit (mandatory, before anything else)

A change can reach review with every task checked off, the runner exiting
0, and almost nothing implemented — if step definitions were left as empty
stubs. Mechanical check, not a judgment call:

0. **Verify @e2e tag integrity.** Grep every delta `.feature` file in
   `givn/changes/<change-id>/specs/` for scenario tags. If any scenario
   contains `@givn.added @e2e` in its git history or tracked state but no
   longer carries `@e2e`, record this as a fabrication-audit finding:
   `@e2e` tags were removed to bypass the gate. The fix is to restore the
   `@e2e` tag and configure `verify.e2e_command`.

1. Grep every step definition file for empty/no-op bodies (`{}`, bare
   `pass`, bare `return`, no assertion library reference). Record findings.
2. For every checked-off `[x]` task, confirm a commit exists and its diff
   touches production source (not just the `.feature` file or a stub).
3. For every component design.md promised, confirm it exists in the tree.
4. Confirm the setup task's proof-of-strictness evidence is present and
   shows a non-zero exit.
5. Read every `@e2e` scenario's step definitions. If its Then steps assert
   ONLY against a repository/database, record it as a downgraded e2e
   scenario — it does not prove the feature works end-to-end.
6. For a browser-UI capability, read every `@e2e` step definition's
   implementation. If it sends an HTTP request or calls `fetch()`/
   `XMLHttpRequest` from the page's JS context instead of driving the
   browser (click/type/navigate), record it as a downgraded e2e scenario —
   the interaction was never real, regardless of what the assertion checks.
7. Identify the exact file(s) `verify.e2e_command` actually invokes — read
   the command, not just a filename containing "e2e". Run `git status` and
   search the tree (including untracked/uncommitted files) for any other
   `@e2e` step implementation of the same capability. Reviewing a weaker
   implementation while a stronger, real-interface one already exists
   elsewhere (committed or not) is itself a fabrication-audit finding.
 8. Read `givn instructions specs --change <change-id>` and apply its
    canonical action normalization and E2E scope. Flag excess `@e2e`
    scenarios on the same action for conversion to regular scenarios.
9. Confirm the local run command (design.md's Local Runnability section)
   starts the full stack, including every digital twin, cleanly.
10. Read `verify.command` and `verify.e2e_command` literally from
    `givn/config.yaml`. If they are the same string, the "e2e run" is not
    isolating `@e2e` at all — always a finding. Prove isolation by running
    both and comparing scenario counts: `verify.e2e_command` MUST report
    strictly fewer scenarios, unless every in-scope scenario is `@e2e`
    (state this explicitly).
11. Diff the actual implementation (commands, file layout, framework)
    against what design.md explicitly named. A deviation implemented
    without updating design.md and re-running design-review first — e.g.
    reusing one step file design.md split into two, or an unfiltered
    `verify.e2e_command` where design.md named a tag-filtered one — is a
    fabrication-audit finding: implementation silently overrode an
    already-reviewed decision. Fix by either conforming to design.md as
    reviewed, or updating design.md + re-running design-review first.
12. Any finding is unfinished implementation, not a coverage gap — reopen
    the task, redo RED→GREEN→REFACTOR→COMMIT for real (fixing the
    real-interface obstacle per design.md, never leaving a repository-only
    or HTTP/fetch() shortcut in place), before continuing.
13. **Interaction coverage verification** — cross-reference the canonical
    specs policy's normalized inventory against the design's Interaction Coverage
    Matrix and the actual `.feature` file + step definitions:
    a. Read the spec `.feature` file and extract the `# User Interaction
       Inventory:` comment block. List every entry.
    b. Read the design's `Interaction Coverage Matrix` table. Confirm
       every inventory entry appears as a row. Flag missing or extra rows
       as a finding — every interaction must be mapped, no unmapped
       interaction may exist.
    c. For each matrix row, confirm an `@e2e` scenario with a matching
       title exists in the `.feature` file. The scenario title need not be
       identical to the inventory description, but it must clearly cover
       the same interaction.
    d. For each matrix row, grep the e2e step definition file(s) for the
       promised driving mechanism. If the matrix says "Playwright: click
       button X", the step file must import/use Playwright (not `reqwest`,
       not raw HTTP). If the driving mechanism is an HTTP client for an
       API-only capability, confirm the step file uses that HTTP client.
       Mismatch = driving-mechanism finding: the implementation silently
       used a different driver than the reviewed design committed to.
    e. Record the full cross-reference as a table in the fabrication audit
       section of review.md.
 14. **Coverage measurement validity.** For `verify.command`, and for the
     measurement plus merge commands when coverage is enabled:
    a. Map the runner and every production process started by tests.
     b. Confirm each process is instrumented, writes collision-safe output,
        flushes on shutdown, and both source reports are passed to the configured
        merge command.
    c. Confirm one known exercised production path has non-zero coverage.
    d. Reject runner-only, test-only, absent, or zero-production coverage.
     e. Confirm the merged report is freshly produced by the merge command; a
        stale or missing output is invalid.
     f. If invalid, record `COVERAGE MEASUREMENT: INVALID` and stop before gap
       classification. Fix instrumentation; do not classify it as missing test
       coverage or hard-to-test code.

Record the result (files scanned, findings, remediation) in review.md even
when clean.

<coverage-boundary-examples>
  <example>
    <bad>Instrument runner; spawn normal application; accept 0% application coverage.</bad>
    <good>Spawn instrumented application; merge its profile; verify a known application path is non-zero.</good>
  </example>
  <example>
    <bad>Share one profile across parallel processes; accept overwritten or unflushed data.</bad>
    <good>Use per-process output; flush on shutdown; merge all profiles before export.</good>
  </example>
</coverage-boundary-examples>

## Arc42 implementation conformance (conditional)

If `addons.arc42` is disabled, record `N/A when the arc42 addon is disabled` in
review.md. Otherwise, run this mandatory check before coverage classification.
Read the durable `docs/arc42/` chapters and the change-level `arc42.md`, then
independently derive the affected chapters and architecture facts from the
completed implementation, `design.md`, and `tasks.md`. For every chapter or
fact named by either assessment, compare the durable source, Arc42 claim,
design treatment, task mapping, and implementation evidence in review.md's
**Arc42 implementation conformance** table.

This check catches both stale Arc42 documentation and design, task, or
implementation drift from documented facts. Every omission or contradiction is
a finding. Fix the affected artifact and repeat the comparison; if design.md
changes, re-run design-review. Record `ARC42 CONFORMANCE: CLEAN` only when all
rows match. Do not write `REVIEW: PASS` while an Arc42 finding remains.

## Deterministic gate dispositions

Before sign-off, inspect the deterministic overlap findings. Complete the
**Overlap dispositions** table for every shape match, add a **split-or-keep**
decision for every long scenario, and explain every removed+added pair as
supersession or unrelated work. Do not use scores as a substitute for a human
disposition.

## Coverage gate (completeness, not percentage)

Classify every uncovered line/branch into exactly one bucket:

1. **Dead code** (YAGNI/KISS) → delete it now.
2. **Missing test coverage** → add a scenario + run RED/GREEN/REFACTOR, or fix steps.
3. **Legitimately hard to test** — rare; needs concrete technical justification.

These three buckets are exhaustive — **never invent a fourth**. Phrases like
"acceptable for this iteration scope," "sufficient for current scope," or
"future scope" that excuse a gap instead of naming one of the three buckets
are themselves a fabrication-audit finding. A missing or downgraded `@e2e`
scenario is always bucket 2 (missing test coverage), never an exception.

Coverage is complete when every gap is classified and resolved (or justified).
**There is no minimum percentage.** A high number is a side effect, not the goal.

## Steps

1. Run `./run-tests.sh` — confirm all non-@e2e tests pass.
   Coverage data is embedded in the output if the runner is configured
    with coverage instrumentation. Extract the summary from the merged report
    produced by `coverage.measure_command` and `coverage.merge_command`.
2. Run `verify.e2e_command` — confirm all @e2e tests pass.
3. Run the configured measurement command and then merge command. Apply check
   14 before extracting gaps.
   Reject absent, runner-only, test-only, or zero-production coverage.
4. Classify each gap (1, 2, or 3 above).
5. Resolve each gap:
   - Dead code → delete.
   - Missing test → add scenario + RED/GREEN/REFACTOR/COMMIT.
   - Hard to test → write justification.
 6. Read `givn instructions specs --change <change-id>` and verify its
    normalized inventory, real-interface assertion, and driver-fidelity rules.
    Missing, downgraded, or over-produced coverage is unfinished work. Before
    concluding a real driver is unavailable, search the tree yourself,
    including untracked files.
7. Run interaction coverage verification (fabrication-audit step 13).
8. Run both runners again — confirm still GREEN.
9. Complete the sign-off checklist in review.md, including the fabrication
   audit result.
10. Write `REVIEW: PASS` at the end of review.md when all items are checked
    AND the fabrication audit is clean.

## After sign-off

```
givn check review --change <change-id>
```

## Verify command

Unit/integration:
```
./run-tests.sh
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
