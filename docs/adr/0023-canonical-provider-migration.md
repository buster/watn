# ADR-0023: Canonical provider migration

- **Status:** accepted
- **Date:** 2026-08-13
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

Existing configurations may use arbitrary provider keys while interactive setup
offers fixed OpenRouter, OpenAI, and Custom choices. Leaving arbitrary selected
keys in place makes the result ambiguous and prevents a stable setup model.

## Decision Drivers

- Canonicalize selected setup results.
- Avoid losing provider defaults or unrelated entries.
- Keep saved credential representations authoritative.

## Considered Options

- **Preserve arbitrary selected keys** - avoids migration but leaves multiple
  identities for the same custom endpoint.
- **Migrate the selected arbitrary key to `custom`** - gives setup one stable
  destination with explicit collision behavior.

## Decision Outcome

The selected arbitrary provider entry migrates to `custom` at successful final
confirmation and the old selected key is removed. If `custom` already exists,
the selected entry wins for endpoint, credential source, and provider-local
catalog state. The destination default model is retained when present; the
source default model is carried only when the destination has none. Unrelated
providers and config fields remain unchanged. A saved literal or environment
reference is never replaced by fallback discovery.

## Consequences

### Good

- Setup produces stable provider names and idempotent reruns.
- Collision and default-model behavior are deterministic.

### Bad

- The selected arbitrary key is removed, which is a visible configuration
  migration.
- A user who intended to retain an arbitrary key must not use this setup path.

## Confirmation

Feature scenarios assert source-key removal, collision default preservation,
unrelated-provider preservation, credential-source authority, and idempotence.
