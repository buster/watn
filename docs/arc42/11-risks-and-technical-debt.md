# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | OpenAI API schema drifts from the implemented subset | Low | Medium | Pin tested provider versions in CI; add schema-version field to config |
| R-002 | Users configure providers with non-standard `/v1/chat/completions` paths | Medium | Low | Document the requirement |
| R-003 | Config file with secrets (API keys) committed to version control | Medium | High | Support env-var-only API key config; warn if config file is world-readable |
| R-004 | LiteLLM `/models` endpoint schema drifts | Low | Low | Parse response leniently; error gracefully on unexpected format |
| R-005 | User confirms execution and the command is destructive | Low | High | Tool prints the command before prompting; user sees what they are confirming |
| R-006 | Execution flow UX — command printed to stdout before prompt, user may interpret as already-executed | Medium | Medium | Command printed to stderr with prompt, execution output to stdout |
| R-007 | Model returns empty reasoning tokens on thinking tier | Medium | Low | Trim reasoning content before printing; skip print if content is empty/whitespace after trimming |
| R-008 | Template config generated from code may include irrelevant fields as Config grows | Low | Low | Template is a starting point — users delete what they do not need; template is meant to be edited |
| R-009 | PTY-based E2E tests are flaky across platforms/terminal emulators | Medium | Medium | Run PTY tests in CI with a known terminal type (`TERM=dumb` or `xterm-256color`); add generous read timeouts; document that local test failures may require `script`/`unbuffer` wrappers |
| R-010 | Arrow/page escape sequences differ across terminal emulators in the ratatui dialog | Low | Medium | Standard sequences (`\x1b[A/B`, `\x1b[5~/6~`); PTY E2E tests pin `TERM=xterm-256color`; ratatui/crossterm parse both classic and application-cursor modes |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Non-OpenAI-compatible provider adapters | Medium | Provider trait is designed for extension; new adapters per provider |
| TD-002 | No input validation of shell commands before execution | Medium | User sees full command before confirming |
| TD-003 | Raw terminal input behavior varies across terminal emulators | Low | Key bindings use standard sequences (arrows, backspace, enter, escape, ctrl-c); non-standard terminals may require `TERM` detection fallbacks |
| TD-004 | Reasoning config parsing edge cases (unknown strength values read from an edited config) | Low | Parse leniently; only `off`/`low`/`medium`/`high` map to `reasoning_effort`, unknown values fall back to no reasoning |