# 5. Building Block View

## Level 1 — System overview

```mermaid
graph TB
    CLI["CLI<br/>(clap dispatch)"]
    Config["Config<br/>(layered merge)"]
    Provider["Provider<br/>(trait + adapters)"]
    Transport["Transport boundary<br/>(configured endpoint / debug test override)"]
    Setup["Provider Setup<br/>(ratatui onboarding)"]
    Wizard["Setup Wizard<br/>(five pages)"]
    Output["Output<br/>(metadata + command)"]
    Models["Models<br/>(catalog and model-picker)"]
    Exec["Exec<br/>(command execution)"]
    Completion["Completion<br/>(closed selector + renderer)"]
    ShellParser["Bash / Zsh / Fish<br/>(caller parser)"]
    Shortcut["Shell Shortcut<br/>(targets + widgets)"]
    LineEditor["Shell line editor<br/>(Readline / ZLE / commandline)"]

    CLI --> Config
    CLI --> Provider
    Provider --> Transport
    CLI --> Setup
    CLI --> Output
    CLI --> Models
    CLI --> Exec
    CLI --> Completion
    Provider --> Config
    Models --> Config
    Setup --> Config
    Setup --> Models
    Setup --> Wizard
    Wizard --> Config
    Wizard --> Models
    Exec --> Config
    Completion --> ShellParser
    CLI --> Shortcut
    Shortcut --> LineEditor
    LineEditor --> CLI
```

| Building block | Responsibility |
|---|---|
| CLI | Parse args (`-1`/`-2`/`-3` tier flags, `-x`, subcommands), route errors to exit codes |
| Config | Load and merge from built-in defaults, user config file, env, CLI; preserve credential sources and resolve the catalog source |
| Provider | Chat with any OpenAI-compatible API via the Provider trait; parse SSE incrementally, invoke the synchronous content sink, accumulate reasoning privately, and require `[DONE]` |
| Transport boundary | Resolve the configured endpoint for all normal/release requests; permit a non-empty test override only in debug `test-support` outbound construction, without touching config or readiness |
| Provider Setup | Guide endpoint and credential selection in a TTY, render a bordered source list plus aligned detail table and guidance paragraph, validate input, return a typed result, persist the selected fixed provider through its caller, and restore the terminal on every exit |
| Setup Wizard | Own the shared URL, API key, and Small/Middle/Large Model pages; show the active tab, cursor, green border around the focused input region, visible model filter query, current page, model selection, save/discard prompt, and optional post-confirmation shortcut selection; save a confirmed provider draft before catalog access and return optional provider, completed model drafts, and shortcut choices |
| Output | Flush each command content chunk once, own spinner finish/clear behavior, and render final metadata separately after successful completion |
| Models | Resolve a dedicated LiteLLM-or-provider catalog source; query list, page, and search endpoints; choose local filtering for complete catalogs and provider search for incomplete catalogs; apply validated reasoning defaults; return a typed setup result and persist tiers without replacing provider/catalog settings |
| Exec | Use the already rendered aggregate command for confirmation and invoke `sh -c` only after successful stream completion; never reprint the command |
| Completion | Parse the closed `CompletionShell` selector, derive scripts from the authoritative Clap command definition, render Bash/Elvish/Fish/PowerShell/Zsh, and write only successful script bytes to stdout |
| Shell parser boundary | Consume an installed completion script; parser acceptance is verified separately for Bash, Elvish, Fish, PowerShell, and Zsh when the executable is available and is not a provider or configuration dependency |
| Shell Shortcut | Resolve selected Bash/Zsh/Fish targets, generate native marked blocks, validate marker counts, replace existing blocks atomically, attempt targets independently, return per-target reports plus aggregate failure, and emit widgets that keep the request visible as a comment above the generated command without evaluation |
| Line editor boundary | Bind Ctrl-W, read the complete current buffer, call `command watn -- "$question"`, replace the buffer with a request comment above the generated command, and never evaluate the captured text |

## Level 2 — Key building blocks

### Provider

**Responsibility:** Abstract over any OpenAI-compatible chat completions API.

| Element | Responsibility |
|---|---|
| `Provider` trait | Defines `chat_completions_streaming()` with a synchronous content-event sink |
| `OpenAICompatibleProvider` | Concrete implementation: builds HTTP request (conditionally adds `reasoning` body field), parses buffered SSE lines, extracts content/reasoning/model/usage, invokes the content sink, and rejects EOF without `[DONE]` |
| `ProviderRegistry` | Public provider lookup boundary that maps provider names (from config) to `Box<dyn Provider>` instances; it remains useful even when the binary currently registers one active provider |

The transport boundary is the only production endpoint-resolution seam. URL
builders receive an effective endpoint and remain free of environment lookups.

### Catalog source

The catalog-source resolver selects `[litellm]` when configured and otherwise
uses the active provider. It carries the raw credential source until the
request boundary, where a literal or exact environment reference is expanded.
An absent LiteLLM key produces no Authorization header. The active provider
resolver used by chat is separate and is never replaced by the catalog source.

### Setup persistence

The wizard's provider-confirmation transition is the first durable boundary. It
validates and saves the provider source before catalog loading, while model-tier
updates remain a later, independent write. This permits catalog failure or
post-confirmation cancellation to preserve the provider without manufacturing
empty or partial tier values.

### Config

**Responsibility:** Load, merge, expose configuration values, and bootstrap the config file on first run.

| Element | Responsibility |
|---|---|
| `ConfigLoader` | Ordered chain: defaults → user config → env → CLI overrides |
| `EnvReader` | Read `WATN_*` environment variables |
| `ProviderReadiness` | Decide locally whether a configured provider and credential can be resolved without probing the network or consulting the E2E transport override |
| `CredentialResolver` | Apply saved-literal/reference precedence, expand a complete `${VARIABLE}` reference at request or model-discovery time, and fall back only when `api_key` is absent |
| `Config` struct | Serde-deserializable root config with `providers`, `tiers`, `pricing`, `litellm` |
| `AutoInit` | On first run (no config file exists), writes a commented-out template to the standard XDG path before proceeding |

### Models

**Responsibility:** Discover available models and let user assign tiers.

| Element | Responsibility |
|---|---|
| `ModelExplorer` | Query provider `/models` endpoint (with optional `?search=` and pagination params), parse response, and expose catalog completeness |
| `SetupWizard` model pages | Own the shared Small, Middle, and Large Model pages, page event loop, visible query, reasoning focus, focused-widget border styling, search-worker lifecycle, and persistence boundary |
| `model-picker` | Provides model-search and local-filter logic; the wizard selects the complete-catalog local path or incomplete-catalog remote path, and remote results use a stale-generation guard |
| `ConfigWriter` | Persist selected tier assignments and per-level reasoning strengths through the existing direct writer, enforcing Unix mode `0600` after every save |

### Exec

**Responsibility:** Execute returned command in system shell.

 | Element | Responsibility |
 |---|---|
   | `Executor` | Read stdin for confirmation and run the already rendered `sh -c <cmd>` only after the stream succeeds; it does not print the aggregate again |

### Completion

**Responsibility:** Generate a shell completion script without entering normal
configuration or provider execution.

| Element | Responsibility |
|---|---|
| `CompletionShell` | Closed selector accepting only `bash`, `elvish`, `fish`, `powershell`, and `zsh`; rejects every other value with the literal `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` parser contract |
| Completion-generation branch | `run_completions` calls `Cli::command()` and maps the validated selector to the corresponding `clap_complete` renderer; it does not maintain a second command tree |
| Output boundary | Writes the selected script to stdout only, leaves stderr empty, and returns before config auto-init, provider resolution, network access, or spinner setup |

## Repository hygiene boundary

The cleanup keeps architectural and consumer boundaries explicit:

- Remove only the unused `_config` parameter from `build_registry()` after the
  implementation is complete.
- Retain public `ProviderRegistry`, `ProviderSetupResult`, and the
  `cancellation_result` and `configured_result` wrapper functions. Current
  feature steps consume the setup result wrappers, and external consumers of the
  public library modules cannot be ruled out.
- Remove only `WatnWorld` fields proven write-only by repository-wide search
  after the active scenarios are migrated. Fields read or written by permanent
  feature steps remain.
