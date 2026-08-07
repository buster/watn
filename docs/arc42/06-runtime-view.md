# 6. Runtime View

## Scenario: Ask a question with default tier

**Trigger:** User runs `watn "find files modified today"`.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Config as Config
    participant Prov as Provider
    participant API as LLM API

    User->>CLI: watn "find files modified today"
    CLI->>Config: load config (layered merge)
    Config-->>CLI: resolved Config (tier: small -> gpt-4o-mini)
    CLI->>Prov: chat_completions(messages, options{model: "gpt-4o-mini"})
    Prov->>API: POST /v1/chat/completions (stream: true)
    API-->>Prov: SSE stream of chunks
    loop Each chunk
        Prov-->>CLI: StreamChunk { content }
        CLI->>User: token (raw text - default tier is for commands)
    end
    API-->>Prov: final chunk with usage
    Prov-->>CLI: finish_reason, usage
    CLI->>CLI: compute tokens/sec from wall clock
    CLI->>CLI: compute cost (if pricing configured)
    CLI-->>User: metadata (model, tok/s, cost)
    CLI-->>User: exit 0
```

**Steps:**
1. CLI parses args, defaults to tier `-1` (small/fast)
2. Config resolves selected tier to model name
3. Provider sends POST with `stream: true`
4. Provider reads SSE chunks, pushes each onto a channel
5. CLI reads channel and writes tokens to stdout
6. After stream ends, compute tok/s from elapsed time and usage
7. Print metadata: model name, tokens/sec, cost (if pricing configured)
8. Exit 0

## Scenario: Ask with execution (`-x`)

**Trigger:** User runs `watn -x "echo hello"`.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Prov as Provider
    participant API as LLM API
    participant Shell as Shell

    User->>CLI: watn -x "echo hello"
    CLI->>Prov: chat_completions(...)
    Prov->>API: POST /v1/chat/completions
    API-->>Prov: streaming response
    Prov-->>CLI: chunks
    CLI-->>User: command: echo hello
    CLI->>User: "Execute now? [Y/n]"
    User->>CLI: Enter (or y)
    CLI->>Shell: sh -c "echo hello"
    Shell-->>User: hello
    CLI-->>User: exit 0
```

**Steps:**
1. CLI detects `-x` flag
2. Normal question flow executes (get command from LLM)
3. CLI prints the command, then prompts "Execute now? [Y/n]" to stderr
4. Reads one line from stdin
5. Empty line or `y`/`Y`: executes via `sh -c <cmd>` with inherited stdio
6. `n`/`N`: exits 0 without executing

## Scenario: Model exploration

**Trigger:** User runs `watn models`.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Config as Config
    participant LLM as LiteLLM

    User->>CLI: watn models
    CLI->>Config: read litellm config
    alt endpoint configured
        CLI->>LLM: GET /models
        LLM-->>CLI: ["gpt-4o-mini", "gpt-4o", "o3-mini", ...]
        CLI->>User: "Select model for small/fast:"
        User->>CLI: "gpt-4o-mini"
        CLI->>User: "Select model for normal:"
        User->>CLI: "gpt-4o"
        CLI->>User: "Select model for thinking:"
        User->>CLI: "o3-mini"
        CLI->>Config: write [tiers] to config file
        CLI-->>User: "Configuration updated"
    else no endpoint
        CLI-->>User: "Configure providers manually at ~/.config/watn/config.toml"
    end
```

## Scenario: Config loading

```mermaid
sequenceDiagram
    participant CLI as watn CLI
    participant Def as Built-in defaults
    participant Sys as /etc/watn/config.toml
    participant Usr as ~/.config/watn/config.toml
    participant Env as Environment
    participant Cmd as CLI args

    CLI->>Def: start with defaults
    alt system config exists
        CLI->>Sys: read and merge
    end
    alt user config exists
        CLI->>Usr: read and merge
    end
    CLI->>Env: read WATN_* variables, merge
    CLI->>Cmd: parse CLI flags, merge (highest priority)
    Note over CLI: resolved Config ready
```

**Notes:** Missing files are silently skipped. Malformed TOML produces exit code 1 with a parse-error diagnostic on stderr.