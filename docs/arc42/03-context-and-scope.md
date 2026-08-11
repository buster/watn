# 3. Context and Scope

## Business context

```mermaid
graph TB
    User["Developer<br/>(terminal user)"]
    CLI["watn CLI"]
    API["OpenAI-compatible API<br/>(LLM provider)"]
    LiteLLM["LiteLLM endpoint<br/>(optional, model discovery)"]
    Shell["System shell<br/>(execution mode)"]
    CompletionCaller["Completion caller<br/>(install/source)"]
    ShellParser["Bash / Zsh / Fish<br/>(completion parser)"]

    User -->|"question via args/stdin"| CLI
    User -->|"watn completions <SHELL>"| CLI
    User -->|"endpoint and credential input in provider setup"| CLI
    User -->|"rerun the original request after successful automatic setup"| CLI
    CLI -->|"POST /v1/chat/completions"| API
    CLI -->|"GET /models, pages, search"| LiteLLM
    CLI -->|"GET /models when LiteLLM absent"| API
    CLI -->|"sh -c (when -x flag)"| Shell
    CLI -->|"completion script on stdout"| CompletionCaller
    CompletionCaller -->|"install or source"| ShellParser
    User -->|"keyboard input (arrows / PageUp / PageDown / Enter / Escape / Tab / Ctrl-R) in SetupWizard model pages"| CLI
    CLI -->|"write provider endpoint and credential representation"| Config
    API -->|"SSE content events and [DONE]"| CLI
    LiteLLM -->|"model list"| CLI
    Shell -->|"command output"| User
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`, `--provider`); page navigation and editing in the shared `watn setup` wizard; `watn models` model-page entry | Incrementally flushed shell command content on stdout, then final metadata on stderr; buffered reasoning on stderr only after successful completion with `-v`; five-page SetupWizard with visible tabs, cursor, provider credential choice, model tables, and save/discard prompt; saved provider/tier setup; actionable non-TTY setup guidance; or confirmation prompt |
| Shell user / completion caller | `watn completions <SHELL>` with one of `bash`, `elvish`, `fish`, `powershell`, or `zsh` | The selected shell's completion script on stdout only; the caller installs or sources it |
| LLM provider | API key, endpoint URL (config) | HTTP POST to `/v1/chat/completions`; SSE must end with `[DONE]` for success |
| LiteLLM (optional) | Endpoint URL (config), optional credential, search query (typed by user) | HTTP GET to `/models`, paginated `/models`, and HTTP GET to `/models?search=...`; never receives chat completions |
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
| LLM provider | HTTPS + SSE (OpenAI chat-completions, complete with `[DONE]`) | Outbound |
| LiteLLM | HTTPS + JSON | Outbound (optional) |
| Config file | TOML | Read (user path), direct write (provider endpoint, credential representation, tier + reasoning assignment) with Unix mode `0600` after every save |
| Environment | `WATN_*` variables | Read |
| Stdin | TTY or pipe | TTY detection and question/input read |
| Stdout | Raw text or ANSI-rendered | Write |
| Confirmation prompt | stdin line read | Read
| Completion selector | Lowercase shell value on the CLI; closed parser contract | Read; invalid values produce a non-zero argument error containing `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` |
| Completion output | Generated Bash, Zsh, or Fish script | Outbound to stdout only; no config, provider, or shell-startup interface is touched |
