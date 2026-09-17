# Design: migrate-0-7-0-to-0-8-0

## Migration metadata

- source_version: `0.7.0`
- target_version: `0.8.0`
- upgrade_commit: `7d1d3988ace0394cb1fb29b9dfa06d9729143186`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Work the following phases strictly in sequence. Follow each bundle's own completion rules; a bundle may direct that the repairs run as named follow-up changes. The complete sequence is also recorded in `migration.yaml`.

### Phase 1: migrate-0-7-0-to-0-8-0 (0.7.0 -> 0.8.0)

# Migration Design: 0.7.0 to 0.8.0

Self-contained procedure for the project LLM. You have no access to the Givn
source repository; everything required is in this document. The managed
refresh and the `givn_config_version` bump are already done by `givn upgrade`
(Commit 7d1d3988ace0394cb1fb29b9dfa06d9729143186). This maintenance change carries the
project-specific work.

## Execution model

This change is the **inventory and plan**, not the repair site.

- Build one row per artifact deviation with its disposition and the follow-up
  change that will fix it. The repairs run in the follow-up changes through
  the normal gates (proposal, specs, design, review, archive), never in this
  change.
- This change edits no corpus artifact except the term collision-check cells
  (evidence gathering, Phase 4).
- This change archives once every row has a disposition and a named or
  in-flight follow-up; the inventory is the durable record. The migration is
  complete when the follow-up changes archive.
- Work the phases in order; within a phase, inventory one artifact kind at a
  time and stop to ask the human when a disposition is not derivable from the
  project's own artifacts.

Follow-up change ids are kebab-case `<verb>-<object>` (for example
`split-run-a-project`). An id is **in-flight** when `givn/changes/<id>/`
exists at inventory time; otherwise it is **named** (recorded here, not yet
scaffolded). An archived change is never named as a pending follow-up.

## Inventory and dispositions

Record the inventory in this change's `design.md` (append it below this
bundle text) as one table per artifact kind, using this shape:

```markdown
| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| givn/specs/run-a-project/usecase.md | R1 goal conjunction, R9 oversized leaf | split into child leaves | split-run-a-project |
```

Every row names exactly one deviation family and one follow-up. When a
`proposed` ADR's owning change has already archived or does not exist,
re-qualify the record in this change (accepted, or rejected with a canonical
destination) or create a new named follow-up; never name an archived change.

## Preflight

1. Confirm `givn/config.yaml` declares `givn_config_version:
   "0.8.0"` and this change exists at
   `givn/changes/migrate-0-7-0-to-0-8-0/`.
2. Confirm the checkout is clean and committed before the first edit.
3. Inventory the corpus: `givn spec tree`, the permanent `givn/specs/`
   directories, `givn/personas/`, every `givn/ideation/*/personas/`,
   `docs/arc42/adr/`, and the term and seed records.
4. Start the disposition tables; one row per artifact kind.

## Enforcement boundary

Only two of the rules below are machine-checked:

- `givn lint` reports a dangling `## Personas` reference as an advisory
  `[PERS ]` finding.
- `givn check review` resolves a declared visual interface's `PERSONAS:`
  value against `givn/personas/`.

The use-case scoping rules, the persona identity criteria, and the ADR
falsification tests are authoring and review guidance. A green suite does not
by itself prove this migration; the checks are the recorded dispositions, the
follow-up changes, and the human-confirmed diffs.

## Phase 1 — Use-case model (inventory)

These are inventory criteria: list the failing criteria per use-case
document and name the follow-up that repairs them. The repair itself runs in
the follow-up change.

### The definition

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

### The eight scoping rules

1. **One goal, one primary actor.** The goal is one active-verb phrase naming
   the primary actor's outcome. An "and" joining two verb phrases in Goal, or
   two distinct outcomes in the success guarantee, means split. Closure test:
   the actor could stop afterward and the system is left consistent.
2. **Goal-shaped, never component-shaped.** No use-case or capability name
   may name a technical component, mechanism, or channel (server, backend,
   store, service, panel, browser, CLI). Placement test for every capability:
   which actor goal does completing this advance? Channel is never an answer;
   a browser or CLI variant of an action lives under the action's goal. A
   failing name is renamed in the follow-up, which moves the `.feature` file
   and updates every reference.
3. **Level discipline.** `!` owns capabilities directly. `+` owns zero direct
   capabilities and only child `!` via `Includes: usecase:`; its coverage is
   the union of its children. A leaf whose flow names three or more distinct
   actor goals, or whose success guarantee is a conjunction, is promoted to
   `+` and split. `-` stays ideation-only.
4. **Mutation containment.** The guarantees declare the case as observing
   (read-only) or changing. An observing case owns zero mutating
   interactions; a capability whose scenarios assert a state change belongs
   under a goal whose success guarantee names that change.
5. **Extension discipline.** Every extension names the main-flow step it
   branches from and the guarantee it protects or the recovery it performs.
   An extension introducing a new actor goal, a new success outcome, or a new
   capability is a new use case or a capability under another goal — never an
   extension.
6. **Precondition and flow consistency.** Preconditions hold before the
   trigger and are never established by the main flow. The success guarantee
   is established by the flow, never assumed. The trigger is an actor action
   or event, never a state or desire.
7. **Implementation independence.** Goal, Trigger, Main flow, and Guarantees
   must remain true if the system were rebuilt in another language and
   framework. Mechanism vocabulary (branches, compare-and-swap, loopback,
   tokens, manifests, signals) never appears in those sections. Non-human
   actors use "X for Y" notation; a bare system actor's behavior is a
   subfunction.
8. **Relationship semantics.** `Includes` means unconditional reuse (summary
   to child, leaf to fragment substrate). `Extends` means optional insertion
   under a stated condition recorded in the extending document; never targets
   fragments.

### The ten review rejection criteria

| # | Criterion | Proving section |
|---|---|---|
| R1 | Goal conjunction | `## Goal` joins two verb phrases, or `## Success guarantee` names two outcomes |
| R2 | Missing primary actor | `## Actors` has no identifiable primary, or `## Trigger` does not name them |
| R3 | Placement failure | `## Capabilities` contains a capability that fails the placement test |
| R4 | Mutation under an observing goal | A capability's scenarios assert a state change while the guarantees declare observation |
| R5 | Extension without a step reference | `## Extensions` omits its main-flow step, or adds a new success outcome |
| R6 | Inconsistent preconditions or flow | A precondition is established by the flow; the success guarantee is assumed; the trigger is a state or desire |
| R7 | Mechanism vocabulary | `## Goal`, `## Trigger`, or the guarantees name a mechanism |
| R8 | Component- or channel-shaped name | A use-case or capability name names a technical component or channel |
| R9 | Oversized leaf | A leaf's `## Main flow` names three or more actor goals |
| R10 | Relationship misuse | `Extends` without a stated condition; an end-user goal placed as a fragment or vice versa |

The same criteria ship in `givn instructions review`; the generated ideation
guidance carries the definition and rules. Use them as the live references.

### Inventory rows

One row per failing document, for example:

```markdown
| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| givn/specs/run-a-project/usecase.md | R1 goal conjunction, R9 oversized leaf | promote to `+` and split into child leaves | split-run-a-project |
| givn/specs/follow-the-run/usecase.md | R4 mutation under an observing goal, R8 channel name | re-home mutations; rename | refocus-follow-the-run |
| givn/specs/run-a-project/repository-backend.feature | R8 component-shaped name | rename capability and file | rename-repository-backend |
```

The follow-up performs the repair: split, promote, re-home, rename, or
rewrite extensions, one capability per edit, running the affected feature
files and `givn lint` after each move.

## Phase 2 — Personas (inventory)

### Identity criteria

A new persona exists only when it differs from an existing one in at least
one of — the criteria are disjunctive, not conjunctive:

- **authority boundary** — what it may do or decide;
- **evaluation capability** — what it can judge;
- **own vocabulary** — terms it uses or refuses.

Topic-specific demands go into a dated topic-stances appendix, never into the
stable core. One role sliced by which mechanism it touches is one persona
with stance blocks when the authority and evaluation match.

### File schema

```markdown
# Persona: <name>

## Stable core

- Role: <one line>
- Authority boundary: <what it may do or decide>
- Evaluation capability: <what it can judge>
- Own vocabulary: <terms it uses or refuses>
- Success criterion: <what success means for it>

## Running record

Dated entries per review or topic stance; append-only, never rewriting
earlier entries.

- <YYYY-MM-DD> — <entry>

## Topic stances

Topic-specific demands, dated, never in the stable core.

- <YYYY-MM-DD> — <topic>: <stance>
```

A reused persona's topic-local file contains a header line plus new record
entries only — never a biography copy:

```text
Reuse: <persona-id>, base <short-hash or date>; this file adds record entries only
New: <persona-id> — <distinguishing criterion>
```

### Inventory rows

One row per persona file, reference, or topic copy: identity-criteria
deviation (which segments collapse and why), schema gap, dangling
reference, or biography copy. The follow-up consolidates, migrates, converts
to the `Reuse:` header, append-merges at handoff with a user-confirmed diff,
and resolves the reference.

Preserved behaviors: the user confirms every persona before use; personas
review every artifact that affects their stakeholder type; at most three
segments per stakeholder type and about ten in total; findings are advisory
with user arbitration; promotion never happens automatically.

## Phase 3 — ADRs (re-qualification here, repairs in the follow-up)

### Falsification tests

A candidate fails the qualification gate when any of these holds, no matter
how the five dimensions were argued:

1. **Single-change reversibility.** Fully implementable and reversible within
   one change, touching one component. File count or interface breadth does
   not change this. Non-example: a project identity key split is a data-model
   decision owned by the design and the glossary.
2. **No multi-change constraint.** Does not constrain at least two future
   changes or two components. Non-example: a terminal state value plus its
   presentation filtering is feature behavior owned by the capability spec
   and design.
3. **Product-behavior exclusion.** Observable workflow behavior ownable by a
   use case plus Gherkin (who authorizes what, in which order, with which
   state values) fails `lower_level_artifact` unless it also moves a
   component, authority, or deployment boundary stated in the verdict.
   Non-examples: an assumption-and-go-ahead gate and a human evaluation gate
   are workflow behavior; a change-request entity with crash-safety
   requirements is a domain entity.

The tests classify by kind, not count; there is no numeric cap.

### Procedure

1. For every **accepted** record in `docs/arc42/adr/`, re-run the five
   mandatory dimensions plus the three falsification tests and the
   cheaper-home challenge: name the specific canonical artifact (feature
   specification, design, Arc42 chapter, process documentation, project
   documentation, or code) that would own the rationale instead. Record the
   verdict and the audit note here.
2. A qualified record stays. A record that fails gets a row: the rationale
   moves once to the chosen canonical home, the record leaves the active
   register (`docs/arc42/09-...`) and `docs/arc42/adr/README.md`, and this
   change keeps the audit note (record ID, failing test, destination). The
   removal and the rationale move are the follow-up's work. Do **not** copy
   the record into `docs/arc42/adr/archive/`: a record that fails the gate is
   not a historical ADR.
3. Every **proposed** record gets a row naming its owning change as the
   follow-up that must qualify it before that change archives. If the owner
   has archived or does not exist, re-qualify the record here or create a new
   named follow-up; never name an archived change as pending.
4. A project-local ADR template copy that carries a pre-passed verdict gets a
   row: the follow-up deletes `docs/arc42/adr/adr-template.md`, runs
   `givn addons enable arc42` to re-scaffold missing files, and confirms the
   file no longer contains `ADR-QUALIFICATION: PASS` or
   `qualification: QUALIFIED`.
5. Record the structured verdict (qualification, the five gates,
   supporting-only shared evidence, routing, and the route target) in the
   change assessment (`arc42.md`) before creating or editing any MADR.
6. New decisions follow the same gate: the design's Technology Decisions
   justification stays in `design.md` unless the falsification tests pass.

## Phase 4 — Smaller adoptions (inventory, one exception)

1. **Dialog policy.** Read `givn instructions dialog` and follow it in every
   interactive step. This is reading, not a repair; no row.
2. **Change seeds.** Every seed gets a row: classify it with one of the five
   statuses (`ready`, `proposed by <change-id>`, `implemented in
   <change-id>`, `superseded by <reference>`, `withdrawn`) and list the
   missing content schema (use-case ID, topic, Personas, suggested start,
   accepted decisions with option treatment, full open questions). A consumed
   seed stamped `ready` is a deviation; `givn lint` reports a missing or
   contradicted status as a `[SEED ]` advisory.
3. **Term records.** Migrate `domain-terms.md` entries to
   term/status/definition/contrast/first-use/collision-check with
   `candidate`, `confirmed`, and `rejected` statuses. **This is the one
   exception to the no-repair rule**: re-run every existing `confirmed`
   term's search now and record the commands and hit counts in the
   collision-check cell; "no collision" without commands is not a record. A
   collision found becomes a follow-up row that renames or rejects the term.
   Rejected terms keep their definition and reason and are never deleted.
4. **Guarantee qualifiers and decision ledger.** Rows for every use-case rule
   or guarantee with an unresolved prerequisite (add `(open: Q<N>)`) and
   every proposal question missing a `D<n>` ID, option treatment, or
   disposition; the follow-up adds them and the design records
   `Decides D<n>: <decision> — <where it is realized>`.
5. **Severity and routing output.** Re-read `givn lint` (findings carry
   `advisory`, `finding`, or `blocking`; advisory findings do not change the
   exit code) and `givn spec route` (`signal` or `no signal`). This is
   reading, not a repair; no row.

## Phase 5 — Ejected overrides (inventory)

1. Inventory overrides: uncommented settings in `givn/config.yaml`, files
   under `givn/artifacts/`, and project skill overrides under `givn/skills/`.
   One row per override that carries pre-0.8.0 rule text or a
   stale default.
2. The follow-up re-ejects each facet from the 0.8.0 binary and
   re-applies only genuine project-specific edits; it never hand-patches the
   old rule text back into a stale copy. Managed generated files (skills,
   commands, AGENTS.md blocks) were already refreshed by `givn upgrade`; only
   ejected facets are project-owned.

## Validation and completion

1. This change edited no corpus artifact except the term collision-check
   cells; every row names a follow-up change or an in-flight one; no archived
   change is named as pending.
2. `givn lint` and `givn check review` pass; the `[PERS ]` advisories are
   inventoried or explicitly recorded as forward references.
3. The project's own suite passes (`verify.command`).
4. The inventory is committed with this change and the maintenance change
   archives through the normal flow; the migration is complete when every
   follow-up change has archived.


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood, including any follow-up changes a bundle names.
