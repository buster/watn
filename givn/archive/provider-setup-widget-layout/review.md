# Review: provider-setup-widget-layout

## Fabrication Audit

### Tag Integrity

The active delta feature contains exactly two scenarios, both still tagged
`@givn.added @e2e`, with no `@wip` tags. The two scenario commits are:

- `a7593b3` — provider setup scenario; production diff includes
  `src/provider/setup.rs`.
- `49c7856` — model picker scenario; production diff includes
  `src/models/dialog.rs`.

No `@e2e` tag was removed. The feature remains in the runner's active feature
tree and is selected by `cargo test --test features_runner -- --tags '@e2e and
not @wip'`.

### Step-Body Audit

All seven Cucumber step modules under `tests/steps/` were scanned for
`unimplemented!()`, `todo!()`, empty bodies, bare returns, and no-op bodies. The
scan found zero empty or stub step bodies. The new layout steps perform real
PTY startup, input, terminal-output assertions, and cleanup. Existing steps
continue to use their HTTP mocks, subprocess helpers, or domain assertions.

### Components And Design Conformance

Every component named by `design.md` exists and is used:

- `src/provider/setup.rs` — bordered provider setup with `List`, `Table`, and
  `Paragraph` widgets.
- `src/models/dialog.rs` — bordered model picker with `Tabs`, `Table`,
  `TableState`, `Scrollbar`, `ScrollbarState`, and paragraphs.
- `tests/steps/provider_setup_layout_steps.rs` — provider layout assertions;
  it reuses the existing provider-start step and does not duplicate it.
- `tests/steps/model_picker_layout_steps.rs` — model layout startup,
  presentation, navigation, and cleanup assertions.
- `tests/steps/mod.rs` — registers both capability-specific modules and owns
  deterministic PTY reader/process cleanup.

The implementation matches the reviewed Ratatui/Crossterm stack, the two
capability-specific step files, and the exact filtered commands in
`givn/commands.yaml`.

### Strict-Mode Proof

The setup evidence in `tasks.md` records:

```text
cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'
1 scenario failed; the matched step panicked on unimplemented!(); 1 step failed.
```

The runner uses `.fail_on_skipped()` in `tests/features_runner.rs`.

### Real-Interface E2E Audit

Both scenarios drive the actual compiled CLI through `portable-pty`. Their
primary assertions inspect the live PTY stream:

- Provider setup asserts the rendered border, source list, detail labels,
  guidance, validation message, and masked credential.
- Model picker asserts the rendered border, tier tabs, table headings,
  scrollbar symbol, active tier after keyboard input, and selected row.

No E2E Then step relies only on config files or repository state. There is no
browser capability and no HTTP/fetch shortcut replacing a real UI driver.

### Command Isolation And Local Stack

The configured commands are distinct:

- Regular: `cargo test --test features_runner -- --tags 'not @wip and not
  @e2e'` — 43 scenarios passed.
- E2E: `cargo test --test features_runner -- --tags '@e2e and not @wip'` — 36
  scenarios passed.

The E2E filter is a strict subset of the regular command's selected scenarios.
The local binary starts cleanly with `cargo run -- --version`; provider API
digital twins are loopback `httpmock::MockServer` instances created inside the
runner, so no external service or shared network is required.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix row | Delta scenario | Step implementation | Driver verified |
|---|---|---|---|---|
| Start `watn provider` in a terminal and inspect the provider setup layout | Provider setup separates choices, details, and guidance | Present, `@e2e` | `tests/steps/provider_setup_steps.rs` starts the existing session; `tests/steps/provider_setup_layout_steps.rs` asserts the live frame and cleanup | `portable-pty` subprocess |
| Start `watn models` in a terminal and inspect the model picker layout | Model picker makes tiers and long model lists easy to scan | Present, `@e2e` | `tests/steps/model_picker_layout_steps.rs` starts the session, sends Down/Enter, asserts live labels, and cleans up | `portable-pty` subprocess with loopback `httpmock` catalog |

The inventory has two entries, the design matrix has exactly two rows, and each
row maps to exactly one matching delta scenario.

## Coverage Measurement

The configured coverage commands instrument both the Gherkin runner and the
compiled `watn` binary used by PTY scenarios. The per-process profile pattern is
`coverage/profraw/%p-%m.profraw`, so concurrent processes do not overwrite one
another. The runner and child binary both execute and flush before report
generation.

Measured outputs:

- Non-E2E Cobertura: `872 / 1877` lines, `line-rate="0.4645711241342568"`.
- E2E Cobertura: `1369 / 1877` lines, `line-rate="0.7293553542887586"`.
- E2E source confirmation: `src/provider/setup.rs` line rate
  `0.8169014084507042`; `src/models/dialog.rs` line rate
  `0.8527315914489311`.
- Branch data: the configured Cobertura export reports `branches-valid="0"`
  and `branch-rate="0"`; no branch denominator was emitted by this Rust
  coverage configuration, so branch percentage is recorded as 0/0 rather than
  treated as unmeasured evidence.

### Coverage Classification

- **Dead code:** None identified in the changed production paths. The
  `Confirmed` setup state remains the state-machine sentinel used by the loop;
  its defensive render arm is not a separate production path.
- **Missing test coverage:** None for the requested observable behavior. The
  new layout, validation, masking, navigation, metadata-table path, overflow
  scrollbar, filter, empty-result, unsupported-search, and newest-result
  behaviors are covered by the delta PTY scenarios plus the permanent Gherkin
  model/provider scenarios executed by the same runner.
- **Legitimately hard to test:** Low-level terminal draw failure, child PTY
  descriptor failure, and process interruption during the exact frame handoff
  remain uncovered. Forcing those branches requires corrupting the terminal or
  killing the harness at a precise I/O boundary, which would not validate a
  user-observable setup behavior; normal Escape/Ctrl-C and catalog failure
  paths are covered through the existing scenarios.

## Verification

```text
givn lint --change provider-setup-widget-layout
clean

cargo test --test features_runner -- --tags 'not @wip and not @e2e'
8 features, 43 scenarios passed, 234 steps passed

cargo test --test features_runner -- --tags '@e2e and not @wip'
9 features, 36 scenarios passed, 208 steps passed

cargo test --lib
13 tests passed
```

All 15 task checkboxes are complete, both scenario commit hashes are recorded,
and no WIP scenarios remain.

REVIEW: PASS
