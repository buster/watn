# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, parse through a buffered blocking reader, and render command content progressively through a synchronous callback with no channel
- Completion is strict: `[DONE]` is mandatory; EOF without it preserves visible content but is a network failure
- Reasoning is buffered in the provider aggregate and printed only after successful completion when `-v` is active; it is never streamed to stderr
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
- Debug verification builds the default-feature and `test-support` binaries sequentially through Cargo's shared default target cache, copies them to unique temporary paths, and passes those absolute paths to the subprocess harness; release verification inspects the exact target artifact and its runtime libraries
- Catalog source resolution is explicit: `[litellm]` owns model listing, pagination, and search when present; otherwise the selected provider is used, while chat construction remains provider-only
- Credential values retain their persisted source representation and are expanded only at the outbound discovery or chat request boundary; a confirmed provider draft is saved before its first catalog request
- A shared reasoning policy validates closed-set strengths, honors model default-enabled and mandatory metadata, and preserves existing valid reasoning when no replacement exists
- Model filtering uses the complete cached catalog locally when available and provider-backed search for incomplete catalogs; both paths keep the query visible, debounce remote work by 200 ms, guard results with generations, and join search workers before exit
- Generate shell completions from `Cli::command()` through a local closed `CompletionShell` selector; render only `bash`, `elvish`, `fish`, `powershell`, or `zsh` to stdout before configuration or provider setup
- Keep the completion parser contract literal: unsupported values return `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh`; the `completions` token is intentionally reserved as a subcommand
- Offer shortcut configuration as an optional post-Large-Model interaction in both explicit and implicit first-use setup; the default Enter path declines without adding a sixth tab
- Generate shell-native Ctrl-W widgets for Bash, Zsh, and Fish using `command watn -- "$question"`, capture-only substitution, trailing-CR/LF normalization, a preserved request comment above the generated command, and no evaluation
- Own startup-file edits through exact marker pairs, atomic same-directory replacement, and independent per-shell result aggregation rather than a multi-file transaction
- Use the existing SetupWizard focus state to color only the active widget border green, preserving the existing layout, selection styles, and cursor contract

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking SSE with a buffered reader and synchronous content callback for progressive rendering; no worker channel |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | ratatui/crossterm `SetupWizard` and the `model-picker` search module | One TTY-only wizard renders URL, API key, and three model pages with `Block`, `Tabs`, `Paragraph`, `Table`, and `Scrollbar`; the focused input block has a green border; command entry points choose the starting page |
| Filter matching | Per-word, order-independent substring over model id | "dee flash" matches "DeepSeek V4 Flash"; each word must appear anywhere in the id, any order |
| Gherkin runner | cucumber-rs | Mature Rust cucumber implementation |
| Pseudo-terminal testing | portable-pty | PTY-based E2E tests for the SetupWizard |
| Transport verification | `httpmock` loopback twins plus explicit subprocess paths | Proves endpoint, path, request count, Authorization, competing-server, and persistence behavior without live providers |
| Catalog resolution | Runtime-only catalog source value | Keeps LiteLLM discovery separate from active chat and makes optional catalog authentication explicit |
| Reasoning policy | Pure closed-set resolver | Prevents TTY, non-TTY, and request-body reasoning behavior from diverging |
| Completion generation | `clap_complete` renderers fed by `Cli::command()` and a local `CompletionShell` parser | Keeps generated options, subcommands, positional arguments, and selector values aligned with the authoritative CLI definition while avoiding config/provider side effects |
| Shell shortcut integration | Native Bash Readline, Zsh ZLE, and Fish commandline blocks plus standard filesystem writes | Preserves each shell's buffer/cursor API, keeps the installed executable on `PATH`, and confines mutation to selected marked startup-file blocks |

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Usability | Default tier produces command in one invocation; execution confirmation is a single Enter |
| Flexibility | OpenAI-compatible wire protocol; model tiers configurable; LiteLLM discovery optional |
| Portability | Release artifacts are verified on the selected host and documented with their dynamic runtime-library requirements; static portability is not claimed |
| Observability | Model name, tok/s, cost printed after `[DONE]`; buffered reasoning printed only under `-v` after success; exit codes 0/1/2/3/130 |
| Recovery | Visible content survives network/truncation failures; output I/O failures retain the prefix, clean up progress, omit metadata and execution, and use status 1 |
| First-run usability and credential safety | The setup wizard has an OpenRouter default, explains compatibility, masks literal input, preserves environment references, makes the active cursor/page and green focused border explicit, gates automatic setup on TTY, and stops after model selection when no implicit provider is ready |
| Transport isolation | The configured endpoint remains the source for readiness, persistence, and display; only debug test-support outbound requests may use a non-empty override, with missing/whitespace values falling back |
| Completion safety | The selector is closed, success writes only deterministic script bytes to stdout, stderr remains empty, shell parsing is verified for each supported shell, and no config/provider operation is entered |
| Shortcut safety | Setup is opt-in, target markers are validated before writes, existing files are replaced atomically, every selected shell is attempted and reported, and widgets never evaluate their captured result |
