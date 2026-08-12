# Design: Fix Fish Ctrl-W Completion Insertion

## Technical Decisions

The fix stays in the existing Rust shell-shortcut generator. No public Rust
API, configuration format, or generated Bash/Zsh block changes.

The Fish widget will build one buffer value containing the request comment, an
actual newline, and the generated command. Fish's `printf` builtin will emit
the newline and `string collect` will keep it inside one value before passing
that value to `commandline -r`. The current `\n` text is not used as a Fish
double-quoted escape because Fish inserts those characters literally.

The implementation changes `src/shell_shortcut.rs` only. The regression
scenario is driven through Fish's interactive reader in a pseudo-terminal so
the assertion observes the same command-line buffer a user sees after Ctrl-W.
This change asserts buffer representation only; committing the buffer and
Fish-specific failure, empty-output, and multiline-output branches remain out
of scope.

## Architecture Impact

- `src/shell_shortcut.rs`: update the generated Fish widget's buffer assembly.
- `givn/changes/fix-fish-ctrl-w-completion/specs/fish-ctrl-w-completion/fish-ctrl-w-completion.feature`: executable delta scenario.
- `tests/steps/fish_ctrl_w_completion_e2e_steps.rs`: Fish-specific end-to-end step definitions.
- `tests/steps/mod.rs`: register the Fish-specific step module.
- `tests/steps/preserve_ctrl_w_requests_steps.rs`: update the existing Fish generated-source contract assertion.

No data model, persistence, provider, or network changes are required.

## Step Definitions

| Capability | File | Responsibility |
|---|---|---|
| Fish Ctrl-W e2e buffer | `tests/steps/fish_ctrl_w_completion_e2e_steps.rs` | Install the Fish fixture, drive Ctrl-W through a real interactive process under `portable-pty`, capture the editable buffer, and assert the actual newline. |
| Existing Fish source contract | `tests/steps/preserve_ctrl_w_requests_steps.rs` | Assert the generated Fish source after the buffer assembly change through in-process generated-block inspection. |

The capability has one end-to-end scenario and no separate in-process step
file for the new behavior. The new e2e steps will:

- install the generated Fish shortcut into an isolated temporary home;
- place a deterministic fake `watn` executable first on `PATH`;
- start the real `fish` executable with `portable-pty` and source only the isolated shortcut;
- send the request text, Ctrl-W, and a test-only capture binding keystroke;
- read the captured Fish command-line buffer and assert the actual newline.

The fake executable does not contact a provider. It returns the command text
specified by the scenario.

## Verification

The configured main runner is the following command. It builds both required
test binaries and executes the permanent and change `.feature` files through
`tests/features_runner.rs`:

```sh
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
```

The end-to-end runner is:

```sh
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --tags '@e2e and not @wip'
```

The exact single-scenario command is:

```sh
cargo test --locked --test features_runner --features test-support -- --name '^Fish inserts a real line break after Ctrl-W$'
```

Strict mode is enforced by `Cucumber::<WatnWorld>::fail_on_skipped()` in
`tests/features_runner.rs`; undefined or skipped steps therefore fail both
the main and end-to-end runs. The e2e runner uses the same Cucumber binary and
the same strict builder, with the `@e2e` tag filter shown above.

## Local Runnability

This capability has no application server, database, queue, or remote service.
The complete local command is the configured end-to-end runner above. It builds
the binary, starts an isolated Fish process under a pseudo-terminal, and uses
a temporary fake `watn` executable for deterministic output.

The real interface is a CLI/terminal interaction: the e2e driver sends text and
the Ctrl-W control character to Fish and reads the resulting editable buffer.
The test uses the Fish executable resolved from `PATH`; its version is not
embedded in the application or lockfile. CI already installs Fish as a test
prerequisite. No network access is needed, and there are no external service
dependencies requiring a digital twin.

The non-interactive shell obstacle is handled by `portable-pty`, not bypassed:
Fish is run in interactive mode with a real pseudo-terminal. A test-only Ctrl-X
binding captures `commandline` after the generated Ctrl-W widget runs, then the
session is terminated and the captured terminal buffer is asserted.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| press Ctrl-W in an installed Fish shortcut and observe the generated command in the editable command line | Fish inserts a real line break after Ctrl-W | CLI / terminal | `portable-pty` drives the real interactive Fish process with the request text, Ctrl-W, and a capture binding; the captured `commandline` buffer is asserted |
