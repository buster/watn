# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, render tokens progressively; non-streaming as config toggle
- Layered XDG configuration with clear precedence (CLI > env > user config > system > defaults)
- Optionally query LiteLLM endpoint for model discovery and interactive tier selection

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking streaming SSE, TLS; chunks piped through mpsc channel for progressive rendering |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | dialoguer (list prompts), console (raw terminal for autosuggest) | Interactive model selection; console is already a transitive dep, promoted to explicit |
| Gherkin runner | cucumber-rs | Mature Rust cucumber implementation |
| Pseudo-terminal testing | portable-pty | PTY-based E2E tests for raw-mode terminal pickers |

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Usability | Default tier produces command in one invocation; execution confirmation is a single Enter |
| Flexibility | OpenAI-compatible wire protocol; model tiers configurable; LiteLLM discovery optional |
| Portability | Single Rust binary; no runtime deps |
| Observability | Model name, tok/s, cost printed per response; exit codes 0/1/2/3/130 |