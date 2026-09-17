# Proposal: migrate-0-7-0-to-0-8-0

## Aggregate migration

This maintenance change migrates a project from Givn 0.7.0 to theinstalled Givn 0.8.0 contract. Managed preparation was recorded inCommit A `7d1d3988ace0394cb1fb29b9dfa06d9729143186`. The migration phases below are one ordered plan; workeach phase in order and follow the completion rules in its bundle.

## Scope

The LLM must inspect project evidence and derive project-specific tasks. It mustpreserve project-owned overrides and must not invent domain behavior, tests,architecture decisions, or provenance.

## Bundle proposals

### Phase 1: migrate-0-7-0-to-0-8-0 (0.7.0 -> 0.8.0)

# Migration Proposal: 0.7.0 to 0.8.0

## Problem / Opportunity

Givn 0.8.0 hardens the model behind the artifacts your project
already owns:

- **Use cases** gain a canonical definition and eight scoping rules. Existing
  documents may be mega-leaves: a goal that joins several goals, a summary
  that owns capabilities directly, a leaf whose flow names three or more
  actor goals, an observing case that owns mutations, or a capability whose
  name and placement fail the model.
- **Personas** gain identity criteria and a stable-core/running-record
  schema. Existing files may be one role sliced by mechanism, frozen topic
  copies, or dangling references.
- **ADRs** gain three falsification tests and a cheaper-home challenge.
  Existing records may pass the old gate only by stretching "durable
  consequence".
- Dialog policy, change-seed statuses, term records, the guarantee qualifier,
  and the decision ledger are new adoptions.

`givn upgrade` refreshed your managed config and guidance and created this
maintenance change from the release-owned bundle. The remaining work is
project-specific: your use-case documents, personas, and ADRs are yours, and
no runtime gate rewrites them.

## Proposed Solution

This maintenance change is the **inventory and plan**, not the repair site.
Work the bundle's `design.md` phases in order and record one row per artifact
deviation with its disposition and the follow-up change that will fix it. The
repairs themselves run as normal changes — each with its own specification
deltas, review, and archive — so a split, merge, or removal passes the same
gates as any other work.

The settled rules:

1. **Full retrofit.** Nothing is grandfathered; every permanent use case,
   persona, and accepted ADR is brought into the model.
2. **Inventory and plan.** This change records every deviation and its
   follow-up; it edits no corpus artifact except the term collision-check
   cells.
3. **Archive timing.** This change archives once every deviation names a
   follow-up change (or an in-flight one); the inventory is the durable
   record, and the migration is complete when the follow-ups archive.
4. **ADR scope.** Accepted records are re-qualified here; a `proposed` record
   is listed against the owning change that must qualify it before that
   change archives. An archived change is never named as a pending
   follow-up.
5. **Non-qualifying ADRs.** The rationale moves once to one canonical home,
   the record leaves the register and both indexes, and this change keeps the
   audit note (ID, failing test, destination). It is never copied into
   `docs/arc42/adr/archive/`.
6. **Renames.** Component- and channel-shaped capability and use-case names
   are renamed in the follow-up, which moves the `.feature` file and updates
   every reference; one use case per follow-up change.
7. **Term evidence.** Every existing `confirmed` term gets its search re-run
   and the commands and hit counts recorded in the record; a collision found
   becomes a follow-up that renames or rejects the term.

## Out of Scope

- Performing the repairs in this change: splits, merges, removals, and
  renames run as the named follow-up changes.
- Changing product behavior, specifications, or test scenarios here.
- Deleting artifacts: a non-qualifying ADR's rationale moves once to its
  canonical home; the record leaves the register but the migration records
  where it went.
- Re-publishing legacy archives or backfilling verification receipts.

## Completion Boundary

This change is complete when every permanent use case, persona, accepted
ADR, term record, R8 name, seed, and ejected override has an inventory row
with a disposition and a named or in-flight follow-up change, and the
collision-check cells carry their re-run evidence. The migration itself is
complete when every follow-up change has archived through the normal gates.


