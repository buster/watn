# Arc42 Impact: simplify-review-card-controls

| # | Chapter | Affected? | Reason / what changed |
|---:|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals, stakeholders, and top-level quality goals are unchanged. |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint. |
| 3 | 03 context-and-scope | Yes | The Developer input and the review-surface boundary gain the direct decision keys and the model chooser. |
| 4 | 04 solution-strategy | Yes | The solution-strategy line now records the flow-first card, direct decisions, and reject-to-regenerate. |
| 5 | 05 building-block-view | Yes | The Review surface and Review card renderer responsibilities drop focus regions and gain direct decisions, the model chooser, and candidate replacement. |
| 6 | 06 runtime-view | Yes | The review/accept sequence, the candidate lifecycle diagram, and the lifecycle prose now show the flow-first keys, the model chooser, and regeneration; the stale portable-panel sentence is removed. |
| 7 | 07 deployment-view | No | No deployment or packaging change. |
| 8 | 08 crosscutting-concepts | Yes | "Inline review surface safety" now records the direct decisions, the reject-to-regenerate flow, and regeneration failure behavior. |
| 9 | 09 architecture-decisions | No | ADR qualification is `NOT_QUALIFIED`; the canonical artifact is the change design. No ADR is created or amended. |
| 10 | 10 quality-requirements | Yes | QS-067 and QS-070 are rewritten for the flow-first card and direct decisions; QS-074 covers rejection and regeneration. |
| 11 | 11 risks-and-technical-debt | Yes | R-077 records regeneration and catalog-availability risk and its mitigations. |
| 12 | 12 glossary | Yes | `Candidate`, `Review surface`, `Review outcome`, and `Review history` are corrected; `Model chooser` is added with anti-terms. |

## Status

STATUS: DONE
