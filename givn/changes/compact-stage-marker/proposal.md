# Proposal: compact-stage-marker

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to clarify-review-card-guidance)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The stage note `⚠ nested syntax` still spends a word on something the card
does not need to explain in place. The stage already shows its exact text and
its model-written purpose underneath; the note only needs to signal that Watn
did not decompose this stage further. A word invites the wrong question ("is
this an error? is it unsupported by my shell?"), while a compact marker is
enough.

## Proposed Solution

Replace the worded stage note with a single amber `…` at the end of the stage
row, in the same position the note occupied. The italic purpose line below is
unchanged. The marker means "this stage contains nested shell constructs and is
shown as one unit", never an error or a safety verdict; the stage text remains
complete and reviewable.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND quicksetup` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The route ranking is generic text overlap; the marker is the existing review card. No new inventory action. |

## Out of Scope

- No change to which stages are marked, to the flow splitter, or to the stage
  and purpose text.
- No legend or help row; the marker is self-explanatory in position.
- No change to the amber color or the mono fallback.

## Open Questions

- None.
