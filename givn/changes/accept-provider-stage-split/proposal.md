# Proposal: accept-provider-stage-split

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to recover-review-stage-purposes)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Providers often group command stages differently from Watn's local split: a
compound shell construct (`while ... do ... done`) or a pair of adjacent pipes
(`sort | uniq`) can be one provider stage. When that happens, the provider's
stage text does not match the locally derived stages, so the review surface
drops purposes the provider actually wrote and shows `purpose-unavailable`.

## Proposed Solution

- When a provider response carries stages whose text is an exact, ordered,
  non-overlapping slice of the command and the slices cover the command, the
  review surface shows that split as the command flow together with the
  model-written stage purposes.
- Between slices only whitespace and shell separators may appear; a stage that
  is not part of the command, out of order, overlapping, or missing a purpose
  is not trusted.
- When the provider split is not trusted, the surface falls back to
  `purpose-unavailable` as today. Watn never invents stage or purpose text.
- All other review behavior is unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | No new interaction; this refines the existing review flow contract in `use-shell`. |

## Out of Scope

- No new interaction or command-surface change.
- No change to disabled or non-review behavior.
- No semantic command-risk validation.
- No change to the canonical structured response schema.

## Open Questions

- None.
