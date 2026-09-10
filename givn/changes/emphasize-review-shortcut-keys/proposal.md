# Proposal: emphasize-review-shortcut-keys

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to simplify-review-card-controls)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review card's hint line renders the decision keys (`⏎/a`, `e`, `r`, `c`,
`d`, `esc`) in the same dim style as the words they act on, so the keys do not
stand out while scanning. The developer reads "edit", "reject", or "cancel"
before noticing which key triggers them.

## Proposed Solution

Each decision shortcut key is drawn as the emphasized letter inside its label
word: the `a` in "accept" (green, the card's default), and the `e` in "edit",
`r` in "reject", `c` in "cancel", and `d` in "disable" (label color, bold).
The rest of the word stays dim, so the hint reads "accept · edit · reject ·
cancel · disable" with the acting letter standing out instead of a duplicated
key token. Enter and Escape stay emphasized key tokens (`⏎`, `esc`). The
command-editor hints (`⏎ commit`, `esc discard`) and the model-chooser hints
(`1-3 tier`, `⏎ choose`, `esc close`) keep their key tokens emphasized with dim
labels.

Every shortcut, hint, and interaction behaves exactly as before; only the
styling and the removal of the duplicated letter tokens change. On a terminal without color the keys are drawn with the same
monochrome text as today.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND quicksetup` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The route ranking is driven by generic text overlap; the card hint styling is part of the existing use-shell review card. No new inventory action, no behavior change. |

## Out of Scope

- No change to the key map, the decisions, the hint wording, or the card
  layout.
- The explanation-only card keeps its single dim `esc close` hint; it offers no
  decision keys to emphasize.
- No new interaction inventory entry; this is presentation polish of an
  existing interaction.

## Open Questions

- None.
