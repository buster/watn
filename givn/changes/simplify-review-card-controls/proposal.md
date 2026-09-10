# Proposal: simplify-review-card-controls

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to render-review-card and review-panel-controls)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review card asks for more ceremony than the decisions deserve:

- The surface opens with an Actions region selected, so the developer must Tab
  or arrow through a Focus strip before reading the stage explanations that are
  the point of the card.
- The `Candidates` region appears to do nothing: a review normally holds one
  candidate, so selecting among candidates is invisible.
- The Actions region is only a cursor over four decisions; Enter on it is the
  only way to choose one, even though the decisions are few and stable.
- Reject is a dead end. Pressing it leaves the card in place and changes
  nothing: no candidate is released, and no alternative is offered. The
  developer who dislikes a candidate has no next step inside the card.
- The command editor only accepts characters at the end. Arrow keys, Home, and
  End do not move the insertion point, so fixing a token in the middle means
  retyping the tail.

## Proposed Solution

**Flow-first card.** When the review card opens, the command flow is the active
element. Left and right move between the stage explanations and their purposes.
Enter accepts the current candidate, which is the card's default action.

**Direct decisions.** The Focus strip, the Actions region, and the
`Candidates` region disappear from the card and from the review state. The
decisions become single-key shortcuts, readable from the card's hint line:

- `a` accept the current candidate.
- `e` open the command editor.
- `r` reject the current candidate.
- `c` cancel the review.
- `d` permanently disable the review surface (existing).
- Escape cancels the review (existing).

Shortcuts are accepted in either letter case. The card shows the decisions with
accept emphasized as the default.

**Reject offers another model.** Rejecting opens a model chooser inside the
review surface. The chooser lists the configured model tiers for fast selection
by number, and it has a text field for any other model. While the developer
types, the installed provider's catalog supplies matching model suggestions;
a suggestion is selected with the arrows and Enter, or the typed text is used
as entered. Choosing a tier or a model generates a new candidate for the same
intent with that choice and returns to the review card. Suggestions load
without blocking the chooser: the tiers and the text field are usable
immediately, and a catalog that is unavailable or slow only means no
suggestions. If generation fails, the chosen candidate is discarded, the
previous candidate and review state stay in place, and the card reports the
failure. Leaving the chooser without a choice returns to the unchanged
candidate. The explanation-only card is unaffected: its decisions stay
"close".

**Cursor editing.** While editing a candidate, left and right move the
insertion point, Home and End jump to the start and end of the command, and
Backspace and Delete remove the character before and at the insertion point.
Typing inserts at the insertion point. Enter commits the edit, Escape discards
it, and the original intent is never changed.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND quicksetup` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The route ranking is driven by generic text overlap ("route", "selection"); the card, its decisions, and candidate generation are the existing use-shell review interaction. One new inventory action is added: rejecting a candidate and regenerating with another model, because it is the only review decision that issues a second provider request and replaces the candidate. |

## Out of Scope

- No change to the structured review response, the candidate buffer, or the
  acceptance-as-release rule.
- No change to rephrasing, higher-tier escalation, or the disabled-review
  behavior.
- Candidate retention and comparison are removed, not redesigned: one candidate
  is reviewed at a time.
- No persistent per-candidate history across reviews.

## Open Questions

- None. The rejected candidate is replaced, not retained; the chooser covers
  the "try another model" case.
