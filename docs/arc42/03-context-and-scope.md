# 3. Context and Scope

## Business context

```mermaid
graph TB
    User["Developer<br/>(terminal user)"]
    CLI["watn CLI"]
    API["OpenAI-compatible API<br/>(LLM provider)"]
    Catalog["Provider-local catalog<br/>(/models, pages, search)"]
    Shell["System shell<br/>(execution mode)"]
    CompletionCaller["Completion caller<br/>(install/source)"]
    ShellParser["Bash / Zsh / Fish<br/>(completion parser)"]
    ShellStartup["Bash / Zsh / Fish<br/>(startup files)"]
    LineEditor["Bash Readline / Zsh ZLE / Fish commandline"]

    User -->|"question via args/stdin"| CLI
    User -->|"watn completions <SHELL>"| CLI
    User -->|"provider, model, reasoning, and shell setup input"| CLI
    User -->|"rerun the original request after successful automatic setup"| CLI
    CLI -->|"POST /v1/chat/completions"| API
    CLI -->|"GET /models, pages, search"| Catalog
    CLI -->|"sh -c (when -x flag)"| Shell
    CLI -->|"completion script on stdout"| CompletionCaller
    CompletionCaller -->|"install or source"| ShellParser
    User -->|"keyboard input (arrows / PageUp / PageDown / Enter / Escape / Tab / Ctrl-R) in SetupWizard pages"| CLI
    User -->|"plain-line quick setup answers (endpoint, credential, models, shell selection)"| CLI
    User -->|"optional shortcut selection after final setup confirmation"| CLI
    CLI -->|"write provider endpoint and credential representation"| Config
    CLI -->|"marked shortcut block and reload report"| ShellStartup
    LineEditor -->|"Ctrl-W current buffer"| CLI
    CLI -->|"replacement buffer text"| LineEditor
    API -->|"SSE content events and [DONE]"| CLI
    Catalog -->|"model list"| CLI
    Shell -->|"command output"| User
    Maintainer["Maintainer / givn workflow"]
    Maintainer -->|"review and archive consolidation"| CLI
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`, `--provider`); `watn setup`, `watn provider`, `watn models`, `watn shell`, and `watn quicksetup`; one-question navigation and editing; typed model filter queries | Incrementally flushed shell command content on stdout, then final metadata on stderr; buffered reasoning on stderr only after successful completion with `-v`; focused setup flows with provider choices, credential source, catalog status, separate model/reasoning questions, review, shell desired state, and safe cancellation; plain-line first-run quick setup with suggested endpoint, credential reference, and models; saved provider/model setup; actionable non-TTY setup guidance; or confirmation prompt |
| Shell user / completion caller | `watn completions <SHELL>` with one of `bash`, `elvish`, `fish`, `powershell`, or `zsh` | The selected shell's completion script on stdout only; the caller installs or sources it |
| LLM provider and provider-local catalog | API key, completion endpoint, provider-local catalog endpoint, search query | HTTP POST to `/v1/chat/completions` and HTTP GET to `/models`, paginated `/models`, and `/models?search=...`; the same provider credential is used and catalog requests never receive chat completions |
| System shell | Confirmation response (`y`/`n`/Enter) | Executed command (when confirmed) |
| Shell startup file | Optional selected-shell installation | One marked native widget block and a reload instruction; malformed or failed targets remain unchanged |
| Bash/Zsh/Fish line editor | Ctrl-W and the complete current command buffer | A successful non-empty `watn` result recorded as a `#`-prefixed history comment of the original request plus a buffer holding only the generated command, without evaluation; a leading `# ` is stripped before asking so recalled comments can be re-asked; failures preserve the buffer |
| Maintainer / givn workflow | Repository-wide scenario review, dispositions, and archive | Duplicate-title, overlap, subset, and net-delta evidence; an archived permanent tree with the same Watn runtime contract |

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
| Legacy `[litellm]` data | TOML configuration | Read and preserved as unrelated data; not contacted by streamlined setup |
| Config file | TOML | Read (user path), atomic snapshot write after command confirmation, provider-local catalog state, credential representation, and tier/reasoning assignment with Unix mode `0600` |
| Environment | `WATN_*` variables | Read |
| Stdin | TTY or pipe | TTY detection and question/input read |
| Stdout | Raw text or ANSI-rendered | Write |
| Confirmation prompt | stdin line read | Read
| Completion selector | Lowercase shell value on the CLI; closed parser contract | Read; invalid values produce a non-zero argument error containing `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` |
| Completion output | Generated Bash, Elvish, Fish, PowerShell, or Zsh script | Outbound to stdout only; no config, provider, or shell-startup interface is touched |
| Shell widget boundary | Native line-editor buffer plus `watn` on `PATH` | Reads one quoted question, captures stdout, keeps stderr visible, and replaces/repaints only after zero status and non-empty output |
