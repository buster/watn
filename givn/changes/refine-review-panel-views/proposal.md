# Proposal: refine-review-panel-views

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review surface opens in its dense presentation for every candidate: Intent,
a command line truncated after two rows, flow strip, stage row, purpose, six
decisions, and a header carrying tier and provider. A developer who only wants
to glance at the command and accept it first has to parse all of it. The full
candidate is not even visible when the command is long or composed of several
parts, and the stage purpose is low-contrast text that is easy to miss.

The detailed presentation also never tells the developer that the stage stack is
navigable with the arrow keys, and no presentation names the configured model
without its provider prefix or tier.

Disabling the review has two gaps. It discards the candidate the developer was
about to accept instead of releasing it, and the only stated re-enable path
(`watn --review-panel`) fails with a usage error when it is run without a
question, so the developer cannot re-enable the review from the command line.

## Proposed Solution

The review surface has two presentations of the same candidate: a simple view
that opens by default and a detailed view.

The simple view shows:

- The model short name in the frame: the last `/`-separated identifier segment
  of the configured model, without provider or tier.
- The command flow as a stack: every stage on its own row, with the separator
  that joined it to the next stage (pipe, `&&`, `;`) shown at the end of the
  preceding stage in an accent color. A stage that is longer than the row
  continues on the following rows.
- The selected stage marked with an amber arrow.
- The selected stage's purpose below the stack, preceded by a cyan `↳` marker
  and rendered in a brighter style than the former dim gray; when no purpose is
  available, its status is shown in the same place.
- Up and down change the selected stage and the purpose follows.
- Enter accepts the candidate, Escape cancels the review, and `d` or `?`
  switches to the detailed view.

The detailed view keeps the existing content (Intent, flow position, stage
position, edit and reject decisions, model context) and adds the same stage
stack, selected-stage arrow, and purpose below the stack. Up and down navigate
the stages here as well, and its shortcut row names that navigation, `d`/`?`
returns to the simple view, and `D` permanently disables the review.

Pressing `D` ends watn, writes the current candidate to the command-output
channel, and prints a re-enable instruction naming `watn --review-panel` on
stderr. Running `watn --review-panel` or `watn --no-review-panel` without a
question persists the setting and exits successfully instead of failing with a
usage error.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | The change refines the existing review surface owned by this capability. |

## Out of Scope

- No change to how the command flow is split, how stages and purposes are
  derived, or how unsupported syntax is marked.
- No change to the model chooser, the command editor, regeneration, escalation,
  or rephrasing behavior.
- No change to the persisted configuration format or provider settings.
- No semantic safety verdict for commands.
- No change to the plain non-review output path beyond the re-enable invocation
  without a question.

## Open Questions

- None.
