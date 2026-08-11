# Proposal: shell-completions

## Problem / Opportunity

Shell users cannot ask watn for completions of its own command tree. Any
hand-maintained completion files would drift as commands, flags, and value
choices change, and users would need to discover or install those files
separately.

## Proposed Solution

Watn shall provide `watn completions <shell>` for Bash, Zsh, and Fish. The
`<shell>` argument is a closed `CompletionShell` selector whose accepted input
values are exactly the lowercase literals `bash`, `zsh`, and `fish`; it is not
the broader shell set exposed by the completion library. Unsupported input is a
normal non-zero CLI argument error containing this literal contract, with the
rejected value substituted for `<value>`:

`unsupported shell '<value>'; choose bash, zsh, or fish`

The command writes only the completion script for the selected shell to stdout,
without loading configuration, creating a config file, contacting a provider,
writing any other file, or modifying shell configuration. Generated scripts
shall include every current root option, positional argument, subcommand, and
exposed value suggestion from the same authoritative command definition used by
the CLI. Repeated generation for the same binary and shell shall be
byte-for-byte deterministic. Successful generation writes only the script to
stdout and nothing to stderr.

The help text shall document the exact `watn completions <SHELL>` usage, the
supported shells, and that the generated script is written to stdout for the
caller to install or source. Unsupported values, including `powershell`, shall
produce the literal error contract above, naming the rejected value and telling
the user to choose bash, zsh, or fish. The new `completions` subcommand reserves
that command name: an unquoted first token `completions` now dispatches to the
subcommand rather than becoming question text. A question whose first token is
literally `completions` must be quoted as one argument or placed after `--`, for
example `watn -- completions find files`. This is an intentional CLI surface
change.

Existing commands other than that newly reserved command name and their
stdout/stderr contracts shall remain unchanged.

## Out of Scope

This change does not install completion files, edit shell startup files, load
provider configuration, contact any model service, or add an interactive shell
shortcut. It does not introduce separately maintained completion definitions or
change existing command behavior.

## Open Questions

No unresolved product decisions remain. Bash, Zsh, and Fish are the supported
shell outputs; all other values are unsupported-shell errors.
