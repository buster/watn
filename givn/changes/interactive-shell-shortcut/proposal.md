# Proposal: interactive-shell-shortcut

## Problem / Opportunity

People must leave their shell command line, form a question for `watn`, copy the
generated command, and return to the shell before they can use it. This breaks
the flow of exploring commands and makes the current line easy to lose. `watn`
does not currently offer a setup path for a reusable shell shortcut.

## Proposed Solution

Add an optional shell-shortcut question to the shared setup flow used by both
explicit `watn setup` and implicit first-use setup. The question is reached
after the Large Model confirmation, rather than becoming a sixth setup tab.
Enter accepts the default decline and leaves the existing five-tab setup path
unchanged. Answering `y` opens a multi-select for Bash, Zsh, and Fish. The
current shell may preselect one option using only the basename of `$SHELL`, but
all supported shells remain available.

For every selected shell, install or replace a clearly marked configuration
block that binds Ctrl-W. Each selected target is attempted independently. Setup
reports every success and failure, keeps successful changes when another target
fails, and returns an aggregate installation failure if any selected target
fails.

Pressing the shortcut must read the complete current command line as one
question and invoke `command watn -- "$question"`. A successful non-empty
result is inserted into the current line without executing it. Only trailing
line terminators are removed from stdout; embedded line breaks are preserved in
the buffer. The cursor moves to the end of the inserted text and the prompt is
redrawn. Empty input, a non-zero `watn` status (including partial stdout), and
empty output leave the current line unchanged. Stderr remains diagnostics and
stdout remains generated command text.

Target updates validate the complete marker contract before writing: exactly one
opening marker and one closing marker, in that order, must exist. Duplicate,
unmatched, or reversed markers fail without changing the target. Valid updates
preserve unrelated content and use safe atomic replacement. Missing targets are
created only for selected shells. The setup report includes each target path,
the reason for any path or write failure, and the reload instruction for every
successful modification.

## Out of Scope

The default shortcut remains Ctrl-W; changing the shortcut interactively is not
part of this change. Existing Ctrl-W behavior is intentionally overridden only
in shells selected by the user.

This change does not execute generated commands, change provider selection,
change model selection, or install shortcuts for unsupported shells. It does not
modify shell files when the user declines or selects no shells. Runtime E2E
verification does not require an interactive shell PTY. Generated Bash and
Fish configurations are checked by their installed shell parsers, and the
Bash widget is exercised through a non-interactive Bash process; Zsh is covered
by generated-block and contract checks when its executable is unavailable.

## Open Questions

None. Bash, Zsh, and Fish configuration locations, atomic target-write rules,
marker validation, aggregate reporting, and reload guidance are defined by the
setup behavior and executable scenarios.
