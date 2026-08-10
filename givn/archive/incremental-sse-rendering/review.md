# Review: incremental-sse-rendering

## Fabrication Audit

The active delta feature contains five `@e2e` scenarios and no `@wip` tags.
The five tags were retained from the reviewed specification; no E2E tag was
removed. The two capability step files were scanned for empty bodies,
`unimplemented!()`, `todo!()`, bare `pass`, and no-op returns. Zero empty or
stub step bodies were found. The `return` statements in the E2E file are loop
completion returns with preceding output assertions.

Strictness is proven by `tests/features_runner.rs:170-173`, which calls
`.fail_on_skipped()`. The setup evidence in `tasks.md` records a targeted
`unimplemented!()` match and non-zero runner result. All promised production
components exist: provider stream contract, OpenAI-compatible parser, CLI
renderer, spinner lifecycle, Cucumber world, and separate non-E2E/E2E step
modules.

The implementation commits include the initial production stream contract and
CLI/parser implementation (`fdd65a2`), output error boundary (`3371a0f`), and
review remediation (`9969ec6`, `9d95c84`). Later scenario commits add real
loopback/PTY/subprocess step implementations and are not spec-only or stub-only
commits; they reuse the production contract established by those implementation
commits. The setup and E2E setup evidence is included in the initial change
implementation commit. No checked task has an absent commit or a spec-only
change.

The exact configured verification files are `run-tests.sh` and
`run-tests.sh --e2e`, which build explicit debug binaries and invoke
`tests/features_runner.rs`. No second E2E implementation for this capability was
found in the tree. The capability is CLI-only, so browser-driver checks are not
applicable. Every tagged scenario asserts terminal, stdout, or stderr output as
its primary result.

### Interaction Cross-Reference

| User interaction inventory | Matrix row | Feature scenario | E2E driver and primary assertion | Result |
|---|---|---|---|---|
| Invoke `watn` and observe a generated command while streaming | `design.md` delayed-stream row | Command text appears before a delayed stream completes | Real `portable-pty` process; observes spinner, first token, clear-line cleanup, and exact terminal command | Clean |
| Invoke verbose `watn` and observe separate command/reasoning channels | `design.md` verbose row | Verbose streaming keeps reasoning on stderr and command text on stdout | Real subprocess with live stdout/stderr pipes; observes early stdout, absent early reasoning, and final channels | Clean |
| Observe a provider failure after visible content | `design.md` failure row | A mid-stream failure preserves visible content and exits unsuccessfully | Real `portable-pty` process and TCP twin with zero-linger RST; observes prefix, network status, cleanup, and omitted prompt/metadata | Clean |
| Execute from a raw terminal | `design.md` raw-terminal row | Raw terminal confirmation happens after the complete command arrives | Real `portable-pty` process; sends raw Enter and observes pre/post confirmation output | Clean |
| Execute from piped input | `design.md` piped-confirmation row | Piped confirmation remains available after streamed output | Real subprocess with piped `y`; observes exact generated and execution stdout lines | Clean |

All five inventory entries have exactly one matrix row, one matching tagged
scenario, and one real CLI driver. Raw-terminal and piped confirmation are
distinct user interactions because their input boundaries and confirmation
implementations differ.

## Arc42 Implementation Conformance

Arc42 is enabled. All twelve durable chapters exist, contain substantive content,
and use Mermaid-only diagrams. The change assessment marks all twelve affected,
which matches the independent implementation assessment. ADR-0015 covers the
callback/no-channel, strict completion, buffered reasoning, and partial-output
decisions; chapter 11 records their consequences.

| Chapter/fact | Durable source | Change assessment | Design/tasks | Implementation evidence | Match |
|---|---|---|---|---|---|
| Progressive command output | Ch. 01, 04, 05 | Affected | Callback sink and flush rules | `StreamRenderer`, provider callback, delayed PTY scenario | Yes |
| Blocking SSE reader | Ch. 02, 04, 06 | Affected | `BufRead`, no worker channel | `parse_sse_stream`, synchronous provider trait | Yes |
| Strict `[DONE]` boundary | Ch. 02, 06, 08, 10, 12 | Affected | EOF is network failure | DONE-held and EOF scenarios, parser tests | Yes |
| Buffered verbose reasoning | Ch. 01, 04, 06, 08, 12 | Affected | Reasoning aggregate only after success | Verbose E2E channel assertions and alias parser test | Yes |
| Response-model/usage authority | Ch. 05, 08, 10 | Affected | Top-level usage/model extraction | Usage-only Gherkin scenario and replacement unit test | Yes |
| Partial output and RST recovery | Ch. 06, 07, 10, 11 | Affected | Preserve prefix, status 3, clean spinner | Zero-linger TCP twin and mid-stream PTY scenario | Yes |
| Output failure boundary | Ch. 06, 08, 10, 11 | Affected | Preserve content, propagate I/O, remain incomplete | `StreamRenderer`, write/flush tests, controlled-output scenario | Yes |
| Verification topology | Ch. 07, 10, 11 | Affected | Explicit binaries, loopback twin, coverage wrapper | `run-tests.sh`, coverage scripts, feature runner | Yes |
| Glossary/decisions/risks | Ch. 09, 11, 12 | Affected | ADR and documented consequences | ADR-0015 and updated chapters | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage

Coverage was measured with `measure-coverage.sh`, which instruments the library,
the `watn` debug binaries, and the Gherkin runner. The report disables
cargo-llvm-cov's default workspace-test exclusion and includes
`tests/features_runner.rs`. The per-line merger takes the maximum hit count for
each `(filename,line)` across the non-E2E and E2E reports, avoiding duplicate
scalar totals.

| Report | Covered / valid lines | Rate | Branch status |
|---|---:|---:|---|
| Non-E2E | 3,870 / 7,187 | 53.8472% | Not claimed: stable cargo-llvm-cov does not support branch mode |
| E2E | 4,850 / 7,187 | 67.4830% | Not claimed: stable cargo-llvm-cov does not support branch mode |
| Per-line union | 6,487 / 7,187 | 90.2602% | Not claimed: `branches-valid=0` is explicit configuration, not a zero-coverage claim |

The Gherkin runner is present in both reports. The merged report contains
`tests/features_runner.rs` at 72/81 covered lines. Changed production classes in
the merged report are:

| File | Covered / valid lines | Rate |
|---|---:|---:|
| `src/main.rs` | 224 / 278 | 80.5755% |
| `src/provider/openai_compat.rs` | 135 / 152 | 88.8158% |
| `src/output/render.rs` | 61 / 67 | 91.0448% |
| `src/output/spinner.rs` | 69 / 77 | 89.6104% |

No changed streaming behavior is untested. Remaining uncovered lines are
classified as **legitimately hard to test** (bucket 3):

- `main.rs` setup-command and OS signal/error branches require process-level
  configuration failures or signal races outside the stream contract.
- `openai_compat.rs` connection-establishment/status mapping branches require
  injecting failures into reqwest's blocking client; the loopback twin covers
  successful framing, malformed data, EOF, RST, usage, alias, and callback
  failures.
- `render.rs` stderr failure and empty-stream formatting branches require an
  unwritable process stderr or a provider that returns no command; the generic
  renderer seam covers stdout write/flush failure and the CLI E2E scenarios cover
  successful rendering.
- `spinner.rs` terminal color and worker timing branches depend on terminal
  capabilities and are covered through PTY cleanup evidence; deterministic
  injection would replace the real terminal contract.
- Existing setup/model branches outside the incremental streaming components are
  exercised by the permanent feature suite where their interface is available;
  the residual OS/filesystem failure branches have no production injection seam
  and are not introduced by this change.

No dead code was introduced. The unused stream wrapper functions discovered
during review were removed in `9d95c84`.

## Verification

- `givn lint --change incremental-sse-rendering`: clean.
- `./run-tests.sh`: 14 features, 62 scenarios, 344 steps passed.
- `./run-tests.sh --e2e`: 17 features, 57 scenarios, 385 steps passed.
- Explicit-binary `cargo test --all-targets --features test-support`: 18
  features, 119 scenarios, 729 steps passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --doc`: passed, 0 tests.
- `cargo build --release`: passed.
- `git diff --check`: passed.
- Coverage measurement and per-line merge: passed; reports include the runner.

The local environment is self-contained. The only external provider dependency
is replaced by the loopback streaming twin inside the Cucumber process. The E2E
command is not identical to the non-E2E command and its scenario count is
strictly smaller.

REVIEW: PASS
