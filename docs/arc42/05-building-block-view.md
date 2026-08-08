# 5. Building Block View

## Level 1 — System overview

```mermaid
graph TB
    CLI["CLI<br/>(clap dispatch)"]
    Config["Config<br/>(layered merge)"]
    Provider["Provider<br/>(trait + adapters)"]
    Output["Output<br/>(metadata + command)"]
    Models["Models<br/>(explorer)"]
    Exec["Exec<br/>(command execution)"]

    CLI --> Config
    CLI --> Provider
    CLI --> Output
    CLI --> Models
    CLI --> Exec
    Provider --> Config
    Models --> Config
    Exec --> Config
```

| Building block | Responsibility |
|---|---|
| CLI | Parse args (`-1`/`-2`/`-3` tier flags, `-x`, subcommands), route errors to exit codes |
| Config | Load and merge from built-in defaults, system file, user file, env, CLI |
| Provider | Chat with any OpenAI-compatible API via the Provider trait |
| Output | Format response with metadata header (model, tok/s, cost) + command body |
| Models | Query LiteLLM `/models` endpoint; interactive tier selection via dialoguer; persist to config |
| Exec | Print command, prompt confirmation, invoke `sh -c` if confirmed |

## Level 2 — Key building blocks

### Provider

**Responsibility:** Abstract over any OpenAI-compatible chat completions API.

| Element | Responsibility |
|---|---|
| `Provider` trait | Defines `chat_completions()` (streaming) and `chat_completions_blocking()` |
| `OpenAICompatible` | Concrete implementation: builds HTTP request (conditionally adds `reasoning` body), parses SSE chunks (extracts both `content` and `reasoning` from delta) |
| `ProviderRegistry` | Maps provider names (from config) to `Box<dyn Provider>` instances |

### Config

**Responsibility:** Load, merge, expose configuration values, and bootstrap the config file on first run.

| Element | Responsibility |
|---|---|
| `ConfigLoader` | Ordered chain: defaults → system config → user config → env → CLI overrides |
| `EnvReader` | Read `WATN_*` environment variables |
| `Config` struct | Serde-deserializable root config with `providers`, `tiers`, `pricing`, `litellm` |
| `AutoInit` | On first run (no config file exists), writes a commented-out template to the standard XDG path before proceeding |

### Models

**Responsibility:** Discover available models and let user assign tiers.

| Element | Responsibility |
|---|---|
| `ModelExplorer` | Query provider `/models` endpoint (with optional `?search=` and pagination params), parse response |
| `ModelPicker` | Raw-terminal autosuggest loop: reads keystrokes, dispatches debounced search requests, renders live suggestion list with stale-result guard |
| `TierSelector` | Interactive dialoguer prompts for small/normal/thinking assignment (pre-autosuggest path) or `ModelPicker` per tier |
| `ConfigWriter` | Persist selected tier assignments to user config file |

### Exec

**Responsibility:** Execute returned command in system shell.

 | Element | Responsibility |
 |---|---|
 | `Executor` | Print command, read stdin for confirmation, run `sh -c <cmd>` |