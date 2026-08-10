# 12. Glossary

| Term | Definition |
|---|---|
| Provider | An LLM API service (e.g. OpenAI) or any OpenAI-compatible endpoint accepting `/v1/chat/completions` |
| Tier | A named difficulty level (`small`, `normal`, `thinking`) mapped to a concrete model ID in config |
| Streaming | Receiving the LLM response token by token over SSE as it is generated |
| SSE | Server-Sent Events — HTTP streaming protocol for progressive token delivery |
| XDG | XDG Base Directory Specification — standard for config (`~/.config`), data (`~/.local/share`), and cache (`~/.cache`) paths |
| LiteLLM | A proxy that exposes multiple LLM providers behind a single OpenAI-compatible API and provides a `/models` endpoint for discovery |
| Raw output | Plain text without ANSI escape codes; suitable for scripting and pipes |
| TTY detection | Runtime check of whether stdin is a terminal (interactive) or a pipe (scripting); automatic onboarding requires an implicit selection and a TTY |
| Tokens/second | Completion tokens divided by wall-clock seconds from first to last SSE chunk |
| Pricing | Per-model cost configuration ($/1M input tokens, $/1M output tokens) stored in config |
| Reasoning | Chain-of-thought or step-by-step explanation produced by the LLM alongside the final answer. Exposed via the API's `reasoning` field in the streaming delta. Displayed on stderr when `-v`/`--verbose` is set. |
| Autosuggest picker | Raw-terminal input loop that updates a suggestion list as the user types; replaces the static scrollable list for model tier assignment |
| SettingsDialog | Ratatui keyboard-driven dialog that walks the small/normal/thinking levels in a guided sequence, showing the filter, the highlighted model list, and a reasoning-strength selector per level |
| Reasoning strength | Graduated per-level setting (`off`, `low`, `medium`, `high`) controlling the `reasoning_effort` sent on that tier's requests |
| Guided sequence | The fixed small → normal → thinking walk performed by the settings dialog, with the ability to return to a previous level before confirming |
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
| Widget | A Ratatui-rendered terminal region with its own layout, border, selection, or text responsibility |
| Tier tabs | The visible small, normal, and thinking labels showing which model-assignment level is active |
| Model table | The aligned model catalog showing model identity and available metadata in columns |
| Scrollbar | The terminal indicator showing the current position within an overflowing model catalog |
| Debounced search | A search started only after the user has stopped changing the filter for the configured short interval |
| Search generation | The monotonically increasing query identifier used to reject stale model-search results |
| Setup wizard | The shared five-page terminal flow for URL, API key, Small Model, Middle Model, and Large Model configuration |
| Setup page | One tab-selected step in the setup wizard; only the active page accepts editing input |
| Visible cursor | The highlighted block marker showing where the next character or current selection is being edited |
| Save/discard prompt | The Escape confirmation that persists valid current settings or abandons all unsaved changes |
