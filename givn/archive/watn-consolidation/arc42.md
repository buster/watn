# arc42 Impact Assessment: watn-consolidation

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals | Yes | Specification maintainability and canonical behavior ownership are new quality goals. |
| 2 | Architecture Constraints | Yes | The permanent scenario tree gains a repository-wide title-ownership constraint. |
| 3 | Context and Scope | Yes | The maintainer/givn review and archive workflow is a new repository-facing interaction. |
| 4 | Solution Strategy | Yes | Consolidation chooses deletion and explicit boundary dispositions instead of additive coverage or score blocking. |
| 5 | Building-Block View | Yes | Specification ownership and consolidation evidence become documented workflow building blocks. |
| 6 | Runtime View | Yes | Review, disposition, archive, rollback, and post-archive runner flows are specified. |
| 7 | Deployment View | No | No Watn executable, runtime service, release artifact, or deployment topology changes. |
| 8 | Crosscutting Concepts | Yes | Ownership, disposition, atomic archive, and no-live-provider test semantics are cross-cutting workflow rules. |
| 9 | Architecture Decisions | Yes | ADR-0025 records repository-wide scenario ownership and consolidation policy. |
| 10 | Quality Requirements | Yes | Duplicate-free ownership, retained contracts, and post-archive green-suite scenarios are added. |
| 11 | Risks and Technical Debt | Yes | Deletion risk, orphaned bindings, and historical overlap debt require explicit mitigation. |
| 12 | Glossary | Yes | Scenario ownership, canonical scenario, consolidation disposition, and net delta become domain terms. |

## Chapter 9 Decision Summary

### ADR-0025: Repository-wide specification ownership

- **Context:** The active Gherkin tree contains exact title duplicates,
  assertion subsets, and repeated boundary coverage across feature families.
- **Decision:** Treat the active tree as one behavior inventory; enforce
  repository-wide title ownership, surface deterministic overlap findings, and
  record human merge/delete/boundary dispositions before archive.
- **Consequences:** We remove weaker scenarios when a stronger canonical
  contract exists, retain genuinely distinct production boundaries, and use
  removed-plus-added deltas for atomic replacements. The Watn runtime and
  deployment artifact remain unchanged.
- **Confirmation:** The F1-F6 removal matrix, review dispositions, archive
  receipt, and complete post-archive runner confirm the decision.

The durable full MADR is maintained in
`docs/arc42/09-architecture-decisions.md` and
`docs/adr/0025-repository-wide-specification-ownership.md`.

## Status

STATUS: DONE
