# 8. Cross-cutting Concepts

## Error handling

All fallible operations return a typed `Error` enum. The top-level dispatch maps
each variant to an exit code and prints a diagnostic to stderr:

| Error variant | Exit code | Example |
|---|---|---|
| `ConfigError` | 1 | Malformed TOML, missing required field |
| `ProviderNotFound` | 1 | `--provider nonexistent` |
| `AuthError` | 2 | Invalid API key, HTTP 401 |
| `ApiError` | 2 | Rate limit (429), server error (5xx) |
| `NetworkError` | 3 | DNS failure, connection refused, timeout |
| `IoError` | 1 | Cannot write config file |

Error messages are human-readable and include context.

Provider setup and model setup return typed results rather than exiting inside
their lower-level functions. Escape cancellation maps to status 1; Ctrl-C maps
to status 130. Provider setup does not save partial input. If provider setup
succeeds and model setup is cancelled or fails, the provider remains saved, the
automatic flow stops, and the original request is not sent. Invalid endpoints
and empty credentials remain in the setup flow with an inline validation
message. A missing saved environment reference is an authentication error and
does not fall through to another environment variable.

## Configuration layering

Config is merged in order (later overrides earlier):

1. **Built-in defaults** — hardcoded in the binary
2. **User config** — `$XDG_CONFIG_HOME/watn/config.toml`
3. **Environment variables** — `WATN_*` (e.g. `WATN_PROVIDER`, `WATN_MODEL`)
4. **CLI flags** — `-1`/`-2`/`-3`, `--model`, `--provider` (highest priority)

Provider readiness is a separate local check. A commented auto-init template is
not ready; a provider is ready only when its endpoint and literal or resolved
credential are available. OpenRouter has a built-in endpoint fallback only when
no `[providers.openrouter]` entry exists. A saved literal or exact environment
reference is authoritative. Only an absent `api_key` permits provider-specific
fallback followed by generic `WATN_API_KEY`. Readiness never consults the
ephemeral E2E transport override.

Environment-backed credentials are persisted as complete references such as
`${OPENROUTER_API_KEY}`. The resolver expands the reference for an outbound
request, while the serializer preserves the reference.

## Credential safety

- Literal credential input is masked in the ratatui setup screen.
- Resolved credentials are not included in setup status, diagnostics, or config
  rewrite output.
- Environment references are preferred because the config contains the variable
  name rather than the secret value.
- Every direct config save is followed by Unix mode `0600`; a pre-existing
  world-readable file may warn on load and is repaired on its next save.
- The fixed onboarding names are `openrouter` and `custom`; rerunning setup
  replaces only the selected fixed entry and preserves unrelated providers and
  configuration.

## Auto-init (first-run template)

On the first invocation, when no config file exists at the standard XDG path
(`$XDG_CONFIG_HOME/watn/config.toml`), the binary writes a template file with
all options commented out. The template is generated from code
(`Config::template_content()`) rather than a hardcoded string, ensuring that
adding a new config field automatically includes it in the template.

The template includes commented-out sections for defaults, tiers, custom
providers, and pricing. The file write is silent and does not interrupt the
command the user issued. If a config file already exists, nothing is written.

All subsequent provider and model saves use the same direct-write mechanism and
apply mode `0600` after the write on Unix. Atomic temp-file/rename behavior is
not promised. An interrupted direct write remains a known risk.

## Model tier resolution

The selected tier (default `-1`) is resolved to a model name via:

1. CLI `--model <NAME>`: explicit override, bypasses tiers
2. Config `[tiers]` section: `small`, `normal`, `thinking` fields
3. Fallback to provider's `default_model` or a hardcoded default

## Cost tracking

Per-model pricing configured in `[pricing]` section of config:

```toml
[pricing]
"gpt-4o-mini" = { input = 0.15, output = 0.60 }
"gpt-4o" = { input = 2.50, output = 10.00 }
```

Values are $ per 1M tokens. Cost = (input_tokens * input_price + output_tokens * output_price) / 1_000_000.
Displayed after response completion. When pricing is not configured, cost is omitted from output.

## Tokens/second

Wall clock measured from first SSE chunk to final chunk via `std::time::Instant`.
tok/s = completion_tokens / elapsed_seconds. Displayed after response completion.

## Execution mode (`-x`)

When `-x` is passed, the returned command is printed to stdout, then the user
is prompted on stderr: `Execute now? [Y/n]`. Empty line or `y`/`Y` runs the
command via `sh -c <cmd>` with inherited stdout/stderr. `n`/`N` exits 0.

## Reasoning and verbose mode

When the thinking tier (`-3` / `--thinking`) is activated, the request body includes a top-level `reasoning_effort` parameter:

```json
{"reasoning_effort": "high"}
```

This signals the API to generate reasoning/chain-of-thought tokens alongside the answer. The `reasoning_effort` is only set for tier 3 (the thinking tier). Tiers 1 and 2 do not send this parameter.

Response chunks from the API may include a `reasoning` field in the delta object alongside the `content` field. The provider accumulates reasoning content separately from command content.

When `-v` / `--verbose` is passed, the accumulated reasoning content is printed to stderr after the response completes, on its own line prefixed with `reasoning:`. If the model returned no reasoning content, nothing additional is printed.

The verbose flag is independent of the thinking tier. Any tier with `-v` will print reasoning content if the API returns it. Without `-v`, reasoning content is accumulated into the response struct but not printed.

## Pipe and TTY detection

The binary detects whether **stdin** is a TTY using
`std::io::stdin().is_terminal()`. When stdin is not a TTY, the question is read
from the pipe. Automatic provider onboarding is allowed only for implicit
provider selection with TTY stdin. An implicit non-TTY first-use request emits
actionable `watn provider` and config-path guidance, exits 1, and does not
initialize ratatui. Explicit `--provider` and `WATN_PROVIDER` selections retain
their existing resolution errors regardless of TTY state. Command output goes
to stdout; metadata and setup guidance go to stderr as plain text (suitable for
scripting).

## Exit code convention

| Code | Meaning | Usage |
|---|---|---|
| 0 | Success | Command generated and printed |
| 1 | User error | Bad argument, bad config, unknown provider, I/O error, setup Escape cancellation, or non-TTY onboarding guidance |
| 2 | API error | Auth failure, rate limit, server error |
| 3 | Network error | DNS, connection, timeout |
| 130 | Interrupted | SIGINT (Ctrl+C) during streaming |

## Model interaction modes

The current model settings dialog uses ratatui and crossterm when stdin is a
TTY. It reads terminal events through crossterm, renders the filter and model
list with ratatui widgets, and restores the terminal before returning a typed
result. The existing dialoguer path remains available for explicit non-dialog
model selection.

## Keyboard-driven model settings dialog

The interactive `watn models` flow (TTY stdin) runs a ratatui-based
`SettingsDialog` for the three-tier selection sequence. It renders a
two-pane view using ratatui's `List`/`ListState` and `Layout`:

- A filter line that always shows the current filter text.
- The matching model list with the current selection highlighted.
- A reasoning-strength selector (off, low, medium, high) for the current level.
- A status line for the empty state or the unsupported-search notice.

Key bindings:
- Up/Down arrows: move selection through the list.
- PageUp/PageDown: move selection a page at a time.
- Printable characters / Backspace: update the filter.
- Tab: cycle reasoning strength.
- Enter: accept the highlighted model and advance to the next level.
- Escape: return to the previous level (not on the first level).
- Ctrl-C: return an interrupted typed result (terminal restored before status
  130 is applied by the caller).

Filter matching is per-word and order-independent against the model id: the
query is split on whitespace and every word must appear (case-insensitive)
anywhere in the id, in any order ("dee flash" matches "DeepSeek V4 Flash").
When the provider cannot be searched remotely, matching falls back to this
local rule over the models already fetched.

## Keyboard-driven provider setup

The `watn provider` command uses a ratatui/crossterm state machine with endpoint,
credential-source, credential-value, and review states. Enter advances or
confirms; Escape and Ctrl-C cancel. The terminal is restored on success,
validation failure, and cancellation. The automatic first-use path invokes this
dialog and the model settings dialog in the same process. A successful
automatic flow stops after model selection; it does not send or resume the
original question. A model cancellation or failure preserves the saved
provider and stops the flow.

The stale-result guard uses `Arc<AtomicU64>` as a generation counter. Each
filter change increments the counter before dispatching a search; the worker
discards a response whose generation has advanced (newest-result-wins).

## Per-level reasoning configuration

Each tier's reasoning strength is persisted in config under `[tiers.reasoning]`:

```toml
[tiers.reasoning]
small = "off"    # one of off | low | medium | high
normal = "low"
thinking = "high"
```

When a request runs on a tier, `reasoning_effort` is resolved from the
configured strength: `off` (or absent config) sends no reasoning; any other
strength sends that value. When a tier has no explicit reasoning configured,
the prior default is preserved (thinking → "high", others → none) for
backwards compatibility.

## PTY-based E2E test harness

The two provider-setup E2E scenarios use `portable-pty` (dev-dep) to create a
real pseudo-terminal for the ratatui/crossterm subprocess. Regular provider
scenarios use the renderer-independent setup/config seam and do not pipe stdin
into a terminal renderer.

The test helper `run_binary_pty`:
1. Creates a PTY pair (master + slave).
2. Spawns the `watn` binary with the slave as its controlling terminal.
3. Writes timed keystroke sequences to the master (`(delay_ms, key_sequence)`).
4. Reads PTY output via non-blocking polling with a timeout.
5. Populates `world.output` for Then-step assertions.

The provider-setup PTY approach is scoped to exactly the two `@e2e` scenarios
listed in the change's interaction inventory. The harness gives each scenario
an ephemeral loopback HTTP transport override at HTTP construction time. The
override covers both `/models` and `/chat/completions`, is never persisted, and
is never used by readiness. The persisted OpenRouter endpoint remains exactly
`https://openrouter.ai/api/v1` in the assertions.
