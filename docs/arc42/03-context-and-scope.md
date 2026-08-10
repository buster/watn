# 3. Context and Scope

## Business context

```mermaid
graph TB
    User["Developer<br/>(terminal user)"]
    CLI["watn CLI"]
    API["OpenAI-compatible API<br/>(LLM provider)"]
    LiteLLM["LiteLLM endpoint<br/>(optional, model discovery)"]
    Shell["System shell<br/>(execution mode)"]

    User -->|"question via args/stdin"| CLI
    User -->|"endpoint and credential input in provider setup"| CLI
    User -->|"rerun the original request after successful automatic setup"| CLI
    CLI -->|"POST /v1/chat/completions"| API
    CLI -->|"GET /models (optional)"| LiteLLM
    CLI -->|"GET /models?search=..."| LiteLLM
    CLI -->|"sh -c (when -x flag)"| Shell
    User -->|"keyboard input (arrows / PageUp / PageDown / Enter / Escape / Tab) in the model settings dialog"| CLI
    CLI -->|"write provider endpoint and credential representation"| Config
    API -->|"SSE streaming response"| CLI
    LiteLLM -->|"model list"| CLI
    Shell -->|"command output"| User
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`, `--provider`); endpoint and credential-source choices in the bordered `watn provider` setup view; keyboard input in the bordered model settings view (arrows, PageUp/PageDown, Enter, Escape, Tab) | Shell command + metadata (model, tok/s, cost); structured provider setup with credential-source list, detail rows, and guidance; model-tier tabs with aligned model columns and overflow position; saved provider setup; actionable non-TTY setup guidance; or confirmation prompt |
| LLM provider | API key, endpoint URL (config) | HTTP POST to `/v1/chat/completions` |
| LiteLLM (optional) | Endpoint URL (config), search query (typed by user) | HTTP GET to `/models`, HTTP GET to `/models?search=...` |
| System shell | Confirmation response (`y`/`n`/Enter) | Executed command (when confirmed) |

## Technical context

```mermaid
graph TB
    TTY["is_terminal() check"]
    Env["WATN_* env vars"]
    Config["~/.config/watn/config.toml"]

    CLI --> TTY
    CLI --> Env
    CLI --> Config
```

| Interface | Technology / Protocol | Direction |
|---|---|---|
| LLM provider | HTTPS + SSE (OpenAI chat-completions) | Outbound |
| LiteLLM | HTTPS + JSON | Outbound (optional) |
| Config file | TOML | Read (user path), direct write (provider endpoint, credential representation, tier + reasoning assignment) with Unix mode `0600` after every save |
| Environment | `WATN_*` variables | Read |
| Stdin | TTY or pipe | TTY detection and question/input read |
| Stdout | Raw text or ANSI-rendered | Write |
| Confirmation prompt | stdin line read | Read
