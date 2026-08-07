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
2. **System config** — `/etc/watn/config.toml`
3. **User config** — `$XDG_CONFIG_HOME/watn/config.toml`
4. **Environment variables** — `WATN_*` (e.g. `WATN_PROVIDER`, `WATN_MODEL`)
5. **CLI flags** — `-1`/`-2`/`-3`, `--model`, `--provider` (highest priority)

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

The binary detects whether stdout is a TTY using `std::io::stdout().is_terminal()`.
When stdout is not a TTY: no ANSI escape codes. Metadata is still included as plain text.

## Exit code convention

| Code | Meaning | Usage |
|---|---|---|
| 0 | Success | Command generated and printed |
| 1 | User error | Bad argument, bad config, unknown provider, I/O error |
| 2 | API error | Auth failure, rate limit, server error |
| 3 | Network error | DNS, connection, timeout |
| 130 | Interrupted | SIGINT (Ctrl+C) during streaming |