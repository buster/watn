# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, render tokens progressively
- Layered XDG configuration with clear precedence (CLI > env > user config > defaults)
- Optionally query LiteLLM endpoint for model discovery and interactive tier selection
- TTY-gated provider onboarding in a ratatui terminal flow, with OpenRouter as the default endpoint and `custom` as the fixed non-OpenRouter name
- Environment-backed API-key references persisted as `${VARIABLE}` and expanded only at request time
- Implicit first-use onboarding chains provider setup into the existing model setup in-process, then stops before the original question
- Explicit provider selections retain existing unknown-provider and missing-key errors; non-TTY implicit first use prints guidance and exits 1
- Direct config writes enforce Unix mode `0600`; no atomic temp-file/rename guarantee is made
- Interactive setup uses native Ratatui widget composition rather than paragraph-flattened or hand-positioned terminal output

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking streaming SSE, TLS; chunks piped through mpsc channel for progressive rendering |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | dialoguer (existing non-TTY model prompts), ratatui/crossterm widgets (provider and model settings dialogs) | Interactive provider and model + reasoning selection in keyboard-driven dialogs; `Block`, `List`, `Table`, `Paragraph`, `Tabs`, and `Scrollbar` make state, metadata, and overflow visible while onboarding remains TTY-only |
| Filter matching | Per-word, order-independent substring over model id | "dee flash" matches "DeepSeek V4 Flash"; each word must appear anywhere in the id, any order |
| Gherkin runner | cucumber-rs | Mature Rust cucumber implementation |
| Pseudo-terminal testing | portable-pty | PTY-based E2E tests for the terminal dialog |

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Usability | Default tier produces command in one invocation; execution confirmation is a single Enter |
| Flexibility | OpenAI-compatible wire protocol; model tiers configurable; LiteLLM discovery optional |
| Portability | Single Rust binary; no runtime deps |
| Observability | Model name, tok/s, cost printed per response; exit codes 0/1/2/3/130 |
| First-run usability and credential safety | Provider setup has an OpenRouter default, masks literal input, preserves environment references, gates automatic setup on TTY, and stops after model selection when no implicit provider is ready |
