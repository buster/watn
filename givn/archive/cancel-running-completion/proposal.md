# Proposal: cancel-running-completion

## Problem / Opportunity

Pressing Ctrl+C while waiting for a completion does not stop the request. The
user typed Ctrl+C, the terminal swallows it, and the process keeps streaming
until the model finishes or the connection times out. The wait is unskippable:
the user cannot interrupt a long or stuck response.

## Proposed Solution

When a completion is in flight, the first Ctrl+C stops the request as soon as
possible:

- Any content already streamed to the terminal stays.
- The process exits with the conventional interrupted status (130).
- No error text is printed; stopping on user request is not a failure.
- If the connection is slow to respond or the stream stalls, the process
  terminates within a bounded grace period (under a second). A single Ctrl+C
  is sufficient in every phase; no second press is required.

## Out of Scope

- Cancelling inside the setup wizard (already handled via its own Ctrl+C path).
- Cancelling the `-x` execute confirmation prompt (already handled).
- Resuming or restarting an interrupted request.
- Server-side cancellation of the model request (provider-unsupported).

## Open Questions

- None.
