# Migration Tasks: Givn 0.8.3 to 0.9.0 (aggregate)

This change has no product scenarios (`.givn-skip: specs`). The proposal forbids a synthetic feature, so the tasks below record evidence-based migration concerns rather than RED/GREEN changes to watn. The design's Phase 1 inventory found no displaced project rules; Phase 2 therefore adds no `extensions:` entries. Verification is the project's existing suite and the normal gates.

## Batch: migration disposition

- [x] **Evidence:** Pre-upgrade revision `e12e90d^` tracks no `skills/givn-*/SKILL.md` or `commands/givn-*|grill-*` files; `.gitignore` excludes the four configured targets and `/AGENTS.md`; the pre-upgrade config declared the default `# rules: local-only`. The project owner confirmed no project-specific rules were kept in the generated files or the managed `givn-project` block. `design.md` records the checked paths and the `none found` inventory row.
- [x] **Verification:** `givn/config.yaml` carries no `extensions:` key and still selects the default local-only rules policy; no generated file in `.opencode`, `.agents`, `.claude`, or `.omp` contains `Project extension` or a rule copy; every target's `givn-*` skills and commands carry only frontmatter and a `givn instructions <id>` direction.
- [x] COMMIT: 6a1dd89380330f33c041b6b4277314f4000b120a -- Batch: migration disposition and arc42 reconciliation

## Batch: arc42 and ADR reconciliation

- [x] **Evidence:** All twelve chapter-impact rows were walked against the proposal and design and marked No with a reason in `arc42.md`; the independent design-review re-derivation agreed row for row. All twelve chapter files exist with substantial content, contain no Unicode box-drawing characters, and the only pseudo-diagram scan matches are GitHub table separators. The chapter 09 register lists ADR-0001 through ADR-0026; the ADR qualification action is `NONE`.
- [x] **Verification:** `arc42.md` contains the full 12-row table and `STATUS: DONE`; `design-review.md` records the independent check and `DESIGN-REVIEW: PASS`; no chapter file was modified.
- [x] COMMIT: 6a1dd89380330f33c041b6b4277314f4000b120a -- Batch: migration disposition and arc42 reconciliation

## Batch: full verification and review

- [x] **Evidence:** `./run-tests.sh` exits 0 with 23 features, 275 scenarios, and 1678 steps. `./run-tests.sh --e2e` exits 0 with 26 features, 92 scenarios, and 699 steps. `givn lint` exits 1 with 8 pre-existing `fragments` interaction findings: the `givn/specs/fragments/fragment.md` group declares `## Interactions - none` while carrying `@e2e` scenarios, last changed in `0e33f41`, and this change touches no spec.
- [x] **Verification:** `review.md` records the command counts and the completed artifact checklist; `givn check review --change migrate-0-8-3-to-0-9-0` passes every configured gate; `givn status --change migrate-0-8-3-to-0-9-0` shows all artifacts complete.
- [x] COMMIT: f6b38030086098998ec0b5136dce618291305423 -- Batch: full verification and review
