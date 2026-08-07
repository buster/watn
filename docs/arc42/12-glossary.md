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