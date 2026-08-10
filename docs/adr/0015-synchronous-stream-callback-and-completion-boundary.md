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

## Consequences

- Good: users see command content and spinner cleanup before a slow response ends
- Good: a response model and usage-only event can drive authoritative metadata
- Good: malformed nonessential events do not erase valid content
- Good: partial output remains available for diagnosis without being executed
- Good: one CLI owner avoids channel lifecycle and cross-thread stderr races
- Bad: providers that omit `[DONE]` now produce a non-zero truncation error
- Bad: verbose reasoning is delayed until successful completion rather than shown progressively
- Bad: one blocking consumer couples provider read progress to stdout write speed
- Bad: callback and terminal failures require careful cleanup and exact-once tests

## Confirmation

The incremental SSE feature scenarios verify content before a release gate,
buffered reasoning absence before completion, completion before a held connection
closes, EOF-without-DONE status 3, mid-stream cleanup and no execution, usage-only
response-model accounting, exact-once command/execution lines, and controlled
I/O failure status 1. Direct parser and spinner lifecycle tests supplement the
real CLI scenarios.
