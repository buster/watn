# Arc42 Assessment: Highlight Active Setup Input

| # | Chapter | Affected | Assessment |
|---|---|---|---|
| 1 | Introduction and goals | Yes | Adds the usability requirement that the focused setup input is visibly marked with a green border. |
| 2 | Architecture constraints | No | Uses the existing Rust, Ratatui, Crossterm, TTY, and terminal interaction constraints without adding a new constraint. |
| 3 | Context and scope | Yes | Refines the terminal-facing setup output to include a green border around the focused input region. |
| 4 | Solution strategy | Yes | Records conditional border styling derived from the existing SetupWizard focus state. |
| 5 | Building block view | Yes | Updates the SetupWizard responsibility to include focused-widget border styling. |
| 6 | Runtime view | Yes | Updates setup, model-focus, and optional shortcut-focus flows to show the green border moving with keyboard focus. |
| 7 | Deployment view | No | Adds no executable, service, configuration, or deployment artifact. |
| 8 | Cross-cutting concepts | Yes | Documents terminal focus styling while preserving cursor, keyboard, layout, and inactive-widget behavior. |
| 9 | Architecture decisions | No | Extends the existing structured-widget decision and does not introduce a new architectural tradeoff. |
| 10 | Quality requirements | Yes | Adds the measurable active-input visibility scenario QS-053. |
| 11 | Risks and technical debt | Yes | Adds the low-impact risk that terminal palettes or capabilities may reduce green-border contrast. |
| 12 | Glossary | Yes | Adds the domain terms active input and focused widget. |

## Status

STATUS: DONE
