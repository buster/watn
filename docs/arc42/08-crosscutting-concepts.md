# 8. Cross-cutting Concepts

## Error handling

All fallible operations return a typed `Error` enum. The top-level dispatch maps
each variant to an exit code and prints a diagnostic to stderr:

| Error variant | Exit code | Example |
|---|---|---|
| `ConfigError` | 1 | Malformed TOML, missing required field |
| `ProviderNotFound` | 1 | Invalid persisted provider identity |
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

Setup returns typed outcomes rather than exiting inside lower-level functions.
Escape cancellation maps to status 1; Ctrl-C maps to status 130. The entire
draft remains in memory until Review's Finish action, so invalid endpoints,
missing sources, catalog failures, and incomplete roles cannot create a partial
config. A missing saved environment reference is an authentication error and
does not fall through to another environment variable. After a successful config
commit, shell reconciliation reports per-target failures as a saved-partial
outcome rather than claiming complete setup.

## Shell shortcut safety and file ownership

The optional shortcut is part of explicit setup and implicit first-use setup,
but Enter accepts the default decline. Selection is runtime-only; no provider
configuration field records the chosen shells. The installer resolves Bash and
Zsh from `HOME`, Fish from `XDG_CONFIG_HOME` or the HOME-based XDG fallback, and
uses marker blocks in startup files for initial selections; `SHELL` is only a
fallback hint when no marker state exists.

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
text. On success the widget replaces the buffer with a `#`-prefixed comment
line containing the flattened original request followed by a newline and the
generated text, with the cursor at the end; pressing Enter runs only the
generated command because the shell ignores the comment. Requests are
flattened by replacing CR, LF, and TAB with spaces so they stay one comment
line. Empty input, non-zero status, empty output, and malformed target files do
not replace user content. The result is assigned as text and never evaluated.
For Fish, the comment and generated text are assembled with a shell-produced
newline inside one collected buffer value; a literal `\\n` sequence is not
treated as a newline and is therefore not emitted.

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

Persisted configuration is authoritative for existing setup sessions. First-run
discovery is a separate, finite suggestion process and does not overlay loaded
values. The effective request configuration is resolved in order (later
overrides earlier where a request-time option remains supported):

1. **Built-in defaults** — hardcoded in the binary
2. **User config** — `$XDG_CONFIG_HOME/watn/config.toml`
3. **Credential environment variables** — only the selected saved reference or
   the normal provider credential resolver at the outbound boundary
4. **CLI flags** — `-1`/`-2`/`-3` (highest priority for retained request behavior)

Provider readiness is a separate local check after first-run path detection. A
comment-only existing file is not a first-run signal. A saved literal or exact
environment reference is authoritative. First-run discovery checks a deliberate
allowlist and returns only variable names and non-empty presence flags; when
multiple candidates exist, the user must choose. Readiness never consults the
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
- The setup identities are `openrouter`, `openai`, and `custom`; rerunning setup
  replaces only the selected entry and preserves unrelated providers and
  configuration.

## First-run discovery and persistence

On first use, the binary checks whether the standard XDG path exists before
loading any configuration. A missing path opens interactive setup when stdin is
a TTY, or prints actionable `watn setup` guidance and exits 1 otherwise. No
template, directory, or config file is created by the read. A comment-only file
counts as existing configuration.

The Provider topic discovers only an explicit allowlist of credential variable
names. It records a detected name and presence, never the resolved value.

A Finish action serializes the reviewed supported configuration once through a
restrictive temporary file, flush/sync, atomic rename, and Unix mode `0600`.
Cancellation leaves an existing file byte-for-byte unchanged and leaves a
first-run path absent.

## Model tier resolution

The selected tier (default `-1`) is resolved to a model name via:

1. Retained request tier selector `-1`, `-2`, or `-3`
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
from the pipe. Automatic first-run onboarding is allowed only with TTY stdin.
An implicit non-TTY first-use request emits actionable `watn setup` and
config-path guidance, exits 1, and does not initialize ratatui. Removed
provider/model overrides do not alter persisted selection. Command content goes
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
filter/status paragraphs, an aligned metadata table, and a stateful scrollbar.
The widget that currently receives input uses a green border; inactive widget
borders retain their existing style. Focus changes are derived from the existing
credential, model, and shortcut focus state, so the terminal layout, keyboard
events, and visible cursor remain unchanged. The terminal is restored before
returning a typed result. The `model-picker` module supplies remote search,
local matching, and stale-generation handling; complete catalogs use local
matching while incomplete catalogs use debounced remote search. The wizard
retains search worker handles, invalidates generations on exit, and joins every
worker before returning; there is no separate legacy model prompt path.

## Four-topic SetupWizard

The interactive `watn setup` flow (TTY stdin) renders one bordered topic rail:
Provider, Model roles, Shell integration, and Review. The draft keeps field
origins, credential presence labels, catalog status, role review state, and
shell intent separate from the persisted TOML:

- The active topic and current setting are always visible.
- A visible block cursor on the active editable line.
- A green border around the input block currently receiving keyboard input; the
  border moves between credential storage/value and model/reasoning regions.
- Model roles appear together with catalog suggestions or manual fallback.
- The current filter query remains visible while suggestions are pending.
- Model-specific reasoning options derived from the catalog's supported efforts,
  default effort, enabled flag, and mandatory flag.
- A Review topic summarizes all supported values and warnings.
- Finish is the only configuration write; Escape/Ctrl-C discard the draft.

Key bindings:
- Up/Down arrows: move identity, role, shell, or credential choices.
- Printable characters / Backspace: edit the active endpoint, credential source,
  or manual role.
- Tab: advance through settings and topics.
- Shift-Tab: return to the previous setting or topic.
- Enter: accept the current input/model or Finish the reviewed draft.
- Ctrl-R: cycle supported reasoning strengths for the active catalog role.
- Escape: open the leave/discard prompt.
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

## Setup topic interaction

The Provider topic explains OpenAI-compatible endpoint requirements and masks
literal credentials. The Model roles topic shows Small / fast, Balanced / normal,
and Thinking together. Shell integration derives completion and Ctrl-W choices
from marker blocks. Review shows endpoint, provider identity, credential source
name without a resolved secret, role IDs, reasoning, catalog status, shell
changes, and warnings. The terminal is restored on success, validation failure,
discard, and cancellation.

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
