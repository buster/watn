# 10. Quality Requirements

## Quality tree

- **Usability** — Command generation, execution confirmation, sensible defaults
  - QS-001: Default tier returns a shell command immediately
  - QS-002: `-x` prompts and executes on Enter
- **Onboarding** — A first-time TTY user can reach model setup without learning TOML, while non-TTY use remains actionable and script-safe
  - QS-011: Provider setup offers a usable OpenRouter default
  - QS-012: Missing provider automatically chains into model setup
- **Flexibility** — Provider-agnostic, config-driven, model tiering
  - QS-003: Custom OpenAI-compatible provider
  - QS-004: Model tier assignment via config
  - QS-005: Model discovery via LiteLLM
- **Security** — Credentials are not unnecessarily copied or exposed
  - QS-013: Environment-backed credentials remain references in config
  - QS-014: Literal credentials are masked during entry
  - QS-015: Every config save repairs Unix mode to `0600`
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
| QS-011 | Onboarding | User runs `watn provider` and accepts the default | The setup displays `https://openrouter.ai/api/v1` and persists it as the provider endpoint |
| QS-012 | Onboarding | User runs a normal command with no recognized provider from a TTY | Provider setup starts automatically, model setup follows without a second command, setup exits after tier selection, and no original chat request is sent |
| QS-013 | Security | User chooses `OPENROUTER_API_KEY` as the credential source | Config contains `${OPENROUTER_API_KEY}`, not its resolved value; a later request uses the environment value |
| QS-014 | Security | User pastes a credential into provider setup | Terminal output masks the input and the resolved secret is absent from setup status output |
| QS-015 | Security | A provider or model save updates an existing config file | Unix mode is exactly `0600` after the direct write |
| QS-016 | Onboarding | User runs first use without a TTY and without a ready implicit provider | Exit 1; stderr names `watn provider` and the config path; no ratatui, `/models`, or chat request starts |
| QS-017 | Onboarding | Provider setup succeeds but model setup is cancelled or fails | Provider remains saved, onboarding stops, and the original chat request is not sent; Escape is 1 and Ctrl-C is 130 |
| QS-018 | Flexibility | A saved `[providers.openrouter]` entry exists | Its endpoint and credential representation take precedence over the built-in OpenRouter fallback |
