# Arc42 Impact Assessment: migrate-0-5-0-to-0-6-0

This aggregate maintenance change upgrades the Givn project contract from 0.5.0
to 0.6.0. The active specification corpus is already in the target stable
use-case and fragment layout from the earlier Watn migration; this change will
verify that state and preserve its evidence rather than repeat file moves.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Watn product goals, stakeholders, and runtime quality priorities are unchanged. | |
| 2 | 02 architecture-constraints | Yes | The active Givn corpus must use stable use-case identities, fragment ownership, resolvable relationships, and an untouched historical archive. | Record the canonical active paths and preservation boundary as repository constraints. |
| 3 | 03 context-and-scope | No | No Watn external interface, provider boundary, or product user-facing surface changes. | |
| 4 | 04 solution-strategy | Yes | The repository's specification strategy now explicitly relies on stable use-case IDs and reusable fragments rather than group narratives. | Document the canonical specification-ownership strategy and migration verification. |
| 5 | 05 building-block-view | Yes | The active specification corpus is a maintained repository building block with use-case roots, fragment roots, capabilities, and typed relationships. | Add the specification-corpus building block and its ownership responsibilities. |
| 6 | 06 runtime-view | No | The ordered migration phases are maintenance workflow, not a Watn runtime flow or sequence. | |
| 7 | 07 deployment-view | No | No executable, deployment target, installed configuration, or service topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | Migration correctness depends on preserving scenario identity, behavior hashes, E2E mappings, interaction coverage, relationships, and ideation state. | Document the specification migration integrity and evidence-preservation boundary. |
| 9 | 09 architecture-decisions | No | The stable corpus layout is fully owned by the migration proposal/design and does not qualify as an independent ADR; ADR-0025 covers scenario ownership, not this migration mechanism. | |
| 10 | 10 quality-requirements | Yes | The migration has explicit measurable preservation gates for schema validity, coverage, behavior hashes, E2E evidence, and source counters. | Add a migration-integrity quality scenario. |
| 11 | 11 risks-and-technical-debt | Yes | A structural migration can lose ownership, relationship, coverage, or historical evidence, and can leave stale 0.5 vocabulary in active paths. | Add migration-specific risks and residual vocabulary debt. |
| 12 | 12 glossary | Yes | The target contract introduces stable use-case, fragment, relationship, behavior-hash, and ideation terms. | Define the migration vocabulary used by the ledger and verification gates. |

## Notes

The project has no active ideation topics or Personas to promote. Historical
archives are excluded from the active-vocabulary audit and are not rewritten.
The architecture decision candidate fails the lower-level-artifact dimension:
the migration contract and rationale belong in `design.md`, while observable
specification behavior remains in the active Gherkin corpus.

ADR action: CANONICAL_ARTIFACT
ADR target: givn/changes/migrate-0-5-0-to-0-6-0/design.md

## Status

STATUS: DONE
