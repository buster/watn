# ADR-0008: Template config generated from code

**Status:** Accepted

**Date:** 2026-08-07

**Decision makers:** Project team

## Context

The tool needs to write a template config file on first run. Two approaches:

1. **Hardcoded string** — a multi-line string literal containing all TOML with comment markers
2. **Generated from code** — serialize a `Config` struct instance and comment every line programmatically

## Options considered

| Option | Pros | Cons |
|---|---|---|
| Hardcoded string | Simple to write; total control over formatting and comments | Manual sync — adding a field to `Config` requires also updating the template string; easy to forget |
| Generated from code | Self-synchronizing — any new field on `Config` automatically appears in the template | Requires a `comment_toml` helper; template formatting is less flexible |

## Decision

Generate the template from code. The `Config::template_content()` method constructs a `Config` with example values, serializes it to TOML, and passes every line through a `comment_toml()` helper that prepends `# ` to each line.

**Reason:** eliminates the manual sync failure mode. A hardcoded string would diverge from `Config` as the struct evolves.

## Consequences

- **Good:** Adding a new config field automatically includes it in the template
- **Good:** The template always reflects the actual config structure
- **Bad (R-008):** The generated template may include fields the user does not need or understand, if the `Config` struct grows many optional sections
- **Bad:** Formatting control is limited — we rely on the TOML serializer's output order and layout

## Confirmation

Template content verified in tests: `cargo test` includes "config file exists at the standard XDG path" and "contains a commented-out defaults section" assertions.

## Pros/Cons

### Pros
- Self-synchronizing with Config struct
- No manual update step for new fields
- Testable — template content is generated deterministicly

### Cons
- Serializer controls formatting, not us
- Every config field visible even if user rarely needs it
