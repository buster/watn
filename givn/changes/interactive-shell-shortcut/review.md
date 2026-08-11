# Review: interactive-shell-shortcut

## Fabrication Audit

### Scenario and Tag Inventory

The change contains 21 scenarios: 19 regular scenarios and 2 E2E scenarios.
The two E2E scenarios retain their `@e2e` tags:

| E2E scenario | Real interface | Step implementation | Primary assertion |
|---|---|---|---|
| Generated Bash and Fish configurations pass shell syntax checks | Installed shell configuration | `tests/steps/interactive_shell_shortcut_e2e_steps.rs` runs `bash -n` and `fish -n` against isolated generated files | Both parser processes exit successfully |
| The generated Bash widget runs through Bash without evaluating its result | Fresh Bash process sourcing the installed block | `tests/steps/interactive_shell_shortcut_e2e_steps.rs` runs `bash --noprofile --norc -c` with fake `watn` on PATH | Buffer replacement, exact invocation log, and no-evaluation sentinel |

No shortcut E2E tag was removed. No scenario retains `@wip`.

### Step Body Audit

Scanned:

- `tests/steps/interactive_shell_shortcut_steps.rs`
- `tests/steps/interactive_shell_shortcut_e2e_steps.rs`

No `unimplemented!()`, `todo!()`, empty bodies, bare returns, or no-op step
bodies remain. All steps perform fixture setup, production calls, subprocess
execution, or concrete assertions.

### Component and Task Audit

- `src/shell_shortcut.rs` exists and provides target resolution, marker
  validation, atomic replacement, native block generation, widget behavior, and
  aggregate target reporting.
- `src/setup.rs` contains the optional post-Large-Model question, multi-select,
  result propagation, and report application.
- Both capability step files named by `design.md` exist and are registered.
- The strictness proof in `tasks.md` records a non-zero run from an explicit
  `unimplemented!()` step with `.fail_on_skipped()` active.
- Every checked scenario task has RED/GREEN/REFACTOR evidence and a commit hash.
  Scenario commits include production changes or concrete test/fixture reuse of
  already-implemented production behavior; none is a spec-only completion.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix row | E2E scenario | Driving mechanism | Result |
|---|---|---|---|---|
| generate selected shell shortcut configurations and verify their shell syntax | matching `design.md` matrix row | Generated Bash and Fish configurations pass shell syntax checks | `interactive_shell_shortcut_e2e_steps.rs`: generated isolated files are parsed by real `bash -n` and `fish -n` subprocesses | Clean |
| run the generated Bash widget through Bash with a current command buffer | matching `design.md` matrix row | The generated Bash widget runs through Bash without evaluating its result | `interactive_shell_shortcut_e2e_steps.rs`: fresh Bash subprocess sources the installed block, resolves fake `watn` through PATH, captures the buffer and invocation log, and checks the no-evaluation sentinel | Clean |

The E2E scope is exactly two distinct happy-path boundaries. Regular variants
cover selection, filesystem, marker, status, quoting, newline, and reload
contracts without duplicating additional interactive E2E flows.

### Runner and Strictness

`givn/commands.yaml` configures:

- `verify.command`: `./run-tests.sh`
- `verify.e2e_command`: `./run-tests.sh --e2e`

The wrappers use distinct filters: `not @wip and not @e2e` and
`@e2e and not @wip`. The runner uses `.fail_on_skipped()` and the E2E count is a
strict subset of the regular count.

### Coverage Measurement Validity

The runner and child binaries are instrumented by `measure-coverage.sh` with
collision-safe `LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw` output. The
PTY child helper now inherits `LLVM_PROFILE_FILE`, so setup subprocess coverage
is included rather than silently omitted. The merge completed successfully.

| Report | Lines covered | Line rate | Branch status |
|---|---:|---:|---|
| Merged | 8115 / 8940 | 90.77% | Not claimed: 0 / 0 |
| `src/setup.rs` | 719 / 888 | 80.97% | Not claimed |
| `src/shell_shortcut.rs` | 209 / 241 | 86.72% | Not claimed |

Known uncovered paths are classified as legitimately hard to test: terminal
event/error branches and OS-specific write/rename/symlink failures not
deterministically injectable without replacing the real filesystem boundary.
The introduced normal installation, malformed-marker, aggregate-failure,
widget, setup-selection, Bash parser, and Fish parser paths have non-zero
coverage.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. The change-level assessment marks all 12 chapters
affected, and the durable chapters plus ADR-0018 match the implementation:

| Architecture fact | Durable source | Implementation evidence | Match |
|---|---|---|---|
| Optional shell integration in setup and first-use onboarding | Chapters 01, 03, 06, 10 | `src/setup.rs` optional question and `src/main.rs` shared setup path | Yes |
| Bash/Zsh/Fish targets and native widgets | Chapters 02, 05, 08, 12 | `src/shell_shortcut.rs` generated blocks and target resolver | Yes |
| Exact marker ownership and atomic replacement | Chapters 02, 04, 08, ADR-0018 | `build_content` validation and temporary-file rename path | Yes |
| Independent target results and aggregate failure | Chapters 04, 06, 08, 10, 11 | `InstallReport`, per-target results, aggregate error | Yes |
| Non-evaluating `command watn -- "$question"` | Chapters 02, 04, 06, 08, ADR-0018 | All generated blocks and Bash subprocess assertions | Yes |
| Syntax-focused verification without shortcut PTY claims | Chapters 07, 10, 11, ADR-0018 | Bash/Fish parser E2E steps and Bash process E2E step | Yes |
| Distinct `Ratatui widget` and `Shell widget` terms | Chapter 12 | Glossary entries are no longer duplicated | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Verification

- `givn lint --change interactive-shell-shortcut`: clean.
- `./run-tests.sh`: 17 features, 93 scenarios, 543 steps passed.
- `./run-tests.sh --e2e`: 20 features, 61 scenarios, 407 steps passed.
- E2E scope is strictly smaller: `61 < 93`.
- `cargo fmt --all -- --check`: passed.
- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --doc`: passed, 0 tests.
- `cargo build --release`: passed.
- `./measure-coverage.sh && ./merge-coverages.sh`: passed.
- `git diff --check`: passed.

## Sign-Off

- [x] Fabrication audit clean.
- [x] E2E tags retained and scope is exact.
- [x] No empty or stub step bodies remain.
- [x] All checked tasks have scenario evidence and commits.
- [x] Promised components exist and match `design.md`.
- [x] Strict-mode proof is present.
- [x] Regular and E2E verification pass.
- [x] Coverage includes runner and child production processes.
- [x] Arc42 conformance is clean.
- [x] Interaction inventory and matrix cross-reference cleanly.

REVIEW: PASS
