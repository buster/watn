# Tasks: migrate-0-7-0-to-0-8-0

This maintenance change has no Gherkin scenarios (the `specs` artifact is
skipped), so the scenario-by-scenario RED→GREEN→REFACTOR loop does not apply.
The implementation is the migration inventory and plan; each task below records
its evidence and the revision that contains it.

## Setup task

- [x] Runner configuration unchanged. `givn/commands.yaml` keeps
      `verify.command: ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`;
      this change adds no `.feature` file, so the strict-mode proof is vacuous
      and the runner stays as proven by the archived corpus.
      Evidence: `givn lint --change migrate-0-7-0-to-0-8-0` -> "no .feature
      files found under givn/changes/migrate-0-7-0-to-0-8-0/specs/", exit 0.
      Revision: `db0c64a` (generated bundle), `f91539a`.

## Task 1 — Preflight and corpus inventory

- [x] Confirm `givn/config.yaml` declares `givn_config_version: "0.8.0"`, the
      checkout is clean, and the migration change exists at
      `givn/changes/migrate-0-7-0-to-0-8-0/`.
      Evidence: `givn status --change migrate-0-7-0-to-0-8-0`; `git status`.
- [x] Inventory the corpus: `givn spec tree`, the permanent `givn/specs/`
      directories, `givn/personas/`, `givn/ideation/terminal-wow-factor/`
      (personas, terms, seed, questions), `docs/adr/`, `docs/arc42/`,
      `givn/config.yaml`, `givn/artifacts/`, `givn/skills/`.
      Evidence: the "Inventory" section in `design.md`.
      Revision: `f91539a`.

## Task 2 — Phase 1 use-case model inventory

- [x] Apply the eight scoping rules and the ten rejection criteria to every
      permanent use-case document and capability; record one row per failing
      document with its follow-up.
      Evidence: `design.md` Phase 1 table plus the R8 name audit and R9 note.
      Revision: `f91539a` (corrected by `28f4a56`).

## Task 3 — Phase 2 persona inventory

- [x] Check every persona file, reference, and topic copy against the identity
      criteria and the stable-core/running-record/topic-stances schema.
      Evidence: `design.md` Phase 2 table.
      Revision: `f91539a`.

## Task 4 — Phase 3 ADR re-qualification

- [x] Re-run the five mandatory dimensions, the three falsification tests, and
      the cheaper-home challenge for every accepted record; re-qualify the two
      proposed records whose owning changes have archived; record the
      structured verdicts in `arc42.md`; disposition every superseded record.
      Evidence: `arc42.md` verdict table and blocks; `design.md` Phase 3 rows.
      Revision: `f91539a` (ADR-0022 re-qualified by `28f4a56`).

## Task 5 — Phase 4 smaller adoptions

- [x] Classify the seed, migrate term records to the 0.8.0 schema (follow-up),
      and re-run the confirmed terms' collision searches now, recording the
      commands and hit counts in the collision-check cells.
      Evidence: `design.md` Phase 4 table; `domain-terms.md` collision-check
      cells ("command flow": 71 lines / 25 files, anti-term `pipeline map` 2
      declaration-only hits; "candidate": 1468 lines / 47 files, anti-term
      `proposal version` 2 declaration-only hits).
      Revision: `f91539a`.
- [x] Check guarantee qualifiers and the decision ledger: no `(open: Q<N>)`
      marker exists and the only active proposal has no open question.
      Evidence: `design.md` Phase 4 observations.

## Task 6 — Phase 5 ejected overrides

- [x] Inventory overrides: no pre-0.8.0 rule text or stale default exists in
      `givn/config.yaml`; `givn/artifacts/` and `givn/skills/` are absent.
      Evidence: `design.md` Phase 5.
      Revision: `f91539a`.

## Task 7 — arc42-docs assessment

- [x] Walk all 12 chapter rows, record the assessment with explicit reasons,
      and record the structured ADR verdicts in `arc42.md`.
      Evidence: `arc42.md` with `STATUS: DONE`; `givn status --change
      migrate-0-7-0-to-0-8-0` shows `arc42-docs` complete.
      Revision: `f91539a`.

## Task 8 — design-review

- [x] Grill the plan (ranked questions, branch walk), harden the artifacts,
      and sign off.
      Evidence: `design-review.md` with `DESIGN-REVIEW: PASS`.
      Revision: `28f4a56`.

## Task 9 — Verification

- [x] `givn lint` runs with advisory findings only; `givn lint --change
      migrate-0-7-0-to-0-8-0` exits 0 (no delta features).
      Evidence: "givn lint: 27 file(s) checked, 1 finding(s)" (advisory only)
      and "no .feature files found ...", exit 0.
- [x] `verify.command` and `verify.e2e_command` exit 0 on the full suite.
      Evidence: run by the review artifact's post hooks (`givn check review`).
      Revision: recorded in `review.md`.

## Named follow-up changes (out of scope for this change)

The migration is complete when these archive; this change only records them.

| Follow-up | Scope |
|---|---|
| split-configure-interactive | R1/R9/level repair for the configure-interactive use case |
| rename-configure-interactive-capabilities | R8 renames for unified-setup-wizard, auto-init-config, highlight-active-setup-input, responsive-setup-model-filtering, setup-persistence |
| split-configure-model | R1/level repair for the configure-model use case |
| rename-configure-model-capabilities | R8 renames for ratatui-model-picker, model-autosuggest |
| refocus-configure-provider | R5 extension rewrite |
| rename-configure-provider-capabilities | R8 rename for provider-setup-widget-layout |
| split-use-shell | R1/R2/R5/R6/R7/R9 repair for the use-shell use case |
| rename-use-shell-capabilities | R8 renames for interactive-shell-shortcut, shell-completions |
| rename-corpus-infra-capabilities | R8 renames for config, transport, incremental-sse-rendering, search-concurrency |
| reformat-terminal-developer-persona | 0.8.0 persona schema conversion |
| refresh-terminal-wow-factor-persona-record | `Reuse:` header plus record entries |
| migrate-adr-storage | move to `docs/arc42/adr/`, index, template, register, README links |
| migrate-adr-schema | 0.8.0 MADR retrofit for all 26 records |
| retire-nonqualifying-adrs | ADR-0005, 0010, 0012, 0022 rationale moves and removals |
| archive-superseded-adrs | ADR-0006, 0007, 0008, 0011, 0013, 0014 archival and status normalization |
| stamp-use-explanatory-shell-shortcut-seed | seed status and content schema |
| migrate-terminal-wow-factor-term-records | 0.8.0 term-record schema |
