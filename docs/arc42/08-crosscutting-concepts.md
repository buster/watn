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
| `Interrupted` | 130 | Cancelled by Ctrl+C |

Error messages are human-readable and include context. `Interrupted` is the
exception: it is a user-cancellation marker, printed as no error and mapping
directly to exit 130.

Streaming adds completion and output rules. A provider response is
successful only after `[DONE]`; clean EOF or a reader failure before that marker
is `NetworkError` (exit 3), even when command content was already flushed.
Already visible content is preserved and the spinner is finished, but final
success metadata, buffered verbose reasoning, and execution confirmation are
omitted. A stdout/stderr write or flush failure is `IoError` (exit 1), with the
same prefix-preservation and cleanup guarantees. A user Ctrl+C is `Interrupted`
(exit 130): on the join path the SSE parser stops at the next event once the
flag is set, then the spinner and partial output are finished and no error text
is printed; a stalled or pending stream that cannot be joined exits within the
500 ms grace without cleanup.

Provider setup and model setup return typed results rather than exiting inside
their lower-level functions. Escape cancellation maps to status 1; Ctrl-C maps
to status 130. Coordinated setup does not save partial input: provider,
catalog, model, reasoning, and shell draft values remain in memory until final
review confirmation. A catalog/model failure or cancellation leaves the
baseline unchanged, and a first-run cancellation leaves the file absent.
Focused provider and model commands save only their owned domain after their own
confirmation. Invalid endpoints, empty credentials, and whitespace-only custom
reasoning remain in the setup flow with inline validation. A missing saved
environment reference is an authentication error and does not fall through to
another environment variable.

## Shell shortcut safety and file ownership

The optional shortcut is part of explicit setup and implicit first-use setup.
The shell pages are list-first: a shell is preselected when its binary is on
`PATH` or its target already holds a watn-managed block, and Enter applies the
shown desired state. Selection is runtime-only; no provider configuration field
records the chosen shells. The installer resolves Bash and Zsh from `HOME`,
Fish from `XDG_CONFIG_HOME` or the HOME-based XDG fallback.

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
text. On success the widget records the flattened original request as a
`#`-prefixed comment in the shell history (Bash `history -s`, Zsh `print -s`,
Fish `builtin history append`) and replaces the buffer with only the generated
text, with the cursor at the end; pressing Enter runs only the generated command
as its own history entry. The comment is recorded before any execution, so the
request stays recallable from the shell history even when the command is never
run, and a leading `# ` prefix is stripped from the buffer before asking so a
recalled comment can be edited and re-asked. Requests are flattened by replacing
CR, LF, and TAB with spaces so they stay one comment line. Empty input, non-zero
status, empty output, and malformed target files do not replace user content.
The result is assigned as text and never evaluated.

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

Review surface preference follows the same direction: per-invocation review flags
override the persisted `[review]` panel setting, which overrides the built-in
enabled default. A disabled panel follows the existing direct output, history,
buffer, and `-x` confirmation paths.

## Inline review surface safety

The review surface is a small transient inline region rendered through the
controlling-terminal channel with ordinary ANSI cursor movement. It does not
switch to the alternate screen. The complete Candidate is buffered until
`[DONE]` and final acceptance; review-surface text is never written to stdout,
because the shell widget captures stdout as the command-output channel. The
surface restores cursor visibility, raw input mode, and occupied rows before
returning control to the shell line editor.

Review decisions are explicit: the card opens on the command flow, Enter
accepts, `e` edits, `r` rejects, `c` or Escape cancels, `d`/`?` switches
the view, and `D` disables the review. Direct edits preserve the original Intent and refresh Command flow
and Purpose status. Rejection releases nothing and opens the model chooser;
choosing a configured tier or a typed or catalog-suggested model regenerates the
Candidate, and a failed regeneration keeps the previous Candidate and reports
the failure. Cancellation and failure preserve input; review display and buffer
replacement never evaluate a Candidate. A card that cannot open returns `Unavailable` and releases no
Candidate. A `D` decision inside the card writes `[review] panel = false`
through the atomic configuration save path. A set `--review-panel` or
`--no-review-panel` override records the same setting before generation, so the
last chosen preference survives; a failed write warns without changing the
current invocation.

The review-mode provider response is structured and contains a version, a
complete Candidate command, exact Stage text, model-written Stage purposes, and
Purpose status. Reading is tolerant before validation: literal line breaks,
carriage returns, and tabs inside string values are repaired, and a complete
provider-written command is recovered from a malformed or truncated payload
when it can be delimited unambiguously. A valid structured response can show
`loading` for delayed purposes. A command-only, invalid, stale, or mismatched
response shows `purpose-unavailable` with a Purpose reason and without
replacing the Candidate with locally authored purpose text. A purpose failure
keeps the Candidate reviewable. A fenced or prose-wrapped structured response
is recognized, and a JSON-shaped payload with a complete command keeps that
provider-written command reviewable as `purpose-unavailable`. A review-shaped
payload is never displayed as the command; when no command can be recovered,
the review is `Unavailable` and releases nothing. Rendered review values
flatten line breaks and tabs so they occupy one inline row. Provider-written
stage purposes are kept only when their stage text provably covers the command
verbatim, in order, without overlap, with every purpose non-empty — either as
an exact provider split or as the derived stage text; missing or mismatched
purposes show `purpose-unavailable` and Watn never authors substitute text.

## Provider response diagnostics

Every unusable provider response is captured best-effort in the single
overwritten state file `$XDG_STATE_HOME/watn/last-unusable-response.txt`
(default `~/.local/state/watn/last-unusable-response.txt`) so a bug report can
attach what the provider actually sent. The capture is never configuration and
keeps no history. A capture failure prints a warning and does not change the
review outcome. The saved path is named on stderr after the review surface
closes, because the surface owns the controlling terminal while it is open.
With `-v`/`--verbose`, the raw provider response itself is printed to stderr at
the same point for review and explanation requests; stdout remains the
command-output channel and is never used for diagnostics.

The review card paints colors only when the terminal supports them
(`NO_COLOR` unset, `TERM` not `dumb`, and a color-capable terminal type);
otherwise the same card renders monochrome. Color is presentation only and
never changes command, stage, or purpose text.

The exact keyboard contract has no focus regions: the stage stack is active
when the card opens, arrows move the selected stage, Enter accepts, Escape (or
`c`) cancels, `d`/`?` toggles the view, and `D` disables the review. A separate
command editor uses Enter to commit and Escape to discard; those editor keys do
not accept or cancel the review.

Provider readiness is a separate local check. An absent config file is not
created as a template during readiness; a provider is ready only when its
endpoint and literal or resolved credential are available. OpenRouter has a built-in endpoint fallback only when
no `[providers.openrouter]` entry exists. A saved literal or exact environment
reference is authoritative. Only an absent `api_key` permits provider-specific
fallback followed by generic `WATN_API_KEY`. Readiness never consults the
ephemeral E2E transport override.

## Verbatim command delivery for explanation

`watn explain` explains a command the developer already has, so the command is
authoritative. The shell performs quoting and expansion before watn starts;
watn receives the command as one literal argument or as literal standard-input
bytes and never joins, re-splits, evaluates, or executes it. The supported
forms are a single-quoted positional argument (canonical), a double-quoted
argument when only single quotes need carrying, `--` before a command that
begins with `-`, the `-` marker or piped standard input for a quoted heredoc,
and `watn explain - <<'EOF'` for embedded single quotes, newlines, `$`, and
backticks. A positional argument other than `-` takes precedence and standard
input is not read. Empty input is a usage error, and one trailing `\n` or
`\r\n` is removed from standard input. A second positional argument is refused
rather than joined, because joining would silently change token boundaries.

Because the caller's shell expands `$`, backticks, and `$(...)` before watn
starts, watn cannot prevent that expansion; it only guarantees the bytes it
receives. The explanation card shows those received bytes, so the developer can
verify the command. The model's own command text is never displayed or adopted;
model-written purposes survive only as an exact echo of the command and stages,
or as an ordered, non-overlapping, verbatim stage cover with a non-empty purpose
per stage. `-x` is rejected for explain, and no path executes the command.

The explanation card needs a controlling terminal but not terminal stdin: the
stdin/heredoc form pipes the command while the card renders and reads keys
through `/dev/tty`. Its eligibility predicate is terminal stderr, `TERM` not
`dumb`, and an open `/dev/tty`; the generated-command review gate additionally
requires terminal stdin. A missing terminal is a non-zero diagnostic and no
card opens.

Explain reuses the question path's readiness rule instead of degrading
silently. When no usable provider or model is configured and no provider or
model was explicitly selected, no card opens: non-terminal command input prints
the existing setup guidance and exits 1; terminal command input starts quick
setup when no configuration file exists and the setup wizard when one exists,
then prints `setup complete; rerun watn explain with the command` and exits 0
without explaining the original command. An explicit `--provider`, `--model`,
or `WATN_PROVIDER` selection never enters setup and keeps its
unknown-provider (exit 1), missing-model (exit 1), or missing-credential
(exit 2) error.

A failed explanation request is visible. The card still opens with the
developer's command and `purpose-unavailable`, `explain request failed:
<error>` is printed to stderr, and the invocation exits with the mapped status
(auth/API 2, network 3, config/IO 1) after the card closes. A response that is
not usable as an explanation prints `explain response was not usable:
<reason>`, keeps the card reviewable with `purpose-unavailable` and the Purpose
reason, captures the raw response in the state file with the saved path named
on stderr, and exits 0 because the request itself succeeded and the safe
degradation is deliberate. Literal control characters inside string values are
repaired before the echo is checked, so a line-broken explanation response is
read like a valid one. Ctrl+C during the request opens no card and exits 130.
The explained command is never generated, replaced, edited, evaluated, or
executed on any path.

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

Model discovery is provider-local. A saved provider catalog base is reused when
it belongs to the selected provider and has not been invalidated; otherwise the
accepted completion endpoint supplies the derived base. Setup probes
`GET <catalog-base>/models`, uses the selected provider credential, and promotes
an edited or newly derived base only after valid model data is returned.
Pagination and search use the same provider-local source. Empty or malformed
data, missing identifiers, duplicate identifiers, unreachable endpoints, and
failed edits switch to manual mode according to the saved-endpoint state.

The legacy `[litellm]` section remains readable and is preserved as unrelated
configuration, but it is not contacted, migrated, or used as a fallback by
setup or model discovery. Chat requests remain on the selected provider.

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

## First-run and atomic configuration snapshots

On the first invocation, when no config file exists at the standard XDG path,
the setup coordinator starts with in-memory defaults. It does not write a
template before or during the draft. The file is created only by successful
final confirmation. Malformed or unreadable existing configuration is reported
and left untouched.

The confirmed candidate is serialized once to a same-directory temporary file,
flushed, given mode `0600` on Unix, and renamed over the destination. A failed
serialization, permission update, or rename leaves the previous destination in
place and prevents shell operations from starting. Shell target files are
separate desired-state operations and are not rolled back with the config.

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

One computation serves every display: `amount::recorded_price` applies the
recorded price of the model the provider reported, or — when that model has no
entry — the recorded price of the model the request was sent with, because a
provider routinely answers with its canonical identifier while the recorded
price carries the configured alias. `amount::billed_amount` returns `None` when
no price applies, and otherwise the amount plus whether the response carried a
usage report at all. The stderr metadata line
keeps its historical behaviour — a price entry with no usage report still prints
`$0.0000` — while the review surface and the explanation card show the Amount
only when a usage report existed, in cents with one significant digit and never
more than four decimal places of a cent, trailing zeros trimmed, whole cents
from one cent on. The Amount is priced by that rule and is shown at the Model
label, which names the model the request was sent with. It is transient, appears only on the controlling-terminal channel, and
gates no review decision.

The explanation path computes an Amount too; it is the first amount that path
has ever displayed, since `watn explain` prints no post-request metadata.

A model catalog publishes prices in USD per token. The catalog parser converts
each component once to $/1M (rounded to six decimals) and treats a missing or
negative component as no price. Assigning tiers captures each chosen model's
normalized price into `[pricing]`, so cost display works without manual price
entry. Capture replaces only the chosen model's entry; entries for unchosen
models and for chosen models without a valid catalog price are preserved.

## Tokens/second

Wall clock measured from the first non-`[DONE]` SSE data line, before JSON
decoding, to the observed `[DONE]` marker via `std::time::Instant`. Time waiting
for a server-side connection close is excluded.
tok/s = completion_tokens / elapsed_seconds. Displayed after response completion.

## Execution mode (`-x`)

When review is disabled or not eligible and `-x` is passed, the command retains
the existing incremental stdout and `Execute now? [Y/n]` confirmation. When
review is enabled and eligible, the complete Candidate is buffered until
`[DONE]`; final review acceptance is the sole execution authorization and no
second confirmation is shown. In both modes, accepted execution uses the
existing `sh -c <cmd>` boundary with inherited stdout/stderr. A stream,
Candidate, panel, or output failure never reaches execution.

## Reasoning and verbose mode

When a tier has a non-`off` reasoning value, the request body includes a top-level `reasoning_effort` parameter:

```json
{"reasoning_effort": "high"}
```

This signals the API to generate reasoning tokens alongside the answer. Any
non-empty configured value, including provider-specific text, is sent exactly as
persisted. `off` omits the field. Catalog metadata may provide suggestions and
may block `off` for mandatory reasoning, but it does not reject a non-empty
custom value.

Response chunks from the API may include a `reasoning` or
`reasoning_content` field in the delta object alongside the `content` field. The
provider accumulates reasoning content separately from command content and does
not send it through the incremental output callback.

When `-v` / `--verbose` is passed and the stream reaches `[DONE]`, the
accumulated reasoning content is printed to stderr on its own line prefixed with
`reasoning:`. It is buffered until completion, so it is absent from stderr while
the provider is still sending content. If the model returned no reasoning
content, or if the stream failed, nothing additional is printed.

For review and explanation requests, `-v` additionally prints the raw provider
response to stderr after the review surface closes, labelled as the raw
provider response. It is never printed while the card owns the terminal and
never written to stdout.

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
| 130 | Interrupted | SIGINT (Ctrl+C) while a completion is in flight; bounded by the 500 ms grace when the worker is unreachable |

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

## Keyboard-driven setup questions

The interactive `watn setup`, `watn provider`, `watn models`, and `watn shell`
flows (TTY stdin) share a ratatui-based draft coordinator. The complete flow
renders one bordered question at a time for provider identity, completion
endpoint, credential source, provider-local catalog, three separate model and
reasoning pairs, two shell desired states, and a final review:

- The active question and its position in the focused command flow.
- A visible block cursor on the active editable line.
- A green border around the input block currently receiving keyboard input; the
  border moves between credential storage/value and model/reasoning regions.
- A filter paragraph and aligned model table on each model page.
- The current filter query remains visible while suggestions are pending.
- Model-specific reasoning options derived from the catalog's supported efforts,
  default effort, enabled flag, and mandatory flag.
- A scrollbar showing position when the catalog exceeds the available rows.
- Catalog-supported reasoning suggestions plus a custom non-empty entry; `off`
  is omitted from requests and is blocked for mandatory reasoning.
- A status line for the empty state or the unsupported-search notice.
- List-first shell completion and Ctrl-W pages: a shell is preselected when its
  binary is on `PATH` or its target already has a watn-managed block; Space
  toggles the highlighted shell and Enter applies the shown desired state.

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
local rule over the models already fetched. Reasoning choices are suggested per
model: mandatory models cannot choose `off`, disabled models offer `off`, and a
custom non-empty value remains available alongside catalog choices. A model
change selects that model's catalog default when available but does not discard
an explicit custom value without user confirmation.

## Setup visual language

Every setup page shares the review panel's visual language. The frame is
labelled `watn · setup`; the active page is marked `◆`; the highlighted list
or table row is marked `▶`; selected and unselected shell choices use `●` and
`○`; guidance uses `↳`; validation and warnings use an amber `⚠`. Labels and
the active tab are cyan, values are white, the focused input accent and
selected state are green, secondary text is dim, and panel borders are dim
gray. Footer hints pair a bold cyan key with a dim label joined by `·`.

The same palette is disabled as a whole when the terminal does not support
color (`NO_COLOR`, `TERM=dumb`, or a terminal without color capability): every
style falls back to the default and only the symbols and text remain. The
decision uses the same pure capability function as the review card.

## Focused provider, model, and shell setup

Provider setup presents OpenRouter, OpenAI, or Custom, then completion endpoint
and credential source. It never probes the catalog. Models setup requires a
ready provider, uses only its catalog source, and persists only roles and
reasoning. Shell setup presents completion and Ctrl-W desired states as
list-first pages: detected shells (PATH binaries plus existing managed blocks)
are preselected, toggled with Space, and applied on Enter. A page where nothing
was detected and nothing is selected performs no target inspection or write.

Coordinated setup retains all values in memory through the final review. Escape
discards the draft and Ctrl-C returns 130; neither changes configuration or
shell targets. The terminal is restored on success, validation failure,
review/discard, and cancellation. Back-navigation keeps draft values and marks
catalog-backed selections stale when the provider changes.

The stale-result guard uses `Arc<AtomicU64>` as a generation counter. Each
filter change increments the counter before dispatching a search; the worker
discards a response whose generation has advanced. Generation order is
user-entry order, not completion order, so a slower newer search may replace an
older completed result while a late older result is discarded.

## Per-level reasoning configuration

Each tier's reasoning value is persisted in config under `[tiers.reasoning]`:

```toml
[tiers.reasoning]
small = "off"    # off omits the request field; other non-empty values are sent verbatim
normal = "low"
thinking = "high"
```

When a request runs on a tier, `off` is omitted and every other non-empty value
is copied byte-for-byte into `reasoning_effort`. Empty and whitespace-only setup
values are rejected. Existing unknown values remain active. Model metadata
supplies defaults and may prohibit `off` for mandatory reasoning, but it does
not turn the persisted value into a closed enum.

## PTY-based E2E test harness

The five setup interaction-inventory E2E scenarios use `portable-pty` (dev-dep)
to create a real pseudo-terminal for the ratatui/crossterm subprocess. Regular
scenarios may use isolated config and loopback seams for focused assertions,
but every E2E scenario drives the actual CLI terminal.

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

## Specification ownership and consolidation

The active Gherkin tree is one behavior inventory. A scenario title is owned
repository-wide, and a duplicate title is a deterministic finding even when
the scenarios live in different feature families. Shape and subset findings
are evidence for review, not automatic semantic classification. The reviewer
records whether a weaker scenario is removed, retained as a distinct boundary,
or replaced by a stronger added scenario.

Archive applies removals and additions atomically and runs the full Watn
runner afterward. The review disposition is the human audit trail: it names
the retained contract and prevents a consolidation from being justified only
by a green test count.

## Archive verification contract

`givn archive` is forward-only and Git-backed: the project must be a committed
Git worktree rooted at the project root before the archive starts. Each
configured verification scope writes one JSON result to `GIVN_RESULT_FILE` from
the runner's own scenario counts (`scope`, `total`, `passed`, `failed`,
`skipped`); counts are never hardcoded or inferred from the exit code. The gate
is fail-closed: a missing file, malformed JSON, wrong scope, `total == 0`,
`failed != 0`, `skipped != 0`, or a non-reconciling sum blocks publication. The
archive receipt is the execution evidence, `givn check review` no longer runs
the test suite, and archives published before 0.7.0 stay historical without a
receipt.

## Specification migration integrity

The active corpus has one stable owner for every capability. Use-case documents
represent independent user goals; fragment documents represent reusable
infrastructure. A migration records the old owner, semantic operation, new
owner, capability, E2E mapping, behavior-change status, and reason in one
ledger. File moves alone are not evidence of a correct migration.

Before mutation, the migration captures the Givn version/configuration, active
changes, ideation state, scenario behavior hashes, E2E mappings, interaction
coverage, and source/branch coverage. After mutation it validates strict
use-case/fragment schemas, exact capability ownership, typed relationship
resolution, interaction mapping, unchanged evidence, and coverage counters.
An absent ideation topic or Persona is recorded as absent; neither is invented
or promoted. Historical `givn/archive/` paths are excluded from active audits
and remain unchanged.
