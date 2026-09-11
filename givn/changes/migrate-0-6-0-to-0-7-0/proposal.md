# Proposal: migrate-0-6-0-to-0-7-0

## Aggregate migration

This maintenance change migrates a project from Givn 0.6.0 to theinstalled Givn 0.7.0 contract. Managed preparation was recorded inCommit A `29b032cc1d6c10b0e87c4e05fd286dfdd4777acb`. The migration phases below are one ordered plan; completeeach phase before continuing to the next.

## Scope

The LLM must inspect project evidence and derive project-specific tasks. It mustpreserve project-owned overrides and must not invent domain behavior, tests,architecture decisions, or provenance.

## Bundle proposals

### Phase 1: migrate-0-6-0-to-0-7-0 (0.6.0 -> 0.7.0)

# Migration Proposal: 0.6.0 to 0.7.0

## Problem / Opportunity

Givn 0.7.0 changes how a completed change is published. `givn archive` is now
forward-only and Git-backed:

- The project must be a Git worktree whose root is the project root, and the
  worktree must be committed before the archive starts.
- Every verification scope (the regular suite and the E2E suite) must write a
  machine-readable result to the path in `GIVN_RESULT_FILE`; a missing,
  malformed, failed, skipped, zero, or non-reconciling result blocks proof.
- `givn check review` no longer runs the test suite; the archive verification
  receipt (`givn/archive/<id>/verification.json`) is the execution evidence.

`givn upgrade` applies the machine-safe part automatically (managed config and
guidance refresh, `givn_config_version` bump, and this maintenance change).
The remaining work is project-specific: the project owns its runner scripts,
and a 0.6.0 project that upgrades without updating them will fail its next
archive closed at the result contract.

## Proposed Solution

Migrate the project's verification commands to the result contract and confirm
the Git publication boundary:

1. Inventory `verify.command` and `verify.e2e_command` from
   `givn/commands.yaml` and the scripts they invoke.
2. Make each scope emit a validated result to `GIVN_RESULT_FILE` with counts
   taken from the runner, never hardcoded.
3. Confirm the project is a Git worktree rooted at the project root and commit
   the migration work.
4. Validate both scopes manually, then prove the contract through the next
   archive.

The complete, self-contained procedure is in this bundle's `design.md`.

## Out of Scope

- Changing product behavior, specifications, or test scenarios.
- Backfilling receipts for archives published before 0.7.0; they remain
  historical and are never re-published.
- Restructuring a repository to make the project the Git root without a human
  decision.

## Completion Boundary

The migration is complete when both verification scopes write a reconciling
result under `GIVN_RESULT_FILE`, the project is a committed Git worktree at its
root, and the next archived change produces a proven verification receipt.


