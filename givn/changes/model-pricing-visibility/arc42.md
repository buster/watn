# Arc42 Impact: model-pricing-visibility

| # | Chapter | Affected? | Reason / what changed |
|---:|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals, stakeholders, and the existing "cost when priced" quality goal are unchanged. |
| 2 | 02 architecture-constraints | No | No new technical, legal, or organisational constraint; no dependency added. |
| 3 | 03 context-and-scope | No | No new external system or user-facing command; `watn models` keeps its interface. |
| 4 | 04 solution-strategy | No | The catalog, display, and configuration strategies are unchanged; only the unit boundary is corrected. |
| 5 | 05 building-block-view | Yes | Updated `ModelExplorer` to normalize per-token catalog prices to $/1M and `ConfigWriter` to persist captured prices; no new building block. |
| 6 | 06 runtime-view | Yes | Catalog discovery and coordinated setup flows now persist captured per-million prices alongside tiers and reasoning. |
| 7 | 07 deployment-view | No | No deployment or packaging change. |
| 8 | 08 crosscutting-concepts | Yes | Cost-tracking section documents per-token catalog prices, parse-time normalization to $/1M, invalid-component handling, and preserving unchosen entries. |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate: the type split, sentinel rule, and capture policy are local design decisions routed to `design.md`; no existing ADR boundary changes. |
| 10 | 10 quality-requirements | No | QS-008 (cost displayed when pricing configured) already covers the observable outcome; no new quality scenario. |
| 11 | 11 risks-and-technical-debt | Yes | Added R-079 for non-OpenRouter pricing shape/unit assumptions and its mitigation. |
| 12 | 12 glossary | Yes | Added `Catalog price` and `Price capture`; clarified `Pricing` as manually written or captured. |

## ADR Qualification

| Candidate | Verdict | Route |
|---|---|---|
| Normalize catalog prices to $/1M during parse instead of introducing a per-token type | NOT_QUALIFIED — no durable architectural boundary; single catalog origin and local unit rule | `design.md` (Justification of Technical Choices) |
| Treat missing or negative catalog price components as absent | NOT_QUALIFIED — observable rule owned by the Gherkin scenarios | `specs/models/models.feature` |
| Insert-only price capture on model assignment | NOT_QUALIFIED — command-local persistence rule owned by the spec and existing ADR-0020/ADR-0021 boundaries | `design.md` |

Existing ADR check: ADR-0020 (final-confirmation setup snapshots), ADR-0021
(provider-local catalog discovery), and ADR-0024 (atomic config replacement)
were reviewed; none owns pricing units or capture, and none requires amendment
or supersession.

## Status

STATUS: DONE
