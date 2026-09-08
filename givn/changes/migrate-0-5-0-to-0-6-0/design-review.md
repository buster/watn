# Design Review: migrate-0-5-0-to-0-6-0

## Grilling log

| # | Branch | Finding | Resolution |
|---|---|---|---|
| 1 | Scope | The target use-case/fragment corpus already exists from `migrate-usecases-watn`; repeating moves would be destructive and outside the current repository state. | The design is verification-only, records `VERIFY_ALREADY_MIGRATED`, and forbids unreviewed corpus mutation. |
| 2 | Tech choices | The configured Givn inventory, Cucumber runner, coverage scripts, and Cobertura output are the existing project seams; no new runtime technology is needed. | Keep the current commands and derive source counters from the configured Cobertura report. |
| 3 | Missing scenarios | No product behavior is introduced; `.givn-skip` excludes change-local specifications. The maintenance contract still requires checks for ownership, relationships, interaction mappings, ideation state, archive preservation, and evidence equality. | Add those checks to the verification task; do not add product scenarios. |
| 4 | Testability | The target state can fail verification through stale `group.md` paths, duplicate capability ownership, unresolved relationships, changed behavior hashes, lost E2E mappings, or changed coverage counters. | Use the active inventory, `givn lint`, coverage baseline comparison, configured verification, and explicit path audit. |
| 5 | Risk | The most likely failure is treating a green feature runner as proof that migration evidence was preserved. | Require behavior-hash, E2E, interaction, source non-regression, and historical-archive comparisons. |
| 6 | Use-case and Persona context | Four active use cases and one fragment document exist; all declare `Personas: none`; there are no ideation topics or Handoff decisions. | Record absence explicitly and do not invent or promote Personas. |
| 7 | ADR qualification | The stable corpus boundary is durable, but the source plan does not supply real alternatives and the complete migration rationale belongs in `design.md`. ADR-0025 is related to scenario ownership but is not the same decision. | Route `CANONICAL_ARTIFACT` to `givn/changes/migrate-0-5-0-to-0-6-0/design.md`; create no ADR. |
| 8 | Architecture documentation | Independent chapter assessment is `1 No, 2 Yes, 3 No, 4 Yes, 5 Yes, 6 No, 7 No, 8 Yes, 9 No, 10 Yes, 11 Yes, 12 Yes`, matching `arc42.md`. All 12 chapter files exist and contain substantive content. | Arc42 files were updated for the stable corpus, evidence boundary, risks, quality scenario, and glossary. `givn check arc42-docs --change migrate-0-5-0-to-0-6-0` passes. |

## Evidence

- The active corpus contains `usecase.md` roots for `configure-interactive`,
  `configure-model`, `configure-provider`, and `use-shell`, plus the
  `fragments/fragment.md` root; no active `group.md` remains.
- The archived `migrate-usecases-watn` ledger and coverage evidence document
  the earlier structural migration. This change verifies the current state and
  does not rewrite that archive.
- `givn lint` checks all 26 active feature files successfully.
- The unsupported `givn check migration` command was removed from the design;
  the supported Arc42, review, lint, coverage, verification, and E2E gates are
  named explicitly.
- The active ADR records were searched before routing the candidate to the
  design artifact.

## Sign-off

- [x] Scope is verification-only for the already-applied target corpus.
- [x] No product behavior or change-local feature is required.
- [x] Use-case, fragment, Persona, Handoff, and archive boundaries are explicit.
- [x] ADR qualification is complete and routed to exactly one canonical artifact.
- [x] Arc42 impact rows and all chapter files were independently checked.
- [x] Evidence commands are executable under the current Givn 0.6 artifact graph.

## Re-review

The evidence rule was corrected after execution showed a two-line increase in
covered source lines between identical no-op runs. The design now requires
non-regression, not equality: covered line/branch counts cannot decrease and
valid counter totals must remain compatible. The affected Arc42 quality wording
and task evidence were updated accordingly. No scope, architecture, Persona,
ADR, or chapter-impact decision changed.

DESIGN-REVIEW: PASS
