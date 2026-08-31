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
| LiteLLM | A legacy configuration section and optional external proxy retained as unrelated data; streamlined setup does not contact or migrate it for model discovery |
| Raw output | Plain text without ANSI escape codes; suitable for scripting and pipes |
| TTY detection | Runtime check of whether stdin is a terminal (interactive) or a pipe (scripting); automatic onboarding requires an implicit selection and a TTY |
| Tokens/second | Completion tokens divided by wall-clock seconds from the first non-DONE SSE data event, before decoding, to the DONE marker |
| Pricing | Per-model cost configuration ($/1M input tokens, $/1M output tokens) stored in config |
| Reasoning | Explanation produced by the LLM alongside the final answer. Accepted from `reasoning` or `reasoning_content`, buffered in the provider aggregate, and displayed on stderr only after successful completion when `-v`/`--verbose` is set |
| Buffered reasoning | Provider-collected reasoning that is deliberately not emitted during the content stream and is discarded from user-visible output when the stream fails |
| Partial output | Command content already flushed before a provider or output failure; it remains visible but is never treated as a successful executable result |
| Model picker | The SetupWizard model-page search flow that updates suggestions as the user types and applies a stale-generation guard to remote results |
| Setup coordinator | The ratatui keyboard-driven draft flow for provider, completion endpoint, credential, provider-local catalog, separate model/reasoning questions, shell desired state, and final review |
| Reasoning effort | A non-empty per-level string controlling `reasoning_effort`; `off` sends no field and every other value is persisted and sent verbatim |
| Guided sequence | The fixed provider → catalog → small model/reasoning → normal model/reasoning → thinking model/reasoning → shell desired-state → review walk, with back-navigation before confirmation |
| Page navigation | Moving the selection through the model list by a full page at a time with PageUp/PageDown keys |
| Per-word filter | Order-independent matching where every whitespace-separated word of the query must appear somewhere in the model id |
| Search query | Free-text filter sent to the provider as `GET /models?search=<query>` to narrow a large model catalog |
| Stale-result guard | A generation counter that prevents an older, slower API response from overwriting newer suggestions already displayed |
| PTY | Pseudo-terminal — a virtual terminal device used in E2E tests to drive the ratatui/crossterm terminal interface as a real user would |
| Provider readiness | Local determination that a provider endpoint and usable literal or environment-backed credential are available without contacting the provider |
| Environment-backed credential | A credential stored in config as a reference such as `${OPENROUTER_API_KEY}` and resolved from the process environment only when used |
| Provider setup | The focused interactive ratatui flow started by `watn provider` to collect provider identity, completion endpoint, and credential source without probing models |
| First-run onboarding | The TTY-only automatic coordinator opened when provider or any required model role is incomplete; successful setup ends before the original request and cancellation leaves no new file |
| Fixed provider name | The stable onboarding name `openrouter` for the normalized OpenRouter endpoint or `custom` for every other endpoint; a selected arbitrary provider key migrates to `custom` |
| Setup result | A typed focused or coordinated outcome: confirmed domain changes, cancelled draft, or validation/IO failure without an unintended write |
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
| Catalog completeness | Whether the loaded model response contains the complete catalog needed for local filtering or only a page requiring provider-backed search |
| Local model filter | Per-word filtering over the complete cached catalog without a provider search request |
| Search worker | The background operation that waits for the debounce interval, performs an incomplete-catalog search, and is joined before setup exits |
| Setup flow | One of the focused provider, models, shell, or coordinated terminal flows; only the active question accepts editing input |
| Setup question | One focused step in a setup flow, such as provider choice, model selection, reasoning, or shell desired state |
| Visible cursor | The highlighted block marker showing where the next character or current selection is being edited |
| Active input | The setup widget that currently receives keyboard input; its border is rendered green while inactive widget borders retain their normal style |
| Focused widget | The Ratatui-rendered setup region selected by the current credential, model, or shortcut focus state |
| Save/discard prompt | The Escape confirmation that confirms valid focused-domain settings or abandons the in-memory draft; coordinated setup writes only after final review confirmation |
| Catalog source | The selected provider's saved or derived endpoint used for model listing, pagination, and search; legacy LiteLLM is not consulted by streamlined setup |
| Credential source | The persisted origin of a secret: literal value, exact `${VARIABLE}` reference, or absent source |
| Provider draft | An in-memory provider identity, endpoint, credential representation, and provider-local catalog state awaiting focused or coordinated confirmation |
| Coordinated draft | The complete in-memory provider, catalog, model, reasoning, and shell desired state held apart from the persisted baseline |
| Final confirmation | The only coordinated configuration write boundary; it serializes one complete candidate snapshot or leaves the baseline unchanged |
| Reasoning policy | The shared rule that offers catalog suggestions, rejects blank values, preserves non-empty values verbatim, and omits only `off` |
| Provider migration | Moving a selected arbitrary provider key to canonical `custom`, removing the selected source key, and applying deterministic collision/default-model rules |
| Provider-local catalog endpoint | A saved or derived `/models` base belonging to the selected provider and authorized with that provider's credential |
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
| Shell widget | A shell-native line-editor function that reads the current buffer, calls `command watn -- "$question"`, records the request as a `#`-prefixed history comment, and replaces the buffer with the generated text without evaluation |
| Request comment | The flattened original request prefixed with `#` that the widget records in the shell history so the request stays recallable and re-askable |
| Request flattening | Replacing CR, LF, and TAB in the captured request with spaces so it forms exactly one comment line |
| History recording | The per-shell native append used at Ctrl-W time: Bash `history -s`, Zsh `print -s`, Fish `builtin history append` |
| Marker pair | The exact opening and closing comments `# >>> watn shell shortcut >>>` and `# <<< watn shell shortcut <<<` delimiting one generated block |
| Target result | The success or failure report for one selected shortcut target, including its path, reason, and reload instruction when applicable |
| Aggregate installation failure | A non-zero setup result returned after all selected targets have been attempted and one or more target results failed |
| Reload instruction | The shell command or guidance that makes a modified startup file active in the current or next shell |
