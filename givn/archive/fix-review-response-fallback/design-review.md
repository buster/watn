# Design Review: fix-review-response-fallback

## Review Scope

The review covered `proposal.md`, the change feature, `design.md`, the
permanent `use-shell` use case and `interactive-shell-shortcut` capability, and
the affected Arc42 chapters. The confirmed Persona
`terminal-developer--interactive` remains a review lens.

## Findings And Dispositions

### Scope

**Finding:** Provider payloads can carry markdown fences or surrounding prose
and still contain a valid structured response.

**Disposition:** Resolved. The response locator extracts a fenced block or the
outermost JSON object before strict validation.

**Finding:** A JSON-shaped payload with an unsupported status or malformed
structure previously became the command verbatim.

**Disposition:** Resolved. Command recovery builds the Candidate from the
provider-written `command` field with `purpose-unavailable`. Watn does not
invent purpose text. A payload without a usable command stays `Unavailable` and
releases nothing.

### Testability And E2E Fidelity

**Finding:** Recovery behaviour must be observable through the review surface,
not only through unit tests.

**Disposition:** Resolved. Scenarios assert rendered stage text, recovered
commands, purpose-unavailable status, no-release, and row bounds through the
existing deterministic review surface and terminal writer. No interaction is
added, so the E2E inventory is unchanged.

### Rendering Safety

**Finding:** Embedded line breaks in provider text break the bounded inline
panel's row accounting.

**Disposition:** Resolved. Sanitization flattens line feed, carriage return,
and tab to a space; a scenario asserts every rendered value stays on one row
and the panel stays bounded.

### Risk

**Finding:** Tolerant recovery must not weaken the no-invented-purpose rule.

**Disposition:** Resolved. Recovery only selects provider-written text.
Purposes come exclusively from a strictly validated structured response.

### ADR Qualification

The recovery and sanitization decisions are `NOT_QUALIFIED` for a new ADR.
They are local to the reviewed review surface, have no deployment or provider
boundary impact, and are fully owned by this design, the Gherkin scenarios, and
the affected Arc42 chapters. ADR-0015 remains the owner of the stream
completion boundary; no amendment is needed.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 6 | Yes | Runtime review flow gains the recovery branch and the no-command outcome. |
| 8 | Yes | Crosscutting sanitization flattens row-breaking characters. |
| 10 | Yes | QS-072 gains fenced, prose-wrapped, and command-recovery cases. |
| 11 | Yes | R-072 mitigation gains payload-shape tolerance and command recovery. |
| 1-5, 7, 9, 12 | No | No goal, constraint, context, strategy, building block, deployment, decision, or glossary change. |

## Hardening Receipt

- `proposal.md`: states the recovery and rendering-safety outcomes without
  implementation detail.
- `specs/use-shell/interactive-shell-shortcut.feature`: four `@givn.added`
  scenarios covering fenced prose, invalid payload recovery, no-command
  release, and multiline row safety.
- `design.md`: defines the five-way payload classification, the flattening
  rule, interfaces, failure outcomes, and ADR/Arc42 routing.
- `arc42.md` and the affected durable chapters: updated with the recovery branch
  and sanitization rule.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
