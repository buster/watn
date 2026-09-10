# Proposal: review-panel-controls

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to render-review-card)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review surface still carries two presentations: the enhanced card and the
old plain panel. The plain panel is no longer wanted; the card is the review
surface. At the same time the developer needs fast control over the surface:

- A quick way to turn the review surface off and on again.
- A way to turn it off permanently from inside the surface, without editing the
  configuration file by hand.
- When `-x` asks for execution confirmation, the developer wants to open the
  explanation for the command on demand instead of being forced through review
  every time.

## Proposed Solution

- The review card is the only review surface. A terminal without color renders
  the same card without color; there is no second panel.
- The review surface remains enabled by default and can be disabled per
  invocation with `--review-panel` / `--no-review-panel` and persistently with
  `[review] panel`.
- Inside the surface, a `d` decision permanently disables the review surface by
  writing `[review] panel = false` to the configuration, reports that review is
  disabled, and closes the surface while preserving the original input.
- On an interactive `-x` confirmation, the prompt accepts `?` in addition to
  yes/no: `?` opens the review card for the generated command as an
  explanation only, and closing it returns to the confirmation. Execution still
  requires the confirmation answer.
- The `[review] enhanced` setting and the `--enhanced-review-panel` /
  `--no-enhanced-review-panel` flags are removed; there is nothing to choose.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | The card, its decisions, and the confirmation option remain the existing review interaction; no new inventory action. |

## Out of Scope

- No change to the structured response contract or the card layout.
- No new interaction inventory entry; `?` is a variant of the existing
  interactive `-x` review action.
- No change to buffering, acceptance-as-release, or non-terminal behavior.

## Open Questions

- None.
