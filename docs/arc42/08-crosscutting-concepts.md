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

## Configuration layering

Config is merged in order (later overrides earlier):

1. **Built-in defaults** — hardcoded in the binary
2. **User config** — `$XDG_CONFIG_HOME/watn/config.toml`
3. **Environment variables** — `WATN_*` (e.g. `WATN_PROVIDER`, `WATN_MODEL`)
4. **CLI flags** — `-1`/`-2`/`-3`, `--model`, `--provider` (highest priority)

## Auto-init (first-run template)

On the first invocation, when no config file exists at the standard XDG path
(`$XDG_CONFIG_HOME/watn/config.toml`), the binary writes a template file with
all options commented out. The template is generated from code
(`Config::template_content()`) rather than a hardcoded string, ensuring that
adding a new config field automatically includes it in the template.

The template includes commented-out sections for defaults, tiers, custom
providers, and pricing. The file write is silent and does not interrupt the
command the user issued. If a config file already exists, nothing is written.

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

## Pipe detection

The binary detects whether stdin is a TTY using `std::io::stdin().is_terminal()`.
When stdin is not a TTY, the question is read from the pipe. Command output goes
to stdout; metadata goes to stderr as plain text (suitable for scripting).

## Exit code convention

| Code | Meaning | Usage |
|---|---|---|
| 0 | Success | Command generated and printed |
| 1 | User error | Bad argument, bad config, unknown provider, I/O error |
| 2 | API error | Auth failure, rate limit, server error |
| 3 | Network error | DNS, connection, timeout |
| 130 | Interrupted | SIGINT (Ctrl+C) during streaming |

## Raw terminal input (autosuggest picker)

The model autosuggest picker operates in raw terminal mode via the `console`
crate (explicit dep). Raw mode disables line buffering and echo — each
keystroke is read individually via `console::Term::read_key()`. The picker
enters raw mode at the start of each tier prompt and restores cooked mode
before returning control to `run_models`.

## Keyboard-driven model settings dialog

The interactive `watn models` flow (TTY stdin) runs a ratatui-based
`SettingsDialog` instead of per-tier raw-mode prompts. It renders a
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
- Ctrl-C: exit process (terminal restored before exit).

Filter matching is per-word and order-independent against the model id: the
query is split on whitespace and every word must appear (case-insensitive)
anywhere in the id, in any order ("dee flash" matches "DeepSeek V4 Flash").
When the provider cannot be searched remotely, matching falls back to this
local rule over the models already fetched.

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

Raw-mode terminal applications cannot be driven through piped stdin (the
picker reads from `/dev/tty`, not fd 0). E2E tests for the autosuggest picker
use `portable-pty` (dev-dep, latest stable, checked crates.io) to create a
real pseudo-terminal for the subprocess.

The test helper `run_binary_pty`:
1. Creates a PTY pair (master + slave).
2. Spawns the `watn` binary with the slave as its controlling terminal.
3. Writes timed keystroke sequences to the master (`(delay_ms, key_sequence)`).
4. Reads PTY output via non-blocking polling with a timeout.
5. Populates `world.output` for Then-step assertions.

This approach is scoped to the `@model-autosuggest` and `@ratatui-model-picker`
features. Existing scenarios continue using the piped-stdin path.