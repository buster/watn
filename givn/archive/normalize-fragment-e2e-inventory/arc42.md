# arc42 Documentation Update: normalize-fragment-e2e-inventory

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goal, stakeholder, or quality attribute changes; the change classifies existing tests. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint is added or removed. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface changes; the CLI scenarios already exist. | |
| 4 | 04 solution-strategy | No | No technical strategy or approach changes; no production code or step definition is added. | |
| 5 | 05 building-block-view | No | No building block, module, or component is added, removed, or repartitioned. | |
| 6 | 06 runtime-view | No | No runtime flow changes; only which test scope executes a scenario changes. | |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or runtime requirement changes. | |
| 8 | 08 crosscutting-concepts | No | No cross-cutting concept is introduced; the e2e scope policy is process guidance, not an architectural concept. | |
| 9 | 09 architecture-decisions | No | No architecturally significant decision qualifies; the reclassification is spec metadata recorded in `design.md`. | |
| 10 | 10 quality-requirements | No | No quality scenario or requirement changes; coverage classification changes, not a quality requirement. | |
| 11 | 11 risks-and-technical-debt | No | The merge and count risks are implementation checks recorded in `design.md`; no durable risk or debt is added. | |
| 12 | 12 glossary | No | No domain term is added or redefined; `@e2e` and regular scope are workflow vocabulary. | |

## ADR qualification

```text
Architectural commitment: none — no component, contract, boundary, or deployment changes.
Real alternatives: none — the decision is which existing tests carry the e2e tag, governed by the canonical specs policy.
Durable consequence: none — every scenario keeps its text and remains executed; only its verification scope changes.
Existing-decision search: chapter 09 register (ADR-0001 through ADR-0026) and docs/adr/ checked; no decision covers test-scope classification, and none is amended or superseded.
ADR action: NONE
ADR target: none
```

## Notes

No chapter files were changed. The assessment was made against the proposal, delta specs, and design. The change is specification-metadata maintenance with no architectural impact.

## Status

STATUS: DONE
