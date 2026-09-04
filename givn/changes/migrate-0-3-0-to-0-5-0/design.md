# Design: .migrate-0-3-0-to-0-5-0.tmp-1045447

## Migration metadata

- source_version: `0.3.0`
- target_version: `0.5.0`
- upgrade_commit: `8cd0d581f25b1f55f4975e8e6040139a11b2b235`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Complete the following phases strictly in sequence. Do not create independentparallel migration changes for these bundles. The complete sequence is alsorecorded in `migration.yaml`.

### Phase 1: migrate-0-3-0-to-0-4-0 (0.3.0 -> 0.4.0)

# Migration Design: 0.3.0 to 0.4.0

This bundle is the release-owned migration phase for projects upgrading to
Givn 0.4.0. Givn 0.4.0 is delivery tooling: it adds the CI, security,
prepare-release, and release workflows, the git-cliff generated changelog, and
corrected repository metadata. It changes no consumer-facing file format and no
Cargo feature contract, so this phase verifies rather than migrates.

When this bundle is materialized, `0.3.0` is the project's current
config version and `0.5.0` is `0.4.0`. The managed preparation
commit is `8cd0d581f25b1f55f4975e8e6040139a11b2b235`.

## Prompt 1: Inventory the managed 0.4.0 upgrade

Inspect the consumer project before editing:

- `givn/config.yaml` `givn_config_version:` — confirm it identifies
  `0.5.0`.
- `givn/commands.yaml` and the generated agent integrations — confirm they
  match the current Givn contract.
- The managed upgrade commit `8cd0d581f25b1f55f4975e8e6040139a11b2b235` — confirm it stages only the
  allowed managed paths (config scaffold, commands, generated skills/commands)
  and nothing user-owned.
- `git status --porcelain` — confirm the project worktree is otherwise clean
  before the migration-change commit.

Record what you inspected and what you found. Do not edit anything yet.

## Prompt 2: Verify no consumer migration is required

Givn 0.4.0 introduces no domain, configuration, specification, or architecture
migration for consumer projects. Verify, using project evidence, that:

- No project specification, step definition, ADR, Arc42 fact, or user-authored
  document needs a 0.4.0 edit.
- No Cargo feature or installation-mode contract changed for consumers.
- The 0.4.0 delivery pipeline (`.github/workflows/`, `cliff.toml`,
  `deny.toml`, `dependabot.yml`) is repository tooling, not consumer project
  content — do not copy it into the consumer project.

If any project-owned surface genuinely needs attention, record it as a
project-specific task with evidence; do not invent work that the project does
not have.

## Prompt 3: Verify and close

- Run `givn status` and confirm the project is healthy.
- If `0.3.0` differs from a previous line, confirm the 0.4.0
  managed scaffold supersedes it without leftover version markers.
- Derive project-specific tasks for this maintenance change, pass its
  design-review and review gates, and archive it when complete.
- Do not add `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence
  to the generated plan; those are earned through the normal Givn gates.

## Required Boundaries

- Do not automatically edit domain specifications or architecture decisions.
- Do not add a product feature spec for this maintenance phase.
- Do not remove `@e2e`, semantic evidence, or review gates to obtain a pass.
- This phase is ordered after the 0.2.0-to-0.3.0 phase in an aggregate plan.


### Phase 2: migrate-0-4-0-to-0-5-0 (0.4.0 -> 0.5.0)

# Migration Design: 0.4.0 to 0.5.0

This bundle is the release-owned migration phase for projects upgrading to
Givn 0.5.0. It is a behavioural major release, not delivery tooling: the
semantic similarity gate leaves the default build, the specification corpus
gains a use-case group layout, deterministic discovery replaces embedding
search, and several gates tighten. When this bundle is materialized,
`0.3.0` is the project's current config version,
`0.5.0` is `0.5.0`, and the managed preparation commit is
`8cd0d581f25b1f55f4975e8e6040139a11b2b235`.

## What changed, in project-visible terms

1. **Feature-free default build.** `spec-retrieval`,
   `spec-similarity-gate`, `cross-encoder-reranker`, and `nli-evidence`
   are no longer default features. `givn spec index`, `spec search`,
   `spec explore`, and `spec review` report retrieval as unavailable
   unless the binary was built with those features explicitly. The
   blocking semantic-review classification step inside
   `givn check review` is compiled out of the default build.
2. **Use-case groups.** The permanent corpus layout is
   `givn/specs/<group>/<capability>.feature` with one `group.md` per
   group (Actor, Goal, Main flow, Interactions, Includes, Extends). The
   flat `givn/specs/<cap>/<cap>.feature` layout still archives and lints,
   but `givn spec tree`, `spec find`, `spec duplicates`, and `spec route`
   only see the grouped layout.
3. **Deterministic discovery.** `spec tree` reports groups, counts, and
   phrasing reuse; `spec find` is lexical AST search; `spec duplicates`
   reports only exact/structural matches; `spec route` ranks
   EXTEND/NEW candidates for a proposal; `spec regroup` prints a grouping
   proposal and never modifies files.
4. **Capability retirement.** A zero-scenario delta tagged
   `@givn.retired @<capability>` deletes the whole capability at archive
   time and records a tombstone in `givn/specs/.retired.yaml`. review.md
   must disclose each retirement with a `RETIRE: <capability>` line.
5. **Disposition decisions are enforced.** In review.md, the
   `## Overlap dispositions` Decision column accepts only `duplicate`,
   `variant`, or `boundary` (case-insensitive). Empty or unrecognised
   words no longer clear a shape-match finding. `## Overlap
   dispositions` and `## Split-or-keep` are exact headings the review
   template now ships.
6. **Interaction inventory home.** The `## Interactions` section of the
   owning group's `group.md` is the durable inventory. Comment blocks at
   the top of delta `.feature` files are no longer read. `givn lint`
   warns when a group's declared interaction count and its `@e2e`
   scenario count disagree. A `@e2e`-tagged Scenario Outline counts once.
7. **Protected review artifact.** `givn/config.yaml` overlays that remove
   the `review` artifact are rejected at load time.
8. **Capability Routing.** The proposal template ships a
   `## Capability Routing` table. `givn check review` compares recorded
   decisions (`EXTEND <cap>` / `NEW in <group>`) against the authored
   delta; a missing table produces a loud warning, a present table is
   enforced strictly. `givn spec route` is the advisory recommender and
   is never re-run by the gate.
9. **Tighter lint.** A `.feature` file with zero scenarios is an error
   (exit 3) unless it is a `@givn.retired` marker. `givn lint --change`
   now runs the overlap scan scoped to the change. The group
   relationship graph (Includes/Extends) is machine-checked: dangling
   targets, unparseable group.md files, and cycles are errors.
10. **Merge projection.** One delta may remove a scenario and re-add a
    replacement with the same title; delta operations are validated
    against the projected state.

## Prompt 1: Inventory the migration boundary

Inspect the consumer project before editing anything:

- `git status --porcelain --untracked-files=all` — must be clean before
  the machine-safe upgrade.
- `givn/config.yaml` — record `givn_config_version`, every uncommented
  override, enabled addons, and in particular any `artifacts:` entry with
  `remove: true` (the `review` artifact can no longer be removed).
- Every project-owned override under `givn/artifacts/`, `givn/skills/`,
  and `givn/commands/` — read each ejected template or instruction and
  note whether it still contains the retired semantic sections
  (`## Semantic review classifications`, `## Semantic remediation
  verification`, `## Semantic Review and Remediation`) or lacks the new
  sections (`## Retirements`, `## Overlap dispositions`,
  `## Split-or-keep`, `## Capability Routing`).
- The permanent corpus layout: is `givn/specs/` flat
  (`<cap>/<cap>.feature`) or grouped (`<group>/<cap>.feature`)? Count
  capabilities and note any capability directories that describe the
  semantic engine (model download, token caps, reranker budgets, review
  output, evidence layers).
- Every active change: its proposal (has it a Capability Routing
  table?), its review.md (disposition decision words used?), its delta
  `.feature` files (interaction inventory comment blocks at the top?),
  and its `@wip` tags.
- Whether any project workflow still invokes `givn spec index`,
  `spec search`, `spec explore`, or `spec review`, or parses the old
  duplicate-audit output format.

Record an evidence table: `path`, `owner`, `current state`,
`0.5.0 action`, `risk`, `verification`. Do not edit anything yet.

## Prompt 2: Confirm the machine-safe upgrade

- Confirm Commit A (`8cd0d581f25b1f55f4975e8e6040139a11b2b235`) contains only managed paths
  (config scaffold, commands.yaml, generated skill/command targets,
  Givn-owned fenced blocks).
- Confirm `givn_config_version` now identifies `0.5.0`.
- Confirm the maintenance change `givn/changes/migrate-0-3-0-to-0-5-0/`
  exists with proposal.md, design.md, and `.givn-skip` containing
  `specs`; confirm no product feature spec, no fabricated sign-off.
- Re-run `givn skills sync` only if a generated file is stale.

## Prompt 3: Reconcile generated guidance and overrides

Bring every project-owned override to the 0.5.0 contract:

1. For each ejected instruction/template flagged in Prompt 1, re-eject
   the 0.5.0 embedded version (`givn eject <facet>` where the project
   wants an override at all) and re-apply only genuinely project-specific
   edits on top. Do not hand-patch the retired semantic sections back
   into existence.
2. Verify with `givn instructions review --change <maintenance-change>`
   that the resolved review guidance contains the deterministic
   disposition contract (`duplicate` / `variant` / `boundary`,
   `## Overlap dispositions`, `## Split-or-keep`) and no semantic
   classification contract.
3. Verify the resolved specs instruction carries the interaction
   inventory policy pointing at `group.md` and the canonical E2E policy.
4. Record the resulting override diff in the maintenance change's design
   or tasks evidence.

## Prompt 4: Project-specific migration decisions

Work through these decisions in order. Each is a human decision the LLM
prepares and executes with explicit confirmation; none is automatic.

1. **Semantic features.** Decide, with the project owner, whether the
   project still wants embedding retrieval. Default: accept the
   feature-free build (the deterministic gates cover duplicates and
   overlap). If retained, document the build invocation
   (`cargo install givn --locked --features spec-retrieval,...`) in the
   project README and keep `givn spec index` in the workflow. Record the
   decision in the maintenance change.
2. **Corpus layout.** If the corpus is flat, run
   `givn spec regroup --format json`, present the proposal to the human,
   and apply only the confirmed grouping: `git mv
   givn/specs/<cap>/<cap>.feature givn/specs/<group>/<cap>.feature`
   (renames, never delete+add), remove emptied directories, scaffold one
   `group.md` per group from the shipped template, and write real
   Actor/Goal/Main flow/Interactions prose — no placeholder text.
   Capabilities with no dominant command surface go into a support-style
   group; they are `«include»` fragments, not use cases. Record one
   relationship per bullet on the depending side (`## Includes` /
   `## Extends`), keep the graph acyclic, and use exact group or
   capability names as targets.
3. **Interaction inventories.** Move any inventory comment block at the
   top of a delta or permanent `.feature` file into the owning group's
   `group.md` `## Interactions` section, one bullet per distinct user
   action. Check `givn lint` for interaction over/under-coverage per
   group and reconcile the declared count with the implemented `@e2e`
   count.
4. **Retire dead capabilities.** For each capability that only described
   removed behaviour (for example semantic-engine capabilities in a
   project that chose the feature-free default), author a zero-scenario
   delta tagged `@givn.delta @givn.retired @<capability>`, add a
   `RETIRE: <capability> — <reason>` line to review.md, and archive the
   change through the normal workflow.
5. **Disposition vocabulary.** For every active change's review.md,
   replace free-form Disposition column values with exactly one of
   `duplicate`, `variant`, `boundary`; ensure the section headings are
   exactly `## Overlap dispositions` and `## Split-or-keep`.
6. **Protected artifact.** If `givn/config.yaml` removes `review`,
   replace the patch with `optional: true` if a skip is intended, or
   remove the patch; the gate itself must not be deleted.
7. **Capability Routing.** For each active change still in flight, run
   `givn spec route --change <id>` and fill the proposal's Capability
   Routing table. Future changes get the table from the template
   automatically.
8. **Workflow references.** Update project documentation and scripts that
   invoke the removed duplicate-audit output format or the
   comment-block inventory location. `givn spec duplicates` now prints
   `scanned permanent corpus: N exact/structural match(es)` or a JSON
   array of matches with `kind`, `left_file`, `left_title`, `right_file`,
   `right_title`.

Apply specification edits through the normal Givn lifecycle (delta →
design-review where the change is architectural → tasks → review →
archive), one concern per commit. Do not claim work that has not been
verified, and never weaken a gate to make a check pass.

## Prompt 5: Verify and archive

1. `givn lint` — confirm zero errors; review each warning (shape match,
   subset, long scenario, singleton group, interaction coverage) and
   disposition it with a valid decision word where the review contract
   requires it.
2. Run the project's `verify.command` and `verify.e2e_command`.
3. Post-migration smoke: `givn spec tree` lists the groups with Actor and
   Goal; `givn spec find <known-term>` returns the expected hit;
   `givn spec duplicates` runs without a model and without an index;
   `givn spec route --change <maintenance-change>` runs (advisory only).
4. For every active change, run `givn check review --change <id>` and
   resolve every blocking finding. In a feature-free build the semantic
   gate is absent; the structural, integrity, retirement-disclosure, and
   routing gates remain.
5. Confirm `givn/config.yaml` loads (a protected-artifact removal now
   fails loudly) and the effective manifest still contains `review`.
6. Complete the maintenance change through design-review, tasks, and
   review with real evidence, then
   `givn archive --change migrate-0-3-0-to-0-5-0`.
7. After archive: `givn status` clean, no synthetic migration feature
   under `givn/specs/`, and the final Git history shows the managed
   commit, the migration change commit, and project-specific commits with
   their own rationale.

## Completion Definition

A project is migrated when:

- the binary is 0.5.0 and `givn_config_version` matches;
- generated guidance and every project override follow the 0.5.0
  contract (no retired semantic sections, new sections present);
- the specification corpus layout decision is applied and the grouped
  tools work, or the deferral is recorded with the flat-layout
  consequences understood;
- every active change's evidence satisfies the deterministic gates;
- the semantic-feature decision (opt-out default or documented opt-in)
  is recorded;
- project-owned content survives intact; and
- the maintenance change is archived as operational history without a
  synthetic capability in `givn/specs/`.


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood.
