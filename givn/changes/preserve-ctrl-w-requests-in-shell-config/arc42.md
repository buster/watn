# Arc42 Assessment: Preserve Ctrl-W Requests In Shell Config

| # | Chapter | Affected | Assessment |
|---|---|---|---|
| 1 | Introduction and goals | Yes | Refines the shell-shortcut goal so the original request remains visible above the generated command. |
| 2 | Architecture constraints | No | Retains the existing Rust, terminal, and native shell widget constraints. |
| 3 | Context and scope | Yes | Extends the line-editor boundary output with a preserved request comment. |
| 4 | Solution strategy | Yes | Extends ADR-0018's widget strategy with a comment-plus-command editable buffer. |
| 5 | Building block view | Yes | Updates the Shell Shortcut building block and generated widget responsibilities. |
| 6 | Runtime view | Yes | Updates the Ctrl-W flow so the buffer ends with the request comment followed by the generated command. |
| 7 | Deployment view | No | Adds no executable, service, or deployment artifact. |
| 8 | Cross-cutting concepts | Yes | Documents request flattening, no-evaluation guarantees, and failure preservation in the generated config. |
| 9 | Architecture decisions | Yes | Updates the ADR-0018 summary and consequences in chapter 09. |
| 10 | Quality requirements | Yes | Adds a quality scenario for request preservation and commit-time execution isolation. |
| 11 | Risks and technical debt | Yes | Documents comment-flattening and interactive-buffer testability limits. |
| 12 | Glossary | Yes | Adds the request comment and preservation contract terms. |

## Status

STATUS: DONE
