# arc42 impact: observe-request-cost

## ADR qualification

Existing-ADR check: the register in `docs/arc42/09-architecture-decisions.md`
lists ADR-0001…ADR-0026. Searching it and `docs/adr/*.md` for a decision about
cost, pricing, or the amount display finds only ADR-0010 (model picker) and
ADR-0011 (provider onboarding), which mention prices as captured configuration
data. No ADR owns the boundary this change touches.

Candidate 1 — the shared Amount computation with usage-presence tracking.

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "design.md plus docs/arc42/05-building-block-view.md and 08-crosscutting-concepts.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["two inline copies vs one shared computation with a usage-presence flag"],
    "architectural_impact": ["a single module inside one process; no component, deployment, or dependency boundary moves"],
    "durable_consequence": ["reversible within one change; no migration, no cross-command coordination"],
    "lower_level_artifact": ["the rationale is completely owned by design.md and the two arc42 chapters"],
    "existing_adr_check": ["docs/arc42/09-architecture-decisions.md register, docs/adr/*.md search for pricing/cost/amount"]
  }
}
```

Candidate 2 — the cents display form (four decimals, trailing zeros trimmed,
zero as `0`).

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "design.md and the capability's Gherkin scenarios",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["four-decimal cents trimmed vs two-decimal cents vs four-decimal dollars"],
    "architectural_impact": ["presentation of one value in one surface"],
    "durable_consequence": ["a formatting change is fully reversible within one change"],
    "lower_level_artifact": ["observable behaviour with no architectural trade-off belongs to the Gherkin specification and design.md"],
    "existing_adr_check": ["register and docs/adr/ search: no cost-display decision recorded"]
  }
}
```

Candidate 3 — the owner of the display behaviour (a new leaf use case rather
than the review goal).

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/specs/observe-request-cost/usecase.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["a new observing use case vs a capability under the review goal"],
    "architectural_impact": ["specification ownership, not a component, authority, or deployment boundary"],
    "durable_consequence": ["the corpus shape outlives one change, but the owning artifact is the use case document"],
    "lower_level_artifact": ["a use-case ownership decision is completely owned by the use-case document and the spec corpus"],
    "existing_adr_check": ["ADR-0025 covers specification ownership mechanics, not this capability's owner"]
  }
}
```

No ADR is created by this change. Note for a later change, recorded here only
as an observation: this repository stores ADR bodies at `docs/adr/NNNN-*.md`
and indexes them from chapter 09, while the current arc42-docs instruction
prescribes `docs/arc42/adr/ADR-NNNN-*.md`. No file moves are part of this
change.

## 12-row chapter assessment

| # | Chapter | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The product goal and stakeholder list are unchanged; the new use case is a capability of the existing product goal. |
| 2 | 02 architecture-constraints | No | No legal, technical, or organisational constraint is added. |
| 3 | 03 context-and-scope | Yes | The review-surface boundary row in the business context now names the Amount shown at the Model label. |
| 4 | 04 solution-strategy | No | No change of technical strategy; the display rides the existing review surface and cost computation. |
| 5 | 05 building-block-view | Yes | Added the `Amount` building block with `BilledAmount`, `billed_amount`, and `cents_text`. |
| 6 | 06 runtime-view | Yes | The review-and-accept and explain-existing-command sequences now show the Amount being displayed at the Model label. |
| 7 | 07 deployment-view | No | No deployment change: same binary, same install path, no new service. |
| 8 | 08 crosscutting-concepts | Yes | The cost-tracking section now describes one shared computation, the usage-report distinction, the cents form, and the deliberate divergence from the stderr line. |
| 9 | 09 architecture-decisions | No | No ADR qualifies; all three candidates route to canonical artifacts. The register stands unchanged. |
| 10 | 10 quality-requirements | No | No new quality scenario; the narrow-terminal readability case is covered by the capability's own scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | Added R-092: a silently absent Amount for a model without a recorded price, with its mitigation and the documented stderr divergence. |
| 12 | 12 glossary | Yes | Amended `Simple review view` and `Model short name`; `Amount` and `Model label` were already added when the change was planned. |

## Files touched

- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md` (amendments; `Amount` and `Model label` were added earlier in this change)

`docs/arc42/README.md` and the chapter-09 register need no update: no chapter
is added or removed and no ADR is created.

## Status

STATUS: DONE
