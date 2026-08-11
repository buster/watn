# Review: highlight-active-setup-input

## Fabrication Audit

### Tag Integrity

The delta contains one feature file with four `@givn.added @e2e` scenarios:

- `The initial URL input has a green border`
- `The green border follows API key focus`
- `The green border follows model focus`
- `The green border follows optional shortcut focus`

All four scenarios retain `@e2e`. No `@wip` tags remain in the delta. No tag
was removed to bypass the E2E gate.

### Step-Body Audit

All files under `tests/steps/` were scanned for empty function bodies,
`unimplemented!()`, `todo!()`, `pass`, and no-op step implementations. Zero
empty or stub bodies were found. The capability-specific file
`tests/steps/highlight_active_setup_input_steps.rs` reconstructs the live PTY
screen, parses ANSI cursor/erase/SGR sequences, and asserts rendered border
styles. Its interaction steps send real terminal bytes through the existing PTY
session.

`tests/features_runner.rs` retains `.fail_on_skipped()`. The setup task records
the required targeted RED run with a non-zero result from an explicit
`unimplemented!()` assertion, so undefined or pending steps cannot pass.

### Task And Commit Audit

Every checked scenario task has a commit touching production code. The hashes
were corrected after the pull rebase:

| Scenario | Commit | Production evidence |
|---|---|---|
| The initial URL input has a green border | `46e5a62` | `src/setup.rs` focused URL block and PTY assertion |
| The green border follows API key focus | `3ccb4e3` | `src/setup.rs` credential focus blocks and PTY assertions |
| The green border follows model focus | `86ac7e4` | `src/setup.rs` model/reasoning focus blocks and PTY assertions |
| The green border follows optional shortcut focus | `b2b9a97` | `src/setup.rs` shared shell-install focus blocks and PTY assertions |

The rebase compatibility follow-up `74f2c5c` updates the PTY step for the
current seven-page wizard and preserves the inactive shell list required by the
scenario. It touches both `src/setup.rs` and the capability step file. No task
is completed by a spec-only or stub-only commit.

### Components And Design Conformance

All components named by `design.md` exist and are registered:

- `src/setup.rs` contains the `setup_block` helper and all affected draw paths.
- `tests/steps/highlight_active_setup_input_steps.rs` is the capability-specific
  PTY step file.
- `tests/steps/mod.rs` registers the capability-specific step module.
- `tests/features_runner.rs` provides strict Cucumber execution and PTY-world
  lifecycle management.

The implementation uses the existing Ratatui/Crossterm stack, the configured
filtered commands in `givn/commands.yaml`, and `portable-pty` with an in-process
`httpmock` catalog twin. The current upstream wizard represents the reviewed
shortcut focus concept with the shared `ShellInstallFocus` state and
`draw_shell_install` renderer; this is a reuse of the existing page renderer,
not a framework, driver, file-layout, or command deviation.

### Real-Interface E2E Audit

All four scenarios start the compiled `watn setup` process in a real
`portable-pty` session, send user-equivalent keystrokes, and inspect the
terminal's rendered ANSI stream. The primary assertions are terminal output
assertions on green and default border glyph styles, not repository or config
state. The model catalog is supplied by the loopback `httpmock` twin; no live
provider is used.

No browser driver is applicable because this is a CLI terminal capability. No
HTTP request or in-page `fetch()` substitutes for the terminal interaction. No
second implementation of these green-border steps exists elsewhere in the
tree; the only matching capability step definitions are in
`tests/steps/highlight_active_setup_input_steps.rs`.

### Command Isolation And Local Stack

The configured commands are distinct and use different Cucumber tag filters:

- Regular: `./run-tests.sh`, selecting `not @wip and not @e2e`.
- E2E: `./run-tests.sh --e2e`, selecting `@e2e and not @wip`.

The regular run passed 95 scenarios and 553 steps. The E2E run passed 65
scenarios and 454 steps. The E2E count is strictly smaller than the regular
count. The E2E command builds the binaries, starts the PTY child, starts the
scenario-local loopback `httpmock` catalog twin when requested, and shuts both
down through the test harness. No Docker stack, external service, live
provider, or shared network is required.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix row | E2E scenario | Driving mechanism and primary assertion | Result |
|---|---|---|---|---|
| launch `watn setup` and inspect the active URL input | matching `design.md` row | The initial URL input has a green border | `highlight_active_setup_input_steps.rs` starts the real PTY process and asserts the rendered `URL (editing)` border SGR | Clean |
| advance to the API key page and move between its credential locations | matching `design.md` row | The green border follows API key focus | The step sends endpoint and credential keys through the PTY and compares live storage/value border styles in both focus states | Clean |
| advance to a model page and switch between model and reasoning input | matching `design.md` row | The green border follows model focus | The step sends catalog selection and Ctrl-R through the PTY and compares live model/reasoning border styles in both focus states | Clean |
| complete model setup and move between the optional shortcut question and shell list | matching `design.md` row | The green border follows optional shortcut focus | The step confirms the model, skips the intervening completion page through the PTY, sends `y`, and compares live shortcut/list border styles in both focus states | Clean |

The feature inventory has four entries, the design matrix has four rows, and each
row maps to exactly one matching `@e2e` scenario. There are no excess E2E
scenarios for input variants or enum values.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. The change-level assessment identifies chapters 1,
3, 4, 5, 6, 8, 10, 11, and 12 as affected. The durable chapters exist and
contain substantive content. The change adds no deployment artifact or new
architectural constraint, so chapters 2, 7, and 9 remain unaffected.

| Architecture fact | Durable source | `arc42.md` claim | `design.md` and `tasks.md` | Implementation evidence | Match |
|---|---|---|---|---|---|
| Focused setup input is visibly distinguished by a green border | Chapters 01, 03, 04, 05, 06, 08, 10, 12 | Visible focused-widget styling is added to the setup flow | Conditional border styling derived from existing focus state; four PTY scenarios | `setup_block` applies `Color::Green` only for the active widget; all four scenarios inspect rendered SGR | Yes |
| Inactive widget styling remains unchanged | Chapters 06, 08, 10, 11 | Inactive styling and layout are preserved | Symmetric inactive-border assertions are required for credential, model, and shortcut transitions | Baseline signatures are captured before each transition and compared after focus moves | Yes |
| Terminal output is the test boundary | Chapters 06, 08, 10, 11 | PTY and ANSI behavior are documented | `portable-pty`, cumulative ANSI reconstruction, semantic SGR parsing, and NO_COLOR removal | Real `watn setup` child process and loopback catalog twin pass the E2E suite | Yes |
| No new dependency, persisted field, or input event is introduced | Chapters 02, 04, 05 | Existing Rust/Ratatui/Crossterm architecture is retained | Existing focus state and a small rendering helper are specified | `Cargo.toml` is unchanged; `src/setup.rs` reuses existing focus state and widgets | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage Measurement

The configured `measure-coverage.sh` instruments both the library tests and the
`features_runner` binary. It also builds instrumented default-feature and
`test-support` `watn` children used by the PTY steps. Each process writes to the
collision-safe `LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw` pattern. The
script clears profiles before each measurement, and
`merge-coverages.sh` combines the fresh non-E2E and E2E Cobertura reports.

The merged report contains non-zero production and runner coverage, including
the active `src/setup.rs` rendering paths:

| Report | Lines covered | Line rate | Branch status |
|---|---:|---:|---|
| Non-E2E Cobertura | 5497 / 9630 | 57.0820% | Not claimed: 0 / 0 reported |
| E2E Cobertura | 5887 / 9630 | 61.1319% | Not claimed: 0 / 0 reported |
| Merged Cobertura | 8688 / 9630 | 90.2181% | Not claimed: 0 / 0 reported |
| Merged `src/setup.rs` | 807 / 994 | 81.1871% | Not claimed |
| Merged `tests/features_runner.rs` | 72 / 81 | 88.8889% | Not claimed |

The raw merged report is `coverage/cobertura-coverage.xml`; its root line rate
is `0.9021806853582555`, and its `src.setup.rs` class line rate is
`0.8118712273641852`. The active URL, credential, model/reasoning, and
shortcut rendering paths all have non-zero execution through the instrumented
PTY child.

### Coverage Classification

- **Dead code:** None identified in the changed implementation or its
  capability steps. No dead helper or obsolete focus path remains.
- **Missing test coverage:** None for the four requested interactions. Each
  changed focus family has a real-interface scenario with active and inactive
  border assertions. The rebase compatibility path through the shell completion
  page is exercised by the optional-shortcut scenario.
- **Legitimately hard to test:** Remaining uncovered regions in the broad
  `src/setup.rs` report are terminal draw/event-read failures, process and PTY
  handoff failures, platform-specific terminal capability behavior, and exact
  scheduler timing around background-search cancellation. These require
  fault-injecting or replacing the real terminal/process boundary rather than
  exercising a user-observable focus interaction. The normal navigation,
  validation, catalog failure, cancellation, search, and active-border paths
  are covered by the regular and E2E suites.

No coverage gap was classified outside the three required buckets.

## Verification

- `givn lint --change highlight-active-setup-input`: clean.
- `cargo fmt --all -- --check`: passed.
- `cargo check --locked`: passed before the final targeted compile; the final
  targeted scenario and full E2E run also compiled the changed production and
  step code successfully.
- Targeted E2E scenario: 1 scenario and 15 steps passed.
- `./run-tests.sh`: 17 features, 95 scenarios, 553 steps passed.
- `./run-tests.sh --e2e`: 21 features, 65 scenarios, 454 steps passed.
- `./measure-coverage.sh`: passed and generated both fresh reports.
- `./merge-coverages.sh`: passed and generated the merged report.
- `git diff --check`: passed.

## Sign-Off

- [x] Fabrication audit clean.
- [x] E2E tags retained and scope is exact.
- [x] No empty or stub step bodies remain.
- [x] Every checked task has evidence and a commit touching production code.
- [x] Promised components exist and match `design.md`.
- [x] Strict-mode proof is present.
- [x] Regular and E2E verification pass.
- [x] Coverage includes the runner and instrumented child processes.
- [x] Coverage gaps are classified under the three permitted buckets.
- [x] Arc42 conformance is clean.
- [x] Interaction inventory and matrix cross-reference cleanly.
- [x] No `@wip` tags remain.

REVIEW: PASS
