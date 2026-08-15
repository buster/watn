# Proposal: watn-consolidation

## Problem / Opportunity

watn's permanent specification tree has accumulated redundant scenarios while
remaining green. The current suite contains exact title duplicates, assertion
subsets, repeated coverage of the same invariant across command families, and
long flows that overlap without making their production boundary explicit.
Green currently proves that the globally registered bindings can execute; it
does not prove that the permanent scenarios are independently valuable.

This redundancy increases review cost, keeps obsolete step bindings alive, and
allows a stronger later scenario to coexist with a weaker earlier scenario.

## Proposed Solution

The watn specification suite is consolidated against the complete permanent
tree, not only within one feature family.

- Exact duplicate titles are reduced to one scenario with the stronger
  observable contract.
- Assertion-subset scenarios are removed when the retained scenario covers the
  same behavior and boundary more completely.
- Repeated scenarios are merged or removed when they describe one invariant;
  genuinely different boundaries remain separate and are named so the
  distinction is visible.
- Every removal has an explicit human-readable disposition identifying the
  retained scenario, the boundary decision, or the reason the scenario is
  obsolete.
- A stronger replacement is represented as a removal plus an addition in one
  change, and the review receipt reports the resulting net delta.
- Each real user action retains one end-to-end evidence scenario. Internal
  scenarios remain only for behavior that the end-to-end scenario does not
  cover.
- After consolidation, the permanent suite, its active change checks, and the
  deterministic overlap checks remain green.
- Removal placeholders are review/archive instructions rather than executable
  scenarios while the change is open; the post-archive permanent suite runs
  only the retained behavior.

## Out of Scope

- watn's runtime commands, provider behavior, configuration contract, and
  release artifacts.
- The deterministic retrieval/index implementation in givn; semantic results
  remain advisory and do not become a blocking score.
- A new identity registry, anchor format, compare-and-swap protocol, or journal
  for scenario files.
- Generic parsing or indexing of watn step-definition source code.
- A hard failure based only on scenario length.
- The runner's separate execution of permanent specifications and active
  deltas; that remains a documented follow-up rather than part of this
  consolidation.

## Open Questions

None. Candidate dispositions are recorded as executable review evidence during
the change; they are not unresolved architecture decisions.
