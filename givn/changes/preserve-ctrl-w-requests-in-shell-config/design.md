# Design: Preserve Ctrl-W Requests In Shell Config

## Technical Approach

The change is confined to the three generated widget blocks in
`src/shell_shortcut.rs` (`BASH_BLOCK`, `ZSH_BLOCK`, `FISH_BLOCK`). `watn` itself
does not change.

On a successful non-empty generation, each widget replaces the editable buffer
with two parts separated by a newline:

1. The original request, flattened to a single line and prefixed with `# ` so
   the shell treats it as a comment.
2. The generated command text.

Pressing Enter then submits the whole buffer; the shell ignores the comment and
executes only the generated command. The generated text is assigned (Bash
`READLINE_LINE`, Zsh `BUFFER`, Fish `commandline -r --`) and never evaluated.

Flattening replaces CR, LF, and TAB in the captured request with a single
space, so a request that itself contains newlines or control characters still
forms exactly one comment line. All other characters are preserved, so quoting,
metacharacters, and wildcards stay readable in the comment.

Failure and empty-output behavior is unchanged: when `watn` exits non-zero or
returns empty output, the widget leaves the original buffer untouched.

## Widget Changes

### Bash

```bash
_watn_widget() {
    local question="$READLINE_LINE"
    local result
    if [[ -z $question ]]; then
        return
    fi
    result=$(command watn -- "$question")
    local status=$?
    if [[ $status -eq 0 ]]; then
        while [[ $result == *$'\r' || $result == *$'\n' ]]; do
            result=${result:0:${#result}-1}
        done
        if [[ -n $result ]]; then
            local comment="${question//$'\n'/ }"
            comment="${comment//$'\r'/ }"
            comment="${comment//$'\t'/ }"
            READLINE_LINE="# $comment"$'\n'"$result"
            READLINE_POINT=${#READLINE_LINE}
        fi
    fi
}
```

### Zsh

Same substitution semantics with `$BUFFER`, trimming inner result newlines only
for the command portion while keeping embedded generated newlines as buffer
text:

```zsh
_watn_widget() {
    local question="$BUFFER"
    local result comment
    if [[ -z $question ]]; then
        zle redisplay
        return
    fi
    if result=$(command watn -- "$question"); then
        while [[ $result == *$'\r' || $result == *$'\n' ]]; do
            result=${result%$'\r'}
            result=${result%$'\n'}
        done
        if [[ -n $result ]]; then
            comment=${question//$'\n'/ }
            comment=${comment//$'\r'/ }
            comment=${comment//$'\t'/ }
            BUFFER="# $comment"$'\n'"$result"
            CURSOR=${#BUFFER}
        fi
    fi
    zle redisplay
}
```

### Fish

```fish
function _watn_widget
    set -l question (commandline)
    if test -z "$question"
        commandline -f repaint
        return
    end
    set -l result (command watn -- "$question" | string collect)
    set -l status_code $pipestatus[1]
    if test $status_code -eq 0
        set result (string replace -r '[\r\n]+$' '' -- "$result")
        if test -n "$result"
            set -l comment (string replace -a '\n' ' ' -- "$question")
            set comment (string replace -a '\r' ' ' -- "$comment")
            set comment (string replace -a '\t' ' ' -- "$comment")
            commandline -r -- "# $comment\n$result"
        end
    end
    commandline -f repaint
end
```

## Architecture Impact

Production changes are limited to the three block constants in
`src/shell_shortcut.rs`. No public API, persisted value, provider, or setup
wizard behavior changes. Installer report text, marker handling, and shell
syntax are unchanged.

## Test Strategy

The existing generated-widget test harness in
`tests/steps/interactive_shell_shortcut_steps.rs` sources the Bash block, sets
`READLINE_LINE`/`READLINE_POINT`, calls `_watn_widget`, and reads back the
buffer via `LINE<<...>>`/`POINT<<...>>` markers with a fake `watn` on `PATH`.
That harness is reused unchanged for the preserved-request scenarios.

New capability-specific steps live in
`tests/steps/preserve_ctrl_w_requests_steps.rs`, registered from
`tests/steps/mod.rs`, and reuse the shared fixture and Bash-run helpers. `@e2e`
update steps live in `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.

### Spec updates

Three existing permanent scenarios are behaviourally changed by this feature
and are updated through `@givn.modified` entries with the same titles:

- `A successful widget inserts one normalized command and moves the cursor to its end`
- `Embedded multiline output remains buffer text without evaluation`
- `The generated Bash widget runs through Bash without evaluating its result`

Their expectations change to include the comment line above the generated
command. Failure/empty scenarios are unchanged (original buffer preserved).

### New assertions

- Commit-execution: the produced buffer is executed with `bash -c`; the
  generated command's side effect runs and no command embedded in the comment
  runs.
- Comment flattening: a request with spaces, metacharacters, and embedded
  newlines yields exactly one `#`-prefixed comment line.
- Zsh/Fish: generated blocks contain comment-plus-command construction and pass
  `zsh -n` / `fish -n` when those shells are installed.
- E2E: the real Bash subprocess keeps `# <request>` above the generated command
  and does not evaluate the replacement text.

### Interface obstacles

The widget replaces terminal-buffer state that only exists inside a real
interactive shell. The Bash path is exercised in a real `bash` subprocess
(fixture + E2E), which is the closest fully-automatable approximation of the
real Readline buffer. Zsh/Fish are verified at the generated-block content and
syntax-check level because their interactive buffer APIs (`zle`, `commandline`)
require interactive sessions; wrapped-line redraw is not separately measurable
in a non-interactive runner, so it is classified as legitimately hard to test
in the review.

## Test Commands

Regular: `./run-tests.sh` (filter `not @wip and not @e2e`).
E2E: `./run-tests.sh --e2e` (filter `@e2e and not @wip`).
Single-scenario:

```text
root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --name '<SCENARIO TITLE>'
```

Strict mode is `Cucumber::fail_on_skipped()` in `tests/features_runner.rs`; new
Rust step bodies use `unimplemented!()` during RED.

## E2E Infrastructure

This is a CLI/terminal capability. The real interface is the generated shell
widget executed inside a real Bash process. The E2E scenario starts the
installed Bash block in `bash --noprofile --norc -c`, sets the current input,
invokes the widget, and asserts the returned buffer plus the absence of
evaluation. No browser, HTTP service, or extra digital twin is required; the
fake `watn` script on `PATH` is the only fixture.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| press Ctrl-W in an installed Bash, Zsh, or Fish widget and observe the original request preserved as a comment above the generated command | The generated Bash widget keeps the request visible and does not evaluate the command | Generated Bash widget in a real Bash subprocess | Real `bash` subprocess sources the installed block, sets the current input, invokes the widget, and reads back the editable buffer and execution state |

## Design Decisions

- Keep the portable `# request` + generated-command fallback; do not add
  shell-specific multi-line buffer-preservation schemes unless needed for
  correctness.
- Flatten only CR/LF/TAB; keep the rest of the request readable in the comment.
- Preserve the existing failure/empty behavior unchanged.
- Reuse the existing Bash widget test harness rather than adding a parallel
  unit test path.
