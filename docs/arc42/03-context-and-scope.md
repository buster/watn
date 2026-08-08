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
    CLI -->|"POST /v1/chat/completions"| API
    CLI -->|"GET /models (optional)"| LiteLLM
    CLI -->|"GET /models?search=..."| LiteLLM
    CLI -->|"sh -c (when -x flag)"| Shell
    API -->|"SSE streaming response"| CLI
    LiteLLM -->|"model list"| CLI
    Shell -->|"command output"| User
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`) | Shell command + metadata (model, tok/s, cost); or confirmation prompt |
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
| Config file | TOML | Read (user path) |
| Environment | `WATN_*` variables | Read |
| Stdout | Raw text or ANSI-rendered | Write |
| Confirmation prompt | stdin line read | Read