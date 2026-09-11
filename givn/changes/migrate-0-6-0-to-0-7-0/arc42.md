# arc42 Documentation Update: migrate-0-6-0-to-0-7-0

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No product goals, stakeholders, or quality goals change; this is repository verification and archive tooling. | |
| 2 | 02 architecture-constraints | Yes | The 0.7.0 contract adds a durable organisational constraint (Git-backed archive boundary) and a verification convention (runner-derived, fail-closed results). | Added the Git-backed archive boundary and the runner-derived fail-closed verification convention. |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface changes; the archive boundary is a repository process. | |
| 4 | 04 solution-strategy | No | No product technical strategy or approach changes. | |
| 5 | 05 building-block-view | No | No product building block or module changes; the result emitter lives inside the existing test harness. | |
| 6 | 06 runtime-view | Yes | The existing archive sequence reported green scenarios; under 0.7.0 the runner returns a machine-readable result and the gate is fail-closed. | Updated the archive sequence to return per-scope result JSON and described the fail-closed gate. |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or verification build topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | The runner result contract and Git-backed archive boundary are a new cross-cutting verification concept. | Added the "Archive verification contract" section. |
| 9 | 09 architecture-decisions | No | Structured verdict NOT_QUALIFIED (alternatives weak; architectural impact inside the existing test harness; lower-level artifact fails because the rationale is owned by design.md plus givn process guidance); routing CANONICAL_ARTIFACT to `design.md`; existing-ADR check found no record for the result contract or archive boundary. | |
| 10 | 10 quality-requirements | No | No product quality scenario changes; archive evidence quality is repository process. | |
| 11 | 11 risks-and-technical-debt | Yes | The project-owned runner can drift from the result contract or misclassify scenario outcomes. | Added R-080 with runner-derived counts and fail-closed mitigation. |
| 12 | 12 glossary | Yes | The workflow now relies on "verification result" and "archive receipt" as fixed terms. | Added both terms. |

## Notes

ADR action: NONE
ADR target: design.md

Structured verdict: NOT_QUALIFIED; routing CANONICAL_ARTIFACT; canonical
artifact `design.md`. The existing-ADR check searched the active register
(`docs/adr/`, linked from chapter 09) and found no record for the result
contract or archive boundary. The existing ADR bodies live at `docs/adr/`
because that is the project's established path; this maintenance change
creates no new ADR and refines no existing one.

---

## Status

STATUS: DONE
