# Design Review: incremental-sse-rendering

## Grilling Results

### Scope

The proposal, feature delta, and design now agree on the following boundary:

- Command content is parsed and flushed incrementally.
- Reasoning is accumulated and printed only after successful completion with
  `--verbose`.
- `[DONE]` is mandatory; EOF without it is a network failure.
- Final model and usage can arrive in a choices-empty event.
- Visible prefixes survive provider and output failures.
- Final metadata and execution are suppressed after failure.
- The existing provider, credential, model-selection, and confirmation
  semantics remain unchanged except that confirmation starts only after a
  successful complete stream.

The controlled output-sink scenario is included because write and flush failure
is an observable CLI boundary required by the failure contract, not a new
provider feature.

### Technology Choices

The blocking HTTP client, buffered reader, synchronous object-safe callback, and
single CLI consumer are the smallest design that exposes first-token output
without adding async execution, a worker thread, or a channel. The callback
returns `Result<(), Error>` so output failures propagate through the existing
typed error path while the provider remains usable through
`Box<dyn Provider>`.

Reasoning is deliberately not sent through the callback. This avoids a race
between verbose stderr output and the spinner and satisfies the user-visible
contract without synchronization machinery.

### Scenario And RED Testability

The scenarios that previously could pass against buffered production were
hardened with release-gated observations:

- Delayed, verbose, partial-read, and malformed-event streams assert content
  before a later event is released.
- The DONE scenario holds the HTTP connection open after `[DONE]` and requires
  watn to exit before the close.
- Usage-only metadata uses a requested model different from the response model
  and prices only the response model.
- Exact-once assertions distinguish the generated command line from execution
  output.
- EOF without `[DONE]`, mid-stream reset, and controlled output failure assert
  status, visible prefix, cleanup, and omission of success actions.
- Raw-terminal confirmation checks the pre-confirmation terminal state before
  sending Enter. Piped confirmation uses a separate subprocess path.

Parser-only framing cases such as an optional space after `data:` and a final
line without a newline are covered by direct parser tests because those are
reader-boundary contracts rather than distinct user interactions.

### E2E Fidelity And Interaction Coverage

The capability is CLI-only. Every retained E2E scenario drives a real built
binary; PTY scenarios exercise raw terminal behavior and subprocess scenarios
exercise captured stdout/stderr or piped stdin. The loopback streaming twin is
the only provider dependency. The five inventory entries map one-to-one to the
five E2E scenarios in `design.md`; each has a concrete driving mechanism.

The non-E2E scenarios use direct provider/parser or controlled-output seams only
where those seams provide more precise evidence for protocol and I/O edge cases.
They do not replace the real-interface assertions in the E2E scenarios.

### Risks And Mitigations

- A provider that closes without `[DONE]` is treated as truncated; visible
  content remains available, but success metadata and execution are blocked.
- A callback can fail after visible output; the spinner is finished, the error
  status is preserved, and later actions are skipped.
- A blocking callback couples provider progress to terminal write speed; this is
  intentional for one consumer and is covered by the controlled writer.
- Buffered verbose reasoning is not visible while a stream is active; the
  feature asserts that it appears only after successful completion.
- PTY output can be terminal-sensitive; the harness uses a known terminal,
  bounded waits, and explicit clear-line evidence.

### Arc42 Review

Arc42 is enabled. The current-change `arc42.md` independently marks chapters 1
through 12 as affected because this change alters runtime behavior, output
contracts, quality requirements, and architecture decisions; chapter 7's
product deployment remains unchanged even though verification topology is
documented. All twelve durable chapter files exist, contain substantive
project-specific content, and use Mermaid for diagrams. No ASCII or Unicode
box-drawing diagrams were introduced.

The durable Arc42 update includes the synchronous callback/no-channel decision,
mandatory `[DONE]`, first-event timing, buffered reasoning, output failure
handling, partial-output risks, and the new focused MADR in chapter 09. The
consequences are recorded in chapter 11.

## Review Outcome

All required questions were resolved through repository inspection or explicit
user decisions. No duplicate step expression is planned; existing ordinary
launch, stderr, and exit-status bindings are reused where appropriate. The
remaining work is scenario-by-scenario implementation.

DESIGN-REVIEW: PASS
