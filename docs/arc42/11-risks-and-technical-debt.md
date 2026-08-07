# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | OpenAI API schema drifts from the implemented subset | Low | Medium | Pin tested provider versions in CI; add schema-version field to config |
| R-002 | Users configure providers with non-standard `/v1/chat/completions` paths | Medium | Low | Document the requirement; validate endpoint returns 200 on OPTIONS |
| R-003 | Config file with secrets (API keys) committed to version control | Medium | High | Support env-var-only API key config; warn if config file is world-readable |
| R-004 | LiteLLM `/models` endpoint schema drifts | Low | Low | Parse response leniently; error gracefully on unexpected format |
| R-005 | User confirms execution and the command is destructive | Low | High | Tool prints the command before prompting; user sees what they are confirming |
| R-006 | Execution flow UX — command printed to stdout before prompt, user may interpret as already-executed | Medium | Medium | Command printed to stderr with prompt, execution output to stdout |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Non-OpenAI-compatible provider adapters | Medium | Provider trait is designed for extension; new adapters per provider |
| TD-002 | No input validation of shell commands before execution | Medium | User sees full command before confirming; add dry-run preview in future |