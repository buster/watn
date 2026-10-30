# Migration Tasks: Givn 0.8.3 to 0.9.0 (aggregate)

This change has no product scenarios (`.givn-skip: specs`). The proposal forbids a synthetic feature, so the tasks below record evidence-based migration concerns rather than RED/GREEN changes to watn. The design's Phase 1 inventory found no displaced project rules; Phase 2 therefore adds no `extensions:` entries. Verification is the project's existing suite and the normal gates.

## Batch: migration disposition

- [ ] **Evidence:** Pre-upgrade revision `e12e90d^` tracks no `skills/givn-*/SKILL.md` or `commands/givn-*|grill-*` files; `.gitignore` excludes the four configured targets and `/AGENTS.md`; the pre-upgrade config declared the default `# rules: local-only`. The project owner confirmed no project-specific rules were kept in the generated files or the managed `givn-project` block. `design.md` records the checked paths and the `none found` inventory row.
- [ ] **Verification:** `givn/config.yaml` carries no `extensions:` key and still selects the default local-only rules policy; no generated file in `.opencode`, `.agents`, `.claude`, or `.omp` contains `Project extension` or a rule copy; every target's `givn-*` skills and commands carry only frontmatter and a `givn instructions <id>` direction.
- [ ] **COMMIT:** pending

## Batch: arc42 and ADR reconciliation

- [ ] **Evidence:** All twelve chapter-impact rows were walked against the proposal and design and marked No with a reason in `arc42.md`; the independent design-review re-derivation agreed row for row. All twelve chapter files exist with substantial content, contain no Unicode box-drawing characters, and the only pseudo-diagram scan matches are GitHub table separators. The chapter 09 register lists ADR-0001 through ADR-0026; the ADR qualification action is `NONE`.
- [ ] **Verification:** `arc42.md` contains the full 12-row table and `STATUS: DONE`; `design-review.md` records the independent check and `DESIGN-REVIEW: PASS`; no chapter file was modified.
- [ ] **COMMIT:** pending

## Batch: full verification and archive

- [ ] **Evidence:** `./run-tests.sh` exits 0 for the regular suite and `./run-tests.sh --e2e` exits 0 for the e2e subset; `givn lint` reports no blocking error; `givn check review --change migrate-0-8-3-to-0-9-0` passes every configured gate.
- [ ] **Verification:** `review.md` records the real command output and the completed artifact checklist; `givn status --change migrate-0-8-3-to-0-9-0` shows tasks and review complete; `givn archive migrate-0-8-3-to-0-9-0` succeeds through its gates.
- [ ] **COMMIT:** pending
