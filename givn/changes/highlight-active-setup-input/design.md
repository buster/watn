# Design: Highlight Active Setup Input

## Technical Approach

The setup wizard already owns the focus state for every interactive area. The
rendering layer will use that state to style only the border of the focused
widget with `Color::Green`.

The existing Ratatui and Crossterm dependencies remain unchanged. No new
dependency, persisted value, public API, or input event is required.

The rendering changes are:

- URL page: mark the `URL (editing)` block as focused.
- API key page: mark either the credential-source list or the credential-value
  block as focused according to `CredentialFocus`.
- Model pages: mark either the model table or reasoning block as focused
  according to `ModelFocus`.
- Shell-shortcut page: mark either the question block or shell-selection list as
  focused according to `ShortcutFocus`.
- Leave every non-focused block without an explicit green border style so its
  current terminal styling remains unchanged.

A small rendering helper will construct a bordered widget and conditionally
apply the green border style. It will not alter widget dimensions, titles,
highlighted rows, keyboard handling, or the visible `█` cursor marker.

## Architecture Impact

Production changes are limited to `src/setup.rs` and its existing drawing
functions. The change adds no state because all required focus state already
exists.

The new Cucumber step definitions live in the capability-specific file
`tests/steps/highlight_active_setup_input_steps.rs`, registered from
`tests/steps/mod.rs`. They drive the existing PTY helpers and inspect the
terminal stream emitted by the real `watn setup` process.

## Rendering Verification

The PTY captures the terminal's ANSI stream without stripping control
sequences. Because Ratatui may emit incremental diffs rather than a complete
frame on every draw, the capability steps reconstruct the current 120-column by
40-row terminal screen from the cumulative stream. The parser handles the
Crossterm sequences used by this harness: cursor positioning/movement, carriage
return and line feed, clear-screen/clear-line, and SGR foreground changes.
Each cell stores its character and foreground style. The capability steps will:

1. Wait until the relevant setup page has rendered.
2. Reconstruct the current screen and locate the titled widget's top border and
   its border-glyph cells, rather than searching raw transcript substrings.
3. Parse SGR parameters semantically and confirm that the border-glyph cells
   for the focused widget have a green foreground: either basic `32` or
   extended `38;5;2`. Combined sequences such as `ESC[38;5;2;49m` are valid;
   background parameters do not invalidate a green foreground.
4. For each two-widget focus transition, capture both border signatures before
   the keypress. The currently inactive companion is the default-style
   baseline; after the keypress, compare the previously active widget against
   that baseline and assert the newly active widget is green. This proves
   inactive styling is unchanged instead of inferring it from the absence of a
   green token. URL has no peer and is checked only for green focus styling.

The parser ignores private-mode and OSC presentation controls, and supports the
CSI cursor/erase forms emitted by this harness (`H`, `f`, `A`, `B`, `C`, `D`,
`G`, `d`, `J`, `K`) plus carriage return and line feed. Unknown controls are
ignored without changing the reconstructed screen.

The assertions target the rendered terminal output, not the `SetupWizard`
fields. This preserves the real-interface guarantee while remaining stable
across frame redraws.

## Test Commands

The configured regular verification command is the following command from
`givn/commands.yaml`; it runs the repository's Cucumber/Gherkin feature runner
against permanent specs and active change specs, excluding E2E and WIP tags:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
```

The configured E2E verification command is:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --tags '@e2e and not @wip'
```

The strict-mode mechanism is `Cucumber::fail_on_skipped()` in
`tests/features_runner.rs`. New step bodies must be real implementations; an
undefined or WIP step is excluded only while it carries `@wip` and therefore
cannot pass the E2E verification command after implementation work begins.

To run one scenario during RED, GREEN, or REFACTOR, use the full E2E bootstrap
with the runner's exact name filter. The binary copies are required because the
PTY harness does not discover `target/debug/watn` implicitly:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --name 'The initial URL input has a green border'
```

Replace the quoted scenario title to run either of the other scenarios. The
runner's `--name` option was verified from the same command with `--help`
before this design was written.

## E2E Infrastructure

This capability is a CLI/terminal UI, not a browser UI or HTTP API. The real
interface is the terminal process a user starts with `watn setup`. Each E2E
scenario starts the instrumented `watn` binary in a real pseudo-terminal using
`portable-pty`, sends the same keystrokes a user would send, and asserts on the
captured terminal display and ANSI styling.

The E2E step definitions are in
`tests/steps/highlight_active_setup_input_steps.rs`. The existing test harness
creates a deterministic `httpmock` model endpoint inside the test process when
the scenario requests an ephemeral E2E transport. It is the digital twin for
the external model provider; no live network service is used.

The PTY child removes inherited `NO_COLOR` before setting
`TERM=xterm-256color`, ensuring the ANSI color assertion is deterministic even
when the invoking environment disables color globally.

The local command that starts and exercises the complete system for this
capability is the configured E2E command above. It builds the binary, starts
the fake provider and PTY from the test harness, runs the scenarios, and shuts
both down when the runner exits. No Docker stack or shared service is required.

The expected interface obstacle is terminal redraw noise from repeated
Ratatui frames. The step definitions handle this by matching the latest
rendered occurrence of each titled border and retaining ANSI sequences during
inspection. They do not downgrade the assertion to an internal render or a
unit-level style value.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `watn setup` | PTY step definitions in `features_runner` | `target/llvm-cov-target/debug/watn` copied to a temporary binary path | `coverage/profraw/%p-%m.profraw` via `LLVM_PROFILE_FILE` | `merge-coverages.sh` after non-E2E and E2E runs | URL, credential, model, and reasoning draw paths in `src/setup.rs` |
| `features_runner` | `measure-coverage.sh` | `cargo llvm-cov test --test features_runner --features test-support` | Same collision-safe `profraw` pattern | `cargo llvm-cov` emits each Cobertura file; `merge-coverages.sh` combines them | Capability step definitions and PTY setup path |
| `httpmock` digital twin | `features_runner` | Test process code | Included in the runner profile, not a production artifact | Included in the corresponding Cobertura output | Model catalog response used to advance the wizard |

Profiles are cleared before each measurement, use process/module-specific
paths, and are flushed when the instrumented processes exit. The existing
`measure-coverage.sh` and `merge-coverages.sh` scripts remain the source of
truth for the coverage gate.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| launch `watn setup` and inspect the active URL input | The initial URL input has a green border | CLI/terminal UI | `portable-pty` starts `watn setup`, waits for the Setup page, and reads the rendered ANSI terminal stream |
| advance to the API key page and move between its credential locations | The green border follows API key focus | CLI/terminal UI | `portable-pty` sends Enter and `p`, then inspects the storage-list and API-key widget borders in the live PTY output |
| advance to a model page and switch between model and reasoning input | The green border follows model focus | CLI/terminal UI | `portable-pty` sends endpoint, credential, and Ctrl-R keystrokes, then inspects the model-table and reasoning-widget borders in the live PTY output |
| complete model setup and move between the optional shortcut question and shell list | The green border follows optional shortcut focus | CLI/terminal UI | `portable-pty` confirms the Large Model, sends `y`, and inspects the shortcut-question and shell-list borders in the live PTY output |
