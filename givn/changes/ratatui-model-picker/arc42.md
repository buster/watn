# arc42 Assessment: ratatui-model-picker

## 12-row assessment

| # | Change introduces or modifies... | Affected? | Reason |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | Same terminal users; usability improvement within the existing tool, no new stakeholders |
| 2 | Constraints (legal, tech, org) | No | No new legal/org constraint. Tech surface unchanged (CLI, TOML config, OpenAI-compatible API). The dialog is an internal UI change |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | The model-tier assignment flow is replaced by a keyboard-driven ratatui dialog; per-level reasoning strength added to the user-facing config surface |
| 4 | Major technical strategy or approach | Yes | Ratatui TUI replaces the console-raw-mode picker for the interactive flow; per-word order-independent filter; per-level reasoning config |
| 5 | New building blocks, modules, components | Yes | New `src/models/dialog.rs` (ratatui SettingsDialog); modified config types (TierReasoning), picker (filter), models/mod.rs, main.rs |
| 6 | New runtime flows, sequences | Yes | Dialog flow: guided per-level sequence with model pick + reasoning selection, back navigation, confirm; reasoning resolution now config-driven |
| 7 | Deployment changes | No | Single binary; adds `ratatui`/`crossterm` deps, no deployment impact |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | Config gains `[tiers.reasoning]`; reasoning_effort resolution moved from flag-hardcode to config; TUI event-loop pattern; per-word filter |
| 9 | Architecture decisions (tradeoffs, ADRs) | Yes | ADR-0010: ratatui keyboard-driven dialog for model + reasoning selection |
| 10 | New quality scenarios | No | No new quality scenarios beyond existing usability; covered by existing QS entries |
| 11 | New risks or technical debt | Yes | TUI escape-sequence portability across emulators; debounce timing in PTY tests; reasoning config parsing edge cases |
| 12 | New domain terms | Yes | SettingsDialog, reasoning strength (off/low/medium/high), per-word filter, guided sequence, page navigation |

## Affected chapters

- **03 context-and-scope**: Update context diagram to show the ratatui dialog
  taking keystrokes (arrows/page/enter/escape) instead of the raw-mode picker.
- **04 solution-strategy**: Add ratatui to technology choices; note per-word
  filter and per-level reasoning.
- **05 building-block-view**: Add `SettingsDialog` building block; update
  `ModelPicker`/`TierSelector` and Config responsibilities.
- **06 runtime-view**: Add the guided dialog sequence diagram; update the model
  exploration scenario to show reasoning selection and back navigation.
- **08 crosscutting-concepts**: Update terminal-interaction section from the
  console raw-mode picker to the ratatui dialog; add per-level reasoning
  configuration and resolution; add per-word filter.
- **09 architecture-decisions**: Add ADR-0010.
- **11 risks-and-technical-debt**: Add TUI escape-sequence portability risk and
  reasoning-config debt.
- **12 glossary**: Add SettingsDialog, reasoning strength, guided sequence,
  page navigation.

## Status

STATUS: DONE
