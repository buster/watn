# Review: unified-setup-wizard

## Fabrication Audit

### Tag Integrity

The active delta contains four `@e2e` scenarios across two feature files:

- `Provider setup separates choices, details, and guidance` in
  `provider-setup-widget-layout/provider-setup-widget-layout.feature`.
- `Setup wizard guides provider and model configuration page by page` in `unified-setup-wizard.feature`.
- `Models command opens the shared wizard on Small Model` in `unified-setup-wizard.feature`.
- `Escape asks whether to save or discard current setup` in `unified-setup-wizard.feature`.

All four retain their `@e2e` tags. No active scenario has an `@wip` tag. The
regular and E2E commands select different scenario sets, with 43 and 40
scenarios respectively.

### Step-Body Audit

All nine files under `tests/steps/` were scanned for `unimplemented!()`,
`todo!()`, empty bodies, bare no-op returns, and bodies without a real
assertion or domain action. Zero empty or stub step bodies were found. The
few `return` statements are inside polling loops after assertions or helper
functions that return real values.

The runner uses `.fail_on_skipped()` in `tests/features_runner.rs`. The setup
task records a targeted non-zero run against an `unimplemented!()` step, and
the three new scenarios each have their own targeted RED evidence.

### Task And Commit Audit

Every checked scenario has recorded implementation evidence and a commit that
touches production code:

- `3a4e027` — provider entry-point migration and shared wizard production code.
- `38e8f4e` — full setup routing and completion output in `src/main.rs`.
- `eababb3` — model reasoning-option refactor in `src/setup.rs`.
- `a590670` plus production follow-up `171cd66` — Escape/discard PTY assertion and cancellation cleanup in `src/setup.rs`.
- `4bb9b7f` — removes the obsolete model dialog path and its unused dependency.
- `ed69540` — removes the obsolete provider dialog path.

The follow-up commits resolved the audit issue that the original Escape
assertion commit did not itself contain production code.

### Components And Design Conformance

Every component named by `design.md` exists and is used:

- `src/setup.rs` owns the five-page Ratatui/Crossterm wizard, page ranges,
  search, reasoning focus, cursor markers, and save/discard state.
- `src/main.rs` exposes `watn setup` and routes the existing commands.
- `src/models/list.rs` parses optional per-model reasoning metadata.
- `src/models/dialog.rs` contains only the shared model reasoning and choice
  value types; the obsolete standalone dialog loop was removed.
- `src/models/mod.rs` preserves the shared TTY path and non-TTY index path.
- `src/provider/setup.rs` contains provider types, guidance, and pure endpoint
  and credential validation; the obsolete standalone provider renderer was
  removed.
- `tests/steps/setup_wizard_steps.rs` drives the new capability through PTY.
- `tests/steps/mod.rs` registers the capability and owns PTY cleanup.

The implementation matches the reviewed Ratatui 0.30.2, Crossterm, Cucumber-rs,
portable-pty, and loopback httpmock decisions. The configured commands match
the exact filtered commands in `givn/commands.yaml`.

### Real-Interface E2E Audit

All four scenarios drive the compiled CLI through the existing `portable-pty`
subprocess harness. Primary assertions inspect live terminal output:

- Provider setup checks active URL/API-key pages, cursor, layout content,
  validation, and masked credentials.
- Full setup checks five tabs, active page, compatibility guidance, cursor,
  model table interaction, and successful exit.
- Models checks the active Small Model page, provider tabs, table headings,
  model-specific reasoning options, and Middle Model navigation.
- Escape checks the rendered save/discard prompt and discard cancellation;
  unchanged config is an additional persistence assertion.

No E2E Then step relies only on a repository or config assertion. This is a CLI
capability, not a browser capability, and no HTTP or `fetch()` shortcut replaces
the PTY driver. Model discovery uses only per-scenario loopback httpmock twins.

### Command Isolation And Local Stack

The configured commands are distinct:

- Regular: `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` — 8 features, 43 scenarios, 234 steps passed.
- E2E: `cargo test --test features_runner -- --tags '@e2e and not @wip'` — 11 features, 40 scenarios, 246 steps passed.

The E2E count is strictly smaller than the regular count. The local command is
`cargo run -- setup`; the application is a single binary and the test transport
is an in-process loopback httpmock server. No database, queue, container, live
provider, or shared external network is required.

### Interaction Coverage Cross-Reference

| Inventory entry | Matrix row | Delta scenario | Step implementation | Driver verified |
|---|---|---|---|---|
| Run `watn setup` and complete the provider and model wizard | Setup wizard guides provider and model configuration page by page | Present, `@e2e` | `tests/steps/setup_wizard_steps.rs` starts `watn setup`, sends page and model keys, and asserts live tabs, cursor, pages, and saved result | `portable-pty` subprocess with loopback httpmock |
| Run `watn models` with provider information configured and enter model selection | Models command opens the shared wizard on Small Model | Present, `@e2e` | `tests/steps/setup_wizard_steps.rs` starts `watn models`, asserts Small Model/table/reasoning output, and advances with Enter | `portable-pty` subprocess with loopback httpmock |
| Leave the setup wizard with Escape and choose whether to discard current settings | Escape asks whether to save or discard current setup | Present, `@e2e` | `tests/steps/setup_wizard_steps.rs` sends Escape and `n`, asserts the live prompt, cancellation status, and unchanged config | `portable-pty` subprocess |
| Open the existing provider setup entry point and identify the active wizard page | Provider setup separates choices, details, and guidance | Present, `@e2e` | `tests/steps/provider_setup_steps.rs` and `provider_setup_layout_steps.rs` drive `watn provider` and assert live pages/layout | `portable-pty` subprocess |

The two feature inventory blocks contain four entries, the design matrix has
four rows, and each row maps to exactly one matching `@e2e` scenario.

## Coverage Measurement

The configured coverage commands instrument both the Cucumber runner and the
compiled `watn` binary used by PTY scenarios. `tests/steps/mod.rs` resolves the
child binary relative to the instrumented runner, so PTY children use
`target/llvm-cov-target/debug/watn`. The profile pattern is
`coverage/profraw/%p-%m.profraw`, which is collision-safe for child processes.
The runner and child process profiles are flushed before Cobertura generation.

Measured final outputs:

- Non-E2E Cobertura: `876 / 2074` lines, `line-rate="0.4223722275795564"`.
- E2E Cobertura: `1532 / 2074` lines, `line-rate="0.7386692381870781"`.
- E2E `src/setup.rs`: line rate `0.8283783783783784`.
- E2E `src/models/dialog.rs`: line rate `1`.
- E2E `src/models/mod.rs`: line rate `0.7553956834532374`.
- E2E `src/provider/setup.rs`: line rate `0.5957446808510638`.
- LLVM summary for the final E2E profile: `2173 / 3003` lines, `72.34%` line coverage.
- Branch data: the configured Cobertura export reports `branches-valid="0"`
  and `branch-rate="0"`; this Rust coverage configuration emits no branch
  denominator, so branch coverage is recorded as 0/0 rather than inferred.

### Coverage Classification

- **Dead code:** The obsolete standalone `SettingsDialog` and provider setup
  event loop were identified during audit and removed in `4bb9b7f` and
  `ed69540`. No remaining dead component exists in the reviewed change paths.
- **Missing test coverage:** None for the four requested user interactions or
  their real-interface behavior. The permanent provider/model scenarios also
  exercise validation, catalog failures, search fallback, stale-result guards,
  non-TTY index selection, and persistence paths.
- **Legitimately hard to test:** Remaining uncovered regions are terminal draw
  and event-read failures, PTY descriptor/process-handoff failures, and exact
  scheduler timing around cancellation of a background search. Exercising those
  requires fault injection or killing the test process at an exact I/O boundary;
  the resulting test would not validate a user-observable setup behavior. Normal
  Escape/Ctrl-C, invalid input, catalog failure, search fallback, and stale
  result behavior are covered through the existing scenarios.

## Verification

```text
givn lint --change unified-setup-wizard
givn lint: 2 file(s) checked - clean

cargo check
Finished successfully

cargo test --test features_runner -- --tags 'not @wip and not @e2e'
8 features, 43 scenarios passed, 234 steps passed

cargo test --test features_runner -- --tags '@e2e and not @wip'
11 features, 40 scenarios passed, 246 steps passed

coverage commands from givn/commands.yaml
non-E2E and E2E Cobertura reports generated successfully
```

`cargo fmt --all -- --check` still reports repository-wide pre-existing
formatting drift in legacy test step files. No formatting rewrite was included
because it is unrelated to this change.

All 24 task checkboxes are complete after the review check, all active tags are
preserved, and the local loopback test environment is deterministic.

REVIEW: PASS
