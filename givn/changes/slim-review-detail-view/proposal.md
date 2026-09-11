# Proposal: slim-review-detail-view

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The detailed review view still carries presentation the developer does not need.
The `Flow` strip and the `Stage x/y` row repeat the selection that the stack
arrow already shows, and the `Stage` row adds a `supported`/`unsupported` word
the developer cannot act on. Stages the flow could not decompose carry an amber
ellipsis that competes with the stage text instead of helping, and the hint row
advertises `cancel` although Escape is the same decision.

The permanent-disable output also reads badly on a terminal: the re-enable hint
is printed after the released command in the same plain style, so the command
line and the hint are indistinguishable.

## Proposed Solution

The detailed view drops the `Flow` row, the `Stage` row, and the amber ellipsis
on stages the flow could not decompose; the stack arrow alone shows which stage
is selected, and the stage text with its purpose describes the stage. The hint
row no longer names `cancel` — Escape stands alone. The simple view keeps the
same minimal hints without the `cancel` word.

The permanent-disable hint is printed before the released command, on stderr,
with an amber `⚠`, the bold phrase `review panel disabled`, and the re-enable
invocation `watn --review-panel`. Without color support the same text prints
without escape sequences. The released command stays the only stdout content.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | The change refines the existing review surface and its disable output. |

## Out of Scope

- No change to how the command flow is split or how unsupported syntax is
  tracked internally; only its visible marking changes.
- No change to accept, edit, reject, view toggle, or stage navigation behavior.
- No change to the flag-only persistence behavior.

## Open Questions

- None.
