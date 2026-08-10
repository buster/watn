# Review: release-truth-and-repository-cleanup

## Fabrication Audit

The delta feature has one `@e2e` scenario and no remaining `@wip` tags. The
inventory contains one CLI interaction and the design matrix contains one
matching row. The E2E step uses unique release-binary wording and does not
duplicate the existing global `watn --version` step.

The regular and E2E step files were scanned for empty bodies, `pass`, bare
returns, `todo!()`, and `unimplemented!()`. Zero empty or stub step bodies were
found. The only remaining `unimplemented!()` strings are historical RED
evidence in `tasks.md` and design prose, not executable step bodies.

All promised components exist: the two release step modules, Cucumber world
state, package-derived version declaration, host-aware release inspection,
Arc42 marker, ADR-0016, and active documentation updates. Strictness is proven
by `.fail_on_skipped()` and the targeted non-zero stub evidence in `tasks.md`.

Checked task commits are present and contain real implementation, documentation,
or executable step changes. The release artifact scenario is documentation and
verification work, so its production boundary is the release artifact and
deployment documentation rather than a runtime source change. The version E2E
commit contains `src/main.rs`; the hygiene commit contains the confirmed source
and test-world cleanup. No task is spec-only or stub-only.

The exact configured commands are `./run-tests.sh` and `./run-tests.sh --e2e`.
Both invoke `tests/features_runner.rs` with explicit debug binary paths. No
second release-truth E2E implementation exists in the tree. The capability is
CLI-only; its primary E2E assertions inspect real subprocess stdout and exit
status. No browser or HTTP shortcut is applicable.

### Interaction Cross-Reference

| Inventory entry | Matrix row | Feature scenario | E2E driver and primary assertion | Result |
|---|---|---|---|---|
| run `watn --version` and inspect the reported package version | `design.md` version row | Version flag reports the package version | `release_truth_e2e_steps.rs` builds and invokes `target/release/watn --version`; stdout and exit status are asserted | Clean |

The release artifact and documentation scenarios are regular maintainer checks,
not extra user interactions. Their file/process assertions are secondary to
the single CLI inventory action.

## Arc42 Implementation Conformance

Arc42 is enabled. All twelve durable chapters and the change-level assessment
exist, are substantive, and contain Mermaid-only diagrams. ADR-0016 is indexed
in chapter 09 and its consequences are recorded in chapter 11.

| Chapter/fact | Durable source | Assessment/design/tasks | Implementation evidence | Match |
|---|---|---|---|---|
| Package-derived CLI version | Ch. 01, 08, 10, ADR-0016 | Yes; version scenario/task | `env!("CARGO_PKG_VERSION")`, release E2E output `0.1.2` | Yes |
| Target-dependent dynamic libraries | Ch. 02, 07, 10, ADR-0016 | Yes; host-aware inspection scenario/task | `file` identifies dynamic ELF; `ldd` reports shared libraries | Yes |
| No universal static claim | Ch. 01, 04, 07, 10 | Yes; documentation scenario/task | Active docs state target-dependent requirements | Yes |
| Config-only XDG storage | Ch. 02, 03, 08, 12 | Yes; docs scenario/task | Active docs and glossary name only XDG config path | Yes |
| Current SetupWizard/model-picker names | Ch. 04, 05, 08, 12 | Yes; docs scenario/task | Active docs contain no obsolete names | Yes |
| Ctrl-R reasoning focus | Ch. 06, 08, 12 | Yes; docs scenario/task | Active docs contain Ctrl-R and no plain-r claim | Yes |
| Current stdout/stderr contract | Ch. 01, 03, 06, 08, 11 | Yes; docs scenario/task | Active docs state streaming stdout and diagnostic stderr | Yes |
| Historical archive status | README, Ch. 03, 12, change arc42 | Yes; archive scenario/task | Active index identifies `givn/archive/` as historical | Yes |
| Conservative public cleanup | Ch. 05, 09, 11 | Yes; hygiene task | Provider registry/setup result APIs retained after consumer search | Yes |
| Confirmed local cleanup | Ch. 05, 08, 11 | Yes; hygiene task | `_config` and write-only world fields removed; suite passes | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage

Coverage was measured with `measure-coverage.sh` and merged with
`merge-coverages.sh`. The instrumented reports include the Cucumber runner and
the instrumented child binaries. The merger unions per-file/per-line hits,
rather than adding the same source universe twice.

| Report | Covered / valid lines | Rate | Branch status |
|---|---:|---:|---|
| Non-E2E | 4,075 / 7,428 | 54.85999% | Not claimed on stable Rust |
| E2E | 4,912 / 7,428 | 66.12816% | Not claimed on stable Rust |
| Per-line union | 6,711 / 7,428 | 90.34733% | `branches-valid=0`; branch mode requires nightly |

The Gherkin runner is present in both reports. New version logic is exercised by
the release E2E scenario; release inspection is exercised by the regular
scenario. Remaining uncovered lines are classified as **legitimately hard to
test** (bucket 3): they are existing setup, terminal-error, OS-signal, and
provider transport branches that require injectable filesystem/TTY/network
failures not introduced by this documentation/release-truth change. No new
release behavior or cleanup branch is unmeasured. No dead code was left by this
change after the confirmed cleanup search.

## Verification

- `givn lint --change release-truth-and-repository-cleanup`: clean.
- `./run-tests.sh`: 15 features, 65 scenarios, 364 steps passed.
- `./run-tests.sh --e2e`: 18 features, 58 scenarios, 390 steps passed.
- Explicit-binary `cargo test --all-targets --features test-support`: 19
  features, 123 scenarios, 754 steps passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test --doc`: passed, 0 tests.
- `cargo build --release`: passed.
- `file target/release/watn`: dynamically linked x86-64 ELF executable.
- `ldd target/release/watn`: shared libraries and dynamic loader reported.
- `git diff --check`: passed.
- Coverage measurement and merge: passed; runner included.

The local environment is self-contained. The only release interaction is the
built CLI and the only artifact inspection uses local host tools. The E2E count
is strictly below the full count.

REVIEW: PASS
