# Proposal: Fix fish ctrl-w completion insertion

## Problem / Opportunity

When a user accepts a completion in fish with `ctrl-w`, the inserted text can
contain an explanatory comment followed by the completed command. The line
break is currently inserted as the visible characters `\\n`, so fish sees the
whole selection as a comment. The command is not available as an executable
command and the user must repair the line manually.

## Proposed Solution

The accepted fish completion must preserve the explanatory comment while
inserting the completed command on a new shell line. The line break must be an
actual line break, not visible `\\n` characters, and fish must treat the
completed command as executable text.

## Out of Scope

Completion content, command generation, and behaviour in shells other than
fish are unchanged.

## Open Questions

None.
