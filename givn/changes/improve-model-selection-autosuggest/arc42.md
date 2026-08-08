# arc42 Assessment: improve-model-selection-autosuggest

## 12-row assessment

| # | Change introduces or modifies... | Affected? | Reason |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | Existing usability improvement within the same tool; no new stakeholders |
| 2 | Constraints (legal, tech, org) | Yes | New technical constraint: provider model endpoint must support `?search=` query parameter for server-side filtering |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | Raw-terminal autosuggest picker replaces scrollable dialoguer list; `GET /models?search=...` added to provider contract |
| 4 | Major technical strategy or approach | Yes | Raw terminal I/O via `console` crate replaces `dialoguer::Select` for model picking; server-side search with stale-result guard |
| 5 | New building blocks, modules, components | Yes | New `src/models/picker.rs` (raw-terminal autosuggest loop); modified `src/models/list.rs` (search_models, fetch_models_page); modified `src/models/mod.rs` (picker integration) |
| 6 | New runtime flows, sequences | Yes | Autosuggest picker flow: raw terminal loop → debounced search worker → server-side filter → render suggestions → user selects |
| 7 | Deployment changes | No | Single binary, no deployment impact |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | New interactive terminal pattern (raw mode, console crate); PTY-based test harness for E2E coverage |
| 9 | Architecture decisions (tradeoffs, ADRs) | Yes | ADR-0009: Server-side filtering for paginated model catalogs |
| 10 | New quality scenarios | No | No new quality scenarios beyond existing usability requirements |
| 11 | New risks or technical debt | Yes | New risk: PTY-based test flakiness across platforms; new debt: raw terminal input behavior varies by terminal emulator |
| 12 | New domain terms | Yes | autosuggest picker, search query, stale-result guard, generation counter |

## Affected chapters

- **02 architecture-constraints**: Add provider search endpoint compatibility constraint.
- **03 context-and-scope**: Update context diagram to show `GET /models?search=...`; add autosuggest picker output to interfaces table.
- **04 solution-strategy**: Add raw terminal I/O and server-side search to technology choices.
- **05 building-block-view**: Add `src/models/picker.rs` and update Models building block responsibilities.
- **06 runtime-view**: Add autosuggest model picker sequence diagram.
- **08 crosscutting-concepts**: Add raw terminal input and PTY-based test harness sections.
- **09 architecture-decisions**: Add ADR-0009.
- **11 risks-and-technical-debt**: Add PTY flakiness risk and terminal-portability debt.
- **12 glossary**: Add autosuggest picker, search query, stale-result guard, generation counter.

## Status

STATUS: DONE
