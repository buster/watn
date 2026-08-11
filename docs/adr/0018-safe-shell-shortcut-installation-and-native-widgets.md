# ADR-0018: Safe shell shortcut installation and native widgets

- **Status:** accepted
- **Date:** 2026-08-11
- **Decision-makers:** architect

## Context and Problem Statement

Users need to turn the command line already typed in Bash, Zsh, or Fish into a
single `watn` question without leaving the shell. The result must be inserted
for inspection, never executed automatically. Setup must install this behavior
without corrupting user-owned startup files, duplicating generated content, or
silently changing an unselected shell.

The shortcut is available during explicit setup and implicit first-use setup.
The normal Enter path must remain a safe default decline. Selected shells can
also fail independently because their startup files may be absent, malformed,
or unwritable.

## Decision Drivers

- Preserve the complete current line as one question, including spaces,
  metacharacters, leading options, and the reserved `completions` token.
- Keep shell startup-file content outside the generated block untouched.
- Prevent generated output from being evaluated by the shell.
- Make repeated setup idempotent and malformed marker layouts fail safely.
- Support native cursor, redraw, and line-editor behavior in Bash, Zsh, and Fish.
- Make partial multi-shell installation observable and actionable.
- Avoid adding a shell-management dependency to the single CLI binary.

## Considered Options

- **Copy a command to the clipboard** - avoids startup-file edits, but does not
  replace the current line and requires a separate clipboard dependency or tool.
- **Install one portable shell script** - reduces generation code, but cannot
  use the native Readline, ZLE, and Fish commandline buffer/cursor APIs safely.
- **Generate native widgets with direct in-place writes** - gives native UX, but
  a failed write can truncate a startup file and duplicate markers are hard to
  diagnose.
- **Generate native widgets with exact marker ownership and atomic replacement**
  - adds a small installer and temporary-file lifecycle, but protects unrelated
  user content and makes setup failures observable.

## Decision Outcome

Choose an opt-in post-Large-Model setup interaction. Enter accepts the default
decline; `y` opens a multi-select for Bash, Zsh, and Fish. The same interaction
is available in explicit and implicit first-use setup and is not a sixth setup
tab. Preselection uses only the basename of `SHELL`; existing files do not
restrict or silently select other shells.

For each selected shell, resolve the conventional target from absolute `HOME`
and XDG configuration paths. Generate a shell-native block delimited by:

```text
# >>> watn shell shortcut >>>
# <<< watn shell shortcut <<<
```

Zero markers appends one block. Exactly one ordered opening/closing pair replaces
that pair. Duplicate, unmatched, or reversed markers fail before any write.
Existing content is assembled in memory, written to a uniquely named temporary
file in the target directory, flushed and synced, and atomically renamed over
the target. Existing file mode is retained where possible. Every selected
target is attempted independently; successful changes are kept, every success
and failure is reported with its path/reason, and any failure produces one
aggregate setup error after all attempts.

The Bash, Zsh, and Fish widgets use their native line-editor APIs and bind
Ctrl-W. Each reads the complete buffer and invokes:

```sh
command watn -- "$question"
```

Only stdout is captured. A zero-status non-empty result has trailing CR/LF
characters removed and is assigned as text; embedded line breaks remain in the
buffer. Empty input, non-zero status, and empty output preserve the original
buffer. The prompt is redrawn and the cursor moves to the end after every
shortcut event. The captured result is never evaluated.

## Consequences

### Good

- The user can generate a command from the current shell line and inspect it
  before pressing Enter.
- Native line-editor APIs provide correct buffer replacement, cursor movement,
  binding, and redraw for each supported shell.
- `command watn -- "$question"` keeps the complete input as one positional
  question and avoids aliases/functions and option/subcommand reinterpretation.
- Exact markers, byte preservation outside the block, and atomic replacement
  make repeated setup idempotent and protect existing startup content.
- Independent target reports make a partial Bash/Zsh/Fish installation visible
  instead of hiding the successful or failed shell.
- The feature has no new persisted schema or runtime service dependency.

### Bad

- Setup intentionally overrides Ctrl-W only in selected shells and can mutate
  user-owned startup files; the default decline and explicit reports mitigate
  surprise.
- The installation is not a multi-file transaction. A later target failure does
  not roll back an earlier successful rename.
- Shell syntax, key maps, `PATH`, and startup-file conventions vary by shell and
  environment. Runtime E2E covers Bash; Zsh and Fish rely on generated syntax
  and contract checks in this change.
- Atomic replacement requires a writable target directory and may not preserve
  every platform-specific file attribute or symlink behavior.
- Embedded line breaks are retained in the editable buffer, so the inserted
  value may be multiline even though it is never evaluated by the widget.
- Supporting another shell requires a new native block, target rule, parser
  checks, and setup choice.

## Confirmation

The interactive-shell-shortcut feature verifies the default decline, empty
selection, basename-only preselection, all three generated blocks, exact marker
validation, atomic failure preservation, independent aggregate reporting,
reload guidance, leading-option/reserved-token quoting, status handling,
trailing and embedded newline behavior, cursor placement, prompt redraw, and
no-evaluation behavior. A real Bash PTY drives setup and Ctrl-W; regular
isolated tests cover Zsh/Fish generation and shell contracts. The existing
`./run-tests.sh` and `./run-tests.sh --e2e` wrappers remain the verification
commands.
