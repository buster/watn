# Arc42 Impact: quicksetup-parameters

| # | Chapter | Affected? | Reason / what changed |
|---:|---|---|---|
| 1 | 01 introduction-and-goals | No | Goals and stakeholders are unchanged. |
| 2 | 02 architecture-constraints | No | No new constraint; the flow stays interactive and network-free. |
| 3 | 03 context-and-scope | Yes | The User input line names the quick-setup parameter prefills. |
| 4 | 04 solution-strategy | Yes | The first-run row records parameter prefills. |
| 5 | 05 building-block-view | Yes | The `quicksetup` module row records parameter prefills. |
| 6 | 06 runtime-view | Yes | The first-run quick-setup scenario mentions parameter prefills. |
| 7 | 07 deployment-view | No | No deployment change. |
| 8 | 08 crosscutting-concepts | No | No new crosscutting concept. |
| 9 | 09 architecture-decisions | No | ADR-0026's plain-line quick-setup decision is unchanged; prefills are a refinement owned by the design. |
| 10 | 10 quality-requirements | No | No quality scenario names the quick-setup questions. |
| 11 | 11 risks-and-technical-debt | Yes | R-011 now records that a literal `--key` argument is visible in shell history and process listings. |
| 12 | 12 glossary | Yes | The Quick setup definition mentions optional parameter prefills. |

## Status

STATUS: DONE
