# Proposal: polish-review-detail-block

## Use-Case Context

- Use-Case ID: use-shell
- Ideation topic: none
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The detailed review view renders the command differently from the simple view:
a `Command` label and a deeper indent set it apart, and the purpose sits
directly under the last command row without the blank separator the simple view
uses. The two views should read as the same block.

The detailed hint also paints the `a` in `accept` green as the default decision,
while the simple view leaves it unmarked. `a` is not needed as a shortcut;
Enter accepts in both views. The `D` decision reads only `disable`, so it is
unclear what gets disabled.

## Proposed Solution

The detailed view renders the command and its purpose exactly like the simple
view: no `Command` label, the same stack indentation, and one blank row between
the command stack and the purpose. The intent row stays.

The `a` accept shortcut is removed from the key handling and from the hints;
`Enter` accepts in both views and the hint shows an unmarked `accept`. `e`, `r`,
`c`, `d`/`?`, `D`, the arrows, and `Escape` are unchanged. The detailed hint
reads `D disable review` so the decision names what it disables.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | The change refines the existing review surface. |

## Out of Scope

- No change to the reason, mode, ordering, or rendering of accept, edit, reject,
  cancel, view toggle, or disable decisions other than the `a` removal.
- No change to the simple view beyond the hint already being unmarked.

## Open Questions

- None.
