# Handoff: terminal-wow-factor

## Status

Handoff complete. Mapping, persona promotion, and durable-term promotion are
confirmed.

## Use-case mapping

| Ideation use-case ID | Permanent owner | Future change seed | Status |
|---|---|---|---|
| `use-explanatory-shell-shortcut` | `use-shell` | `changes/use-explanatory-shell-shortcut.md` | Ready for `/givn-propose` |

`review-command-candidate` and `route-reviewed-command` remain subfunctions of
the same change-sized user goal. They do not become separate permanent use-case
roots.

## Confirmed persona

- `terminal-developer--interactive`

Promotion to `givn/personas/` is confirmed.

## Candidate durable terms

- `Candidate`: selectable generated or directly edited command in the current
  review.
- `Command flow`: visible syntactic structure of a candidate.
- `Review surface`: transient terminal presentation for reviewing a proposal.
- `Review decision`: explicit developer action during review.
- `Presentation adapter`: terminal-specific renderer of the review surface.

Promotion into `docs/arc42/12-glossary.md` is confirmed.

## Parked Design questions

- Review result transport separate from command stdout.
- Controlling-terminal and shell-specific redraw mechanics.
- Renderer selection and fallback adapters.
- Provider response shape for adaptive explanations.
- Exact command-flow grammar and unsupported syntax behavior.

## Handoff rule

This handoff does not create a Givn change, proposal, specification, design, or
code. After all handoff decisions are confirmed, the user starts the normal
proposal flow for `use-explanatory-shell-shortcut`.
