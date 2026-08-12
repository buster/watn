# ADR-0019: Reviewed four-topic setup draft

- **Status:** accepted
- **Date:** 2026-08-12
- **Decision-makers:** watn maintainers

## Context and Problem Statement

The previous setup flow exposed implementation fields as seven pages and wrote
provider state before the user had reviewed model and shell choices. Reading an
absent config path also created a template, which made first-run cancellation
and first-run detection ambiguous. Credential discovery could be confused with
credential persistence and expose the wrong provenance to the user.

## Decision Drivers

- Make first-run onboarding deliberate even when an environment credential is present.
- Keep secrets out of renderer state and persisted discovery data.
- Give the user one clear review and persistence boundary.
- Preserve the existing supported TOML schema and shell marker ownership.

## Considered Options

- **Keep field pages and add warnings** - smaller renderer change, but still exposes implementation structure and leaves multiple save boundaries.
- **Use separate provider/model/shell commands** - simple local flows, but provenance and validation remain split across commands.
- **Use one four-topic draft with Finish-only persistence** - more runtime state, but one review boundary and clear cancellation semantics.

## Decision Outcome

Use one `SetupDraft` for Provider, Model roles, Shell integration, and Review.
The draft carries field origins, credential source kind, catalog status, role
review state, and shell intent. A physical config-path check returns an
existence signal without writing. Finish validates and commits the supported
configuration once through a secure atomic writer; shell marker reconciliation
then runs independently and reports partial failures. First-run completion
prints retry guidance and never replays the original request.

## Consequences

### Good

- Existing configuration can remain byte-for-byte unchanged on cancellation.
- Detected credentials are represented by names and presence only.
- Provider edits cannot silently pass stale model-role assignments.
- The review page makes warnings, shell changes, and persisted values visible.

### Bad

- The wizard has more runtime state and a larger renderer than the old linear flow.
- Shell changes happen after config commit, so a shell failure can produce a saved partial result that needs retry.
- Removing focused commands and overrides requires completion and documentation migration.

## Confirmation

The setup-refactoring feature covers first-run precedence, no-secret leakage,
Finish-only writes, cancellation preservation, role invalidation, manual catalog
fallback, responsive help, CLI removal, and partial shell outcomes. Unit tests
cover read/commit and discovery boundaries; PTY tests cover the four topics and
wide/narrow help layouts.
