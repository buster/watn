# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, render tokens progressively
- Layered XDG configuration with clear precedence (CLI > env > user config > defaults)
- Optionally query LiteLLM endpoint for model discovery and interactive tier selection

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking streaming SSE, TLS; chunks piped through mpsc channel for progressive rendering |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | dialoguer (non-TTY list prompts), ratatui (interactive model settings dialog) | Interactive model + reasoning selection in a keyboard-driven dialog; ratatui provides List/Layout/crossterm event handling |
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