# Proposal: render-review-card

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to accept-provider-stage-split)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review surface renders as undifferentiated text: labels, command, stages,
purposes, actions, and key hints all share one style. The developer cannot
quickly tell what the command is, which stage is selected, what each stage
means, or what can be done. The surface works but does not make the review
decision easy or pleasant.

## Proposed Solution

An enhanced review card becomes the default presentation when the review
surface is eligible:

- A framed card with a header showing the product, the selected position, and
  the tier/provider/model context.
- The intent and the command are visually separated; the command is syntax
  colored (plain text, flags, quoted strings, separators) and wrapped.
- One command-flow stage is shown at a time with its model-written purpose; a
  compact stage position shows where the developer is in the flow, and
  unsupported syntax is marked visibly.
- The four review actions appear as distinct actions with the selected one
  highlighted, followed by a key-hint line.
- The plain review panel remains the fallback when the card is disabled, when
  the terminal cannot render color, or when an enhanced renderer fails.

The card is optional and enabled by default:

- Persisted setting `[review] enhanced` (default enabled).
- Per-invocation overrides `--enhanced-review-panel` and
  `--no-enhanced-review-panel`.
- A terminal without color support (`NO_COLOR`, `TERM=dumb`, or no color
  capability) falls back to the plain panel automatically.
- Disabling review entirely keeps the existing `[review] panel` precedence.

Everything else is unchanged: buffering until completion, explicit acceptance
as the only release, cancellation and failure behavior, disabled and
non-review routing, and the structured response contract.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND credential-sources` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The confirmed permanent owner is `use-shell`; this is presentation of the existing review interaction, not credential handling. |

## Out of Scope

- No change to the structured response schema or provider contract.
- No new interaction and no new end-to-end scenario.
- No alternate-screen or full-window interface.
- No semantic command-risk classification or locally invented labels.
- No change to disabled, non-review, or non-terminal behavior.

## Open Questions

- None.
