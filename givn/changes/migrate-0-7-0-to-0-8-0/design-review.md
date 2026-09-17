# Design Review: migrate-0-7-0-to-0-8-0

## Grilling

A grilling subagent read `proposal.md`, `design.md` (bundle plus inventory),
`arc42.md`, `migration.yaml`, and the corpus (`givn spec tree`, specs, personas,
ideation records, `docs/adr/`, arc42 chapters, config), then produced a ranked
question list. The orchestrator re-derived each branch and resolved every
question from the project's own artifacts; no question required operator
arbitration, so none was deferred.

| # | Question (branch) | Resolution |
|---|---|---|
| 1 | Is "every R8 name has an inventory row" checkable when only violations are listed? (scope) | Resolved: added the full R8 name audit table to `design.md` (all 32 permanent use-case, fragment, and capability names with PASS/FAIL and a one-line rationale), so the R8 line is explicit and auditable. |
| 2 | Nothing enforces the 17 named follow-ups; the migration could archive as a graveyard. (risk) | Resolved per the migration contract: named (not yet scaffolded) follow-ups are explicitly permitted, the inventory is the durable record and is committed with this change, and the migration is only complete when the follow-ups archive. Recorded as an accepted risk with that mitigation in `design.md`. |
| 3 | ADR-0022 (verbatim reasoning values) is retained but the decision is a state-value contract, which F3 excludes. (ADR qualification) | Confirmed: the accepted value domain and the `off` omission are state values owned by the capability spec; re-qualified `NOT_QUALIFIED` with destination `givn/specs/configure-model/reasoning-policy.feature` and added to the retirement follow-up. ADR-0020 and ADR-0023 were re-tested the same way and retain explicit boundaries (focused-vs-coordinated write authority; persisted provider-identity migration contract). |
| 4 | R8 is applied inconsistently: some mechanism-shaped names are flagged, others not. (coverage) | Resolved by the audit table's stated line: FAIL for names that identify how the system is built (component, framework, internal mechanism), PASS for actor outcomes, flows, and glossary domain concepts. `streamlined-setup`, `catalog-source`, `credential-sources`, `provider-setup`, `quicksetup`, and `providers` are PASS with recorded rationale; `model-autosuggest` is FAIL and joins the rename follow-up. |
| 5 | configure-provider may fail R1/R9, making `refocus-configure-provider` the wrong disposition; R9 was applied to no leaf. (use-case context) | Re-checked all four leaves: configure-provider has one success outcome and a single-goal flow (R1/R9 PASS), so only its extension reference fails; R9 FAIL is now recorded for configure-interactive and use-shell, and R9 PASS for configure-model and configure-provider, in the R9 note and rows. |
| 6 | ADR-0026 carries a pre-0.8.0 `## Qualification` block, so the schema row's evidence is false. (coverage) | Confirmed: the schema row now says the block exists and must be replaced with the 0.8.0 verdict; the `AMEND_ADR` routing is correct because the record exists. Noted in `arc42.md` as well. |
| 7 | The superseded-status list is wrong: ADR-0014 has the same multi-ID status pattern as ADR-0011, and ADR-0011 is only partially superseded (its rationale remains in chapters 2, 4, and 11). (coverage) | Confirmed: ADR-0014 added to the nonconforming list; ADR-0011 recorded as partially superseded with its still-current homes named; the archive follow-up now normalizes status text for 0007, 0008, 0011, and 0014 and moves any still-current rationale once. |
| 8 | Two canonical destinations are capability files that the rename follow-ups move, and `docs/arc42/README.md` links to `../adr/` but is not in the storage row. (coverage/risk) | Confirmed: the suggested order now runs the capability renames before `retire-nonqualifying-adrs` (or records destinations by capability ID), and `migrate-adr-storage` now also updates the `docs/arc42/README.md` links. |

Branches with no issue found: tech choices (no stack change, no Technology
Decisions section); testability (checks are `givn lint` plus the recorded
dispositions, and `givn lint --change` exits 0); decision ledger (the proposal
has no open questions and no `(open: Q<N>)` marker exists); visual design
contract (no declared interface); persona disposition (no user-visible surface);
interface classification (no interface; the opposite classification does not
hold because nothing is rendered); arc42 assessment (see the independent
re-derivation below).

## Independent arc42 re-derivation

The 12 chapter rows were re-derived from `proposal.md`, `design.md`, and
`arc42.md` before opening the assessment: this change creates no product
behavior, no building block, no runtime flow, no deployment change, no quality
scenario, no constraint, and no glossary term; it creates no ADR and edits no
chapter content, so all 12 rows are legitimately "No". The claim in `arc42.md`
matches row by row; no row is missing or contradicted.

Chapter integrity: all 12 chapter files plus `README.md` exist under
`docs/arc42/` (53-1159 lines each) and contain real content, not scaffold
placeholders. No ASCII-art diagram exists (zero Unicode box-drawing characters;
diagrams are Mermaid fenced blocks). Every qualified ADR candidate in the
inventory has a chapter-09 register entry today, and the three records whose
rationale moves are routed to canonical artifacts rather than chapter 09; no
new ADR exists whose "Bad, because..." consequence would need a chapter-11
line. The one stale chapter-11 item found by the inventory (R-008, template
config generated from code) is assigned to the `archive-superseded-adrs`
follow-up, not to this change.

## Hardening applied

- `design.md`: added the R8 name audit table and the R9 note; added R9 evidence
  to the configure-interactive and use-shell rows; recorded the
  configure-provider R1/R9 re-check; replaced the streamlined-setup rename with
  model-autosuggest; added the ADR-0022 retirement row; corrected the ADR-0026
  qualification evidence; added ADR-0014 and the ADR-0011 partial-supersession
  homes to the archive row; added `docs/arc42/README.md` to the storage row;
  recorded the follow-up ordering and the accepted enforcement risk.
- `arc42.md`: moved ADR-0022 to `NOT_QUALIFIED` with its verdict block; updated
  the ADR action target list; added the ADR-0026 note; corrected the superseded
  record note.
- `specs/`: no scenarios added — this maintenance change has no Gherkin delta
  and `givn lint --change migrate-0-7-0-to-0-8-0` exits 0.
- `proposal.md`: unchanged; grilling found no scope mismatch.

## Sign-off

All branches walked, all questions resolved from project artifacts, all
hardening edits applied. No unresolved question and no deferral remains.

DESIGN-REVIEW: PASS
