# Migration Tasks: Givn 0.2.0 to 0.3.0

This change has no product scenarios. The proposal explicitly forbids a
synthetic feature, so the tasks below record evidence-based migration
concerns rather than RED/GREEN changes to Watn.

## Setup: strict Gherkin runner

- [x] **RED:** A temporary undefined step was added only for the proof and run
  with `./run-tests.sh --name "Strict mode rejects an undefined migration step"`.
  The runner exited non-zero and reported `Step doesn't match any function`,
  `1 scenario (1 failed)`, and `1 step (1 failed)`. The temporary feature was
  removed immediately and is not part of the migration.
- [x] **GREEN:** `tests/features_runner.rs:178` configures
  `.fail_on_skipped()`, and the real runner executes both permanent
  `givn/specs/` features and change features. Existing runner provenance is
  `2f1423a`.
- [x] **REFACTOR:** `run-tests.sh` uses explicit copied debug binaries and
  distinct `not @wip and not @e2e` versus `@e2e and not @wip` filters. Existing
  E2E-filter provenance is `9969ec6`.
- [x] **COMMIT:** No production implementation was required for this
  feature-free migration. The verified runner infrastructure is recorded by
  `2f1423a` and `9969ec6`.

## Inventory and managed boundary

- [x] **Evidence:** `givn/config.yaml` is at marker `0.3.0` and preserves the
  project overrides `addons.arc42`, `absolute_mode`, `readme`, `coverage`,
  `domain_modeling`, `dev_principles`, `grill_me`, and `grill_with_docs`, all
  enabled. No persisted `skills.targets` override is present, so the generated
  target remains the default `opencode`. `givn/commands.yaml` configures
  `./run-tests.sh` and `./run-tests.sh --e2e`. The only active change is this
  migration; the existing archived changes and permanent `givn/specs/` tree
  remain outside its scope. Existing `.agents/` and `.claude/` outputs are
  retained; Commit A manages the configured `.opencode/` target and `AGENTS.md`.
- [x] **Verification:** `givn addons list`, `givn graph`,
  `givn status --change migrate-0-2-0-to-0-3-0`,
  `git status --short --untracked-files=all`, and both Commit A/B allowlist
  inspections confirm the inventory. The worktree contains only the two
  migration documentation files added after the automatic commits; no unrelated
  path is staged or committed.
- [x] **COMMIT:** Managed preparation is recorded by
  `d319e3e6aa04884fa53864d7548408bebc9e7c5e` and aggregate plan preparation by
  `5cf7bd1631f155a98b8e60485dd9ac10a4e9e983`. Neither commit stages the whole
  worktree or an unrelated migration file.

## Generated guidance and ownership

- [x] **Evidence:** The generated guidance under `.opencode/` and the Givn-owned
  blocks in `AGENTS.md` match Commit A; `git diff --exit-code
  d319e3e6aa04884fa53864d7548408bebc9e7c5e -- .opencode AGENTS.md
  givn/config.yaml` returned no differences. Project-owned content and all
  `givn/` overrides remain unchanged, while the pre-existing `.agents/` and
  `.claude/` outputs remain retained and untouched.
- [x] **Verification:** The resolved `specs`, `design`, `review`, and
  `arc42-docs` instructions were each loaded with `givn instructions ...
  --change migrate-0-2-0-to-0-3-0`; all four commands succeeded. The current
  worktree was clean after the comparison.
- [x] **COMMIT:** Verified managed guidance commit:
  `d319e3e6aa04884fa53864d7548408bebc9e7c5e`. No second policy source was
  created and no project-owned content was overwritten.

## Permanent specification and change review

- [ ] **Evidence:** Review the permanent `givn/specs/` tree and confirm that
  the migration contributes no feature file, no removed `@e2e` tag, no copied
  obsolete guidance, and no active competing change beyond this maintenance
  change.
- [ ] **Verification:** Run `givn lint`; when retrieval support is available,
  run `givn spec index` and the advisory/review commands required by the
  current instructions. Record every finding and disposition; unresolved
  semantic evidence remains blocking.
- [ ] **COMMIT:** No product or specification commit is expected. The migration
  plan commit `5cf7bd1631f155a98b8e60485dd9ac10a4e9e983` is the only plan
  provenance, and `.givn-skip` remains exactly `specs`.

## Arc42 and ADR reconciliation

- [ ] **Evidence:** Walk all twelve Arc42 chapter-impact rows independently,
  compare them with `arc42.md`, inspect all twelve durable chapter files, and
  search active and archived ADR indexes before routing any candidate.
- [ ] **Verification:** Confirm the all-`No` assessment, `STATUS: DONE`, no
  prohibited ASCII-art diagrams, no qualified ADR candidate, and no durable
  architecture fact changed by this process migration.
- [ ] **COMMIT:** Record the commit that adds the completed `arc42.md` marker
  and this evidence; no durable Arc42 chapter or ADR is to be modified.

## Full verification and archive

- [ ] **Evidence:** Run the configured non-E2E and E2E runners, record their
  outputs and scenario counts, and confirm that the E2E filter is a strict
  subset. Run coverage measurement including the Gherkin runner when the
  configured coverage addon permits it.
- [ ] **Verification:** Run `givn check review --change
  migrate-0-2-0-to-0-3-0`, resolve all blocking findings, re-run
  `givn status --change migrate-0-2-0-to-0-3-0`, then archive only after all
  gates are complete. Confirm no migration feature enters `givn/specs/`.
- [ ] **COMMIT:** Record the final evidence commit before running
  `givn archive --change migrate-0-2-0-to-0-3-0`.
