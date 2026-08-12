# Design Review: cancel-running-completion

Grilling subagent findings and resolutions. Grilled in a fresh context;
findings ranked by severity.

## Findings

1. **MAJOR — Testability: scenario 1 cannot exercise the parse-loop check; both
   scenarios would terminate via the grace/detach path.**
   Resolution: A held-open (no `[DONE]`, no drip) twin is intentionally kept:
   the observable contract (content preserved, exit 130, no error) is identical
   through either path, and a fully-spec'd RED run still fails against the
   current hanging code. The parse-loop check is recorded as required for the
   flowing case (design "Scope And Decisions") and for latency, but is not
   e2e-distinguishable from the grace path. Accepted; no fixture change.

2. **MAJOR — Tech: worker ownership as written does not compile (spinner/output
   moved into the worker yet used by main afterwards); grace path cannot finish
   the spinner.**
   Resolution: The worker closure now returns
   `(Result<StreamingResponse, Error>, Option<Spinner>, StreamRenderer<Stdout>)`
   via the `JoinHandle`; `main` performs all cleanup on the join path. The
   grace path exits 130 directly without cleanup, documented in design, arc42
   06/08/11, and ADR-0019. Design updated.

3. **MAJOR — Scope: the proposal's second-Ctrl+C requirement was silently
   dropped.**
   Resolution: Amended proposal.md: a single Ctrl+C terminates within a bounded
   grace in every phase; the second-press sentence was removed because the
   500 ms bound satisfies the original intent. Recorded so the recommendation
   is explicit.

4. **MINOR — Testability: `finish_pty_session` repopping and stderr assertions.**
   Resolution: `I press Ctrl+C` finishes the session before the Then-steps;
   "stderr should not contain" asserts against merged `world.output` (PTY
   merges the streams). Design step-binding updated.

5. **MINOR — Design: the listed non-E2E single-scenario command matches nothing
   (both scenarios are `@e2e`).**
   Resolution: Removed the non-E2E command; single-scenario E2E commands added.

6. **MINOR — Tech: poll mechanics unspecified.**
   Resolution: ~20 ms outer poll, ~10 ms grace poll with continued
   `is_finished()` checks during grace; detach on expiry, exit 130. Recorded.

7. **MINOR — Missing scenario: content-preservation asserted only before
   Ctrl+C.**
   Resolution: Added `And the terminal output contains "printf first"` to
   scenario 1 (reusing the existing step).

8. **MINOR — Arc42: spinner-finished overclaim in 06/08/11/ADR-0019; row 4
   justification thin.**
   Resolution: Wording scoped to the join path in all four files; row 4 note
   hardened in arc42.md.

9. **MINOR — Risk: PTY `\x03` → SIGINT delivery is new; black-hole determinism.**
   Resolution: Recorded smoke-test-first note plus a SIGINT fallback
   (`kill(PID, SIGINT)` via nix) in design.md; black-hole reads request headers
   and holds; 10 s `finish_pty_session` deadline is the safety valve.

10. **MINOR — Interaction coverage: no explicit inventory↔scenario matrix.**
    Resolution: Added an Interaction Coverage Matrix to design.md mapping both
    inventory entries to scenarios with their driving mechanisms.

## Verified (no issue)

- reqwest 0.12.28 blocking has no split read timeouts; premise confirmed.
- `Error::Interrupted` must be added to the exhaustive `exit_code()` match.
- Send bounds and scoped-thread limitation (worker must be `'static`, so the
  registry is moved into the thread) confirmed.
- E2E fidelity: two scenarios, one per inventory entry, real subprocess/PTY.
- Step name namespace: all new bindings unique against existing registrations.
- Arc42 file integrity: all 12 chapters exist with substantive content and no
  ASCII diagrams; re-derived 12-row table matches arc42.md.

## Implementation-time adjustments (recorded, reviewed)

- **Held-open twin needs no `Content-Length`.** The "held open without
  `[DONE]`" fixture originally planned the existing
  `StreamingServer::start_with_initial_delay` + `hold_after`. During GREEN
  the client returned clean EOF after the single event: with
  `Content-Length` the message body is complete at the delimiter even while
  the socket stays open. Added `StreamingServer::start_held_open`
  (no-`Content-Length`, connection-close delimited) in
  `tests/steps/incremental_sse_rendering_steps.rs`. Test-twin detail; no
  production change. `design.md` updated to match.
- **`io::ErrorKind::Interrupted` maps to `Error::Interrupted` even before
  the flag is observable.** The first GREEN attempt returned status 3 when
  the blocked read was interrupted by SIGINT microseconds before the flag
  store was visible. Ramification of the same design, not a new decision.

## Decisions requiring user attention

- **Double press dropped**: A single Ctrl+C now terminates within ~500 ms in
  every phase; no second press. Aligned with the original request; the proposal
  no longer promises immediate second-press termination.

## Sign-off

DESIGN-REVIEW: PASS