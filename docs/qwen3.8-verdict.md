# Verdict: Terra Pro vs DeepSeek Pro Givn Plans

Date: 2026-08-14

Reviewer model: qwen3.8-max

Compared documents:

- `docs/feature-step-overlap-report.md` (baseline findings)
- `docs/givn-plan-terrapro.md` (Terra Pro plan)
- `docs/givn-plan-deepseek-pro.md` (DeepSeek Pro plan)

Validation method: every falsifiable claim in both plans was checked against
the givn source at `~/projects/givn` (v0.2.0, commit `a3a3b79`, 2026-08-10),
the watn repository (`~/projects/watn`, including the generated
`.opencode/commands` and `.opencode/skills`), and watn git history. Where
possible claims were verified empirically (running `givn lint`, counting
bindings, `git show` on cited commits), not only by reading code.

## Claim Validation: DeepSeek Pro

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| A1 | `merge.rs:42-65` blocks `@givn.added` only when the title already exists in the same permanent file | Verified, exact | The validation loop at `src/gherkin/merge.rs:42-65` compares only against `perm_scenarios` parsed from the single target file. The merge target is `givn/specs/<cap>/<cap>.feature`, routed by capability tag (`src/cli/archive.rs:136-149`). No cross-file check exists. The report's F1/F2 duplicate pairs are in different files, so archive could never detect them. |
| A2 | `delta.rs:4-12` has Added/Modified/Removed but no supersedes operation | Verified, exact | `DeltaOp` enum at those exact lines. `grep -rn supersedes` across givn `src/` and `assets/`: zero hits. |
| B | `givn-spec` says "check permanent specs" only as a baseline for the same capability; nothing instructs a whole-tree invariant search | Verified | Generated command step 3: "Also check permanent specs at `givn/specs/` — existing scenarios for a capability are the baseline; delta scenarios extend or modify them." No ownership-search command exists in the CLI (`givn --help`). The report's Behavior Ledger and Scenario Decision Tree describe tooling that does not exist. |
| C | `givn-steps` instructs reuse but provides no mechanism; bindings are global (858 declarations) | Verified | `givn-steps` skill: "Check for a matching step definition before writing a new one. Reused: keep as-is" — prose only, no search tool. Counted exactly 858 `#[given/when/then]` attributes in watn `tests/steps/*.rs`, matching the report. |
| D | Three prose layers repeat the rules and drift: `commands/givn-spec.md` says "one `@e2e` per distinct happy-path action" while `instructions/specs.md` says "one per User Interaction Inventory entry" | Verified, worse than claimed | `assets/commands/givn-spec.md:94`: "one `@e2e` scenario per distinct happy-path action". `assets/instructions/specs.md:94`: "one `@e2e` scenario per User Interaction Inventory entry". The skill template `assets/skills/givn-spec/SKILL.md.tmpl` contains **both** wordings (line 69 "one `@e2e` smoke-test scenario per User [Interaction Inventory entry]" and line 79 "one `@e2e` scenario per distinct happy-path action") — the drift exists even within a single generated file. |
| E1 | `lint.rs` checks only parse errors and `@wip` (plus a tasks.md e2e cross-check) | Verified | `src/cli/lint.rs:79-82` handles only `LintKind::ParseError` and `LintKind::Wip`; the tasks.md e2e-intent cross-check is at `:98-121`. No overlap, title, or fingerprint checks. |
| E2 | `givn lint` without `--change` recurses into `givn/archive` (`parse.rs:216-237` has no archive skip) | Verified, empirically proven | `collect_feature_files`/`collect_recursive` at `src/gherkin/parse.rs:216-237` contain no archive exclusion; default lint scope is all of `givn/` (`lint.rs:34-41`). Running `givn lint` in watn reports **"51 file(s) checked"** = 25 active specs + 26 archived features. This contradicts the report's requirement that archived files be evidence, not active findings. |
| E3 | `givn check review` / `archive` run verify, verify-e2e, integrity | Partially verified | True for `givn check review`: mandatory checks are verify, verify-e2e, integrity (`src/cli/check.rs:97-109`). **False for `givn archive`**: archive runs post-merge verify and verify-e2e (`src/cli/archive.rs:156-208`) plus the `apply.requires` post hooks, which are empty in the default manifest and in watn's overlay. The integrity hook never runs at archive. |
| — | No scenario statistics anywhere in `status`/`archive`/`lint` | Verified | `src/cli/status.rs` tracks only artifact/checkbox state; no scenario or step counters in lint or archive output. |

## Claim Validation: Terra Pro

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| T1 | Givn `merge.rs` replaces and deletes by title | Verified | `merge.rs:7-8` doc contract; `:77-84` implements Modified/Removed as title lookups. |
| T2 | `tests/features_runner.rs:153-164` runs permanent specs plus raw active deltas; a modified scenario can require both old and new behavior before archive; a removed scenario remains executable until archive | Verified | `features_runner.rs:150-165` hard-codes `CARGO_MANIFEST_DIR/givn`, collects `givn/specs` plus every change's `specs/` directory, with no delta projection. The only mitigation is `GIVN_ARCHIVE_ONLY` skipping change specs during archive verify (`:157`). |
| T3 | The Watn runner ignores `GIVN_FEATURES` and hard-codes `givn/specs` | Verified | Zero references to `GIVN_FEATURES` anywhere in watn `tests/`, `src/`, or `run-tests.sh`. Givn nevertheless sets the variable (`archive.rs:161`, `hooks.rs:194`) — it is a dead contract. |
| T4 | Default manifest hooks are empty; `archive` runs configured post hooks but does not invoke the full mandatory `check review` path | Verified | `assets/default-manifest.yaml`: every artifact has `post: []`. Watn's `givn/config.yaml` overlay enables only addons, no post hooks. Archive therefore runs gate hooks that do nothing, then post-merge verify + verify-e2e (+ coverage gate), but never the integrity check that `givn check review` mandates. |
| T5 | Skills can claim checks block archive when the binary has not enforced them | Verified | The generated archive skill claims `givn archive` "automatically checks" that all tasks are checked "each with evidence and commit hash filled in". The binary only counts checkboxes and checks artifact status/sign-off markers (`archive.rs:43-74`); it never inspects evidence or commit hashes. (Note: the archive skill does not itself claim integrity blocks archive; that part of the diagnosis is directionally right but not literally present in the skill text.) |
| T6 | `givn-steps` requires one file per capability and permits immediate GREEN for all-reused steps | Verified | `givn-steps` skill template: "One file per capability — never a single file for the whole change" (line 42) and "Zero exit, all steps reused → legitimate immediate GREEN" (lines 126-127). |
| T7 | `eb328dd` introduced a `@givn.modified` stale-search scenario; archive commit `57688f6` left the original autosuggest scenario unchanged and added a separate permanent feature | Verified | `git show eb328dd` ("feat(search): The newest search result stays visible when an older result arrives later", 2026-08-10) adds `@givn.modified Scenario: The newest search result stays visible when an older result arrives later` plus an `@givn.added @e2e` PTY variant to a `search-concurrency` delta. `git show 57688f6` ("archive: complete model discovery and setup correctness") merges it while `model-autosuggest` retains the same-titled scenario. |
| T8 | `c0a3fcf` made `givn check review` stronger while leaving archive dependent on empty default post-hooks | Verified | givn commit `c0a3fcf` (2026-07-16) "feat(hooks): harden e2e enforcement as mandatory binary checks" introduced the mandatory review checks; archive's gate hooks remained `post`-hook-driven and empty by default. |
| T9 | Generated guidance says config lives in `givn/config.yaml` when it actually resolves from `givn/commands.yaml` | Verified as drift, overstated as stated | `src/config/overlay.rs:89-108` loads **both** files: `givn/config.yaml` first, then `givn/commands.yaml` as a second overlay that wins conflicts. In watn the entire `verify`/`coverage` block lives in `commands.yaml`, and `config.yaml`'s own comments say so. Yet the archive skill ("verify.e2e_command (configured in givn/config.yaml)") and the binary's own error message (`hooks.rs:142`: "set verify.e2e_command in givn/config.yaml") point at the wrong file for this project. Real doc/binary drift; the precise fix is to name both overlay files and their precedence, not to say config.yaml is never used. |
| T10 | Current archive behavior strips all `@givn.*` tags | Verified | `merge.rs:9` doc contract, `strip_givn_tags` (`delta.rs:80-85`), `render_scenario_stripped` (`merge.rs:209-217`). The archive skill documents this too. |
| T11 | Archive "has already lost feature-level comments during merge" | Plausible, unproven | Loss mechanisms exist: new-capability merges synthesize the header as `Feature: <name>` dropping delta header comments/description (`merge.rs:36-39`), and modified scenarios are re-rendered from the AST, dropping intra-scenario comments. But no concrete historical loss was evidenced in the current tree (watn permanent specs contain no feature-level comments to compare). |
| T12 | `givn instructions --json` exists (basis for Phase 5 extension) | Verified | Flag present in `givn instructions --help`. |

## Comparison

### Diagnosis quality

Both plans correctly identify the report's structural causes, and both are
unusually well-grounded: nearly every falsifiable claim in both plans survives
contact with the code.

- **DeepSeek Pro** triangulates five root causes (A–E), each citing exact code
  lines, and every citation checked out. Its lint-into-archive finding (E2)
  was proven empirically: lint currently treats 26 archived feature files as
  active findings. It also caught the `@e2e` rule contradiction, which is even
  messier than claimed (both wordings coexist inside the skill template).

- **Terra Pro** goes deeper into runtime semantics and catches three real
  correctness bugs that DeepSeek misses entirely:
  1. `GIVN_FEATURES` is a dead contract — givn sets it, watn never reads it.
  2. The runner executes permanent specs plus raw deltas, so a
     `@givn.modified` scenario runs beside its obsolete permanent version
     until archive, and `@givn.removed` placeholders stay executable.
  3. Archive skips the integrity check that `givn check review` mandates,
     while the generated archive skill overstates what the binary enforces.

### Solution shape

- **DeepSeek Pro** is incremental and shippable against givn 0.2.0 as-is:
  Phase 0 lint scope fix → Phase 1 overlap lint (normalization, fingerprints,
  length gates, `givn steps report`) → Phase 2 `@givn.supersedes` +
  cross-file title checks + archive net-delta receipt → Phase 3
  `behavior.yaml` ledger + `givn spec ownership` → Phase 4 skill rewrites and
  prose consolidation → Phase 5 harness dogfooding and watn consolidation
  changes. It keeps the text-level, title-based merge engine and explicitly
  refuses automatic Scenario Outline rewrites — a realistic risk posture. Its
  multishot examples reuse the report's actual findings (F2, F4, S1), and its
  false-positive policy (exact titles hard-fail; semantic matches require a
  one-tag disposition) matches the report's "merge examples, not invariants"
  rule.

- **Terra Pro** is a platform redesign: stable scenario anchors, a versioned
  behavior registry with semantic digests, effective-spec projection in the
  runner, staged archive transactions with a journal and compare-and-swap,
  project-provided binding indexes, contract tests across all doc surfaces,
  and a V1/V2 migration using the report's findings as golden fixtures. It
  attacks the identity and transaction root causes that DeepSeek's
  title-based `@givn.supersedes` inherits. But it front-loads a large V2
  contract before any detection exists — its Phase 0 alone is bigger than
  DeepSeek's entire plan — and it never quantifies a minimal first increment.

### Gaps

- **DeepSeek Pro** misses the runner/`GIVN_FEATURES`/projection problem, so
  even perfect consolidation would still merge into a tree whose execution
  view is wrong (old + new scenario both run until archive). It overstates
  archive's gates (integrity does not run there). And its riskiest item,
  `givn steps report` with per-language regex parsing of bindings, is exactly
  what Terra Pro convincingly argues is unsafe for a generic CLI — binding
  similarity is better served by a project-provided index with advisory
  status.

- **Terra Pro** does not explicitly fix the lint-into-archive scope bug that
  DeepSeek caught empirically. Two of its claims are oversimplified
  (T9 commands.yaml, T11 comment loss). Its scope risks the very
  over-engineering the report warns about: registry schema, anchors, digests,
  journals, merge queues, and 14 rewritten skills before a single duplicate
  is detected mechanically.

### Shared ground

Both plans adopt the report's core concepts faithfully: behavior ledger,
decision tree, net-delta archive receipt, boundary naming, length thresholds
(14/19), "merge examples, not invariants", and advisory-not-automatic
treatment of fuzzy matches. Neither plan invents findings the report does not
support.

## Verdict

Both plans are factually reliable; the choice is about sequencing and risk.

- **DeepSeek Pro is the better execution plan.** Smaller verified increments,
  correct sequencing (detection before discipline), and every mechanism
  traces to a specific report finding. It would start preventing the next F2
  within a few changes to givn 0.2.0.

- **Terra Pro is the better systems analysis.** Its verified runner, archive,
  and doc-drift findings are real bugs the other plan ignores, and
  anchor-based identity plus effective-spec projection is the correct
  long-term answer to title-based merge fragility and old/new
  double-execution.

Recommended combined path:

1. Adopt DeepSeek Phases 0–2 now: lint archive-scope fix, overlap lint with
   length gates, cross-file `@givn.added` collision checks,
   `@givn.supersedes` with atomic removal, and the archive net-delta receipt.
2. Add Terra Pro's cheap, high-value binary fixes alongside: stop emitting
   `GIVN_FEATURES` claims the runner contract does not honor (or honor it in
   watn), run the integrity hook at archive, and correct the
   config.yaml/commands.yaml guidance in generated skills and error messages.
3. Before the title-based `@givn.supersedes` mechanism calcifies, adopt
   Terra Pro's stable scenario anchors and effective-spec projection so
   deltas target identities, not prose, and modified/removed scenarios stop
   executing beside their obsolete permanent versions.
4. Treat Terra Pro's full registry/journal/merge-queue machinery as a later
   decision, justified only after Phases 0–2 evidence shows title-based
   identity is the binding constraint.
