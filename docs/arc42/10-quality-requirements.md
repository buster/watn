# 10. Quality Requirements

## Quality tree

- **Usability** — Command generation, execution confirmation, sensible defaults
  - QS-001: Default tier returns a shell command immediately
  - QS-002: `-x` prompts and executes on Enter
- **Flexibility** — Provider-agnostic, config-driven, model tiering
  - QS-003: Custom OpenAI-compatible provider
  - QS-004: Model tier assignment via config
  - QS-005: Model discovery via LiteLLM
- **Portability** — Single binary, no runtime deps
  - QS-006: Standalone binary across Linux/macOS
- **Observability** — Model, speed, cost, reasoning in output
  - QS-007: Tokens/second displayed after response
  - QS-008: Cost displayed when pricing configured
  - QS-009: Reasoning content displayed on stderr when verbose flag is set
  - QS-010: Exit code categories for scripting

## Quality scenarios

| ID | Quality attribute | Scenario | Metric / Acceptance criterion |
|---|---|---|---|
| QS-001 | Usability | User runs `watn "find . -name *.rs"` | Output contains the find command; tokens/sec and model name are printed |
| QS-002 | Usability | User runs `watn -x "echo ok"` and presses Enter | Command executes; "ok" appears on stdout |
| QS-003 | Flexibility | User configures custom provider in config | Request is sent to the configured endpoint |
| QS-004 | Flexibility | User assigns models to tiers in config | `-1` uses small model, `-3` uses thinking model |
| QS-005 | Flexibility | User runs `watn models` with LiteLLM endpoint | Interactive selection works; tier config is persisted |
| QS-006 | Portability | User downloads single binary on clean machine | Binary runs without runtime installation |
| QS-007 | Observability | Response completes | Output contains "tokens/s" with a numeric value |
| QS-008 | Observability | Pricing configured in config | Output contains "cost:" with a monetary value |
| QS-009 | Observability | User runs `watn -3 -v "..."` and API returns reasoning | Reasoning content printed to stderr on its own line |
| QS-010 | Observability | API returns HTTP 429 | Exit code 2; stderr contains "rate limit" |