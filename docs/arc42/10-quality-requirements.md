# 10. Quality Requirements

## Quality tree

- **Usability** — Command generation, execution confirmation, sensible defaults
  - QS-001: Default tier returns a shell command immediately
  - QS-002: `-x` prompts and executes on Enter
- **Onboarding** — A first-time TTY user can reach model setup without learning TOML, while non-TTY use remains actionable and script-safe
  - QS-011: Provider setup offers a usable OpenRouter default
  - QS-012: Missing provider automatically chains into model setup
  - QS-019: Setup information is separated into scannable terminal regions
   - QS-020: Background model search keeps the newest query authoritative
   - QS-021: The setup wizard makes the active page and cursor explicit
   - QS-022: Provider and model setup share one page sequence
   - QS-023: Test transport cannot redirect release-profile binaries by source guard
   - QS-024: Test transport assertions identify the exact endpoint and credential
   - QS-027: Normal debug transport ignores a non-empty test override
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
  - QS-009: Buffered reasoning content displayed on stderr after successful completion when verbose flag is set
  - QS-010: Exit code categories for scripting
  - QS-032: Command content is visible before a delayed stream completes
  - QS-033: A completed stream is distinguished from EOF without `[DONE]`
  - QS-034: Partial output and output failures have safe cleanup behavior

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
| QS-019 | Onboarding / Usability | User opens provider or model setup in a terminal | Provider setup exposes a titled border, selectable credential list, aligned detail rows, and guidance paragraph; model setup exposes a titled border, three tier tabs, aligned model columns, and a scrollbar for an overflowing catalog |
| QS-020 | Usability / Responsiveness | User changes the model filter while a provider search is delayed | The UI remains able to redraw and accept input; after the debounce only the newest query's results are applied, and an older result cannot replace them |
| QS-021 | Onboarding / Usability | User opens setup and edits a page | The active tab, page number, prompt, and visible cursor identify exactly what is being edited |
| QS-022 | Onboarding / Usability | User advances through setup | URL, API key, Small Model, Middle Model, and Large Model appear as one ordered wizard; Enter/Tab advances, Shift-Tab returns, and Escape presents save/discard |
| QS-023 | Security / Portability | The transport resolver is compiled for a release profile with `test-support` enabled | The negated `cfg(all(feature = "test-support", debug_assertions))` branch is selected and the release binary has no active override lookup; runtime smoke verification is deferred to `release-truth-and-repository-cleanup` |
| QS-024 | Security / Testability | A debug test-support request uses a local twin | The expected full URL and method/path are exact, request count is exactly 1, Authorization is exactly `Bearer sk-configured`, and the persisted configured endpoint is unchanged |
| QS-025 | Security / Testability | The override is absent or whitespace | The configured endpoint receives exactly 1 request with the exact Authorization header; the competing endpoint receives exactly 0; persisted TOML is unchanged |
| QS-026 | Correctness | Readiness is evaluated while a competing override is present | Readiness is true from configuration alone; both local twins receive exactly 0 requests |
| QS-027 | Security / Testability | One default-feature debug request runs with a non-empty override pointing at a competing local twin | The configured twin receives exactly 1 request with the exact method/path and `Authorization: Bearer sk-configured`; the competing twin receives exactly 0 requests; output comes from the configured twin and persisted TOML is unchanged |
| QS-028 | Correctness / Security | User runs `watn models` with `[litellm]` configured | List, pagination, and search requests use the exact LiteLLM endpoint; optional Authorization is present only when configured; chat remains on the active provider |
| QS-029 | Correctness / Recovery | User confirms a provider and catalog discovery fails or setup is cancelled | Provider source is persisted before catalog access; tiers remain unchanged; pre-confirmation cancellation writes nothing; no original chat request is sent |
| QS-030 | Correctness | Model metadata or persisted config contains disabled, mandatory, minimal, empty, or unknown reasoning values | Shared policy selects a valid strength, persists/sends `minimal` when selected, excludes `off` for mandatory models, and emits no reasoning field for empty/unknown values |
| QS-031 | Responsiveness / Correctness | Slow and fast model searches overlap | Fast/newest IDs remain visible after the slow result completes; stale IDs are absent and all workers are cleaned up before scenario exit |
| QS-032 | Responsiveness / Usability | A provider flushes a first command event and delays a later event | The first content is observable before release; the spinner was visible beforehand and a clear-line cleanup is observable after first content; the complete generated command appears exactly once after success |
| QS-033 | Correctness / Recovery | A provider sends `[DONE]` and keeps the connection open, or closes without `[DONE]` | `[DONE]` permits exit 0 before connection close; EOF without `[DONE]` preserves visible content, reports a network error, emits no success metadata or prompt, and exits 3 |
| QS-034 | Observability / Correctness | Verbose reasoning and command content arrive in the same stream | Command stdout is visible before completion; reasoning is absent before completion, then appears on stderr only under `-v` after `[DONE]`; stdout never contains reasoning |
| QS-035 | Correctness / Observability | A choices-empty usage event supplies response model and usage after content | Final metadata names the response model rather than the requested model, uses pricing for that response model, and contains a non-zero cost and positive throughput |
| QS-036 | Recovery / Usability | A provider resets or reaches EOF after a visible command prefix | The prefix remains visible, spinner clear-line cleanup is observable, mapped network status is 3, final success metadata is absent, and no execute prompt appears |
| QS-037 | Recovery / Correctness | A command-output write or flush fails after a visible prefix | The existing I/O error status 1 is returned; the prefix and spinner cleanup remain observable; final metadata and execution confirmation are omitted |
| QS-038 | Correctness / Usability | A command is confirmed from a raw terminal or a pipe | The generated command is one complete line exactly once; execution output is asserted separately and occurs only after successful completion and confirmation |
