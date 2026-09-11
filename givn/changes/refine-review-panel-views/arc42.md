# Arc42 Impact: refine-review-panel-views

| # | Chapter | Affected? | Reason / what changed |
|---:|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals and stakeholders are unchanged. |
| 2 | 02 architecture-constraints | No | No new constraint; the review surface stays inline and controlling-terminal-only. |
| 3 | 03 context-and-scope | Yes | Developer keys now include the `d`/`?` view toggle and `D` disable, the flag-only review switch persists without a request, and the command-output channel may carry the Candidate released by a permanent disable. |
| 4 | 04 solution-strategy | Yes | The review strategy now names two views, the model short name, the stage stack, and the disable release. |
| 5 | 05 building-block-view | Yes | The Review surface and Review card renderer responsibilities and the Review preference row record the two views, stack windowing, separators, purpose marker, and flag-only persistence. |
| 6 | 06 runtime-view | Yes | The review narrative, lifecycle state diagram, and channel-routing alternatives record the view toggle, the disable release, and the flag-only exit. |
| 7 | 07 deployment-view | No | No deployment change. |
| 8 | 08 crosscutting-concepts | No | No new crosscutting concept; rendering and persistence reuse existing seams. |
| 9 | 09 architecture-decisions | Yes | ADR-0015's release gate is amended: a permanent disable is an explicit final decision that releases the current Candidate without executing it. No new ADR. |
| 10 | 10 quality-requirements | Yes | QS-067, QS-070, and QS-073 now describe the simple/detailed views, the view toggle, the marked purpose, and the disable release. |
| 11 | 11 risks-and-technical-debt | Yes | R-073 wording follows the explicit-final-decision gate; R-078 records the disable release risk. |
| 12 | 12 glossary | Yes | New terms: Simple review view, Detailed review view, Review view toggle, Stage stack, Model short name; Review decision, Review outcome, and Command-output channel updated. |

## Status

STATUS: DONE
