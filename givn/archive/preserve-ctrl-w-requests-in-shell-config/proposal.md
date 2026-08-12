# Proposal: Preserve Ctrl-W Requests In Shell Config

## Problem / Opportunity

After pressing Ctrl-W to turn a question into a generated command, the original
question disappears from the editable line and is replaced by the generated
command. The user loses a visible record of what they asked and how the shell
line became the generated command.

## Proposed Solution

When command generation succeeds, keep the original request visible in the
editable buffer. Above the generated command, show the original request as a
shell comment line:

```text
# original request
generated command
```

Pressing Enter ignores the comment and executes only the generated command.
The generated command remains ordinary editable text and is never executed
automatically. If generation fails or returns nothing, the original buffer is
left exactly as it was.

This must be implemented entirely in the generated Bash, Zsh, and Fish
configuration. `watn` itself does not change.

## Out of Scope

- Changing `watn` CLI behavior, output, or exit codes.
- Shell history persistence (only guaranteed to be visible in the editable
  buffer and terminal transcript after the successful replacement).
- Shells other than Bash, Zsh, and Fish.
- Changing line-editor key bindings or the way the request is sent to `watn`.

## Open Questions

None. The portable fallback described above is the required behavior; richer
buffer arrangements are only allowed if they preserve the same guarantees.
