# watn Improvement Handoff Plan

Handoff snapshot: 2026-08-11

This file is the working handoff for the next agent. It describes the current
repository state, the decisions already made, and the remaining implementation
work.

## Current Repository State

- Repository: `/home/buster/projects/watn`
- Branch: `main`
- Worktree: clean; shell-completions archive committed as `f2398ad`
- Remote/upstream: `origin` configured
- Active givn change: none
- Archived transport work: `incremental-sse-rendering`, `isolate-test-transport`
- Archived release work: `release-truth-and-repository-cleanup`, `shell-completions`
- Current package version: `0.1.2` in `Cargo.toml`
- Current CLI version: read from `CARGO_PKG_VERSION` in `src/main.rs`

Do not amend existing commits. Do not push unless explicitly requested.

## Required Session Start

Run this before exploring or editing:

```text
givn instructions
```

The project uses givn. The required lifecycle is:

```text
new -> propose -> spec -> design -> design-review -> tasks -> implement -> review -> archive
```

Keep exactly one active change. Complete one scenario at a time. Use RED,
GREEN, REFACTOR, and one atomic scenario commit. Record the commit hash in
`tasks.md` immediately after the scenario commit. Do not batch-check task
boxes.

## Baseline Contracts

These contracts must remain true unless a later change explicitly changes them
through a reviewed proposal and specification:

- Commands and generated command content go to stdout.
- Metadata, prompts, spinners, setup guidance, and diagnostics go to stderr.
- LiteLLM is a model-discovery service only. It must never replace the active
  chat-completion provider.
- `watn models` changes model tiers without replacing the active provider.
- `watn setup` and implicit first-use setup save a valid provider before the
  first model-catalog request.
- An absent thinking-tier reasoning value retains the existing `high` default.
- Empty and unknown persisted reasoning values disable reasoning.
- The test endpoint seam is not available in normal or release-profile builds.
- A release binary is currently dynamically linked. Do not claim universal
  static deployment without producing and verifying static artifacts.
- Saved literal credentials and exact environment references are authoritative.
- A missing saved environment reference is an authentication error and does
  not fall through to another environment variable.

## Coverage And Test Command Contract

- Coverage and test commands must be `.sh` scripts configured in `commands.yaml`.
- Those scripts must compute overall line and branch coverage by merging E2E and
  non-E2E coverage.
- The README must contain the resulting overall line and branch coverage counts
  once, rather than multiple different counts.

## Completed Work

- `incremental-sse-rendering` is archived and verified.
- `release-truth-and-repository-cleanup` is archived and verified.
- `shell-completions` is implemented, verified, and archived.
- Current shell-completion verification passed: 74 regular scenarios and 59
  E2E scenarios. The all-target run passed 133 scenarios and 842 steps.

## Remaining Work

### 1. Interactive Shell Shortcut For Watn

Create this as a separate givn change after shell completions, unless the
implementation order is deliberately changed through a reviewed proposal:

```text
givn new interactive-shell-shortcut
```

#### Overview

Extend the existing multi-step `watn setup` wizard with an optional step that
installs a shell key binding. The default binding is Ctrl-W. It reads the
entire current shell command-line buffer, passes that buffer to `watn` as one
quoted question, and replaces the current buffer with the generated command.
The generated command is inserted but never executed automatically.

Example:

```text
$ find all images<Ctrl-W>
$ find . -type f \( -iname '*.jpg' -o -iname '*.png' \)
```

The user can inspect or edit the replacement before pressing Enter.

#### Setup Flow

Add the following optional flow to the existing setup wizard:

1. Ask: `Configure a shell shortcut for generating commands with watn?`
2. If enabled, show a multi-select list containing Bash, Zsh, and Fish.
3. Preselect shells detected from the current environment when appropriate,
   without preventing selection of any supported shell.
4. Allow zero, one, or multiple selected shells.
5. Install the selected shell configuration.
6. Report modified files and shell-specific reload instructions.

If the user declines or selects no shells, leave shell configuration untouched
and continue setup. The shell choice is independent of the user's default
shell; users may select shells that are not currently running.

#### Shortcut Contract

For every selected shell, the generated widget must:

- Read the complete current command-line buffer.
- Avoid invoking `watn` for empty input, or otherwise leave the line unchanged.
- Invoke `watn "$question"` with quoted expansion so the entire buffer is one
  question.
- Capture the generated command without evaluating it.
- Replace the current buffer only when `watn` succeeds and output is non-empty.
- Move the cursor to the end of the replacement.
- Redisplay the prompt.
- Preserve the original input when `watn` fails or produces no output.
- Never execute the generated command automatically.
- Normalize trailing newlines so the inserted value remains a single command
  line.
- Keep stderr visible or handle it consistently with the existing CLI output
  contract.

#### Bash Implementation

Install a function and Readline binding using the current line and cursor:

```bash
# >>> watn shell shortcut >>>
_watn_widget() {
    local question=$READLINE_LINE
    local result

    if result=$(watn "$question") && [[ -n "$result" ]]; then
        READLINE_LINE=$result
        READLINE_POINT=${#READLINE_LINE}
    fi
}

bind -x '"\C-w":_watn_widget'
# <<< watn shell shortcut <<<
```

Bash-specific requirements:

- Use `READLINE_LINE` for the current buffer.
- Use `READLINE_POINT` to position the cursor.
- Register the function with `bind -x`.
- Load the binding when the shell starts.
- Explicitly document that the selected shell's existing Ctrl-W binding is
  overridden, or preserve it if the project decides that is required.

#### Zsh Implementation

Install a ZLE widget and binding:

```zsh
# >>> watn shell shortcut >>>
_watn_widget() {
    local question=$BUFFER
    local result

    if result=$(watn "$question") && [[ -n "$result" ]]; then
        BUFFER=$result
        CURSOR=${#BUFFER}
    fi

    zle redisplay
}

zle -N _watn_widget
bindkey '^W' _watn_widget
# <<< watn shell shortcut <<<
```

Zsh-specific requirements:

- Use `$BUFFER` for the current line.
- Set `$CURSOR` after replacement.
- Register the widget using `zle -N`.
- Bind the widget using `bindkey`.
- Bind the default map and, when applicable, `viins` as well:
  `bindkey -M viins '^W' _watn_widget`.

#### Fish Implementation

Install a Fish function and binding:

```fish
# >>> watn shell shortcut >>>
function _watn_widget
    set -l question (commandline)
    set -l result (watn "$question" | string collect)
    set -l status_code $pipestatus[1]

    if test $status_code -eq 0; and test -n "$result"
        commandline -r -- "$result"
    end

    commandline -f repaint
end

bind \cw _watn_widget
# <<< watn shell shortcut <<<
```

Fish-specific requirements:

- Use `commandline` to read the current buffer.
- Use `commandline -r -- "$result"` to replace it.
- Repaint after the command completes.
- Preserve the original line on failure or empty output.
- Bind Ctrl-W in the appropriate default mode; support insert mode if the
  project's Fish configuration conventions require it.

#### Configuration Installation

Follow existing setup conventions for prompt rendering, multi-select inputs,
configuration discovery, status output, error handling, and generated blocks.
The typical target files are:

| Shell | Typical configuration file |
|---|---|
| Bash | `~/.bashrc` |
| Zsh | `~/.zshrc` |
| Fish | `~/.config/fish/config.fish` |

The installer must:

- Use existing shell and configuration path detection when available instead of
  hard-coded assumptions.
- Create missing parent directories, especially for Fish.
- Append a clearly delimited shell-appropriate generated block.
- Use the markers `# >>> watn shell shortcut >>>` and
  `# <<< watn shell shortcut <<<`.
- Replace an existing exact generated block rather than append a duplicate.
- Preserve unrelated user configuration.
- Define the behavior for manually customized content inside an existing
  marked block; the preferred default is replacing only exact generated-marker
  blocks, with confirmation if the project chooses to preserve custom edits.
- Fail clearly when a target file cannot be read or written.
- Ideally create a backup before modifying an existing file, following any
  existing repository backup convention.
- Use the installed `watn` executable resolved naturally from the user's PATH;
  never embed a development-time or repository-local executable path.

#### Setup Edge Cases And Runtime Reporting

- Running setup twice is idempotent.
- Declining the shortcut makes no shell-file changes.
- Selecting no shells makes no shell-file changes.
- Selecting multiple shells updates each selected file independently.
- Existing Ctrl-W behavior is overridden only for selected shells.
- Empty input, non-zero `watn` status, and empty output preserve the original
  buffer.
- Generated output is inserted as text and is never shell-evaluated during
  replacement.
- Report each modified file and how to reload it. Examples:

```text
Configured Bash in ~/.bashrc.
Run: source ~/.bashrc

Configured Zsh in ~/.zshrc.
Run: source ~/.zshrc

Configured Fish in ~/.config/fish/config.fish.
Run: source ~/.config/fish/config.fish
```

It is also valid to state that the shortcut becomes available in newly
started shells.

#### Required Tests

Use temporary HOME/XDG-style directories and shell fixture files so tests do
not modify a developer's real configuration. Cover:

- Declining the wizard step leaves all shell files byte-for-byte unchanged.
- Zero, one, and multiple shell selections behave correctly.
- Environment-based preselection does not restrict manual selection.
- Missing parent directories are created only when installation is selected.
- Bash, Zsh, and Fish blocks contain their required widget APIs and bindings.
- Repeated setup replaces or preserves the generated block without duplicates.
- Unrelated user configuration remains byte-for-byte unchanged.
- Read/write failures produce actionable errors.
- A successful widget replaces the line, moves the cursor, repaints, and does
  not execute the generated command.
- A failed or empty `watn` result preserves the original line.
- The complete input is passed as one quoted question, including spaces and
  shell metacharacters.
- Trailing output newlines are normalized consistently.
- Reload instructions identify the exact modified file for every selected
  shell.

### 5. Highlight Active Setup Input

Improve the setup dialog's visual indication of where user input is currently
being entered. The border or box surrounding the active input location shall be
green; inactive input locations retain their existing styling.

### 6. Responsive Setup Model Filtering

Improve the setup dialog's model filter so the typed query remains visible while
the user is entering it. Typing must remain responsive while model searches run
in the background, and filter updates shall be debounced by 200 ms. The model
list shall update continuously as the debounced query changes, without blocking
further input on an in-flight search.

When the complete model list fits in one catalog request (for example, fewer
than 500 models), load the list once and filter it client-side instead of making
server-side search requests. Use server-side filtering when the catalog requires
multiple requests or otherwise cannot be loaded in one request. Results from an
older in-flight search must not replace the results for a newer query.

## Handoff Rules

- Read `givn instructions` before acting.
- Inspect `givn status --change <id>` before editing an active change.
- Preserve user changes and never reset or checkout unrelated work.
- Use `apply_patch` for manual edits.
- Use one active change at a time.
- Use one scenario commit for RED, GREEN, and REFACTOR.
- Do not amend existing commits.
- Do not push unless the user explicitly requests it.
- Keep secrets out of diagnostics, persisted test output, and commits.
- Prefer the smallest correct implementation. Do not add compatibility layers
  without a concrete persisted-data or external-consumer requirement.
