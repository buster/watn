# Proposal: Highlight Active Setup Input

## Problem / Opportunity

When the setup dialog contains more than one place where the user can enter or
select a value, the current input location is not visually distinct. Users must
infer the active location from the cursor alone, which makes navigation through
the setup flow harder to follow.

## Proposed Solution

The setup dialog shall surround the input location currently receiving keyboard
input with a green border or box. Every inactive input location shall retain its
existing styling. As the user moves between setup inputs, the green indication
shall move with the active location.

The existing setup layout, keyboard navigation, selection behavior, and visible
cursor shall remain unchanged.

## Out of Scope

This change does not alter setup fields, labels, prompts, validation, saved
values, keyboard shortcuts, cursor behavior, or the styling of inactive input
locations. It does not add a new theme or change the appearance of dialogs
outside setup.

## Open Questions

None.
