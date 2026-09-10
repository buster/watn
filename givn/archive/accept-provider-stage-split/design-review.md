# Design Review: accept-provider-stage-split

## Review Scope

The review covered `proposal.md`, the change feature, `design.md`, the
permanent `use-shell` capability, and the affected Arc42 chapters.

## Findings And Dispositions

### Correctness

**Finding:** Trusting a provider split could let a model fabricate or reorder
stages.

**Disposition:** Resolved. Every stage text must occur verbatim in the command,
in order, without overlap; gaps are limited to whitespace and shell
separators; the final remainder must be whitespace. Otherwise nothing is
trusted.

**Finding:** A trusted split must not weaken the no-invented-purpose rule.

**Disposition:** Resolved. Purposes are copied from the provider response only
when every stage carries a non-empty purpose.

### Testability

**Finding:** Both trust and rejection need observable coverage.

**Disposition:** Resolved. Scenarios cover a real compound split (loop and
grouped pipes) and a fabricated stage; unit tests cover fabricated,
out-of-order, non-covering, and missing-purpose payloads.

### ADR Qualification

`NOT_QUALIFIED`. Local to the reviewed review-surface capability; ADR-0015
still owns the stream completion boundary.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 6 | Yes | Stage acceptance changes the runtime review flow. |
| 8 | Yes | Verbatim proof and no-invention rules extend. |
| 10 | Yes | QS-072 gains compound-split cases. |
| 11 | Yes | R-072 mitigation gains split-trust rules. |
| 1-5, 7, 9, 12 | No | No other chapter changes. |

## Hardening Receipt

- `proposal.md`: states the outcome without implementation detail.
- `specs/use-shell/interactive-shell-shortcut.feature`: two `@givn.added`
  scenarios.
- `design.md`: split contract, interfaces, failure outcomes, ADR and Arc42
  routing.
- `arc42.md` and affected durable chapters: updated.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
