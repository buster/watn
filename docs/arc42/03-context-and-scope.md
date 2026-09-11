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
    User -->|"plain-line quick setup answers (endpoint, credential, models, shell selection) with optional parameter prefills"| CLI
    User -->|"optional shortcut selection after final setup confirmation"| CLI
    CLI -->|"write provider endpoint and credential representation"| Config
    CLI -->|"marked shortcut block and reload report"| ShellStartup
    LineEditor -->|"Ctrl-W current buffer"| CLI
    CLI -->|"replacement buffer text"| LineEditor
    CLI -->|"transient review surface on controlling-terminal channel"| LineEditor
    API -->|"SSE content events and [DONE]"| CLI
    Catalog -->|"model list"| CLI
    Shell -->|"command output"| User
    Maintainer["Maintainer / givn workflow"]
    Maintainer -->|"review and archive consolidation"| CLI
```

| Partner / User | Input to system | Output from system |
|---|---|---|
| Developer | Positional question, stdin, flags (`-1`/`-2`/`-3`, `-x`, `--model`, `--provider`, review override, flag-only review switch); review decision keys (`e`/`r`/`c`, `d`/`?` view toggle, `D` disable review) and model-choice input in the review card; setup commands; one-question navigation and editing; typed model filter queries | In non-review mode, incrementally flushed shell command content on stdout and final metadata on stderr; in eligible review mode, a complete Candidate after `[DONE]`, a simple or detailed transient review surface on the controlling-terminal channel, and only the accepted Candidate or the Candidate released by a permanent review disable on the command-output channel; focused setup flows, first-run setup, guidance, or the existing confirmation prompt |
| Shell user / completion caller | `watn completions <SHELL>` with one of `bash`, `elvish`, `fish`, `powershell`, or `zsh` | The selected shell's completion script on stdout only; the caller installs or sources it |
| LLM provider and provider-local catalog | API key, completion endpoint, provider-local catalog endpoint, search query | HTTP POST to `/v1/chat/completions` and HTTP GET to `/models`, paginated `/models`, and `/models?search=...`; the same provider credential is used and catalog requests never receive chat completions |
| System shell | Confirmation response (`y`/`n`/Enter) | Executed command (when confirmed) |
| Shell startup file | Optional selected-shell installation | One marked native widget block and a reload instruction; malformed or failed targets remain unchanged |
| Bash/Zsh/Fish line editor | Ctrl-W and the complete current command buffer | In review mode, final acceptance records a `#`-prefixed history comment of the original Intent and places only the accepted Candidate in the shell line-editor buffer without evaluation; a permanent disable releases the current Candidate to the same buffer with a re-enable instruction on stderr; cancellation or failure records no review history and preserves the buffer; a leading `# ` is stripped before asking so recalled comments can be re-asked |
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
| Review surface boundary | Controlling-terminal channel, ANSI redraw, and review keys | Shows a small transient Command flow review in a simple default view or a detailed view without writing review-surface text to stdout; direct decision keys accept/edit/reject/cancel, switch the view, or disable the review permanently with the current Candidate released to the command-output channel, rejection opens a model chooser, and the typed results are `Accepted(candidate)`, `Cancelled`, `RejectRequested`, `RegenerateWith`, `DisableReviewPermanently`, or `Unavailable` |
