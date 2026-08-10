# arc42 impact: provider-setup-widget-layout

| # | Chapter | Affected | Reason / summary |
|---|---|---|---|
| 1 | Introduction and Goals | Yes | Adds scannable interactive setup presentation as a usability requirement. |
| 2 | Architecture Constraints | No | The existing Rust, Ratatui/Crossterm, TTY, config, and API constraints remain unchanged. |
| 3 | Context and Scope | Yes | The user-facing CLI output now exposes structured setup regions, tabs, aligned columns, and overflow position. |
| 4 | Solution Strategy | Yes | Records native Ratatui widget composition as the rendering strategy. |
| 5 | Building Block View | Yes | Refines Provider Setup and SettingsDialog responsibilities around lists, tables, paragraphs, tabs, and scrollbars. |
| 6 | Runtime View | Yes | Updates provider and model dialog render stages to show the new widget composition. |
| 7 | Deployment View | No | The single binary and deployment process do not change. |
| 8 | Cross-cutting Concepts | Yes | Documents terminal rendering structure, stateful selection, overflow indication, and control-sequence normalization in tests. |
| 9 | Architecture Decisions | Yes | Adds ADR-0012 for choosing native widget composition over paragraph-only or hand-positioned output. |
| 10 | Quality Requirements | Yes | Adds QS-019 for setup layout usability and QS-020 for responsive newest-result search behavior. |
| 11 | Risks and Technical Debt | Yes | Adds the narrow-terminal layout risk and mitigation, including the ADR consequence. |
| 12 | Glossary | Yes | Adds terms for widgets, tier tabs, model tables, and scrollbars. |

## Status

STATUS: DONE
