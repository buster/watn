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
| TTY detection | Runtime check of whether stdout is a terminal (interactive) or a pipe (scripting) |
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
| PTY | Pseudo-terminal — a virtual terminal device used in E2E tests to drive raw-mode terminal applications as a real user would |