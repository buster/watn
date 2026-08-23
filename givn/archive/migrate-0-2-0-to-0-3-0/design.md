# Design: .migrate-0-2-0-to-0-3-0.tmp-2487432

## Migration metadata

- source_version: `0.2.0`
- target_version: `0.3.0`
- upgrade_commit: `d319e3e6aa04884fa53864d7548408bebc9e7c5e`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Complete the following phases strictly in sequence. Do not create independentparallel migration changes for these bundles. The complete sequence is alsorecorded in `migration.yaml`.

### Phase 1: migrate-0-2-0-to-0-3-0 (0.2.0 -> 0.3.0)

# Migration Design: 0.2.0 to 0.3.0

This bundle is the canonical release-owned migration phase for projects created
from Givn 0.2.0. The complete public prompt pack is embedded below into the
generated aggregate design by the migration engine. Future bundles must contain
their complete prompts here rather than relying on the consumer having this
source repository available.

## Required LLM Actions

1. Inventory the consumer project's config, commands, generated integrations,
   active changes, permanent specs, ejected facets, and optional Arc42/ADR
   documents before editing.
2. Confirm the machine-safe managed upgrade commit and inspect its allowlist.
   The allowlist may include generated paths ignored by the consumer's
   `.gitignore`; Givn force-stages only those exact managed paths, never a
   directory or the complete worktree.
3. Synchronize generated guidance without overwriting project-owned overrides.
4. Review active changes and permanent specifications against the 0.3.0
   interaction, overlap, semantic, E2E, and review contracts.
5. Reconcile Arc42 and ADR content using project evidence and the canonical
   qualification procedure; do not invent decisions or provenance.
6. Verify all applicable commands and archive the maintenance change only after
   its design-review, tasks, and review gates pass.

## Required Boundaries

- Do not automatically edit domain specifications or architecture decisions.
- Do not add a product feature spec for this maintenance phase.
- Do not remove `@e2e`, semantic evidence, or review gates to obtain a pass.
- Keep this phase ordered before the next migration bundle in an aggregate
  plan.

## Full Prompt Pack

The following marker is replaced with the complete version-specific prompt
pack when this bundle is materialized. Future bundles must carry their full
prompt content directly in this asset.

# Givn 0.2.0 to 0.3.0 Migration Guide

This guide migrates a project created from the Givn 0.2.0 master branch to the
Givn 0.3.0 contract. The source is the master branch, not only the last stacked
PR. The target includes the complete stacked change set that is present in the
0.3.0 binary. A normal Cargo installation enables all declared retrieval,
strict E5, BGE, and NLI features; `--no-default-features` is the explicit
feature-free installation path.

The guide is an LLM prompt pack. Run the prompts in order from the project root.
The LLM must inspect the project before editing it and must preserve project
ownership boundaries. The prompts are deliberately explicit because a version
marker alone cannot decide whether a project's domain specification or
architecture documentation is still correct.

## Migration Contract

The 0.3.0 binary performs the machine-safe preparation automatically when all
of these conditions hold:

- `givn/config.yaml` contains the active marker `givn_config_version: "0.2.0"`;
- the project is inside a Git worktree;
- the worktree is clean, including untracked files; and
- `migrate-0-2-0-to-0-3-0` does not already exist in active or archived changes.

The automatic preparation updates the managed config scaffold and regenerates
managed agent integrations. It creates two commits:

1. Commit A contains only the managed config, generated skill/command targets,
   and Givn-owned fenced blocks.
2. Commit B contains the maintenance migration change and records Commit A's
   hash.

The generated maintenance change is
`givn/changes/migrate-0-2-0-to-0-3-0/`. It contains a populated proposal, a
detailed design, and `.givn-skip` with `specs`. It does not contain a synthetic
product feature, a fabricated design-review pass, implementation evidence, or
review sign-off. The project LLM must complete the remaining Givn workflow.

If the worktree is dirty or is not a Git repository, `givn upgrade` refuses
before writing. Resolve that boundary first; do not bypass it by staging the
whole project.

The migration guide does not upgrade the Givn binary itself. Install or select
the target binary separately, then use `givn upgrade` only against the consumer
project.

`givn_config_version: "0.3.0"` means that the managed scaffold preparation is
complete. It does not mean that the project-specific migration is complete. The
maintenance change is the visible completion worklist.

## Aggregate Migrations

The same release mechanism supports later version gaps. Givn ships migration
bundles in `assets/migrations/catalog.yaml`. If several bundles connect the
project version to the installed binary, `givn upgrade` creates one aggregate
maintenance change with `migration.yaml`, ordered bundle phases, and all
Proposal/Design prompt content. Complete those phases strictly in sequence;
do not create competing migration changes for individual bundles. If the
catalog contains no route, `givn upgrade` performs only the managed config and
generated-guidance refresh and creates no maintenance change.

## Prompt 1: Inventory

Copy the following prompt to the LLM working in the Givn project:

```text
You are migrating a project created with Givn 0.2.0 to Givn 0.3.0. Work from
the project root. Do not edit files in this prompt.

Inspect the migration boundary before making any change:

1. Run `git status --short --branch`, `git log --oneline -10`, and
   `git rev-parse --show-toplevel`. Stop if the worktree is not clean or if
   this is not a Git worktree.
2. Read `givn/config.yaml` and record the active `givn_config_version`, every
   uncommented override, the persisted `skills.targets` if present, and every
   enabled addon. Read `givn/commands.yaml` if it exists.
3. Run `givn status`, `givn graph`, and `givn addons list`. Record every active
   change, every archived change, and every ejected override under
   `givn/artifacts/`, `givn/skills/`, or `givn/commands/`.
4. List generated agent files under `.agents/`, `.claude/`, and `.opencode/`,
   and identify which files are expected outputs versus project-authored files.
5. If Arc42 is enabled, inspect `docs/arc42/README.md`, all affected chapters,
   `docs/arc42/09-architecture-decisions.md`, and `docs/arc42/adr/README.md`.
6. Inspect the permanent `givn/specs/` tree and every active change's
   `specs/`, `design.md`, `tasks.md`, and `review.md` where present. Note old
   E2E inventory wording, old guidance copied into artifacts, unresolved
   `@wip`, duplicate titles, and any semantic reports.

Return an evidence table with these columns:
`path`, `owner`, `current state`, `0.3.0 action`, `risk`, `verification`.
Do not change a domain specification, step definition, source file, ADR,
Arc42 fact, ejected facet, or user-authored document during inventory.
```

Expected result: a project-specific inventory and no file changes.

## Prompt 2: Machine-safe upgrade

Copy the following prompt only after Prompt 1 confirms a clean Git boundary:

```text
Prepare the machine-safe part of the Givn 0.2.0 to 0.3.0 migration.

1. Confirm again that `git status --porcelain --untracked-files=all` is empty.
   If it is not empty, stop without writing.
2. Run `givn upgrade` with the Givn 0.3.0 binary. Do not use `git add -A`.
3. Capture the command output, both commit hashes, and the path of
   `givn/changes/migrate-0-2-0-to-0-3-0`.
4. Inspect Commit A with `git show --stat --oneline <commit-a>` and
   `git show --name-only <commit-a>`. Confirm that it contains only
   `givn/config.yaml`, generated Givn skill/command outputs for the configured
   targets, and Givn-owned fenced-block changes. These explicit managed paths
   may be force-staged when the project `.gitignore` excludes generated agent
   files; this does not authorize force-staging a directory or the worktree.
   If any unrelated path is in Commit A, stop and report it; do not repair
   history automatically.
5. Inspect Commit B with `git show --stat --oneline <commit-b>` and confirm
   that it contains only the migration plan. Confirm the plan records exactly
   Commit A's hash.
6. Read the generated proposal and design in full. Confirm `.givn-skip`
   contains `specs`, no product feature file was invented, and no
   `DESIGN-REVIEW: PASS`, implementation commit, or `REVIEW: PASS` was
    fabricated.

7. If the binary was built from source, record whether it was installed with
   the normal all-feature Cargo defaults or with `--no-default-features`; do
   not claim retrieval-capable behavior for the feature-free build.

Do not edit domain specs, step definitions, source code, ADRs, Arc42 facts,
ejected facets, or user-authored documentation in this prompt. Do not delete
old agent directories merely because the current target selection is narrower.
Return the two commit hashes and the exact path review as evidence.
```

Expected result: the config marker is 0.3.0, Commit A and Commit B are visible,
and the maintenance change is active at the next required Givn artifact.

## Prompt 3: Synchronize generated guidance

Copy the following prompt to the LLM after the automatic preparation:

```text
Bring the generated Givn guidance to the 0.3.0 contract without overwriting
project-owned overrides.

1. Open `givn/changes/migrate-0-2-0-to-0-3-0/design.md` and read its recorded
   Commit A hash and generated-path boundary.
2. Read the resolved current policies with:
   `givn instructions specs --change migrate-0-2-0-to-0-3-0`,
   `givn instructions design --change migrate-0-2-0-to-0-3-0`,
   `givn instructions review --change migrate-0-2-0-to-0-3-0`, and, when
   Arc42 is enabled, `givn instructions arc42-docs --change migrate-0-2-0-to-0-3-0`.
3. Compare generated files in `.agents/`, `.claude/`, and `.opencode/` with
   the current embedded guidance. Use `givn skills sync` only when a generated
   file is stale or the prior upgrade could not complete it.
4. Preserve every project override under `givn/` and all content outside
   Givn-owned fenced blocks in `AGENTS.md`. If a generated file has intentional
   project edits that are not represented by an ejected override, stop and
   record the conflict instead of silently deleting those edits.
5. Keep the canonical interaction/E2E policy in the resolved `specs`
   instruction. Remove copied, contradictory policy only after confirming that
   it is generated content; do not rewrite project-authored guidance.

Record the resulting generated-file diff in the maintenance change's design or
tasks evidence. Do not create a second competing policy source.
```

Expected result: generated guidance follows the 0.3.0 policy, selected agent
targets remain intentional, and project overrides remain effective.

## Prompt 4: Review project changes

Copy the following prompt to the LLM after Prompts 2 and 3:

```text
Complete the project-specific analysis in the active maintenance change
`migrate-0-2-0-to-0-3-0`. Use the normal Givn lifecycle and do not claim work
that has not been verified.

1. Run `givn status --change migrate-0-2-0-to-0-3-0` and perform the required
   design-review step. Derive tasks from the generated design only after
   checking the actual project inventory. Each task must name the exact file,
   evidence, and verification command.
2. Run `givn lint`. The default scope excludes `givn/archive/`. Treat duplicate
   active titles as errors. For shape matches, subsets, long scenarios, and
   removed/added replacements, complete the review disposition required by the
   current guidance. Do not dismiss a finding with an invented "future scope"
   exception.
3. For every active change, read its `.feature` files, design, tasks, and
   review. Apply the current canonical specs instruction: normalize the user
   interaction inventory, keep one E2E evidence scenario per distinct action,
   assert through the real interface, and keep browser UI tests on a real
   browser driver. Do not remove `@e2e` to make a gate pass.
4. If the binary has retrieval support, run `givn spec index` after the
   permanent specs are understood. Use `givn spec explore` or advisory
   `givn spec review --change <id>` for authoring evidence. Use blocking
   `givn check review --change <id>` only for the active change being reviewed.
   Classify every admitted semantic candidate as `DUPLIKAT`, `VARIANTE`, or
   `VALID-BOUNDARY` with evidence and rationale. Remediate the projected tree
   and rerun the report. Treat token-cap, unavailable, and unscored evidence
   as unresolved evidence, not as a classification.
5. If the project has `addons.arc42: true`, walk all twelve Arc42 chapter
   decisions, then reconcile only affected durable chapters. Apply the strict
   ADR qualification procedure from the resolved Arc42 instruction. Search the
   active and archived ADR register before adding anything. Route ordinary
   decisions to exactly one canonical lower-level artifact. Amend or supersede
   an existing ADR when required; do not duplicate it.
6. The LLM must not automatically edit domain specifications or architecture
   decisions. These edits require project evidence and the LLM's explicit review. Never
   invent a missing business invariant, alternative, consequence, ADR, or
   provenance. If evidence is missing, leave the item unresolved and report it.
7. Keep the maintenance change free of a product feature spec. The migration
   record is operational evidence; it must not add a synthetic capability to
   `givn/specs/`.

Create and complete tasks one scenario or project-specific migration concern at
a time. Use RED/GREEN/REFACTOR/COMMIT for executable behavior changes and record
the actual commit hash immediately after each completed task. Re-run design
review whenever the design changes.
```

Expected result: the maintenance change contains project-specific tasks and
evidence, while only justified project changes are made. A project may need
several implementation commits and several review cycles.

## Prompt 5: Verify and archive

Copy the following prompt after all project-specific migration tasks have been
implemented:

```text
Prove and close the Givn 0.2.0 to 0.3.0 migration.

1. Run `git status --short --branch` and inspect every changed path. Confirm
   that no unrelated file was included in the automatic commits and that all
   later project changes have their own rationale and commit evidence.
2. Run `givn lint` and record the result. Explicitly inspect any duplicate,
   shape, subset, long-scenario, parse, or WIP finding.
3. Run the project's configured `verify.command` and
   `verify.e2e_command`. Confirm both are real Gherkin/Cucumber runners and
   that the E2E command is genuinely scoped to E2E scenarios.
4. For every active change, run `givn check review --change <id>` when the
   project has retrieval support, resolve all blocking structural and semantic
   evidence, and record the final report. A feature-free build must retain the
   explicit retrieval-unavailable result rather than passing silently.
5. If Arc42 is enabled, complete the implementation conformance review against
   the durable chapters and ADR register. Ensure no qualified decision lacks
   its ADR and no non-qualified decision has been duplicated as an ADR.
6. Read `givn status --change migrate-0-2-0-to-0-3-0`. It may be archived only
   after its design-review, tasks, and review artifacts are genuinely complete.
   Do not add a fake `REVIEW: PASS` marker. Archive it with:
   `givn archive --change migrate-0-2-0-to-0-3-0`.
7. After archive, run `givn status`, inspect `givn/archive/`, and confirm that
   no migration feature appeared under permanent `givn/specs/`. Record the
   final Git diff and all verification outputs.

If any check fails, keep the maintenance change active, describe the blocker,
and continue through the appropriate Givn artifact. Do not weaken a gate by
removing tags, deleting evidence, or replacing a real-interface test with a
lower-level assertion.
```

Expected result: the maintenance change is archived as operational history,
the permanent domain specification has not acquired a synthetic migration
capability, and the project has evidence for every applicable 0.3.0 contract.

## Projects Without Git

The automatic two-commit route intentionally refuses projects without Git. A
project owner may still use this prompt pack manually after creating a clean
Git boundary, but the LLM must not claim that an automatic upgrade commit
exists when it does not.

## Completion Definition

A project is migrated when all of the following are true:

- the package in use is Givn 0.3.0 or a later binary whose migration contract
  is explicitly compatible;
- the managed scaffold and generated integrations have a reviewed Commit A;
- the migration maintenance change records Commit A and is archived only after
  its own gates pass;
- every applicable active change has current structural, E2E, semantic, and
  architecture evidence;
- project-owned overrides and user content remain intact; and
- `givn/specs/` contains no artificial migration feature.



## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood.
