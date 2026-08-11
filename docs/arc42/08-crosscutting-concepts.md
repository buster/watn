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

Streaming adds two completion and output rules. A provider response is
successful only after `[DONE]`; clean EOF or a reader failure before that marker
is `NetworkError` (exit 3), even when command content was already flushed.
Already visible content is preserved and the spinner is finished, but final
success metadata, buffered verbose reasoning, and execution confirmation are
omitted. A stdout/stderr write or flush failure is `IoError` (exit 1), with the
same prefix-preservation and cleanup guarantees.

Provider setup and model setup return typed results rather than exiting inside
their lower-level functions. Escape cancellation maps to status 1; Ctrl-C maps
to status 130. Provider setup does not save partial input. If provider setup
succeeds and model setup is cancelled or fails, the provider remains saved, the
automatic flow stops, and the original request is not sent. Invalid endpoints
and empty credentials remain in the setup flow with an inline validation
message. A missing saved environment reference is an authentication error and
does not fall through to another environment variable.

## Shell shortcut safety and file ownership

The optional shortcut is part of explicit setup and implicit first-use setup,
but Enter accepts the default decline. Selection is runtime-only; no provider
configuration field records the chosen shells. The installer resolves Bash and
Zsh from `HOME`, Fish from `XDG_CONFIG_HOME` or the HOME-based XDG fallback, and
uses only the basename of `SHELL` for preselection.

Each target is treated as user-owned bytes. A target with no shortcut markers
gets one generated block appended. An existing target must contain exactly one
opening marker and one closing marker in that order; duplicate, unmatched, or
reversed markers fail before any write and leave the target unchanged. Content
outside the block is preserved. The replacement is written to a uniquely named
temporary file in the target directory, flushed and synced, then atomically
renamed over the target while retaining an existing mode where possible.

Selected targets are attempted independently. The installer reports each
successful path and reload instruction and each failure with its exact path and
operating-system reason. Successful changes are not rolled back when another
target fails; the aggregate setup result is non-zero if any selected target
fails.

The generated Bash, Zsh, and Fish widgets use their native line-editor buffer
and cursor APIs. They call `command watn -- "$question"` through `PATH`, so a
leading option or the reserved `completions` token remains one question. Only
stdout is captured; stderr remains visible. A zero-status non-empty result has
trailing CR/LF characters removed, while embedded line breaks remain buffer
text. Empty input, non-zero status, empty output, and malformed target files do
not replace user content. The result is assigned as text and never evaluated.

## Completion generation

`watn completions <SHELL>` uses a local closed `CompletionShell` selector. The
only accepted values are the lowercase literals `bash`, `elvish`, `fish`,
`powershell`, and `zsh`; the
CLI does not expose the broader `clap_complete::Shell` selector. The parser's
stable literal error contract is
`unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh`, embedded in the
normal non-zero CLI argument error for an unsupported value.

Successful generation is an output boundary distinct from normal command
execution. It derives the root options, `question` positional argument,
subcommands, and selector value suggestions from `Cli::command()`, writes the
selected script only to stdout, leaves stderr empty, and returns before config
loading, config auto-init, provider resolution, model discovery, network access,
or spinner setup. It does not write a completion file, shell startup file, or
any other file. Repeated generation from the same binary and selector is
byte-for-byte deterministic, and the generated script must be accepted by its
target shell parser.

The no-config verification snapshots the absent isolated
`$XDG_CONFIG_HOME/watn/config.toml` and a provider-request sentinel with zero
hits before execution. Both observations remain unchanged after successful
generation. Help is also explicit: `watn completions --help` exits 0, includes
`Usage: watn completions <SHELL>`, names `bash`, `elvish`, `fish`, `powershell`, and `zsh`, and explains
that stdout carries the script for the caller to install or source.

The `completions` subcommand reserves an unquoted first token of that name.
Question text beginning with the token must be quoted or passed after `--`.

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

## Transport isolation

The endpoint override is a compile-time test capability, not a configuration
setting. The only branch that may read `WATN_TEST_ENDPOINT_OVERRIDE` is guarded
by `cfg(all(feature = "test-support", debug_assertions))`. The negated branch
returns the configured endpoint for default-feature debug binaries, default
release binaries, and release binaries built with `test-support`.

The override is resolved only while constructing an outbound HTTP request. URL
builders receive the effective endpoint and perform no environment lookup.
Configuration loading, readiness, provider persistence, and endpoint display
always retain the configured `<base>/v1` URL. A missing or whitespace override
falls back to that URL in the debug test-support branch.

Transport verification starts separate loopback twins and asserts full endpoint,
method/path, per-child and aggregate request counts, exact
`Authorization: Bearer <key>`, competing server zero hits, response source, and
unchanged persisted endpoint. The `cfg` guard keeps the override out of
release-profile compilation. Release verification builds the artifact and
inspects its target-dependent runtime libraries with `file` and `ldd` on Linux
or `otool -L` on macOS.

Environment-backed credentials are persisted as complete references such as
`${OPENROUTER_API_KEY}`. The resolver expands the reference for an outbound
request, while the serializer preserves the reference.

## Catalog source resolution

Model discovery does not share the chat provider's endpoint by accident. When
`[litellm]` exists, its endpoint is used for `/models`, pagination, and search;
the selected provider remains the chat destination. LiteLLM authentication is
optional. If its key is absent, the request omits `Authorization`; if a key is
an environment reference, the reference is expanded at the request boundary.
When `[litellm]` is absent, discovery falls back to the selected provider and
uses that provider's credential-source precedence.

Request tests use separate loopback twins and match exact method, path, query,
and Authorization. A mock hit without source-specific assertions is not
considered evidence of correct routing.

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
The response model from the final valid provider aggregate selects the pricing
entry, including when it was supplied by a choices-empty usage event. Displayed
only after `[DONE]`. When pricing is not configured, cost is omitted from output.

## Tokens/second

Wall clock measured from the first non-`[DONE]` SSE data line, before JSON
decoding, to the observed `[DONE]` marker via `std::time::Instant`. Time waiting
for a server-side connection close is excluded.
tok/s = completion_tokens / elapsed_seconds. Displayed after response completion.

## Execution mode (`-x`)

When `-x` is passed, the command has already been rendered incrementally to
stdout exactly once before the user is prompted on stderr: `Execute now? [Y/n]`.
The final aggregate is used for the confirmation command but is not printed
again. Empty line or `y`/`Y` runs the command via `sh -c <cmd>` with inherited
stdout/stderr. `n`/`N` exits 0. A stream or output failure never reaches the
prompt.

## Reasoning and verbose mode

When the thinking tier (`-3` / `--thinking`) is activated, the request body includes a top-level `reasoning_effort` parameter:

```json
{"reasoning_effort": "high"}
```

This signals the API to generate reasoning tokens alongside the answer. A valid non-`off` configured strength may be sent for any model tier; the thinking tier retains its compatibility default of `high` when no value is configured.

Response chunks from the API may include a `reasoning` or
`reasoning_content` field in the delta object alongside the `content` field. The
provider accumulates reasoning content separately from command content and does
not send it through the incremental output callback.

When `-v` / `--verbose` is passed and the stream reaches `[DONE]`, the
accumulated reasoning content is printed to stderr on its own line prefixed with
`reasoning:`. It is buffered until completion, so it is absent from stderr while
the provider is still sending content. If the model returned no reasoning
content, or if the stream failed, nothing additional is printed.

The verbose flag is independent of the thinking tier. Any tier with `-v` will print reasoning content if the API returns it. Without `-v`, reasoning content is accumulated into the response struct but not printed.

## Pipe and TTY detection

The binary detects whether **stdin** is a TTY using
`std::io::stdin().is_terminal()`. When stdin is not a TTY, the question is read
from the pipe. Automatic provider onboarding is allowed only for implicit
provider selection with TTY stdin. An implicit non-TTY first-use request emits
actionable `watn provider` and config-path guidance, exits 1, and does not
initialize ratatui. Explicit `--provider` and `WATN_PROVIDER` selections retain
their existing resolution errors regardless of TTY state. Command content goes
to stdout incrementally; final metadata, buffered verbose reasoning, errors, and
setup guidance go to stderr as plain text (suitable for scripting).

## Exit code convention

| Code | Meaning | Usage |
|---|---|---|
| 0 | Success | Command generated and printed |
| 1 | User error | Bad argument, bad config, unknown provider, I/O error, setup Escape cancellation, or non-TTY onboarding guidance |
| 2 | API error | Auth failure, rate limit, server error |
| 3 | Network error | DNS, connection, timeout |
| 130 | Interrupted | SIGINT (Ctrl+C) during streaming |

## Model interaction modes

The SetupWizard uses ratatui and crossterm when stdin is a TTY. It reads
terminal events through crossterm, renders a bordered model page with tier tabs,
filter/status paragraphs, an aligned metadata table, and a stateful scrollbar,
then restores the terminal before returning a typed result. The `model-picker`
module supplies remote search, local matching, and stale-generation handling;
there is no separate legacy model prompt path.

## Keyboard-driven SetupWizard model pages

The interactive `watn setup`, `watn provider`, and `watn models` flows (TTY
stdin) share a ratatui-based setup wizard. It renders one bordered page at a
time with tabs for URL, API key, Small Model, Middle Model, and Large Model:

- The active page and `Page n of 5` position.
- A visible block cursor on the active editable line.
- A filter paragraph and aligned model table on each model page.
- Model-specific reasoning options derived from the catalog's supported efforts,
  default effort, enabled flag, and mandatory flag.
- A scrollbar showing position when the catalog exceeds the available rows.
- A reasoning-strength selector (off, low, minimal, medium, high) for the current level.
- A status line for the empty state or the unsupported-search notice.

Key bindings:
- Up/Down arrows: move selection through the list.
- PageUp/PageDown: move selection a page at a time.
- Printable characters / Backspace: update the filter.
- Tab: advance to the next wizard page.
- Shift-Tab: return to the previous wizard page.
- Enter: accept the current input/model and advance.
- Ctrl-R: toggle focus between the model table and model-specific reasoning;
  Up/Down changes the selected supported effort while reasoning is focused.
- Escape: open the save/discard prompt.
- Ctrl-C: return an interrupted typed result (terminal restored before status
  130 is applied by the caller).

Filter matching is per-word and order-independent against the model id: the
query is split on whitespace and every word must appear (case-insensitive)
anywhere in the id, in any order ("dee flash" matches "DeepSeek V4 Flash").
When the provider cannot be searched remotely, matching falls back to this
local rule over the models already fetched. Reasoning choices are then derived
per model: mandatory models cannot choose `off`, disabled models offer `off`,
and supported efforts are limited to the catalog response. A model change
resets the tier's reasoning choice to that model's default or first valid
effort.

## Keyboard-driven provider setup

The shared setup wizard uses URL, API key, Small Model, Middle Model, and Large
Model pages. URL input explains OpenAI/LiteLLM compatibility. API key input
first selects configuration storage or an environment reference, then asks for
the corresponding value. Model pages use the same searchable table and visible
cursor. `watn provider` starts at URL and ends after API key; `watn models`
starts at Small Model; `watn setup` and automatic first use traverse all pages.
Escape asks whether to save current valid settings or discard them. The
terminal is restored on success, validation failure, save/discard, and
cancellation.

The stale-result guard uses `Arc<AtomicU64>` as a generation counter. Each
filter change increments the counter before dispatching a search; the worker
discards a response whose generation has advanced. Generation order is
user-entry order, not completion order, so a slower newer search may replace an
older completed result while a late older result is discarded.

## Per-level reasoning configuration

Each tier's reasoning strength is persisted in config under `[tiers.reasoning]`:

```toml
[tiers.reasoning]
small = "off"    # one of off | low | minimal | medium | high
normal = "low"
thinking = "high"
```

When a request runs on a tier, `reasoning_effort` is resolved from the closed
set `off`, `low`, `minimal`, `medium`, and `high`. `off`, empty, and unknown
values send no reasoning. When a tier has no explicit reasoning configured, the
prior default is preserved (thinking -> `high`, others -> none) for backwards
compatibility. Model metadata defaults are applied by the same policy in the
interactive and non-interactive selection paths; mandatory reasoning preserves
a valid existing non-off value or returns a typed policy error when metadata
has no usable effort, and no empty value is serialized.

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

The provider-setup PTY approach remains scoped to its existing inventory. The
transport change's three CLI `@e2e` scenarios use explicit subprocess binary
paths and local loopback twins. Only the debug test-support binary receives an
ephemeral HTTP construction override; default and all release-profile binaries
use the configured endpoint. The override is never persisted and never used by
readiness. The transport scenarios assert the exact captured configured
loopback endpoint rather than a live or example provider URL.
