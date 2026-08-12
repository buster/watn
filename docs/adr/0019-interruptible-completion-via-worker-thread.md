# ADR-0019: Interruptible completion via worker thread and bounded grace

- **Status:** accepted
- **Date:** 2026-08-12
- **Decision-makers:** architect

## Context and Problem Statement

A user cannot cancel a running completion with Ctrl+C. The SIGINT handler only
sets a flag; the blocking `reqwest` client and SSE parsing loop never consult it,
so cancellation is delayed until the response finishes or the 120 s timeout
fires. The `reqwest` blocking client (0.12.28, verified in Cargo.lock) exposes a
single `timeout` that governs both the connect/header phase and every response
read, and it has no separate `read_timeout` on the blocking builder — so a
per-read polling loop cannot be layered on the response body timeout.

How should a single Ctrl+C stop an in-flight completion promptly in every phase
(connect, stalled stream, actively streaming) without adding an async runtime?

## Decision Drivers

- Ctrl+C must be acknowledged within a bounded time in all phases
- Keep the blocking HTTP stack, SSE parser, and callback contract as-is
- Avoid a new async runtime or a rewrite of the provider transport
- Preserve already-streamed stdout content and suppress errors on interrupt
- Exit with the conventional interrupted status (130)

## Considered Options

- **Worker thread plus bounded grace** — run the streaming call on a thread; the
  main thread polls completion and the interrupt flag, waits a short grace
  window, then detaches the worker and exits 130
- **Per-read timeout polling** — wrap the response body reader to poll the flag
  on timeout; impossible with reqwest blocking's single timeout governing both
  the connect and read phases
- **Kill the process from the signal handler** — terminate directly; skips
  cleanup, makes exit status and terminal state unreliable, and does not
  preserve streamed content
- **Async client plus `tokio::select!`** — cleanest cancellation, but requires
  threading an async runtime through the whole provider chain

## Decision Outcome

Chosen: **a dedicated worker thread for the streaming call plus a bounded
grace period in the main thread.** Each phase is now cancellable:

- **Stream flowing:** `parse_sse_stream` checks the interrupt flag every loop
  iteration and returns the new `Interrupted` error on the next SSE line.
- **Stream stalled or connection pending:** the worker is unreachable, so the
  main thread polls for completion plus the flag, waits up to 500 ms, detaches
  the worker (`Drop` on the `JoinHandle`) and exits 130.

A single Ctrl+C suffices in all phases; termination is bounded by the grace
period. `Interrupted` is a new `Error` variant with exit code 130. On the join
path it skips the error message but still finishes the spinner and partial
output; on the grace path `main` exits 130 directly without cleanup. The
already-streamed stdout content remains visible.

## Consequences

- Good: Ctrl+C is acknowledged in well under a second in every phase
- Good: no new runtime dependency; provider and parser surface stay blocking/sync
- Good: a clean interrupt (join path) preserves the visible prefix, finishes
  the spinner, and prints no error text
- Bad: in the stalled/connect case the grace path skips spinner/output cleanup
  and the final buffered bytes can be cut off by the hard exit
- Bad: a detached worker thread can outlive the main thread by microseconds
  while the process tears down; per-thread output writes remain safe
- Bad: the 500 ms grace is a fixed heuristic, not an event-driven wakeup

## Confirmation

The `cancel-running-completion` feature scenarios drive the real binary in a PTY
against a held-open streaming twin and a black-hole listener, press Ctrl+C, and
assert exit status 130 with no reported error and no final metadata.