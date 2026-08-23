# Proposal: .migrate-0-2-0-to-0-3-0.tmp-2487432

## Aggregate migration

This maintenance change migrates a project from Givn 0.2.0 to theinstalled Givn 0.3.0 contract. Managed preparation was recorded inCommit A `d319e3e6aa04884fa53864d7548408bebc9e7c5e`. The migration phases below are one ordered plan; completeeach phase before continuing to the next.

## Scope

The LLM must inspect project evidence and derive project-specific tasks. It mustpreserve project-owned overrides and must not invent domain behavior, tests,architecture decisions, or provenance.

## Bundle proposals

### Phase 1: migrate-0-2-0-to-0-3-0 (0.2.0 -> 0.3.0)

# Migration Proposal: 0.2.0 to 0.3.0

## Problem / Opportunity

A project created from Givn 0.2.0 does not yet contain the current 0.3.0
authoring, review, semantic-evidence, generated-guidance, and architecture
documentation contract. A config refresh alone cannot determine which
project-owned specifications and architecture decisions need attention.

## Proposed Solution

Use the ordered LLM migration prompts in the companion design. Inspect the
consumer project, preserve its ownership boundaries, update only applicable
artifacts, and complete the normal Givn review and archive gates. This bundle is
one phase in an aggregate migration plan when a project skips additional
versions.

## Out of Scope

- Inventing or changing domain behavior without project evidence.
- Automatically editing project specifications, source code, tests, ADRs, Arc42
  facts, ejected facets, or user-authored documentation.
- Adding a synthetic migration feature to the permanent specification tree.

## Completion

The LLM must derive project-specific tasks from the design after inventory and
must complete this phase before the next bundle phase in an aggregate plan.


