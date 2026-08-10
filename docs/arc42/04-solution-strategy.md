# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, render tokens progressively
- Layered XDG configuration with clear precedence (CLI > env > user config > defaults)
- Optionally query LiteLLM endpoint for model discovery and interactive tier selection
- TTY-gated provider onboarding in a ratatui terminal flow, with OpenRouter as the default endpoint and `custom` as the fixed non-OpenRouter name
- Environment-backed API-key references persisted as `${VARIABLE}` and expanded only at request time
- Implicit first-use onboarding uses the shared five-page setup wizard in-process, then stops before the original question
- Explicit provider selections retain existing unknown-provider and missing-key errors; non-TTY implicit first use prints guidance and exits 1
- Direct config writes enforce Unix mode `0600`; no atomic temp-file/rename guarantee is made
- Interactive setup uses native Ratatui widget composition rather than paragraph-flattened or hand-positioned terminal output
- Provider and model onboarding share one page-based Ratatui wizard; commands select its initial and final page rather than owning separate event loops
- Test transport is a compile-time debug capability: only `test-support` plus `debug_assertions` can read the endpoint override; release-profile builds use configured endpoints even when the feature is enabled
- Debug verification builds the default-feature and `test-support` binaries sequentially through Cargo's shared default target cache, copies them to unique temporary paths, and passes those absolute paths to the subprocess harness; release runtime verification is deferred to `release-truth-and-repository-cleanup`
- Catalog source resolution is explicit: `[litellm]` owns model listing, pagination, and search when present; otherwise the selected provider is used, while chat construction remains provider-only
- Credential values retain their persisted source representation and are expanded only at the outbound discovery or chat request boundary; a confirmed provider draft is saved before its first catalog request
- A shared reasoning policy validates closed-set strengths, honors model default-enabled and mandatory metadata, and preserves existing valid reasoning when no replacement exists
- Search workers carry generations and can update the picker only when current; the test twin coordinates overlapping workers and joins them before exit

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking streaming SSE, TLS; chunks piped through mpsc channel for progressive rendering |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | dialoguer (existing non-TTY model prompts), ratatui/crossterm widgets (shared setup wizard) | One TTY-only wizard renders URL, API key, and three model pages with `Block`, `Tabs`, `Paragraph`, `Table`, and `Scrollbar`; command entry points choose the starting page |
| Filter matching | Per-word, order-independent substring over model id | "dee flash" matches "DeepSeek V4 Flash"; each word must appear anywhere in the id, any order |
| Gherkin runner | cucumber-rs | Mature Rust cucumber implementation |
| Pseudo-terminal testing | portable-pty | PTY-based E2E tests for the terminal dialog |
| Transport verification | `httpmock` loopback twins plus explicit subprocess paths | Proves endpoint, path, request count, Authorization, competing-server, and persistence behavior without live providers |
| Catalog resolution | Runtime-only catalog source value | Keeps LiteLLM discovery separate from active chat and makes optional catalog authentication explicit |
| Reasoning policy | Pure closed-set resolver | Prevents TTY, non-TTY, and request-body reasoning behavior from diverging |

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Usability | Default tier produces command in one invocation; execution confirmation is a single Enter |
| Flexibility | OpenAI-compatible wire protocol; model tiers configurable; LiteLLM discovery optional |
| Portability | Single Rust binary; no runtime deps |
| Observability | Model name, tok/s, cost printed per response; exit codes 0/1/2/3/130 |
| First-run usability and credential safety | The setup wizard has an OpenRouter default, explains compatibility, masks literal input, preserves environment references, makes the active cursor/page explicit, gates automatic setup on TTY, and stops after model selection when no implicit provider is ready |
| Transport isolation | The configured endpoint remains the source for readiness, persistence, and display; only debug test-support outbound requests may use a non-empty override, with missing/whitespace values falling back |
