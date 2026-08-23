---
description: Complete the coverage review for a givn change — classify every gap, sign off, confirm GREEN
---

Complete the coverage review for a givn change.

**Input**: Optionally specify a change ID after `/givn-review`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Role: orchestrator

Delegates all review work to a **review subagent**: runs the test suite,
measures coverage, classifies every gap, resolves them, writes `review.md`.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Announce: "Using change: <id>". If `tasks` is not `"status": "done"`, warn
the user and confirm before proceeding.

### 2. Collect context

Read `tasks.md`, `specs/**/*.feature`, `design.md`, `verify.command` and
`verify.e2e_command` from `givn/config.yaml`.

```sh
givn instructions review --change <id>
```

When `--json` is supplied, capture `id`, `generates`, `requires`, and
`instruction`. Use `generates` and `requires` to orient the review; the
instruction text is the resolved policy. Without `--json`, capture the
resolved instruction text only.

### 3. Spawn a review subagent

Pass a self-contained prompt:

```
You are a review agent completing the coverage review for givn change '<id>'.

## Your objective
Produce a complete, signed-off review.md at givn/changes/<id>/review.md.
The final line MUST be REVIEW: PASS. Do not write it until all sign-off
conditions are met.

## Review instructions
<paste full output of: givn instructions review --change <id>>

## Verify commands
Unit/integration: <verify.command>
E2E smoke tests:   <verify.e2e_command>

## Steps

### 0. Fabrication audit (mandatory, first)
A change can reach review with every task checked off, the runner exiting
0, and almost nothing implemented — if steps were left as empty stubs.
Mechanical check, before trusting anything else:

1. Grep every step definition file for empty/trivial bodies (`{}`, bare
   `pass`/`return`, whitespace-only, no assertion reference). Record files
   scanned and findings (file:line).
2. For every checked `[x]` task, confirm a commit exists and `git show
   --stat <hash>` touches production source, not just spec/stub. Record
   any checked task with no commit or a spec-only commit.
3. For every component design.md said would be created, confirm it exists
   in the tree. Record anything promised but missing.
4. Confirm the setup task's proof-of-strictness evidence is present with
   non-zero exit. Missing/zero = the whole suite's GREEN is unverified.
5. Read every `@e2e` scenario's step definitions. If its Then steps assert
   ONLY against a repository/database (no assertion on page content, HTTP
   response, redirect, or CLI output), record it as a downgraded e2e
   scenario — an integration test wearing an `@e2e` tag. It does not prove
   the feature works end-to-end, regardless of the runner reporting GREEN.
6. For a browser-UI capability, read every `@e2e` step definition's
   implementation, not just its assertion. If it sends an HTTP request or
   calls `fetch()`/`XMLHttpRequest` from the page's JS context instead of
   driving the browser (click/type/navigate via a real driver), record it
   as a downgraded scenario — the interaction was never real.
7. Identify the EXACT file(s) `<verify.e2e_command>` actually invokes — read
   the command, not just a filename containing "e2e". Then run `git status`
   and search the tree (including untracked/uncommitted files) for any
   other `@e2e` step implementation of the same capability. If more than
   one exists, read all of them. Reviewing a weaker implementation while a
   stronger, real-interface one already exists elsewhere in the tree is
   itself a fabrication-audit finding — use and credit the strongest one.
8. Read `givn instructions specs --change <id>` and apply its normalized
   action scope. Record any capability with more than one `@e2e` scenario
   covering the same action as over-production; convert the excess to regular
   scenarios.
9. Confirm the local run command (design.md's Local Runnability section)
   still starts the full stack — including every digital twin — cleanly.
10. Read `verify.command` and `verify.e2e_command` from `givn/config.yaml`
    literally, as strings. If they are identical, the "e2e run" re-runs the
    whole suite and proves nothing about e2e isolation — always a finding,
    never acceptable even if both exit 0. Prove real isolation: run both,
    record the reported scenario count for each; `verify.e2e_command`'s
    count MUST be strictly smaller than `verify.command`'s, unless every
    in-scope scenario is `@e2e` (state this explicitly if true).
11. Diff the built implementation against design.md's explicitly named
    commands, file layout, and framework/driver choices — not just "does
    the file exist" but "does it match what design.md said." A deviation
    (different step-file split than design.md named, an unfiltered
    `verify.e2e_command` where design.md specified a tag filter, a
    different framework) implemented without first updating design.md and
    re-running design-review is a fabrication-audit finding: an unreviewed
    design decision made silently by the tasks/implementation phase. Fix
    by conforming to design.md as reviewed, or by updating design.md and
    re-running design-review (reassess arc42 chapters if structural)
    before accepting the deviation.
12. Any finding from 1-11 is unfinished implementation, not a coverage gap.
    Uncheck the task, drive it through RED→GREEN→REFACTOR→COMMIT for real —
    fixing the real-interface obstacle per design.md, never by leaving the
    repository-only assertion or HTTP/fetch() shortcut in place. Do not
    write REVIEW: PASS while any finding remains.

Paste the result into review.md's Fabrication Audit section even when clean.

### Arc42 implementation conformance (conditional)
If `addons.arc42` is disabled, record `N/A when the arc42 addon is disabled`.
If it is enabled, read the durable `docs/arc42/` chapters and the change-level
`arc42.md`, independently derive the affected chapters and architecture facts,
and compare each one against `design.md`, its mapped task(s) in `tasks.md`, and
evidence from the completed implementation. Fill review.md's **Arc42
implementation conformance** table.

This is a fresh post-implementation check, not a copy of design-review. It
must catch both stale Arc42 chapters and implementation drift from documented
facts. Any omission or contradiction is a finding. Fix the affected artifact
or implementation and repeat the comparison; if design.md changes, re-run
design-review. Record `ARC42 CONFORMANCE: CLEAN` and continue only when every
row matches. Do not write `REVIEW: PASS` with an unresolved Arc42 finding.

### Deterministic gate dispositions

Complete the **Overlap dispositions** table for every shape match, add a
**split-or-keep** decision for every long scenario, and explain every
removed+added pair as supersession or unrelated work before sign-off.

### Semantic review classifications

When `semantic-review.md` exists, classify every candidate as `DUPLIKAT`,
`VARIANTE`, or `VALID-BOUNDARY` and write a rationale. `UNCERTAIN` or a missing
rationale blocks `givn check review --change <id>`. There is no `UNRELATED`
escape classification. A seemingly unrelated pair that remains above the
active boundary requires sharper domain vocabulary and observable invariants,
followed by a fresh review run.

Token-cap evidence is not a classification. `BGE_TOKEN_CAP` and
`NLI_TOKEN_CAP` are per-candidate states for complete pairs that exceed the
actual model-tokenizer limit; they remain visible, unresolved, outside filtered
collections, and blocking. `BGE_UNAVAILABLE` and `NLI_UNAVAILABLE` are
run-level states only when that layer scores no pair. Do not classify a token-cap
candidate as resolved from another layer's score. Tell the author to shorten or
split the scenarios and rerun review; automatic chunking, automatic scenario
edits, and verbosity inflation are not valid remediation.

### 2. Confirm all tests pass
Run <verify.command> and <verify.e2e_command>. All scenarios GREEN, no @wip.
Failures: diagnose infrastructure (fix plumbing only) or coverage gap
(classify below). Never change production code to force a pass.

### 3. Measure and record coverage
Run `coverage.measure_command`, then `coverage.merge_command` from
`givn/commands.yaml`. Record the merged line and branch percentages and raw
report counters in review.md — a fact, not a threshold gate.

If either coverage command is still the sentinel (`givn missing-coverage`), the
review gate is blocked. Install and configure the project scripts before
continuing. Do not write "not measured" or any placeholder text.

### 4. Classify every gap
| Bucket | Meaning | Action |
|---|---|---|
| Dead code | YAGNI/KISS | Delete; re-run tests. |
| Missing test | Not covered by a scenario | Add scenario (@givn.added), RED/GREEN/REFACTOR/COMMIT. |
| Hard to test | Infra/I-O boundary | Concrete technical justification. |

These three buckets are exhaustive — **do not invent a fourth**. Phrases like
"acceptable for this iteration scope," "sufficient for current scope," or
"future scope" used to wave off a gap instead of naming one of the three
buckets above are themselves a fabrication-audit finding. A missing or
downgraded `@e2e` scenario is always "Missing test," never excused.

### 5. Resolve each gap
Per the table above; hard-to-test gaps get a written justification, no code change.

### 6. Confirm e2e coverage
Read the resolved specs instruction and verify its normalized inventory,
real-interface primary assertions, and driver fidelity. Missing, downgraded,
or over-produced coverage is unfinished work and must be added, fixed, or
converted under the three buckets. Before concluding a real driver is
unavailable, search the tree yourself, including untracked files.

### 7. Final GREEN confirmation
Re-run both runners. Must pass after all gap resolutions.

### 8. Write review.md
Coverage percentage + tool output, gap table, test suite output, sign-off
checklist, final line REVIEW: PASS.

### 9. Run the review hook
`givn check review --change <id>`. Fix failures before reporting back.

## Report back with
Fabrication audit result, coverage percentage, gaps classified by bucket,
final test result, path written, confirmation REVIEW: PASS is the final line.
```

### 4. Wait for the review subagent

Must finish the fabrication audit, all gap resolution, write `review.md`,
and run `givn check review` first. A reported blocker goes to the user.

### 4a. Orchestrator spot-check (do not skip)

Before accepting `REVIEW: PASS`, independently re-check a sample rather than
trusting the subagent's report:

1. Grep 2-3 step definition files yourself for empty bodies (or all if
   fewer than 5). Confirm any "0 findings" claim.
2. Confirm 2-3 checked-off tasks have commits touching production code, via
   `git show --stat`.
3. Read every `@e2e` scenario's step definitions yourself. Confirm each has
   a real-interface assertion, not repository-only, and — for a browser-UI
   capability — is actually driven through a real browser, not HTTP/fetch().
4. Read the canonical specs instruction and confirm no capability has more
   than one `@e2e` scenario for the same normalized action.
5. Independently confirm which file(s) `<verify.e2e_command>` invokes, and
   run `git status` / search the tree yourself for any other e2e
   implementation (including untracked files) the subagent's report did not
   mention. A subagent report that never names the exact invoked file(s) or
   never mentions checking for a parallel implementation is incomplete —
   reject it.
6. Read `verify.command` and `verify.e2e_command` yourself from
   `givn/config.yaml`. If identical, reject `REVIEW: PASS` outright — this
   is never a passable state, regardless of what the subagent's report
   claims about scenario counts. If the subagent's report does not include
   both scenario counts with the e2e count strictly smaller, reject it.
7. Spot-check one command/file/framework decision design.md names against
   what was actually built (e.g. open `verify.e2e_command`'s config value
   and the step-file paths named in design.md's "Step Definition
   Locations"). Any mismatch not backed by an updated design.md +
   re-run design-review = reject `REVIEW: PASS`.
8. Scan review.md itself for invented exception language ("acceptable for
   now," "future scope," "sufficient for current scope," or similar). Any
   such phrase = reject `REVIEW: PASS`, regardless of what it excuses.
9. Any contradiction: reject `REVIEW: PASS`, re-spawn the subagent with the
   specific discrepancy named.

### 5. Show final status

```sh
givn status --change <id>
```

Display: "Run `/givn-archive` to archive this change."

---

## Output

```
## Review complete: <id>

**Fabrication audit:** CLEAN (or: N findings remediated — see review.md)

**Coverage:** <percentage>
**Coverage gaps classified:** N
  - Dead code removed: N
  - Missing tests added: N
  - Hard to test (justified): N

**Final test run:** GREEN

**Review written:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change.
```

---

## Guardrails

- The orchestrator delegates coverage tools and review.md writing.
- Fabrication audit (step 0) is mandatory, always first, never optional.
- Orchestrator runs its own spot-check (4a) — never trusts the subagent's
  audit at face value.
- Any fabrication finding is unfinished implementation, not a coverage gap
  — remediate with a real RED/GREEN/REFACTOR/COMMIT cycle, never waved through.
- Coverage percentage is a fact, not a threshold.
- `REVIEW: PASS` requires all gaps resolved AND a clean fabrication audit.
- New scenarios added during review go through the full loop before sign-off.
- `givn lint` is static only; GREEN comes from the runners.
- Apply the canonical E2E policy from the resolved specs instruction. Verify
  normalized action coverage, real-interface primary assertions, and driver
  fidelity; more than one scenario for the same action is over-production.
- The local run command must start the full stack, digital twins included,
  cleanly — confirmed during this review, not assumed.
- The three-bucket classification is exhaustive. No fourth bucket, no
  invented exception language ("acceptable for now," "future scope,"
  "sufficient for current scope," etc.) may excuse a gap — this applies with
  special force to `@e2e` coverage, which is always "Missing test" when
  absent or downgraded, never waved off as out of scope.
- `verify.e2e_command` identical to `verify.command` is always a
  fabrication-audit finding, never a passable configuration, regardless of
  whether both exit 0 — it means the e2e gate is not isolating anything.
- Implementation must match what design.md explicitly named (commands,
  file layout, framework/driver). A silent deviation implemented without
  first updating design.md and re-running design-review is unfinished
  implementation — an unreviewed design decision made by the wrong
  artifact — never something tasks or review may wave through or work
  around unilaterally.
- Before declaring a real browser driver "not installed" or "not yet
  implemented," search the tree yourself (including untracked files) for an
  existing implementation — do not accept the subagent's or a prior
  artifact's word for its absence.
- Archive hard-blocks if `REVIEW: PASS` is absent or hooks fail.
