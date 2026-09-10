# Proposal: recover-review-stage-purposes

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to fix-review-response-fallback)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Providers do not always emit the exact review status vocabulary: a model may
answer `incomplete` or another word instead of the contracted status, and it
may format a long command across several lines. When either happens, the
review surface recovers the command but hides the stage purposes the provider
actually wrote. The developer sees `purpose-unavailable` even though usable
model-written explanations were present.

## Proposed Solution

- The reviewed command is a single line: line breaks and repeated whitespace in
  a provider-written command are normalized before the command flow and the
  candidate are shown, so stage boundaries are stable.
- When a provider response is not a canonical structured response, its
  model-written stage purposes are still shown if, and only if, the response's
  stage text matches the locally derived stages and every stage carries a
  non-empty purpose.
- An unknown purpose-status word alone no longer hides otherwise matching
  purposes.
- When stage text does not match, or a purpose is missing, the surface shows
  `purpose-unavailable` exactly as today. Watn never invents purpose text.
- All other review behavior is unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | No new interaction; this strengthens the existing review purpose contract in `use-shell`. |

## Out of Scope

- No new interaction or command-surface change.
- No change to disabled or non-review behavior.
- No semantic command-risk validation.
- No change to the canonical structured response schema.

## Open Questions

- None.
