# Proposal: incremental-sse-rendering

## Problem / Opportunity

Watn waits for the complete provider response before showing anything. Long
answers therefore look stalled, the spinner remains active while useful text is
already available, and a failure near the end can hide text that was received
successfully. Final usage information is also unreliable when a provider sends
usage separately from the content events.

## Proposed Solution

Watn writes each non-empty generated-command content chunk to standard output as
soon as its complete SSE event has been parsed, and flushes that chunk. The
spinner stops and is cleaned up when the first command content becomes visible.
The provider is consumed synchronously through a callback owned by the CLI; no
background channel is introduced.

Reasoning is accumulated privately until the provider stream completes. It is
printed to standard error only after a successful completion and only when
`-v`/`--verbose` is active. Reasoning is never rendered incrementally and never
races the spinner or command output. The command is not printed again from the
final aggregate.

`[DONE]` is mandatory. It is the only successful stream terminator. Valid command
content followed by EOF without `[DONE]` is a truncated stream: visible content
is preserved, the progress indicator is cleaned up, a network/stream error is
reported, and watn exits unsuccessfully. A provider read failure has the same
partial-output behavior. Malformed nonessential stream events do not discard
otherwise valid command text.

After `[DONE]`, watn reports the response model, elapsed time, token usage, cost,
and throughput using all information supplied by the provider, including a
choices-empty usage-only event. A response model from any valid event is
authoritative, including a usage-only event. If a command-output write or flush
fails, watn keeps the visible prefix, cleans up the spinner, reports the existing
I/O error, omits final metadata and execution, and exits with the existing I/O
status.

## Out of Scope

Provider selection, model selection, credential handling, command generation
semantics, confirmation behavior, and the existing final metadata format are not
changing. Watn will continue to use the configured active chat provider; model
discovery remains a separate service. This change does not add asynchronous
execution or change normal commands unrelated to provider responses.

## Open Questions

No unresolved product decisions remain. The observable requirements in this
proposal are ready to be expressed as executable scenarios.
