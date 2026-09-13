# arc42 Documentation Update: explain-failure-guidance

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Requirement 40 described silent `purpose-unavailable` degradation when no model is available; the requirement now covers setup delegation and visible failure diagnostics. | Rewrote requirement 40. |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organisational constraint; the setup TTY gate, atomic persistence, and explicit-selection error contract are existing constraints that the explain path now reuses. | |
| 3 | 03 context-and-scope | Yes | The explain entry point now has a setup/guidance outcome and a reported-failure outcome on the same boundary. | Updated the Developer input/output row and the Explanation entry point interface row. |
| 4 | 04 solution-strategy | Yes | The solution strategy gains the readiness-rule reuse for the explanation entry point. | Added one key decision bullet. |
| 5 | 05 building-block-view | Yes | The CLI building block now delegates explain to the setup surfaces and reports request failures with the mapped status. | Extended the CLI responsibility row. |
| 6 | 06 runtime-view | Yes | The "Explain an existing command" flow gains the setup delegation, request-failure, and unusable-response branches. | Updated the sequence diagram and the prose. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, dependency, or runtime-library change. | |
| 8 | 08 crosscutting-concepts | Yes | Explain's readiness delegation, rerun hint, request-failure diagnostic, and mapped exit status are new cross-cutting error-handling behaviour. | Extended "Verbatim command delivery for explanation". |
| 9 | 09 architecture-decisions | No | The candidate decision is NOT_QUALIFIED (its complete rationale is owned by `design.md` and the Gherkin contract); routing CANONICAL_ARTIFACT to the change design. No ADR file or register change. | |
| 10 | 10 quality-requirements | Yes | QS-077 changes from "unready provider receives zero requests" to visible failure handling, and a new onboarding quality scenario covers the setup delegation. | Updated QS-077 and added QS-079. |
| 11 | 11 risks-and-technical-debt | Yes | The change introduces a configuration-writing side effect on `watn explain`, a changed exit status for request failures, and a residual script ambiguity for unusable responses. | Added R-086 to R-088 and extended the command-explanation consequence coverage. |
| 12 | 12 glossary | Yes | Two new domain terms describe the explanation request and the non-terminal setup guidance. | Added "Explanation request" and "Setup guidance". |

## ADR qualification

Candidate decision: `watn explain` delegates to the existing setup surfaces
(quick setup, setup wizard) or prints setup guidance when no usable model is
configured and no provider or model was explicitly selected, and surfaces a
failed explanation request with the mapped exit status while keeping the card
reviewable.

Structured verdict:

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/explain-failure-guidance/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep the soft degradation: the card opens with purpose-unavailable and prints nothing.",
      "Print an error and exit non-zero without starting setup.",
      "Mirror the question path: quick setup, setup wizard, or setup guidance, and surface request failures with the mapped exit status."
    ],
    "architectural_impact": [
      "Fixes a user-facing CLI compatibility boundary: when explain triggers setup and which exit status a failed explanation reports."
    ],
    "durable_consequence": [
      "Reversal would require coordinated changes to the explain path, the explain-command capability spec, the use-shell precondition, and the arc42 quality and risk chapters."
    ],
    "lower_level_artifact": [
      "The complete rationale (failure-mode table, exit codes, delegation order, step mechanics) is owned by design.md.",
      "The observable contract is owned by the explain-command Gherkin scenarios and the use-shell precondition."
    ],
    "existing_adr_check": [
      "Searched the chapter-09 register and docs/adr/: 26 records; none covers the explain readiness delegation or explain failure exit statuses.",
      "ADR-0026 defines the quick setup surface and ADR-0011 the interactive onboarding boundary; both are reused unchanged, not refined.",
      "ADR-0015, ADR-0017, and ADR-0018 are adjacent to review, completion, and widget behavior but not to this boundary."
    ]
  }
}
```

MADR checklist evidence:

- Hard to reverse: checked as supporting evidence - the setup trigger and the
  new exit statuses are documented, tested, and may be relied on by scripts.
- Shapes quality attributes: checked as supporting evidence - usability (no
  silent degradation), observability (mapped exit codes), and safety (nothing
  is ever released or executed).
- Structural: unchecked - no module ownership or dependency direction changes;
  the existing setup surfaces and review card are reused.
- Must be shared: supporting only - contributors must respect the
  no-silent-degradation rule, but the Gherkin spec and design already state it.

Routing: `CANONICAL_ARTIFACT` to
`givn/changes/explain-failure-guidance/design.md` (with the observable contract
in the updated `explain-command` Gherkin scenarios). No ADR was created, so no
MADR file is added and neither the chapter-09 register nor `docs/adr/README.md`
changes.

## Status

STATUS: DONE
