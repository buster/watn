# ADR-0025: Repository-wide specification ownership

Status: Accepted

Date: 2026-08-15

## Context

The permanent Gherkin tree accumulated exact title duplicates, subset
scenarios, and repeated assertions across feature families. A green runner
does not show whether each scenario contributes an independent contract.
Change-local reviews cannot see ownership conflicts elsewhere in the tree.

## Decision Drivers

- Preserve independent production-boundary coverage while removing redundant
  specification contracts.
- Make repository-wide ownership findings deterministic and reviewable before
  archive.
- Keep archive mutation atomic and preserve the existing rollback boundary.
- Avoid making embedding retrieval or heuristic similarity the source of truth.

## Considered Options

- Keep the additive tree and rely on local feature reviews; this leaves exact
  duplicates and subset contracts undiscovered across capability boundaries.
- Introduce anchors, a registry, and compare-and-swap journal semantics for
  every scenario mutation; this adds a second ownership system without being
  necessary for the current consolidation need.
- Treat the active tree as one behavior inventory, surface deterministic
  ownership findings, and record semantic decisions in review dispositions.

## Decision Outcome

Treat the active permanent specification tree as one behavior inventory. A
scenario title must be unique across that tree. Deterministic title, shape, and
subset findings are surfaced before archive. The maintainer records the
disposition in the change review and removes weaker scenarios when a stronger
canonical contract already exists. Distinct production boundaries remain
separate and are named to make the distinction observable.

Supersession uses a removed scenario plus an added replacement in one change;
the existing archive transaction and net-delta receipt remain the source of
truth. Scenario length remains a warning/disposition concern, not an automatic
hard failure. Embedding retrieval is advisory evidence only and never replaces
the deterministic gate.

## Consequences

### Good

- Duplicate behavior cannot silently accumulate in separate feature families.
- The permanent suite becomes smaller without changing Watn runtime behavior.
- Every removal has a retained-contract or boundary explanation.
- The existing archive rollback and Gherkin runner remain the only merge and
  execution boundaries.

### Bad

- Consolidation requires repository-wide review rather than a local feature
  edit.
- A maintainer must distinguish a genuine production boundary from a weak
  duplicate; deterministic checks cannot make that semantic decision.
- Removing a scenario may orphan a step binding, so binding usage must be
  scanned before deleting test support.

## Confirmation

The `watn-consolidation` change removes the evidence-backed duplicate/subset
scenarios, records review dispositions, archives the delta, and runs the full
permanent and E2E suites afterward.
