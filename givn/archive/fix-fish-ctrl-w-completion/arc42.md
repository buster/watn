# Arc42 Impact Assessment: Fix Fish Ctrl-W Completion Insertion

| # | Arc42 area | Affected | Assessment |
|---|---|---|---|
| 1 | Introduction and goals | No | The existing shell-integration usability and safety goals remain unchanged. |
| 2 | Architecture constraints | No | No language, runtime, provider, configuration, or deployment constraint changes. |
| 3 | Context and scope | Yes | The existing Fish line-editor boundary now specifies an actual newline in the replacement buffer rather than visible `\\n` text. |
| 4 | Solution strategy | No | The existing native shell-widget strategy remains in place; this is a Fish implementation correction within that strategy. |
| 5 | Building-block view | Yes | The existing Shell Shortcut and line-editor building blocks now document the corrected Fish buffer assembly; no new block is added. |
| 6 | Runtime view | Yes | The Ctrl-W flow now explicitly records that Fish receives an actual line break between the request comment and generated command. |
| 7 | Deployment view | Yes | Verification deployment documentation now records the interactive Fish PTY and isolated temporary startup environment; no production deployment artifact changes. |
| 8 | Cross-cutting concepts | Yes | Shell-widget safety documentation now distinguishes a real Fish newline from literal `\\n` text. |
| 9 | Architecture decisions | Yes | ADR-0018 is clarified to record Fish's shell-produced, collected newline as part of the existing native-widget decision; no new ADR is needed. |
| 10 | Quality requirements | Yes | QS-056 adds the Fish-specific observable buffer contract and acceptance metric. |
| 11 | Risks and technical debt | Yes | Existing shell-version and PTY risk mitigations are updated to reflect real Fish interactive coverage; no new risk is introduced. |
| 12 | Glossary | No | No new domain term is required; existing Shell widget and Request comment terms cover the behavior. |

## Status

STATUS: DONE
