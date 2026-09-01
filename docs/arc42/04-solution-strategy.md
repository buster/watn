# 4. Solution Strategy

## Key decisions

- OpenAPI-compatible wire protocol as the single provider integration point
- Three-tier model dispatch with user-configurable model assignment
- Streaming-first: always request SSE, parse through a buffered blocking reader, and render command content progressively through a synchronous callback with no channel
- Completion is strict: `[DONE]` is mandatory; EOF without it preserves visible content but is a network failure
- Reasoning is buffered in the provider aggregate and printed only after successful completion when `-v` is active; it is never streamed to stderr
- Layered XDG configuration with clear precedence (CLI > env > user config > defaults)
- Provider-derived catalog probing for model discovery and interactive tier selection
- Four focused TTY setup commands plus a coordinated ratatui flow, with explicit OpenRouter, OpenAI, and Custom provider choices
- Environment-backed API-key references persisted as `${VARIABLE}` and expanded only at request time
- Implicit first-use onboarding opens the plain-line quick setup when no configuration file exists, the coordinator when a configuration exists but provider or any required model role is incomplete; both stop before the original question
- Explicit provider selections retain existing unknown-provider and missing-key errors; non-TTY implicit first use prints guidance and exits 1
- Confirmed config snapshots use same-directory atomic replacement and Unix mode `0600`; shell target writes remain independent
- Interactive setup uses native Ratatui widget composition rather than paragraph-flattened or hand-positioned terminal output; the first-run quick setup is the deliberate plain-line exception (five suggested questions, no probing)
- Provider, model, shell, and coordinated onboarding share one draft/state-machine boundary; focused commands own only their domain persistence, and the quick setup reuses the same provider-migration and atomic-save seams
- Test transport is a compile-time debug capability: only `test-support` plus `debug_assertions` can read the endpoint override; release-profile builds use configured endpoints even when the feature is enabled
- Debug verification builds the default-feature and `test-support` binaries sequentially through Cargo's shared default target cache, copies them to unique temporary paths, and passes those absolute paths to the subprocess harness; release verification inspects the exact target artifact and its runtime libraries
- Catalog source resolution is provider-local: the selected provider owns model listing, pagination, and search; legacy `[litellm]` configuration is retained but not contacted
- Credential values retain their persisted source representation and are expanded only at the outbound discovery or chat request boundary; coordinated setup saves only after final review
- A shared reasoning policy offers catalog suggestions but persists and sends any non-empty value verbatim, with `off` omitted and mandatory metadata preventing `off`
- Model filtering uses the complete cached catalog locally when available and provider-backed search for incomplete catalogs; both paths keep the query visible, debounce remote work by 200 ms, guard results with generations, and join search workers before exit
- Generate shell completions from `Cli::command()` through a local closed `CompletionShell` selector; render only `bash`, `elvish`, `fish`, `powershell`, or `zsh` to stdout before configuration or provider setup
- Keep the completion parser contract literal: unsupported values return `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh`; the `completions` token is intentionally reserved as a subcommand
- Offer shortcut configuration as an optional post-Large-Model interaction in both explicit and implicit first-use setup; the default Enter path declines without adding a sixth tab
- Generate shell-native Ctrl-W widgets for Bash, Zsh, and Fish using `command watn -- "$question"`, capture-only substitution, trailing-CR/LF normalization, a `#`-prefixed request comment recorded in the shell history, a buffer holding only the generated command, and no evaluation
- Own startup-file edits through exact marker pairs, atomic same-directory replacement, and independent per-shell result aggregation rather than a multi-file transaction
- Use the existing SetupWizard focus state to color only the active widget border green, preserving the existing layout, selection styles, and cursor contract
- Treat the permanent Gherkin tree as one behavior inventory: deterministic
  ownership findings come before human dispositions, and consolidation removes
  weaker contracts instead of adding another scenario beside a stronger one
- Preserve distinct production boundaries explicitly in titles and review
  evidence; do not use embedding scores or scenario length as blocking policy

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language | Rust (latest stable) | Single binary, zero-cost streaming, strong typing for config |
| CLI parsing | clap v4 | Derive macros, `-1`/`-2`/`-3` flag groups, subcommand dispatch |
| HTTP client | reqwest (blocking) | Blocking SSE with a buffered reader and synchronous content callback for progressive rendering; no worker channel |
| Config format | TOML via `toml` crate | Rust ecosystem standard |
| Terminal interaction | ratatui/crossterm setup coordinator and the `model-picker` search module | TTY-only focused flows render provider, credential, catalog, separate model/reasoning questions, review, and shell desired state with `Block`, `Tabs`, `Paragraph`, `Table`, and `Scrollbar`; the focused input block has a green border |
| Filter matching | Per-word, order-independent substring over model id | "dee flash" matches "DeepSeek V4 Flash"; each word must appear anywhere in the id, any order |
| Gherkin runner | cucumber-rs | Mature Rust cucumber implementation |
| Pseudo-terminal testing | portable-pty | PTY-based E2E tests for the SetupWizard |
| Transport verification | `httpmock` loopback twins plus explicit subprocess paths | Proves endpoint, path, request count, Authorization, competing-server, and persistence behavior without live providers |
| Catalog resolution | Provider-local catalog state | Keeps catalog and chat paths provider-local while preserving a legacy `[litellm]` section as unrelated data |
| Reasoning policy | Pure non-empty string resolver | Prevents TTY, non-TTY, and request-body reasoning behavior from diverging while preserving provider-specific values |
| Completion generation | `clap_complete` renderers fed by `Cli::command()` and a local `CompletionShell` parser | Keeps generated options, subcommands, positional arguments, and selector values aligned with the authoritative CLI definition while avoiding config/provider side effects |
| Shell shortcut integration | Native Bash Readline, Zsh ZLE, and Fish commandline blocks plus standard filesystem writes | Preserves each shell's buffer/cursor API, keeps the installed executable on `PATH`, and confines mutation to selected marked startup-file blocks |

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Usability | Default tier produces command in one invocation; execution confirmation is a single Enter |
| Flexibility | OpenAI-compatible wire protocol; model tiers configurable; provider-local catalog endpoint editable |
| Portability | Release artifacts are verified on the selected host and documented with their dynamic runtime-library requirements; static portability is not claimed |
| Observability | Model name, tok/s, cost printed after `[DONE]`; buffered reasoning printed only under `-v` after success; exit codes 0/1/2/3/130 |
| Recovery | Visible content survives network/truncation failures; output I/O failures retain the prefix, clean up progress, omit metadata and execution, and use status 1 |
| First-run usability and credential safety | The quick setup asks six plain-line questions with suggestions (endpoint, credential, three models, shell selection), stores a `${VARIABLE}` reference instead of a secret, and performs no network request; the setup wizard has an OpenRouter default, explains compatibility, masks literal input, preserves environment references, makes the active cursor/page and green focused border explicit, gates automatic setup on TTY, and stops after model selection when no implicit provider is ready |
| Transport isolation | The configured endpoint remains the source for readiness, persistence, and display; only debug test-support outbound requests may use a non-empty override, with missing/whitespace values falling back |
| Completion safety | The selector is closed, success writes only deterministic script bytes to stdout, stderr remains empty, shell parsing is verified for each supported shell, and no config/provider operation is entered |
| Shortcut safety | Setup is opt-in, target markers are validated before writes, existing files are replaced atomically, every selected shell is attempted and reported, and widgets never evaluate their captured result |
