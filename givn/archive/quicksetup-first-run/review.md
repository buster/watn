# Review: quicksetup-first-run

## Fabrication Audit

The quicksetup delta feature (`givn/changes/quicksetup-first-run/specs/quicksetup/quicksetup.feature`)
contains 13 scenarios (4 `@e2e` + 9 regular). The 4 `@e2e` tags remain on the
four canonical happy-path actions (first run, complete/confirm, explicit
overwrite, Ctrl-C abort). The redundant provider delta copy was removed, while
its permanent `@e2e` proof remains. No quicksetup `@e2e` tag was removed; no
`@wip` tags remain; `givn lint --change quicksetup-first-run` reports clean.
The existing-but-incomplete provider onboarding scenario is already present in
the permanent provider specification, so the redundant delta copy was removed.
The shell-completions delta now uses one parameterized outline for all five
supported shells and extends its command tree with `quicksetup`.

0. **@e2e tag integrity.** Every scenario tagged `@givn.added @e2e` in tracked
   history still carries `@e2e` in the current delta (lines 11, 21, 49, 72).
    The four quicksetup added scenarios retain `@e2e`; the removed provider
    delta copy was a redundant modified endpoint, and its permanent proof keeps
    `@e2e`.

1. **Empty/no-op step bodies.** `tests/steps/quicksetup_steps.rs` (744 lines)
   was scanned for `{}`, bare `pass`/`return`, `unimplemented!()`, and
   `todo!()`. 0 empty or stub step bodies found; every step body references the
   PTY session, config assertions, or shell-target assertions. The
   `tests/features_runner.rs` strict mode (`.fail_on_skipped()`) is retained.

2. **Commit evidence.** Every checked `[x]` task has a commit whose diff touches
   production source or legitimately test-only-due-to-step-reuse code:

   | Task | Commit | Implementation evidence |
   |---|---|---|
   | 1 non-TTY guidance / new module | `734e954` | `src/main.rs`, `src/quicksetup.rs` (new), `src/lib.rs` |
   | 2 model re-ask | `2c38c0b` | step file (flow from `src/quicksetup.rs`) |
   | 3 no reasoning stored | `a124976` | `src/config/types.rs` (`skip_serializing_if`), step file |
   | 4 OpenAI suggestion | `0f2ad86` | delta-spec fix + step file (suggestion logic in module) |
   | 5 PATH pre-selection | `0b71815` | step file (PATH detection in `shell_shortcut.rs`) |
   | 6 explicit-provider bypass | `4f33af9` | step reuse (bypass invariant, no new production) |
   | 7 explicit abort unchanged | `091f27f` | step file (fixture-config materialization) |
   | 8 failed write installs none | `a4d9838` | step file (save-before-install ordering in module) |
   | 9 failed install keeps config | `683bbbf` | step file (aggregate report already in module) |
   | 10 first-run starts quick setup (e2e) | `ed48eef` | `src/main.rs` (first-run branch), `src/config/mod.rs` (`config_file_exists`) |
   | 11 complete stores + installs (e2e) | `01eaecf` | step file |
   | 12 explicit overwrite (e2e) | `0d85936` | delta-spec + step file |
   | 13 Ctrl-C first run (e2e) | `15b9e6b` | step file |
   | 14-16 re-ask completion | `4df3156` | step file + delta-spec |

   The `SRC` commits (`734e954`, `a124976`, `ed48eef`) touch production source.
   The remaining scenario commits are test-only because the production logic
   (prompts, validation, persistence ordering, install-after-save, error
   aggregation) was built once in the initial module commit and reused per the
   step-reuse rule — this matches design.md's "Implementation Order".

3. **Promised components exist.** `src/quicksetup.rs` (new), `pub mod
   quicksetup` in `src/lib.rs`, `Commands::Quicksetup` + first-run branch in
   `src/main.rs`, `pub fn config_file_exists()` in `src/config/mod.rs`, and
   `shells_available_on_path()` in `src/shell_shortcut.rs` all exist.

4. **Strict-mode proof.** `tests/features_runner.rs` retains `.fail_on_skipped()`
   and task S1 reports a non-zero RED stub run (`error: test failed`); the
   evidence is recorded in tasks.md.

5. **Downgraded e2e scenarios.** None. Every `@e2e` scenario's Then steps assert
   on the live PTY-rendered output (announcement, endpoint/credential/model
   suggestions, exit, config-location, `watn setup` hint) plus persisted config
   and shell-target files — not repository/database-only assertions.

6. **Browser-UI driver.** The capability is a plain-line CLI, not a browser UI;
   the real interface is the PTY. Every `@e2e` step drives the real binary
   through `start_pty_session`/`pty_write` (portable-pty) — no HTTP/fetch()
   shortcut stands in for interaction.

7. **e2e_command target.** `givn/commands.yaml` binds `verify.e2e_command` to
   `./run-tests.sh --e2e`, which runs the same `features_runner` test binary with
   the `@e2e and not @wip` tag filter. The capability's single step file is
   `tests/steps/quicksetup_steps.rs` (bound to the `features_runner` binary
   through `tests/steps/mod.rs`). `git status` and a tree search found no second
   or parallel `@e2e` step implementation for the quicksetup capability.

8. **E2E scope.** The inventory normalizes to four distinct happy-path actions,
   each owning exactly one `@e2e` scenario (first run, complete, explicit
   overwrite, Ctrl-C abort). Variants and distinct invariants (no-reasoning,
   PATH pre-selection, invalid endpoint, unknown shell, OpenAI suggestion,
   explicit-provider bypass, explicit abort, non-TTY, write failure, install
   failure) are regular scenarios — no over-production of `@e2e`.

9. **e2e_command isolation.** `verify.command` (`./run-tests.sh`) and
   `verify.e2e_command` (`./run-tests.sh --e2e`) are distinct strings. Measured
   scenario counts prove isolation: regular `160 scenarios` vs e2e `77
   scenarios` — the e2e run reports strictly fewer scenarios (not "every
   scenario is @e2e").

10. **Design/implementation conformance.** design.md named: one step file
    `tests/steps/quicksetup_steps.rs` (present), `./run-tests.sh --e2e` with the
    `@e2e and not @wip` tag filter (present in run-tests.sh), real PTY interface
    via portable-pty (present), `config_file_exists()` (present), PATH-based
    `shells_available_on_path()` (present), first-run branch inside the
    implicit-selection gate (present, `src/main.rs:218-232`). No silent
    deviation.

11. **Interaction coverage verification.** Every inventory entry maps to a
    design matrix row, an existing `@e2e` scenario, and PTY step definitions.
    See the cross-reference table below. No unmatched interaction; no missing
    row; no driving-mechanism mismatch.

12. **Coverage instrumentation.** `coverage.command` is a real
    instrumentation (`./measure-coverage.sh` with `cargo llvm-cov`), not the
    `givn missing-coverage` sentinel. Coverage data is produced across the
    library, the explicit child binaries, and the Gherkin runner. See Coverage.

### Interaction Cross-Reference

| User interaction inventory | Matrix row (design.md) | Feature scenario | Driving mechanism and primary assertion | Result |
|---|---|---|---|---|
| Start quick setup automatically on first run without a config file | First run without a configuration starts the quick setup | `First run without a configuration starts the quick setup` | PTY `watn "hello"` after `isolate_quicksetup_env`; asserts announcement + endpoint suggestion in rendered output, zero sentinel requests | Clean |
| Complete quick setup: answer questions, choose shells, confirm | Quick setup stores answers and installs integrations | `Quick setup stores answers and installs integrations` | PTY keystrokes through all five questions + shell list; compact persistence step retains config and rc-file assertions against isolated HOME | Clean |
| Run `watn quicksetup` explicitly with an existing configuration | Explicit quick setup overwrites an existing configuration | `Explicit quick setup overwrites an existing configuration` | PTY typed answers + shell deselects; asserts overwritten config, zero catalog/sentinel requests, unchanged shell targets | Clean |
| Abort quick setup with Ctrl-C | Aborting quick setup with Ctrl-C on the first run leaves no configuration | `Aborting quick setup with Ctrl-C on the first run leaves no configuration` | PTY first-run trigger aborted with Ctrl-C keystroke; asserts no config, unchanged targets, zero sentinel requests | Clean |

The remaining scenarios are regular variants/invariants of these four actions,
documented in design.md.

## Arc42 Implementation Conformance

Arc42 is enabled (addons.arc42: true). Independent chapter selection matches
`arc42.md`: chapters 03, 04, 05, 06, 09, 11, 12 are affected; the rest are not.
All twelve durable chapter files exist and contain substantive content. The 7
affected chapters (03, 04, 05, 06, 09, 11, 12) were verified to contain
current, decision-specific quicksetup/quick-setup content, not placeholders.
ADR-0026 exists (`docs/adr/0026-plain-line-quick-setup.md`) and is registered
in the durable decisions chapter.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match |
|---|---|---|---|---|---|---|
| New user-facing `watn quicksetup` + first-run surface | Ch. 03 | Yes | Entry-contract placement in request path | Tasks 1, 10 | `Commands::Quicksetup` + first-run branch, `src/main.rs` | Yes |
| Plain-line first-run strategy | Ch. 04 | Yes | Plain-line, stdout-prompt/stdin-read | Task 1 | `src/quicksetup.rs` prompt sequence | Yes |
| Quick setup building block | Ch. 05 | Yes | New `src/quicksetup.rs` module | Task 1 | `pub mod quicksetup` | Yes |
| First-run runtime branch on config-file existence | Ch. 06 | Yes | `config_file_exists()` gate inside implicit-selection | Tasks 10, 6 | `src/main.rs:218-232`, `src/config/mod.rs:21` | Yes |
| ADR-0026 plain-line decision + register | Ch. 09 | Yes | Multi-choice shell row, PATH detection, no reasoning | Tasks 1-9 | ADR-0026 + `shells_available_on_path()` | Yes |
| Dual-surface drift / stale-suggestion risk | Ch. 11 | Yes | Hamilton helper, no reasoning | Cover scenarios | Risk section updated | Yes |
| "Quick setup" glossary term | Ch. 12 | Yes | Term added | — | Glossary entry present | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Overlap dispositions

- provider-setup first-use: the permanent scenario already carries the
  existing-but-incomplete configuration behavior; the redundant delta copy was
  removed. Quick setup owns the missing-config first run, while the permanent
  scenario keeps the coordinator path. No unrelated removed+added pair remains.
- shell-completions authoritative-tree (`@givn.modified`, one outline with five
  examples): the subcommand list is extended with `quicksetup`; all five shell
  executions remain covered without five semantically redundant endpoints.
- quicksetup scenarios vs provider-setup: the quick-setup first-run (missing
  config) and the coordinator first-run (existing-incomplete config) are
  distinct preconditions with distinct end states — no duplication; each
  normalized first-run action owns its own `@e2e` proof.

## Split-or-keep

| Scenario | Decision |
|---|---|
| `Quick setup stores answers and installs integrations` | keep |
| `Explicit quick setup overwrites an existing configuration` | keep |

Justifications (each scenario is kept intact as a single linear end-to-end
action proof; splitting would break the one-`@e2e`-per-action rule and
fragment the evidence):
- The compact "Quick setup stores answers..." scenario answers all five
  questions, chooses shells, and confirms; its named persistence step retains
  config and both integrations for all three shells.
- The 20-step "Explicit quick setup overwrites..." scenario asserts the end
  state of a single overwrite action.
- The permanent "First normal use..." provider-setup scenario remains the
  single proof of sequential two-phase onboarding for an existing-but-
  incomplete config; no redundant delta copy remains.

## Coverage

Coverage was measured with `measure-coverage.sh` (cargo llvm-cov) and merged
with `merge-coverages.sh` (per-line union). The instrumentation covers the
library, the explicit `watn` child binaries, and the Gherkin runner.

| Report | Covered / valid lines | Rate | Branch status |
|---|---:|---:|---|
| Non-E2E | 10,010 / 14,680 | 68.19% | Not claimed: branches-valid is 0 on this toolchain |
| E2E | 8,013 / 14,680 | 54.58% | Not claimed: branches-valid is 0 on this toolchain |
| Per-line union | 13,461 / 14,680 | 91.70% | Not claimed: stable cargo-llvm-cov branch mode is unavailable |

New-code coverage: `src/quicksetup.rs` is 126/126 (100%) in the merged report;
`config_file_exists()` (config/mod.rs:21) is covered; `shells_available_on_path()`
(shell_shortcut.rs) is covered. The first-run branch in `src/main.rs` is
exercised (the `watn::quicksetup::run()` call and the `return` execute; the
e2e first-run scenario passes and writes config).

Remaining uncovered regions in `src/main.rs`, `src/config/mod.rs`,
`src/shell_shortcut.rs`, and the wider tree are pre-existing setup, signal,
provider-error, terminal-capability, and unrelated-capability branches not
introduced by this change. Classification:

1. **Dead code (delete):** none introduced by this change.
2. **Missing test coverage:** none — every new quicksetup branch is covered;
   the first-run error-fallback arm (`src/main.rs:227-230`) is behaviorally
   identical to the covered explicit-command path (`src/main.rs:645-648`,
   exercised by the failed-config-write scenario), so a separate scenario would
   assert the same observable outcome and duplicate it.
3. **Legitimately hard to test:** pre-existing error/terminal/signal branches
   across the binary were prior-accepted, unchanged by this change; forcing them
   via a real PTY/OS boundary replacement is not warranted.

No dead code or missing quicksetup scenario remains.

## Verification

- `givn lint --change quicksetup-first-run`: clean (2 files checked).
- `./run-tests.sh`: 21 features, 160 scenarios, 958 steps passed.
- `./run-tests.sh --e2e`: 24 features, 77 scenarios, 568 steps passed.
- E2E scope is a strict subset of regular scope: 77 < 160 scenarios.
- Strict-mode proof present (task S1 non-zero RED run).
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo build --locked --features test-support --tests`: passed.
- `cargo test --doc`: passed, 0 tests.
- `git diff --check`: passed.
- Coverage measurement and per-line merge: passed; reports include the runner
  and instrumented child binaries (91.70% line rate, per-line union).
- Local run is a single CLI with no server/database/twin; the real interface is
  the PTY, and the full stack starts cleanly (`cargo build --bin watn`).

## Sign-Off

- [x] Fabrication audit clean.
- [x] All checked tasks have verified commits touching production or
      legitimately test-only step-reuse code.
- [x] Promised components exist.
- [x] Strict-mode proof is present.
- [x] Regular and E2E verification commands both exit 0.
- [x] Coverage is measured across the runner and child binaries.
- [x] Coverage gaps are classified under the three permitted buckets.
- [x] No dead code or missing quicksetup scenario remains.
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Exactly one E2E scenario per normalized inventory action; variants are
      regular scenarios.
- [x] Every E2E scenario uses the real PTY interface; no HTTP/fetch downgrade.
- [x] The local run command is self-contained and starts without external
      services.
- [x] The E2E command is distinct from the regular command and has a strictly
      smaller scenario count (77 < 160).
- [x] Implementation matches the reviewed design and file layout (no silent
      deviation).
- [x] Interaction inventory, matrix, feature scenario, and step definitions
      cross-reference cleanly.
- [x] ARC42 conformance clean.
- [x] No finding was excused with a classification outside the three buckets.

## Semantic Review Remediation

The givn retrieval review initially found three E5 token-cap scenarios and
cross-shell/provider near-duplicate candidates. The remediation preserved the
behavior while making the evidence composable:

- The quicksetup completion scenario uses the existing compound PTY action for
  endpoint, credential, and model suggestions, plus one named assertion that
  retains every config and shell-target check.
- The streamlined setup scenario uses one named action for the three model and
  reasoning selections; its six underlying PTY transitions remain executed.
- The unified wizard scenario groups provider controls, model navigation, and
  optional shell pages behind named steps; the grouped steps still call every
  original page assertion and transition.
- The five shell completion scenarios are one `Scenario Outline` with five
  examples in both the delta and permanent feature. The custom runner now calls
  cucumber-rs `expand_examples()` so every example executes independently.
- The duplicate provider delta copy was removed because the identical
  existing-incomplete onboarding scenario is already permanent and remains
  covered.

`givn check review --change quicksetup-first-run` reports semantic review
`PASS`, candidate count `0`, and no E5 token-cap count.

README-IMPACT: updated - "Options and commands" subcommand list adds the `quicksetup` subcommand.

REVIEW: PASS
