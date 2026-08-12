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
    ShellStartup["Bash / Zsh / Fish<br/>(startup files)"]
    LineEditor["Bash Readline / Zsh ZLE / Fish commandline"]

    User -->|"question via args/stdin"| CLI
    User -->|"watn completions <SHELL>"| CLI
    User -->|"review setup draft in four topics"| CLI
    User -->|"rerun the original request after successful automatic setup"| CLI
    CLI -->|"POST /v1/chat/completions"| API
    CLI -->|"GET /models, pages, search"| LiteLLM
    CLI -->|"GET /models when LiteLLM absent"| API
    CLI -->|"sh -c (when -x flag)"| Shell
    CLI -->|"completion script on stdout"| CompletionCaller
    CompletionCaller -->|"install or source"| ShellParser
    User -->|"keyboard input in Provider, Model roles, Shell integration, and Review"| CLI
    User -->|"optional shell integration selection"| CLI
    CLI -->|"write the complete reviewed draft at Finish"| Config
    CLI -->|"marked shortcut block and reload report"| ShellStartup
    LineEditor -->|"Ctrl-W current buffer"| CLI
    CLI -->|"replacement buffer text"| LineEditor
    API -->|"SSE content events and [DONE]"| CLI
    LiteLLM -->|"model list"| CLI
    Shell -->|"command output"| User
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`, `--provider`); page navigation and editing in the shared `watn setup` wizard; `watn models` model-page entry; typed model filter queries | Incrementally flushed shell command content on stdout, then final metadata on stderr; buffered reasoning on stderr only after successful completion with `-v`; five-page SetupWizard with visible tabs, cursor, green border around the focused input region, visible filter query, continuously updated model suggestions, provider credential choice, model tables, and save/discard prompt; saved provider/tier setup; actionable non-TTY setup guidance; or confirmation prompt |
| Shell user / completion caller | `watn completions <SHELL>` with one of `bash`, `elvish`, `fish`, `powershell`, or `zsh` | The selected shell's completion script on stdout only; the caller installs or sources it |
| LLM provider | API key, endpoint URL (config) | HTTP POST to `/v1/chat/completions`; SSE must end with `[DONE]` for success |
| LiteLLM (optional) | Endpoint URL (config), optional credential, search query (typed by user) | HTTP GET to `/models`, paginated `/models`, and HTTP GET to `/models?search=...`; never receives chat completions |
| System shell | Confirmation response (`y`/`n`/Enter) | Executed command (when confirmed) |
| Shell startup file | Optional selected-shell installation | One marked native widget block and a reload instruction; malformed or failed targets remain unchanged |
| Bash/Zsh/Fish line editor | Ctrl-W and the complete current command buffer | A successful non-empty `watn` result inserted below a comment line containing the original request, without evaluation; Fish receives an actual line break in the editable buffer; failures preserve the buffer |

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
| Config file | TOML | Read-only existence-aware load; one secure Finish commit of provider endpoint, credential representation, tier and reasoning assignment with Unix mode `0600` |
| Environment | `WATN_*` variables | Read |
| Stdin | TTY or pipe | TTY detection and question/input read |
| Stdout | Raw text or ANSI-rendered | Write |
| Confirmation prompt | stdin line read | Read
| Completion selector | Lowercase shell value on the CLI; closed parser contract | Read; invalid values produce a non-zero argument error containing `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` |
| Completion output | Generated Bash, Elvish, Fish, PowerShell, or Zsh script | Outbound to stdout only; no config, provider, or shell-startup interface is touched |
| Shell widget boundary | Native line-editor buffer plus `watn` on `PATH` | Reads one quoted question, captures stdout, keeps stderr visible, and replaces/repaints only after zero status and non-empty output |

## Current setup boundary

The supported configuration surface is entered through `watn setup`. Its
visible topics are Provider, Model roles, Shell integration, and Review. The
focused `watn provider` and `watn models` commands and provider/model selection
overrides are not part of the current CLI boundary. Setup writes supported TOML
only after Finish; shell marker state remains in startup files rather than in
the configuration file.
