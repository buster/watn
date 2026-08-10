# arc42 impact: unified-setup-wizard

| # | Chapter | Affected | Reason / summary |
|---|---|---|---|
| 1 | Introduction and Goals | Yes | Adds explicit active-page and cursor usability goals for setup. |
| 2 | Architecture Constraints | No | Existing Rust, Ratatui/Crossterm, TTY, API, and single-binary constraints remain unchanged. |
| 3 | Context and Scope | Yes | Adds `watn setup` and changes provider/model terminal surfaces into one wizard. |
| 4 | Solution Strategy | Yes | Replaces separate interactive loops with a shared page-based setup strategy. |
| 5 | Building Block View | Yes | Adds the Setup Wizard building block and changes Provider/Models ownership. |
| 6 | Runtime View | Yes | Adds URL/API key/model page navigation and save/discard runtime flows. |
| 7 | Deployment View | No | No deployment or runtime infrastructure changes. |
| 8 | Cross-cutting Concepts | Yes | Changes keyboard, credential, cursor, cancellation, and partial-save behavior. |
| 9 | Architecture Decisions | Yes | Adds ADR-0013 for the shared five-page wizard. |
| 10 | Quality Requirements | Yes | Adds QS-021 and QS-022 for active-page/cursor clarity and unified navigation. |
| 11 | Risks and Technical Debt | Yes | Adds partial-save and shared-state risks with mitigations. |
| 12 | Glossary | Yes | Adds setup wizard, page, visible cursor, and save/discard terms. |

## Status

STATUS: DONE
