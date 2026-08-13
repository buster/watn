# ADR-0020: Final-confirmation setup snapshots

- **Status:** accepted
- **Date:** 2026-08-13
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

The shared setup flow combines provider, catalog, model, reasoning, and shell
choices. Saving one part before the remaining choices are confirmed can leave a
configuration that the user did not approve.

## Decision Drivers

- Cancellation must be safe on both existing and first-run configuration.
- A failed catalog probe must not create a partial provider configuration.
- Unrelated configuration must survive focused-command saves.

## Considered Options

- **Save after each page** - preserves partial progress but makes cancellation
  and failure observable as unintended writes.
- **Snapshot until final confirmation** - keeps the baseline unchanged and
  permits one complete candidate write.

## Decision Outcome

Coordinated `watn setup` keeps the complete draft in memory until final review
confirmation. It writes one atomic configuration snapshot. Focused provider and
model commands save only their owned domain after their own confirmation. Shell
target operations run after a successful configuration write and remain
independent of one another.

## Consequences

### Good

- Cancellation and draft failure leave the baseline unchanged.
- First-run cancellation leaves no configuration file.
- One confirmed snapshot preserves cross-domain consistency.

### Bad

- A user must repeat unconfirmed setup after cancellation or failure.
- Configuration and shell files cannot be rolled back as one transaction.

## Confirmation

Gherkin scenarios assert byte-for-byte preservation before confirmation,
unchanged files after cancellation and failed writes, and successful shell
changes retained after later target failure.
