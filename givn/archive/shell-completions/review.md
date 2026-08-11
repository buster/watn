# Review: shell-completions

## Fabrication Audit

The delta feature contains ten completion scenarios: five regular native-shell
variants, one Bash E2E scenario, the unsupported-value error, the no-config
side-effect contract, help, and the reserved-token compatibility contract. The
single E2E tag remains on `Built Bash completion generation emits the current
command tree`; no `@e2e` tag was removed. No `@wip` tags remain.

The two capability step files, `tests/steps/shell_completions_steps.rs` and
`tests/steps/shell_completions_e2e_steps.rs`, were scanned for
`unimplemented!()`, `todo!()`, empty bodies, bare returns, and no-op bodies.
Zero empty or stub step bodies were found. The parser helper's unavailable-shell
branch emits an explicit environment-limitation message and asserts that the
generated output is non-empty; it does not claim syntax acceptance for an
unavailable executable.

The checked scenario commits are present and are not spec-only or stub-only:

| Scenario evidence | Commit | Implementation evidence |
|---|---|---|
| Initial Bash/Zsh/Fish command-tree generation | `341671d` | `src/main.rs`, `Cargo.toml`, `Cargo.lock`, regular step modules |
| Unsupported selector contract | `48ab5ec` | typed `CompletionShell` parser and error handling in `src/main.rs` |
| No-config/provider isolation | `2c6a483` | subprocess fixture and side-effect assertions, reusing the production early-dispatch implementation |
| Help contract | `9b787cf` | authoritative Clap help metadata in `src/main.rs` |
| Native Elvish/PowerShell expansion | `e811e75` | five-shell enum/mapping, feature scenarios, parser probes, durable design/Arc42 updates |
| Reserved-token compatibility | `95381a9` | regular subprocess scenarios for both `--` and quoted forms |
| Built Bash E2E path | `1409080` | real built-binary subprocess assertions and Bash-function assertion |

The promised production components exist: the local `CompletionShell` selector,
the early completion dispatch, the `run_completions` renderer boundary, the
five explicit native mappings, the separate regular/E2E step modules, and the
coverage-preserving subprocess environment seam. Strictness is proven in the
setup task: `.fail_on_skipped()` is retained and an explicit stub produced a
non-zero targeted run.

The configured commands are read from `givn/commands.yaml`: `./run-tests.sh`
and `./run-tests.sh --e2e`; the latter uses the Cucumber `@e2e and not @wip`
filter. No second shell-completions E2E implementation exists in the tree. The capability is CLI-only, so browser
driver checks are not applicable. The E2E assertion is on real CLI subprocess
stdout/stderr and exit status, not repository state.

The local environment needs no application server, database, or external
provider. The regular suite exercises Bash and Fish parser binaries. Zsh,
Elvish, and PowerShell executables are unavailable in this environment; each
parser step reports that limitation explicitly. The generated scripts and all
observable CLI contracts are still exercised. This is a bucket-3,
legitimately-hard-to-test limitation, not a missing or downgraded interface
assertion.

### Interaction Cross-Reference

| User interaction inventory | Matrix row | Feature scenario | Driving mechanism and primary assertion | Result |
|---|---|---|---|---|
| Run `watn completions <shell>` for a supported shell and receive its script | `design.md` completion row | `Built Bash completion generation emits the current command tree` | `shell_completions_e2e_steps.rs` launches `WATN_TEST_SUPPORT_DEBUG_BIN` as a real subprocess; assertions inspect generated Bash stdout, selector values, stderr, determinism, and status | Clean |
| Same interaction, regular native-shell variants | Explicitly described in `design.md` as regular variants of the matrix row | Bash, Elvish, Fish, PowerShell, and Zsh completion scenarios | `shell_completions_steps.rs` launches the explicit built test-support binary; assertions inspect each generated script, root tree, determinism, stdout/stderr, and shell syntax when the executable exists | Clean |

The inventory has one distinct CLI happy-path action, so one E2E scenario is
correct. The other four shell values are enum/rendering variants of that same
action and are covered by regular real-subprocess scenarios rather than
duplicating E2E budget.

## Arc42 Implementation Conformance

Arc42 is enabled. Independent chapter selection matches `arc42.md`: chapters
1-6 and 8-12 are affected; chapter 7 is unaffected because the CLI change adds
no deployment topology, service, or artifact. All twelve durable chapter files
exist and contain substantive content. No ASCII-art diagrams were introduced;
the completion flow remains Mermaid-based where a diagram is needed.

| Chapter/fact | Durable source | `arc42.md` | `design.md`/tasks | Implementation evidence | Match |
|---|---|---|---|---|---|
| User-facing five-shell goal | Ch. 01, README | Affected | Proposal and five-shell scenarios | `completions <SHELL>` with Bash, Elvish, Fish, PowerShell, Zsh | Yes |
| Closed selector/error boundary | Ch. 02, 03, 08, glossary | Affected | Local parser and `nushell` contract | `CompletionShell::parse`, Clap framing, stderr assertions | Yes |
| Authoritative command metadata | Ch. 04, 05, ADR-0017 | Affected | `Cli::command()` only | `run_completions` renders the Clap command tree | Yes |
| Five renderer mappings | Ch. 05, 06 | Affected | Native `clap_complete 4.6.9` variants | Bash/Elvish/Fish/PowerShell/Zsh imports and match arms | Yes |
| Stdout and side-effect boundary | Ch. 06, 08, 10 | Affected | Early dispatch, empty stderr, isolated XDG/sentinel | Regular no-config scenario and subprocess capture | Yes |
| Determinism and parser portability | Ch. 10, 11 | Affected | Repeat generation and explicit unavailable-shell reporting | Five regular scenarios and shell parser step | Yes |
| Reserved token compatibility | Ch. 06, 08, 11 | Affected | `--` and quoted forms | Reserved-token scenario covers both forms | Yes |
| Architecture decision and consequences | Ch. 09, 11 | Affected | ADR-0017 and risk entries | Local selector aligned to the pinned native set | Yes |
| Deployment topology | Ch. 07 | Unaffected | No deployment change | Single CLI remains unchanged | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage

Coverage was measured with `measure-coverage.sh`, which instruments the library,
the explicit `watn` child binaries, and the Gherkin runner. The subprocess
helper preserves `LLVM_PROFILE_FILE` after `env_clear`, so child production
processes write collision-safe profiles. `merge-coverages.sh` performs the
per-line union. The merged report contains non-zero production and runner
coverage, including `src/main.rs` at 247/301 lines (82.06%) and
`tests/features_runner.rs` at 72/81 lines (88.89%).

| Report | Covered / valid lines | Rate | Branch status |
|---|---:|---:|---|
| Non-E2E | 4,367 / 7,809 | 55.9227% | Not claimed: branches-valid is 0 on this toolchain |
| E2E | 5,033 / 7,809 | 64.4513% | Not claimed: branches-valid is 0 on this toolchain |
| Per-line union | 7,058 / 7,809 | 90.3829% | Not claimed: stable cargo-llvm-cov branch mode is unavailable |

The new completion paths, all five selector mappings, unsupported parser, help,
reserved-token paths, and no-config side-effect path have non-zero production
coverage through the instrumented child binary. Remaining uncovered production
regions are pre-existing setup, signal, provider-error, and terminal capability
branches not introduced by this change. They are classified as bucket 3,
legitimately hard to test without replacing the real CLI/terminal boundary or
injecting OS-level failures. No dead code or missing completion scenario remains.

## Verification

- `givn lint --change shell-completions`: clean.
- `./run-tests.sh`: 16 features, 74 scenarios, 443 steps passed.
- `./run-tests.sh --e2e`: 19 features, 59 scenarios, 399 steps passed.
- E2E scope is a strict subset of regular scope: 59 < 74 scenarios.
- `RUST_TEST_THREADS=1` explicit-binary `cargo test --all-targets --features test-support`: 20 features, 133 scenarios, 842 steps passed.
- Serial library tests: 19 passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --doc`: passed, 0 tests.
- `cargo build --release`: passed.
- `git diff --check`: passed.
- Coverage measurement and per-line merge: passed; reports include the runner and instrumented child binaries.

## Sign-Off

- [x] Fabrication audit clean.
- [x] All checked tasks have evidence and scenario commits.
- [x] Promised components exist.
- [x] Strict-mode proof is present.
- [x] Regular and E2E verification commands pass.
- [x] Coverage is measured across the runner and child binaries.
- [x] Coverage gaps are classified under the three permitted buckets.
- [x] No dead code or missing completion scenarios remain.
- [x] No `@wip` tags remain.
- [x] Exactly one E2E scenario covers the distinct CLI happy-path action.
- [x] The local run command is self-contained and starts without external services.
- [x] The E2E command is distinct from the regular command and has a strictly smaller scenario count.
- [x] Implementation matches the reviewed five-shell design and file layout.
- [x] Interaction inventory, matrix, feature scenario, and step definitions cross-reference cleanly.

REVIEW: PASS
