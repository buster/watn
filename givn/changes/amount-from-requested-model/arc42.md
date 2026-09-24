# arc42 impact: amount-from-requested-model

## ADR qualification

Existing-ADR check: the register in `docs/arc42/09-architecture-decisions.md`
(ADR-0001…ADR-0026) and `docs/adr/*.md` hold no decision about how a cost is
priced or looked up; ADR-0010 and ADR-0011 mention prices only as captured
configuration data.

Candidate — the price-lookup rule (reported model first, requested model
second).

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
  "canonical_artifact": "the observe-request-cost use case rules, design.md, and docs/arc42/08-crosscutting-concepts.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["reported-model-only lookup vs requested-model fallback"],
    "architectural_impact": ["one lookup order inside one process; no component, authority, or deployment boundary moves"],
    "durable_consequence": ["reversible within one change; no migration and no cross-command coordination"],
    "lower_level_artifact": ["the rule is completely owned by the use case contract and chapter 8"],
    "existing_adr_check": ["register and docs/adr/ search: no cost-pricing decision recorded"]
  }
}
```

No ADR is created.

Candidate — the cents rendering form (one significant digit, never more than
four decimal places, whole cents from one cent on).

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
  "canonical_artifact": "the observe-request-cost use case rules, design.md D2, and docs/arc42/08-crosscutting-concepts.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["one significant digit with a four-decimal cap vs four decimals with a never-zero guarantee"],
    "architectural_impact": ["one formatting function inside the Amount building block; no component, authority, or deployment boundary moves"],
    "durable_consequence": ["reversible within one change; the rule coordinates no other decision and migrates no data"],
    "lower_level_artifact": ["the use case rules state the form, design.md D2 records the operator's choice, and chapter 8 states the rendering"],
    "existing_adr_check": ["register and docs/adr/ search: no display-form or unit-format decision recorded"]
  }
}
```

No ADR is created for the form either: it is a rendering rule of one
capability, owned by the use case contract and chapter 8.

## 12-row chapter assessment

| # | Chapter | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Product goal and stakeholders unchanged. |
| 2 | 02 architecture-constraints | No | No new constraint. |
| 3 | 03 context-and-scope | No | The interface and its outputs are unchanged; only the price source behind an existing value changes. |
| 4 | 04 solution-strategy | No | Strategy unchanged. |
| 5 | 05 building-block-view | Yes | The `Amount` section now names `recorded_price` and the current `cents_text()` form; both rows were stale after the rendering rule changed. |
| 6 | 06 runtime-view | No | The flows are unchanged; the value they display is sourced differently. |
| 7 | 07 deployment-view | No | No deployment change. |
| 8 | 08 crosscutting-concepts | Yes | The cost-tracking section now names `amount::recorded_price` and the requested-model fallback. |
| 9 | 09 architecture-decisions | No | No ADR qualifies; the register stands unchanged. |
| 10 | 10 quality-requirements | No | No new quality scenario. |
| 11 | 11 risks-and-technical-debt | Yes | Added R-093 (the fallback prices the requested model when the provider serves another) and R-094 (the four-decimal cap can read `0`), and R-092 now names the fallback's residual case. |
| 12 | 12 glossary | Yes | The `Amount` definition and the `Model label` anti-term now name the price-source rule, so the vocabulary matches the amended use case. |

## Files touched

- `docs/arc42/05-building-block-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`

## Status

STATUS: DONE
