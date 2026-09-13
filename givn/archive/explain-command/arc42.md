# arc42 Documentation Update: explain-command

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a new top requirement for explaining an existing command. | Added requirement 40. |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organisational constraint; the never-execute rule is an existing review invariant owned by the specs. | |
| 3 | 03 context-and-scope | Yes | New user-facing surface (`watn explain`) and a new interaction form (single argument, `--`, or standard input) with the explanation card on the controlling-terminal channel. | Added the entry point to the context diagram, the Developer input/output row, and a new interface row. |
| 4 | 04 solution-strategy | No | The solution reuses the existing review strategy, structured response contract, and provider/session machinery; no new strategy or approach. | |
| 5 | 05 building-block-view | Yes | The Review surface building block gains the explanation-only entry point and the CLI dispatch gains the `explain` subcommand. | Added the explain edge and extended the Review surface responsibility. |
| 6 | 06 runtime-view | Yes | New runtime flow from command resolution through the optional purpose fetch to the explanation-only card. | Added the "Explain an existing command" sequence. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or dependency change. | |
| 8 | 08 crosscutting-concepts | Yes | New cross-cutting contract for verbatim command delivery, safe quoting forms, and the explain-specific terminal eligibility. | Added "Verbatim command delivery for explanation". |
| 9 | 09 architecture-decisions | No | Structured verdict NOT_QUALIFIED; routed CANONICAL_ARTIFACT to `design.md`. The complete rationale is correctly owned by the change design plus the observable Gherkin contract. Existing-ADR check found no record for the explain entry point or verbatim argument delivery (ADR-0017 covers the `completions` reserved token; ADR-0018 covers widget invocation). | None. |
| 10 | 10 quality-requirements | Yes | Four new quality scenarios cover explanation delivery, command safety, degradation, and independence from the review preference. | Added QS-075 to QS-078. |
| 11 | 11 risks-and-technical-debt | Yes | Subcommand shadowing, pre-argv shell expansion, the strict single-argument rule, model fidelity, and stdin/terminal eligibility are new risks. | Added R-081 to R-085 and a command-explanation consequence-coverage section. |
| 12 | 12 glossary | Yes | New domain terms for the explanation entry point. | Added Explained command, Explanation-only card, and Verbatim command delivery. |

## ADR qualification

Candidate decision: `watn explain` accepts the command as exactly one
positional argument (with `--` for a leading dash and `-`/standard input for
heredoc-safe delivery), never joins, re-splits, evaluates, or executes it, and
degrades to `purpose-unavailable` when no model is usable.

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
  "canonical_artifact": "givn/changes/explain-command/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Join one-or-more positional words with spaces, like the question path: lossy for embedded quoting.",
      "Accept exactly one positional argument with -- and stdin forms: verbatim, explicit.",
      "Accept only standard input: safest from the shell but awkward for short commands."
    ],
    "architectural_impact": [
      "Fixes a public CLI contract and a compatibility boundary for future CLI changes."
    ],
    "durable_consequence": [
      "Reversal would change the documented invocation forms, the Gherkin delta, and the user-facing safety guarantee."
    ],
    "lower_level_artifact": [
      "The complete rationale (worked quoting examples, precedence, degradation, alternatives) is owned by design.md.",
      "The observable invocation contract is owned by the explain-command Gherkin scenarios.",
      "The never-execute invariant is already an existing review contract (use case use-shell, ADR-0015, ADR-0018)."
    ],
    "existing_adr_check": [
      "Searched the chapter-09 register and docs/adr/: 26 records, none covers the explain entry point or verbatim argument delivery.",
      "ADR-0017 covers the completions reserved token; ADR-0018 covers the Ctrl-W widget invocation. Neither is the same boundary."
    ]
  }
}
```

MADR checklist evidence:

- Hard to reverse: checked as supporting evidence - the public CLI forms and
  their tests would need coordinated changes.
- Shapes quality attributes: checked as supporting evidence - safety
  (no execution, no silent expansion) and usability (documented forms).
- Structural: unchecked - no module ownership or dependency direction changes;
  the change reuses the Review surface building block.
- Must be shared: supporting only - contributors must respect the verbatim
  contract, but the Gherkin spec and design already state it.

Routing: `CANONICAL_ARTIFACT` to
`givn/changes/explain-command/design.md` (with the observable contract in the
capability's Gherkin scenarios). No ADR was created, so no MADR file is added
and neither index changes. Note on storage: the project's 26 ADR bodies and the
chapter-09 register live at `docs/adr/`; that established path is retained and
no `docs/arc42/adr/` directory is introduced by this change.

## Status

STATUS: DONE
