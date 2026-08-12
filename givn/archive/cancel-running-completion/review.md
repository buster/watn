# Review: cancel-running-completion

## Fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity | PASS — both delta scenarios carry `@givn.added @e2e` in tracked state; no tag removal |
| 1 | Empty/no-op step bodies | PASS — grepped touched step files; no `{}`, bare `pass`/`return`, or `unimplemented!` remains (matches were format strings) |
| 2 | Checked tasks have commits touching production | PASS with note — S1 commit `a55e930` touches `src/error.rs`, `src/main.rs`, `src/provider/openai_compat.rs`; S2 commit `0d1130e` touches tests/spec only because the shared worker/grace production code was implemented in S1 and S2 exercises it (documented in tasks.md); fmt commit `ac61dfb`; verification commit `5aaca57` |
| 3 | design.md components exist | PASS — `Error::Interrupted` (error.rs), `interrupt: Arc<AtomicBool>` + `parse_sse_stream` flag check (openai_compat.rs), `wait_for_stream_result` + worker wiring (main.rs), `HangServer` (cancel_completion_steps.rs), `StreamingServer::start_held_open` + `drip_wait` (incremental_sse_rendering_steps.rs) |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero `unimplemented!` proof (1 step failed) |
| 5 | `@e2e` Then steps assert real interface | PASS — both scenarios drive the real binary in a PTY and assert exit status 130, merged terminal output, absence of error text/metadata |
| 6 | Browser-UI check | N/A — CLI capability |
| 7 | verify.e2e_command binding | PASS — `./run-tests.sh --e2e` (givn/commands.yaml) invokes the same Cucumber harness with the `@e2e and not @wip` filter; no competing implementation exists; isolation proven (70 e2e vs 103 non-e2e scenarios) |
| 8 | One `@e2e` per distinct action | PASS — 2 inventory entries, 2 scenarios |
| 9 | Local runnability | PASS — loopback twins (StreamingServer, HangServer) start inside the Cucumber process; no external service |
| 10 | verify.command != verify.e2e_command | PASS — `./run-tests.sh` vs `./run-tests.sh --e2e`; e2e count strictly smaller |
| 11 | Implementation vs design.md | PASS with note — single-scenario commands corrected in design.md before use (`--name` conflicts with `--tags`); held-open twin gained a no-`Content-Length` + drip mode, recorded in design.md and design-review.md before the review |
| 13 | Interaction coverage cross-reference | PASS — see matrix below |
| 14 | Coverage measurement validity | PASS — `measure-coverage.sh` instruments the real `watn` binaries via llvm-cov, merges per-process profraws, and reports known exercised production paths non-zero |

### Interaction coverage matrix

| Inventory entry | Scenario | Driving mechanism (in step file) |
|---|---|---|
| press Ctrl+C while a completion is streaming response content | One Ctrl+C cancels a completion waiting for streamed output | Real `watn` subprocess in a PTY (`start_pty_session`); `\x03` via `pty_write`; dripping held-open SSE twin (`configure_held_open_without_done`) |
| press Ctrl+C while the connection is still being established | One Ctrl+C cancels a completion waiting for a connection | Real `watn` subprocess in a PTY; `\x03` via `pty_write`; black-hole listener (`HangServer`) |

## Coverage classification

Runner: `cargo llvm-cov` via `./measure-coverage.sh` (both suites, merged
per-process profraws). New/modified production lines:

| Region | Status |
|---|---|
| `error.rs` Interrupted variant, Display, exit-code arm | Covered (e2e 130-exit paths; Display arm also hit) |
| `openai_compat.rs` parse-loop top-of-loop flag check | Covered (unit test `aborts_when_interrupt_flag_is_set` + S1 drip path) |
| `openai_compat.rs` flag-on-read-error mapping | Covered (unit test `read_error_with_flag_set_maps_to_interrupted`) |
| `openai_compat.rs` `io::ErrorKind::Interrupted` arm | DELETED as dead code (bucket 1): `BufReader::read_line` retries `Interrupted` internally forever, so the arm is unreachable through the only reader used; the hanging repro proved it |
| `main.rs` worker spawn + `wait_for_stream_result` poll/join/grace/hard-exit | Covered (S1 drip path joins with `Error::Interrupted`; S2 black-hole path hits the 500 ms grace hard-exit) |
| `main.rs` Err-arm `matches!(Interrupted)` → exit 130 | Covered (S1 join path) |
| `main.rs` 387 (`spinner.finish()` in Err arm), 402 (end-of-main flag fallback) | Bucket 3 — defensive edge paths: 387 only runs when the spinner is still present at a join-path error (always taken by first content in the new scenarios); 402 only runs if the flag lands after an Ok result. Pre-existing patterns, not deterministically reachable in the fixtures |
| `error.rs` `From<io::Error>` impl (lines 31–33) | Pre-existing uncovered line, out of this change's scope |

No bucket-2 (missing test coverage) gaps remain.

## Verification runs

- `./run-tests.sh` → 18 features, 103 scenarios, 594 steps passed
- `./run-tests.sh --e2e` → 23 features, 70 scenarios, 485 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `cargo fmt --all -- --check`, `cargo check --all-targets`,
  `cargo clippy --all-targets --all-features`, `cargo test --doc`,
  `cargo build --release`, `git diff --check` — all clean
- `cargo test --lib --features test-support` → 21 passed

## Sign-off

REVIEW: PASS