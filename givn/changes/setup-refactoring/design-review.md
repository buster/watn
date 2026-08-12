# Design Review: Setup Refactoring

## Grilling

### Scope

The proposal, feature, and design describe the same product boundary: one
four-topic setup flow, physical-path first-run detection, presence-only
credential discovery, manual model fallback, Finish-only persistence, and
post-commit shell reconciliation. The review found that the original design
needed explicit runtime states for manual roles, shell intents, OpenAI mapping,
and the catalog source. Those decisions were added to `design.md` and the
feature now includes focused OpenAI, semantic-preservation, LiteLLM, and Ctrl-C
coverage.

### Technology choices

Rust, Ratatui, crossterm, blocking reqwest for catalog requests, and the
existing portable-PTY/httpmock seams remain appropriate. Catalog discovery is
run by a worker so the event loop can process Ctrl-C and keep rendering while a
provider responds. The secure writer uses standard-library temporary files and
atomic rename rather than adding a persistence dependency.

### Missing scenarios

The review added scenarios for explicit OpenAI identity and aliases, preservation
of supported configuration fields, independent LiteLLM catalog routing, and
Ctrl-C during delayed catalog discovery. Marker inspection/removal and partial
shell failure remain covered by the existing active scenarios and the shell
reconciliation unit seam.

### Testability

Non-interactive contracts use isolated XDG directories and subprocesses. Draft
and discovery behavior is pure or unit-testable without a terminal. PTY tests
use explicit terminal dimensions; the renderer uses 100 columns as the wide
layout threshold and retains all help sections when stacking on narrow layouts.
Secret assertions inspect UI/diagnostic/config boundaries rather than resolved
credential values. The Cucumber builder uses `.fail_on_skipped()` and the
configured normal and E2E commands use distinct tag filters.

### Risk

The highest implementation risk is leaving an old checkpoint save or old CLI
overlay behind the new draft. The implementation removes those paths, builds a
complete cloned config at Finish, and keeps shell side effects after the single
config commit. The resulting shell failure is reported as a saved partial
outcome rather than hidden.

### Arc42

The independent 12-row assessment agrees that rows 1 through 6 and 8 through 12
are affected. Row 7 is also affected because shell startup files are a per-user
deployment integration, even though binary and service topology do not change.
All twelve chapter files contain substantive content, all diagrams remain
Mermaid, ADR-0019 records the new decision, and chapter 11 records its bad
consequences and mitigations.

## Hardening Applied

- Added typed role review states and explicit manual-role reasoning behavior.
- Closed OpenRouter/OpenAI/Custom endpoint, provider-name, and credential alias mapping.
- Centralized configured LiteLLM catalog-source precedence for setup discovery.
- Defined per-shell `EnsurePresent`, `EnsureAbsent`, `Unchanged`, and `NeedsAttention` intent states.
- Defined 100-column wide layout behavior and 120x40/80x40 verification dimensions.
- Reconciled durable Arc42 chapters and added ADR-0019.
- Added focused unit coverage for secret-safe provider debug output and marker removal.

## Design Review Outcome

All grilling branches are resolved by repository evidence or recorded design
decisions. No scope, technology, scenario, testability, or documentation issue
remains open.

DESIGN-REVIEW: PASS
