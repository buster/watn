# 12. Glossary

| Term | Definition |
|---|---|
| Provider | An LLM API service (e.g. OpenAI) or any OpenAI-compatible endpoint accepting `/v1/chat/completions` |
| Tier | A named difficulty level (`small`, `normal`, `thinking`) mapped to a concrete model ID in config |
| Streaming | Receiving complete LLM response events over SSE as they are generated and flushing command content through the CLI sink |
| SSE | Server-Sent Events — HTTP streaming protocol for progressive token delivery |
| Content event | A complete, valid SSE JSON event containing a non-empty command delta; it is the only event sent to the incremental CLI output sink |
| Stream sink | The synchronous CLI callback that writes and flushes command content; it is not a worker channel and does not render reasoning |
| DONE marker | The exact SSE data payload `[DONE]` that is required for successful stream completion |
| Truncated stream | A provider response that reaches EOF or a read failure before the DONE marker; visible content is preserved but the result is a network error |
| XDG | XDG Base Directory Specification; watn uses the config directory (`$XDG_CONFIG_HOME/watn/`, normally `~/.config/watn/`) and does not use an XDG data directory |
| LiteLLM | A proxy that exposes multiple LLM providers behind a single OpenAI-compatible API and provides a `/models` endpoint for discovery |
| Raw output | Plain text without ANSI escape codes; suitable for scripting and pipes |
| TTY detection | Runtime check of whether stdin is a terminal (interactive) or a pipe (scripting); automatic onboarding requires an implicit selection and a TTY |
| Tokens/second | Completion tokens divided by wall-clock seconds from the first non-DONE SSE data event, before decoding, to the DONE marker |
| Pricing | Per-model cost configuration ($/1M input tokens, $/1M output tokens) stored in config |
| Reasoning | Explanation produced by the LLM alongside the final answer. Accepted from `reasoning` or `reasoning_content`, buffered in the provider aggregate, and displayed on stderr only after successful completion when `-v`/`--verbose` is set |
| Buffered reasoning | Provider-collected reasoning that is deliberately not emitted during the content stream and is discarded from user-visible output when the stream fails |
| Partial output | Command content already flushed before a provider or output failure; it remains visible but is never treated as a successful executable result |
| Model picker | The SetupWizard model-page search flow that updates suggestions as the user types and applies a stale-generation guard to remote results |
| SetupWizard | The ratatui keyboard-driven five-page flow for URL, API key, Small Model, Middle Model, and Large Model configuration |
| Reasoning strength | Graduated per-level setting (`off`, `low`, `minimal`, `medium`, `high`) controlling the `reasoning_effort` sent on that tier's requests; `off` sends no field |
| Guided sequence | The fixed Small Model → Middle Model → Large Model walk performed by the SetupWizard, with the ability to return to a previous page before confirming |
| Page navigation | Moving the selection through the model list by a full page at a time with PageUp/PageDown keys |
| Per-word filter | Order-independent matching where every whitespace-separated word of the query must appear somewhere in the model id |
| Search query | Free-text filter sent to the provider as `GET /models?search=<query>` to narrow a large model catalog |
| Stale-result guard | A generation counter that prevents an older, slower API response from overwriting newer suggestions already displayed |
| PTY | Pseudo-terminal — a virtual terminal device used in E2E tests to drive the ratatui/crossterm terminal interface as a real user would |
| Provider readiness | Local determination that a provider endpoint and usable literal or environment-backed credential are available without contacting the provider |
| Environment-backed credential | A credential stored in config as a reference such as `${OPENROUTER_API_KEY}` and resolved from the process environment only when used |
| Provider setup | The interactive ratatui flow started by `watn provider` to collect an endpoint and credential source |
| First-run onboarding | The TTY-only automatic provider setup followed by model setup when an implicitly selected provider is not ready; successful setup ends before the original request |
| Fixed provider name | The stable onboarding name `openrouter` for the normalized OpenRouter endpoint or `custom` for every other endpoint; reruns replace only that entry |
| Setup result | A typed provider/model outcome: configured or saved, cancelled by Escape/Ctrl-C, or failed without lower-level process exit |
| Ephemeral transport override | A non-empty endpoint selected at outbound HTTP construction only by a debug binary compiled with `test-support`; it is not persisted and is not used for readiness |
| Test-support binary | A binary compiled with the opt-in `test-support` feature; only its debug profile may use the ephemeral transport override |
| Release-profile binary | A binary built with a release profile; it never reads the test transport override, even when `test-support` is enabled |
| Configured endpoint | The `<base>/v1` endpoint loaded from provider configuration and retained for readiness, display, and persistence |
| Competing provider twin | A separate local mock server intentionally configured as the wrong destination so a redirected request is observable as a non-zero hit |
| Automatic setup completion | Successful first-use provider and model setup that exits after tier selection without sending the original question |
| Ratatui widget | A Ratatui-rendered terminal region with its own layout, border, selection, or text responsibility |
| Tier tabs | The visible small, normal, and thinking labels showing which model-assignment level is active |
| Model table | The aligned model catalog showing model identity and available metadata in columns |
| Scrollbar | The terminal indicator showing the current position within an overflowing model catalog |
| Debounced search | A search started only after the user has stopped changing the filter for the configured short interval |
| Search generation | The monotonically increasing user-entry identifier used to reject stale model-search results, regardless of completion order |
| Setup wizard | The shared five-page terminal flow for URL, API key, Small Model, Middle Model, and Large Model configuration |
| Setup page | One tab-selected step in the setup wizard; only the active page accepts editing input |
| Visible cursor | The highlighted block marker showing where the next character or current selection is being edited |
| Save/discard prompt | The Escape confirmation that persists valid current settings or abandons all unsaved changes |
| Catalog source | The endpoint used for model listing, pagination, and search; configured LiteLLM takes precedence over the active provider |
| Credential source | The persisted origin of a secret: literal value, exact `${VARIABLE}` reference, or absent source |
| Provider draft | A validated endpoint and credential source confirmed before complete model-tier setup |
| Reasoning policy | The shared rule that resolves model metadata and persisted values to a valid reasoning strength |
| Valid reasoning strength | One of `off`, `low`, `minimal`, `medium`, or `high`; unknown and empty values mean no reasoning request |
| Completion script | A Bash, Elvish, Fish, PowerShell, or Zsh script generated from watn's current Clap command definition for a caller to install or source |
| CompletionShell | The local closed selector for completion output; it accepts only `bash`, `elvish`, `fish`, `powershell`, and `zsh` and is not exposed as `clap_complete::Shell` |
| Completion selector value | One of the lowercase literals `bash`, `elvish`, `fish`, `powershell`, or `zsh`; every other value is unsupported |
| Completion generation | The side-effect-free CLI path that parses a selector, renders the authoritative command tree, and writes only the script to stdout |
| Unsupported-shell contract | The parser-owned literal `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` embedded in a normal non-zero argument error |
| Shell parser check | Validation of generated Bash, Elvish, Fish, PowerShell, or Zsh text by the corresponding installed shell executable when available |
| Reserved completion token | The unquoted first token `completions`, which dispatches to the completion subcommand; question text using it must be quoted or placed after `--` |
| Authoritative command definition | The `Cli::command()` metadata used by Clap for parsing/help and by completion generation for options, positional arguments, subcommands, and values |
| Provider-request sentinel | The isolated test HTTP mock whose request count proves completion generation did not contact a provider |
| Shell shortcut | An optional Ctrl-W binding installed in a selected Bash, Zsh, or Fish startup file to turn the current command buffer into one `watn` question |
| Shortcut target | The resolved user startup file for one supported shell: `.bashrc`, `.zshrc`, or Fish `config.fish` under the XDG configuration directory |
| Generated block | The marker-delimited shell text owned by watn inside a shortcut target; unrelated target bytes remain user-owned |
| Shell widget | A shell-native line-editor function that reads the current buffer, calls `command watn -- "$question"`, and assigns successful stdout as text |
| Marker pair | The exact opening and closing comments `# >>> watn shell shortcut >>>` and `# <<< watn shell shortcut <<<` delimiting one generated block |
| Target result | The success or failure report for one selected shortcut target, including its path, reason, and reload instruction when applicable |
| Aggregate installation failure | A non-zero setup result returned after all selected targets have been attempted and one or more target results failed |
| Reload instruction | The shell command or guidance that makes a modified startup file active in the current or next shell |
