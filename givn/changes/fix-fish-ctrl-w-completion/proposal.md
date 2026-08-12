# Proposal: Fix fish ctrl-w completion insertion

## Problem / Opportunity

When a user presses `ctrl-w` in the Fish shortcut, the inserted text can
contain an explanatory comment followed by the generated command. The line
break is currently inserted as the visible characters `\\n`, so the editable
buffer does not contain two separate shell lines and the user must repair the
line manually.

## Proposed Solution

The Fish shortcut must preserve the explanatory comment while inserting the
generated command on a new shell line. The line break must be an actual line
break in the editable buffer, not visible `\\n` characters.

## Out of Scope

Completion content, command generation, and behaviour in shells other than
Fish are unchanged. Executing the resulting buffer and Fish-specific failure,
empty-output, and multiline-output handling are not changed by this proposal.

## Open Questions

None.
