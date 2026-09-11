# ADR-0015: Synchronous stream callback and completion boundary

- **Status:** accepted
- **Date:** 2026-08-10
- **Decision-makers:** architect

## Context and Problem Statement

The CLI must expose generated command content while an OpenAI-compatible SSE
response is still arriving. The current final-result-only path hides usable
prefixes, leaves the spinner active, and cannot distinguish a complete response
from a provider that closes early. Reasoning has a different presentation
policy: it is useful in verbose diagnostics, but must not race command output or
spinner cleanup.

How should one blocking CLI consumer receive content, determine completion, and
handle reasoning and terminal failures without duplicating the command?

## Decision Drivers

- Show command content after each complete SSE event without adding an async runtime
- Keep provider parsing and CLI output failures on the existing typed error path
- Make successful completion unambiguous for providers and scripts
- Keep reasoning off the incremental command path and visible only under `-v`
- Preserve visible prefixes and prevent metadata or execution after failure
- Make timing and terminal cleanup observable in deterministic tests

## Considered Options

- **Synchronous content callback with a blocking reader** — the provider parses
  each event and invokes the CLI-owned sink directly; no channel is needed
- **Worker thread plus channel** — a producer parses the response while the CLI
  consumes separate event messages
- **Buffered final aggregate** — parse the entire body and render only after the
  provider returns

## Decision Outcome

Chosen: **a synchronous callback for command content, with no channel**. The
blocking provider reads complete SSE data events through a buffered reader and
invokes the callback for each non-empty content delta. The callback writes and
flushes stdout, stops the spinner on first content, and propagates write/flush
failures as the existing I/O error.

The provider accumulates reasoning in the final response aggregate but does not
emit user-visible incremental reasoning. After a successful stream only, `-v`
prints the buffered reasoning to stderr before final metadata. The command's
incremental chunks are the only command rendering; the final aggregate is used
for trimming and execution, not printed again.

`[DONE]` is mandatory and is the only successful terminator. EOF without it is a
truncated stream, even after valid content, and maps to the existing network
status 3. The CLI preserves visible content, finishes the spinner, omits final
success metadata and execution, and reports the mapped error. A command-output
write or flush failure maps to the existing I/O status 1 with the same cleanup
and omission rules.

Elapsed time starts at the first non-DONE data event, before JSON decoding, and
ends when `[DONE]` is observed. A client may complete and drop the response
without waiting for the server to close its connection after `[DONE]`.

## Review-mode amendment

Review-eligible requests use the same synchronous provider callback and the
same `[DONE]` completion boundary with a mode-specific buffered sink. The sink
assembles the complete Candidate without writing Candidate text to stdout. The
existing progress line remains the first feedback, and the review surface opens
only after `[DONE]` and Candidate validation. The review surface uses the
controlling-terminal channel, never stdout.

Only explicit final review acceptance releases the selected Candidate. Ctrl-W
then records the original Intent and replaces the shell line-editor buffer
through the existing widget; direct positional and interactive-stdin requests
write only the accepted Candidate through the existing command-output channel;
eligible `-x` uses review acceptance as its sole execution authorization. A
cancelled, rejected, failed, empty, or unavailable review releases no Candidate.
Disabled and non-review requests retain the incremental output and existing
`Execute now?` confirmation behavior described above.

## Permanent-disable amendment

A permanent review disable is the second explicit final decision. When the
developer disables the review from the surface, the current Candidate is
released to the existing command-output channel, the re-enable instruction is
written to stderr, and the invocation ends without executing the Candidate,
including under eligible `-x`. Review-surface text still never reaches stdout,
and every other release rule of the review-mode amendment is unchanged. A
review-panel switch without a request only persists the preference and exits.

## Consequences

- Good: users see command content and spinner cleanup before a slow response ends
- Good: a response model and usage-only event can drive authoritative metadata
- Good: malformed nonessential events do not erase valid content
- Good: partial output remains available for diagnosis without being executed
- Good: one CLI owner avoids channel lifecycle and cross-thread stderr races
- Good: review-surface bytes cannot contaminate stdout, and final acceptance is
  a single release and execution gate for eligible review paths
- Bad: providers that omit `[DONE]` now produce a non-zero truncation error
- Bad: verbose reasoning is delayed until successful completion rather than shown progressively
- Bad: one blocking consumer couples provider read progress to stdout write speed
- Bad: callback and terminal failures require careful cleanup and exact-once tests
- Bad: review-eligible requests do not expose Candidate bytes incrementally and
  therefore depend on the existing progress line for generation feedback
- Bad: a permanent review disable releases the current Candidate without
  execution, so a consumer of the command-output channel must distinguish that
  explicit release from execution authorization

## Confirmation

The incremental SSE feature scenarios verify content before a release gate,
buffered reasoning absence before completion, completion before a held connection
closes, EOF-without-DONE status 3, mid-stream cleanup and no execution, usage-only
response-model accounting, exact-once command/execution lines, and controlled
I/O failure status 1. Direct parser and spinner lifecycle tests supplement the
real CLI scenarios. Review-mode scenarios additionally verify that the buffered
sink releases no Candidate before `[DONE]` or final acceptance, that the
controlling-terminal channel remains separate from stdout, and that disabled and
non-review paths retain the incremental output and confirmation contracts.
