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
    CLI->>Prov: chat_completions(messages, options{model: "gpt-4o-mini"}, content sink)
    Prov->>API: POST /v1/chat/completions (stream: true)
    API-->>Prov: SSE stream of events
    loop Each complete content event
        Prov-->>CLI: content callback
        CLI->>User: flushed command content (stdout, raw text)
    end
    API-->>Prov: usage event and [DONE]
    Prov-->>CLI: final aggregate after [DONE]
    CLI->>CLI: compute tokens/sec from first data event to [DONE]
    CLI->>CLI: compute cost (if pricing configured)
    CLI-->>User: metadata (stderr: model, tok/s, cost)
    CLI-->>User: exit 0
```

**Steps:**
1. CLI parses args, defaults to tier `-1` (small/fast)
2. Config resolves selected tier to model name
3. Provider sends POST with `stream: true`
4. Provider reads complete SSE events through a buffered reader and invokes the
   synchronous content callback; there is no channel
5. The callback writes and flushes each content delta once, finishing the
   spinner on first content
6. After `[DONE]`, compute tok/s from the first data event to the completion
   marker and use authoritative final usage/model values
7. Print metadata: response model, tokens/sec, cost (if pricing configured)
8. Exit 0; the command is not printed again from the final aggregate

## Scenario: Generate a shell completion script

**Trigger:** A caller runs `watn completions <SHELL>` for `bash`, `zsh`, or
`fish`.

```mermaid
sequenceDiagram
    participant User as Caller
    participant CLI as watn CLI
    participant Metadata as Clap command definition
    participant Generator as Completion generator
    participant Shell as Bash / Zsh / Fish

    User->>CLI: watn completions bash
    CLI->>CLI: parse arguments and validate CompletionShell
    CLI->>Metadata: request Cli::command()
    Metadata-->>CLI: root options, question, subcommands, and values
    CLI->>Generator: map bash to the Bash renderer
    Generator->>Metadata: render the authoritative command tree
    Metadata-->>Generator: completion script bytes
    Generator-->>User: script on stdout only
    CLI-->>User: empty stderr and exit 0
    User->>Shell: install/source script
    Shell-->>User: parser accepts generated syntax
```

**Steps:**
1. Parse the subcommand and the closed selector before normal command dispatch.
2. Return before configuration loading, config auto-init, provider resolution,
   model discovery, network access, or spinner setup.
3. Render from the same command definition used by Clap parsing and help.
4. Write only the selected script to stdout; successful stderr is empty.
5. Generate twice for each supported shell and compare bytes exactly.
6. Pass each generated script through the corresponding installed shell parser.

The regular no-config scenario snapshots the absent isolated
`$XDG_CONFIG_HOME/watn/config.toml` and a provider-request sentinel at zero hits
before invocation. The file remains absent, no file is written in that isolated
config directory, and the sentinel remains at zero afterward. The generated
script contains the complete root option list, preserves the `question`
positional acceptance from `Cli::command()` even when a renderer does not emit
a literal placeholder, all root subcommands, and selector values `bash`,
`elvish`, `fish`, `powershell`, and `zsh` where the renderer exposes them.

## Scenario: Completion selector error and help

`watn completions nushell` stops in argument parsing with a non-zero status
and stderr containing the literal
`unsupported shell 'nushell'; choose bash, elvish, fish, powershell, or zsh`. It does not enter
the generation or configuration path. `watn completions --help` exits 0 and
prints `Usage: watn completions <SHELL>`, the five supported values, and the
instruction that the generated script is written to stdout for the caller to
install or source; stderr remains empty.

The subcommand reserves an unquoted first token `completions`. Existing question
text beginning with that token must be quoted as one argument or passed after
`--`.

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
    API-->>Prov: streaming response and [DONE]
    Prov-->>CLI: content callbacks, then final aggregate
    CLI-->>User: flushed command on stdout: echo hello
    CLI->>User: "Execute now? [Y/n]" on stderr
    User->>CLI: Enter (or y)
    CLI->>Shell: sh -c "echo hello"
    Shell-->>User: hello
    CLI-->>User: exit 0
```

**Steps:**
1. CLI detects `-x` flag
2. Normal question flow streams command content and requires `[DONE]`.
3. After successful completion, the CLI terminates the streamed line and prompts
   "Execute now? [Y/n]" to stderr without printing the aggregate a second time
4. Reads one line from stdin
5. Empty line or `y`/`Y`: executes via `sh -c <cmd>` with inherited stdio
6. `n`/`N`: exits 0 without executing; any stream or output failure skips the prompt

## Scenario: Isolated debug test transport

**Trigger:** A test-support debug binary is run with a loopback endpoint
override.

```mermaid
sequenceDiagram
    participant Harness as Test harness
    participant CLI as watn debug test-support binary
    participant Config as Config
    participant Transport as Transport boundary
    participant ConfigTwin as Configured provider twin
    participant TestTwin as Isolated provider twin

    Harness->>ConfigTwin: start loopback twin at <base>/v1
    Harness->>TestTwin: start loopback twin at <base>/v1
    Harness->>Config: write configured endpoint, key, and default model
    Harness->>CLI: set WATN_TEST_ENDPOINT_OVERRIDE=<test-twin>/v1
    CLI->>Config: load configured provider
    Config-->>CLI: configured endpoint and credential
    CLI->>Transport: resolve outbound endpoint
    Transport-->>CLI: test twin endpoint (debug + test-support only)
    CLI->>TestTwin: POST /v1/chat/completions with Bearer key
    TestTwin-->>CLI: configured test response
    CLI-->>Harness: response and exit 0
    Harness->>Config: read persisted TOML
    Config-->>Harness: configured endpoint unchanged; override absent
```

The harness asserts the exact full URL, `POST /v1/chat/completions`, one hit on
the isolated twin, zero hits on the configured competing twin, and the exact
`Authorization: Bearer sk-configured` header. The source guard makes a
release-profile binary with `test-support` use the configured-endpoint branch;
release verification inspects that exact artifact and its target runtime
libraries.

## Scenario: Normal debug requests ignore test routing

The harness builds and copies both the default-feature debug binary and the
debug `test-support` binary from Cargo's shared target cache. With a configured
loopback provider twin and a separate competing twin selected by
`WATN_TEST_ENDPOINT_OVERRIDE`, it invokes exactly one child: the copied
default-feature debug binary. That child must request the configured
`<base>/v1/chat/completions` URL with `Authorization: Bearer sk-configured`,
return the configured response, and leave the competing twin at zero requests.
The test-support debug copy is intentionally not invoked in this scenario; its
override-honoring behavior is covered by the isolated-routing scenario. The
single configured hit is asserted for this child rather than inferred from a
two-child aggregate.

## Scenario: Missing or whitespace override fallback

The debug test-support binary is run once with the override absent and once
with whitespace. In both flows the transport boundary returns the configured
`<base>/v1` endpoint. The configured twin receives exactly one
`POST /v1/chat/completions` with the exact Authorization header; the competing
twin receives zero requests; and the persisted configured endpoint is exact.

## Scenario: Readiness ignores transport override

The readiness predicate resolves the configured provider and credential without
constructing an HTTP URL. With a competing loopback override present, it
returns ready and both local twins receive zero requests. The configured
endpoint in the provider record is unchanged.

## Scenario: Ask with thinking tier and verbose flag

**Trigger:** User runs `watn -3 -v "design a distributed queue"`.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Config as Config
    participant Prov as Provider
    participant API as LLM API

    User->>CLI: watn -3 -v "design a distributed queue"
    CLI->>Config: load config (layered merge)
    Config-->>CLI: resolved Config (tier: thinking -> o3-mini)
    CLI->>CLI: reasoning_effort from [tiers.reasoning] (thinking -> high by default), verbose = true
    CLI->>Prov: chat_completions(messages, options{model, reasoning_effort: "high"}, content sink)
    Prov->>API: POST /v1/chat/completions (body includes reasoning_effort: "high")
    API-->>Prov: SSE stream of chunks with content + reasoning fields
    loop Each complete content event
        Prov-->>CLI: content callback; accumulate reasoning privately
    end
    API-->>Prov: usage event and [DONE]
    Prov-->>CLI: StreamingResponse { full_content, reasoning_content, usage }
    CLI->>CLI: compute tokens/sec, cost, and finish command line
    CLI-->>User: command content (stdout, already streamed)
    CLI-->>User: buffered reasoning and metadata (stderr)
    CLI-->>User: exit 0
```

**Steps:**
1. CLI parses args, detects tier `-3` (thinking) and flag `-v` (verbose)
2. Config resolves thinking tier to model name
3. CLI resolves `reasoning_effort` from the tier's configured strength (default
   "high" for thinking when unset) and sets `verbose = true`; builds the POST body with `reasoning_effort`
4. Provider builds POST body with `reasoning_effort: "high"` in addition to standard fields
5. Provider reads SSE events, invokes the content sink, and extracts
   `delta["reasoning"]` or `delta["reasoning_content"]` into its private aggregate
6. Content is accumulated into `full_content` and flushed to stdout once per delta
7. Reasoning is not written to stderr while the stream is active
8. After `[DONE]`, compute tok/s from the first data event to the marker and use
   final usage/model values
9. When `-v` is active, print buffered reasoning to stderr, then final metadata;
   on failure, print neither

## Scenario: Keyboard-driven SetupWizard model pages

**Trigger:** User runs `watn models` from a terminal.

```mermaid
sequenceDiagram
    participant User as User
    participant Wizard as Setup Wizard
    participant Worker as Search worker (thread)
    participant API as Provider API
    participant Config as Config

    User->>Wizard: runs `watn models` (TTY)
    Wizard->>Wizard: show five tabs with Small Model active, page position, and cursor
    User->>Wizard: types "dee flash"
    Wizard->>Worker: spawn: per-word local/remote match (gen=N)
    API-->>Worker: matching models
    Worker->>Wizard: newest result wins → update suggestions
    User->>Wizard: ↓ (select), Ctrl-R, Up/Down (reasoning minimal)
    User->>Wizard: Enter or Tab (confirm small, advance to Middle Model)
    loop normal, thinking
        User->>Wizard: pick model, Enter or Tab to advance
    end
    User->>Wizard: Shift-Tab → previous page
    User->>Wizard: Escape → save/discard prompt
    Wizard->>Config: persist [tiers] + [tiers.reasoning]
    Wizard-->>User: "Tiers configured: ..."
```

**Steps:**
1. The wizard opens on the Small Model page with five tabs, a border, filter
   paragraph, aligned model table, visible selected-row cursor, and scrollbar
   when needed.
2. Keystrokes update the visible filter; results match per-word,
   order-independent and are debounced with a stale-result guard.
3. Arrow/page keys move selection; Enter and Tab accept/advance; Shift-Tab
   returns to the previous page.
 4. Ctrl-R focuses the closed reasoning set (off/low/minimal/medium/high) on a
       model page; mandatory models exclude off.
5. Escape opens a save/discard prompt; saving persists the provider and all
   completed model choices.

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
        CLI->>User: SetupWizard: model + reasoning for small tier
        User->>CLI: "gpt-4o-mini", reasoning off
        CLI->>User: SetupWizard: model + reasoning for normal tier
        User->>CLI: "gpt-4o", reasoning low
        CLI->>User: SetupWizard: model + reasoning for thinking tier
        User->>CLI: "o3-mini", reasoning high
        CLI->>Config: write [tiers] and [tiers.reasoning] to config file
        CLI-->>User: "Configuration updated"
    else no endpoint
        CLI-->>User: "Configure providers manually at ~/.config/watn/config.toml"
    end
```

## Scenario: Model-picker search in the SetupWizard

**Trigger:** User runs `watn models`, types a search query on a SetupWizard model page.

```mermaid
sequenceDiagram
    participant User as User
    participant Picker as Setup Wizard model page
    participant Worker as Search worker (thread)
    participant API as Provider API

    User->>Picker: types "o3"
    Picker->>Picker: increment generation counter
    Picker->>Worker: spawn: GET /models?search=o3 (gen=N)
    Picker-->>User: render active model tab, spinner, and previous suggestions
    API-->>Worker: { data: [{id:"o3-mini"}, {id:"o3-pro"}] }
    Worker->>Picker: check generation == N → update suggestions
    Picker-->>User: render table rows with visible selected cursor
    User->>Picker: ↓ (arrow down)
    Picker-->>User: move cursor to "o3-pro"
    User->>Picker: Enter
    Picker-->>User: selection confirmed, advance to next wizard page
```

**Steps:**
1. The setup wizard enters raw terminal mode and shows the active page/tab.
2. Keystrokes append to or remove from the live query string.
3. Each change bumps an `Arc<AtomicU64>` generation counter and spawns a
   blocking worker thread that calls `GET /models?search=<query>`.
4. Worker thread captures the generation at spawn time. On response, if the
   generation has advanced, the result is discarded (stale-result guard).
5. Valid results update the suggestion list; the terminal is repainted.
6. Arrow keys move the table cursor; Ctrl-R toggles reasoning focus and
   Up/Down chooses one of the current model's supported efforts.
7. Enter or Tab confirms the selection and advances; Shift-Tab returns;
   Escape opens save/discard rather than clearing the query.
8. A 4xx/5xx on a non-empty search shows "Model search is not supported by
   this provider" and retains the previous suggestions.
9. After final selection or save/discard, the wizard restores cooked terminal
   mode and returns provider/completed model drafts to the caller.

## Scenario: Config loading

```mermaid
sequenceDiagram
    participant CLI as watn CLI
    participant Def as Built-in defaults
    participant Usr as ~/.config/watn/config.toml
    participant Env as Environment
    participant Cmd as CLI args

    CLI->>Def: start with defaults
    alt user config exists
        CLI->>Usr: read and merge
    end
    CLI->>Env: read WATN_* variables, merge
    CLI->>Cmd: parse CLI flags, merge (highest priority)
    Note over CLI: resolved Config ready
```

**Notes:** Missing files are silently skipped. Malformed TOML produces exit code 1 with a parse-error diagnostic on stderr.

## Scenario: First normal use with no recognized provider

**Trigger:** User runs `watn "hello"` without a configured provider or supported
provider credential environment variable and provider selection is implicit.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Setup as Setup Wizard
    participant Config as Config
    participant Twin as OpenAI-compatible endpoint

    User->>CLI: watn "hello"
    CLI->>Config: load config and inspect provider readiness
    Config-->>CLI: Missing
    alt stdin is not a TTY
        CLI-->>User: actionable `watn provider` and config-path guidance
        CLI-->>User: exit 1; no ratatui and no network request
    else stdin is a TTY
        CLI->>Setup: open five-page setup wizard
        User->>Setup: enter endpoint and choose credential storage
        User->>Setup: explicitly confirm the credential
        Setup->>Config: persist the confirmed provider draft
        Setup->>Twin: GET /models
        Twin-->>Setup: model catalog or catalog failure
        User->>Setup: select small, middle, and large models
        Setup->>Config: save completed tier assignments only after selection
        CLI-->>User: setup complete; exit 0
        Note over CLI: original question is not sent; user reruns it
    end
```

**Steps:**
1. Load the config and check actual provider data plus recognized environment
   variables; do not use a network probe for readiness.
2. If stdin is not a TTY, print actionable setup guidance and exit 1 without
   initializing ratatui.
3. If stdin is a TTY, open the shared setup wizard when no implicit provider is
   ready.
4. Save the endpoint and either the literal credential or the `${VARIABLE}`
   reference without printing the resolved secret.
5. Discover models and walk the three model pages in the same process and
   terminal.
6. Save all three model tiers and exit successfully. Do not send or resume the
   original question.

## Scenario: Explicit provider setup

**Trigger:** User runs `watn provider`.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn CLI
    participant Setup as Provider Setup
    participant Config as Config

    User->>CLI: watn provider
    CLI->>Setup: open ratatui provider flow
    Setup-->>User: URL tab, compatibility explanation, and visible cursor
    User->>Setup: accept or edit endpoint; choose literal or environment source
    Setup->>Config: save default provider and credential representation
    Config-->>Setup: save result
    Setup-->>CLI: configured
    CLI-->>User: completion status
```

The explicit command ends after provider configuration. It does not invoke
model setup; only the automatic first-use TTY path performs that chain. An
explicit `--provider` or `WATN_PROVIDER` selection never enters automatic
onboarding and retains normal unknown-provider and missing-key errors.

## Scenario: Unified setup wizard

**Trigger:** User runs `watn setup` from a terminal.

```mermaid
sequenceDiagram
    participant User as User
    participant Wizard as Setup Wizard
    participant Catalog as Provider model catalog
    participant Config as Config

    User->>Wizard: watn setup
    Wizard-->>User: URL tab, compatibility explanation, cursor
    User->>Wizard: Enter endpoint
    Wizard-->>User: API key tab and storage choice
    User->>Wizard: choose configuration or environment reference
    Wizard->>Catalog: GET /models after valid provider credentials
    Catalog-->>Wizard: model rows
    loop Small, Middle, Large Model pages
        Wizard-->>User: active tab, table, selected row, cursor/page position
        User->>Wizard: Enter or Tab
    end
    User->>Wizard: save confirmation
    Wizard->>Config: persist provider and completed tiers
    Wizard-->>User: saved setup result
```

## Scenario: Catalog discovery with independent LiteLLM source

**Trigger:** User runs `watn models` with a `[litellm]` section.

```mermaid
sequenceDiagram
    participant User as User
    participant CLI as watn models
    participant Config as Config
    participant Catalog as LiteLLM catalog
    participant Provider as Active provider

    User->>CLI: watn models
    CLI->>Config: load active provider and optional LiteLLM source
    Config-->>CLI: catalog endpoint + raw optional credential; active provider unchanged
    CLI->>Catalog: GET /models or paginated /models with optional Bearer key
    Catalog-->>CLI: model metadata
    User->>CLI: assign small, normal, and thinking tiers
    CLI->>Config: save tier assignments only
    Note over CLI,Provider: Later chat requests still use Provider, never Catalog
```

The source is resolved once per discovery operation. A configured LiteLLM key
is expanded at request time; no key means no Authorization header. Search and
pagination reuse the same endpoint and credential policy. Without LiteLLM, the
selected provider receives the catalog requests.

## Scenario: Provider confirmation before catalog failure

**Trigger:** User confirms a provider credential in `watn setup`, then catalog
discovery fails.

```mermaid
sequenceDiagram
    participant User as User
    participant Wizard as Setup Wizard
    participant Config as Config
    participant Catalog as Catalog source

    User->>Wizard: confirm endpoint and credential source
    Wizard->>Wizard: validate and resolve credential
    Wizard->>Config: save provider draft and raw credential source
    Wizard->>Catalog: request model catalog
    Catalog-->>Wizard: error
    Wizard-->>User: catalog failure; return to credential/setup state
    Config-->>User: confirmed provider remains saved; tiers unchanged
```

Cancellation before credential confirmation performs no write. Cancellation
after confirmation preserves the provider. Neither path sends the original chat
question.

## Scenario: Reasoning policy and stale search generation

The shared reasoning resolver filters invalid strengths, honors disabled and
mandatory metadata, and is used before both tier persistence and chat request
construction. Search workers receive monotonically increasing generations; a
late worker may finish its HTTP request but cannot apply its result after a
newer generation has been applied. The wizard joins or discards workers before
returning to the caller.

## Scenario: Completion marker and first-event timing

The provider can send `[DONE]` and keep its HTTP connection open. Watn completes
from the marker rather than waiting for the server-side close. The elapsed-time
clock starts at the first non-DONE data line, before decoding, and ends at the
marker.

```mermaid
sequenceDiagram
    participant CLI as watn CLI
    participant Prov as Provider
    participant API as LLM API
    participant User as User

    CLI->>Prov: request with content sink
    Prov->>API: POST /v1/chat/completions
    API-->>Prov: first data line
    Prov->>Prov: set first_event_at before JSON decode
    API-->>Prov: content event
    Prov-->>CLI: content callback
    CLI-->>User: flushed command prefix; spinner cleared
    API-->>Prov: usage-only event (choices empty)
    API-->>Prov: [DONE]
    Prov-->>CLI: final model, usage, reasoning, elapsed
    CLI-->>User: final metadata and optional buffered reasoning
    CLI-->>User: exit 0 before API closes connection
```

The response model and usage are read at the top level of every valid event.
Thus a choices-empty usage event can select the response model for metadata and
pricing. A later usage event replaces earlier usage. The final aggregate command
is used for trimming and optional execution, not rendered a second time.

## Scenario: Truncated or failed provider stream

Valid content remains visible when a provider closes without `[DONE]` or resets
the connection after a content event. Both cases finish the spinner and report a
network error with status 3. Neither prints final success metadata or enters
execution confirmation.

```mermaid
sequenceDiagram
    participant CLI as watn CLI
    participant Spinner
    participant Prov as Provider
    participant API as LLM API
    participant User as User

    CLI->>Spinner: start
    CLI->>Prov: request with content sink
    Prov->>API: POST /v1/chat/completions
    API-->>Prov: valid content event
    Prov-->>CLI: content callback
    CLI-->>User: flushed visible prefix
    API--xProv: clean EOF without [DONE] or connection reset
    Prov-->>CLI: NetworkError
    CLI->>Spinner: finish and clear
    CLI-->>User: preserve prefix and print network error
    CLI-->>User: omit metadata and execute prompt; exit 3
```

Malformed nonessential data events take a different path: the provider ignores
the malformed event, continues reading, and can still expose later valid content
before `[DONE]`.

## Scenario: Output failure during streaming

The callback owns stdout writes and flushes. If the output sink fails after a
prefix is visible, the provider stops through the existing I/O error path.

```mermaid
sequenceDiagram
    participant CLI as watn CLI
    participant Spinner
    participant Prov as Provider
    participant Out as Output sink
    participant User as User

    CLI->>Spinner: start
    Prov-->>CLI: first content callback
    CLI->>Out: write and flush prefix
    Out-->>CLI: success
    Prov-->>CLI: next content callback
    CLI->>Out: write or flush next chunk
    Out-->>CLI: I/O error
    CLI->>Spinner: finish and clear
    CLI-->>User: preserve prefix and report I/O error
    CLI-->>User: omit metadata and execution; exit 1
```

The direct output-writer test observes the prefix, status 1, spinner lifecycle,
absence of final metadata, and absence of the execute prompt without relying on
a platform-specific closed-pipe behavior.
