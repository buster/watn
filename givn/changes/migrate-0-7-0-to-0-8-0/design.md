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

---

# Inventory (migrate-0-7-0-to-0-8-0)

Recorded 2026-09-17. Corpus inspected: `givn spec tree` (4 permanent use cases, 1 fragment), `givn/specs/`, `givn/personas/`, `givn/ideation/terminal-wow-factor/` (personas, terms, seed, questions), `docs/adr/` (26 records), `docs/arc42/` (chapters, register), `givn/config.yaml`, `givn/artifacts/`, `givn/skills/`.

This change edited no corpus artifact except the term collision-check cells in `givn/ideation/terminal-wow-factor/domain-terms.md` (Phase 4 exception). Every row below names exactly one deviation family and one follow-up change. No archived change is named as a pending follow-up.

## Phase 1 — Use-case model

| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| givn/specs/configure-interactive/usecase.md | R1 goal conjunction ("Complete Watn's guided setup flows" + "persist a working configuration"); level discipline: `!` owns six capabilities and also Includes `!` children; R8 channel-shaped use-case name (`interactive`); Includes overlap with configure-model | promote to `+` or split into child leaves, re-home the six capabilities, resolve the overlapping Includes, rename the use case | split-configure-interactive |
| givn/specs/configure-interactive/usecase.md | R5 extension ("Cancellation before confirmation leaves no unconfirmed state") omits its main-flow step and the guarantee it protects | rewrite the extension with its step reference and protected guarantee | split-configure-interactive |
| givn/specs/configure-interactive/unified-setup-wizard.feature | R8 component-shaped capability name (wizard UI component) | rename capability and file, update every reference | rename-configure-interactive-capabilities |
| givn/specs/configure-interactive/auto-init-config.feature | R8 component-shaped capability name (config artifact) | rename capability and file, update every reference | rename-configure-interactive-capabilities |
| givn/specs/configure-interactive/highlight-active-setup-input.feature | R8 presentation-mechanism capability name | rename capability and file, update every reference | rename-configure-interactive-capabilities |
| givn/specs/configure-interactive/responsive-setup-model-filtering.feature | R8 mechanism/quality-shaped capability name | rename capability and file, update every reference | rename-configure-interactive-capabilities |
| givn/specs/configure-interactive/setup-persistence.feature | R8 mechanism-shaped capability name | rename capability and file, update every reference | rename-configure-interactive-capabilities |
| givn/specs/configure-model/usecase.md | R1 goal conjunction (model tiers + catalog sources + reasoning behavior as three outcomes); level discipline: `!` owns eight capabilities and Includes the `!` child configure-provider | promote to `+` or split into child leaves, re-home the eight capabilities, resolve the Includes overlap | split-configure-model |
| givn/specs/configure-model/usecase.md | R5 extension ("A missing catalog source leaves the active provider unchanged") omits its main-flow step | rewrite the extension with its step reference | split-configure-model |
| givn/specs/configure-model/ratatui-model-picker.feature | R8 framework-component capability name (ratatui) | rename capability and file, update every reference | rename-configure-model-capabilities |
| givn/specs/configure-model/streamlined-setup.feature | R8 mechanism-shaped capability name | rename capability and file, update every reference | rename-configure-model-capabilities |
| givn/specs/configure-provider/usecase.md | R5 extension ("An incomplete provider request opens setup...") omits its main-flow step | rewrite the extension with its step reference | refocus-configure-provider |
| givn/specs/configure-provider/provider-setup-widget-layout.feature | R8 widget-layout mechanism capability name | rename capability and file, update every reference | rename-configure-provider-capabilities |
| givn/specs/use-shell/usecase.md | R1 goal conjunction (interactive shortcut that explains and refines + direct explanation entry point); R2 three listed actors with no declared primary and a bare system actor (`Shell line editor`, rule 7 notation); R7 mechanism vocabulary in Main flow and guarantees (`[DONE]`, command-output channel, controlling-terminal channel, insertion point) | split into child leaves, declare one primary actor, rewrite Main flow and guarantees in goal language | split-use-shell |
| givn/specs/use-shell/usecase.md | R5 every extension omits its main-flow step reference; R6 preconditions carry main-flow behavior ("A usable model fills in stage purposes; when none is usable...") | rewrite extensions with step references; move flow behavior out of preconditions | split-use-shell |
| givn/specs/use-shell/interactive-shell-shortcut.feature | R8 mechanism/channel-shaped capability name (shortcut) | rename capability and file, update every reference | rename-use-shell-capabilities |
| givn/specs/use-shell/shell-completions.feature | R8 mechanism-shaped capability name | rename capability and file, update every reference | rename-use-shell-capabilities |
| givn/specs/fragments/fragment.md | R8 mechanism capability names (transport, config, incremental-sse-rendering, search-concurrency) | rename capabilities and files, update every reference | rename-corpus-infra-capabilities |

Observations (no row): the ideation use-case `givn/ideation/terminal-wow-factor/use-cases/use-explanatory-shell-shortcut.md` is preserved exploratory material, not a permanent use case, and is not retrofitted; its capabilities were mapped into `use-shell`. The `configure-*` use cases record `Personas: none` without a reason — a forward reference for those use cases' next change.

## Phase 2 — Personas

| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| givn/personas/terminal-developer--interactive.md | schema gap: flat Role / Interests / Goals / Pain points / Vocabulary / Decision record instead of Stable core (Role, Authority boundary, Evaluation capability, Own vocabulary, Success criterion) plus Running record and Topic stances; topic-specific stances ("safe" = retained control; direct editing must not alter intent; cancellation preserves state) sit in the decision record | convert to the 0.8.0 persona schema and move topic-specific demands to `## Topic stances` | reformat-terminal-developer-persona |
| givn/ideation/terminal-wow-factor/personas/terminal-developer--interactive.md | biography copy of the permanent persona instead of a `Reuse:` header plus new record entries; topic-only `## Open demands` and `## Confirmation status` sections | convert to `Reuse: terminal-developer--interactive, base <date>; this file adds record entries only` plus append-only record entries | refresh-terminal-wow-factor-persona-record |

Observations (no row): no dangling `## Personas` reference exists (`givn lint` reports no `[PERS ]` finding; the single declared reference resolves to `givn/personas/terminal-developer--interactive.md`). The permanent persona is not sliced by mechanism and does not collapse multiple authority boundaries, so no identity-criteria split is needed.

## Phase 3 — ADRs

Re-qualification verdicts (all 20 accepted and proposed records) are recorded in this change's `arc42.md`. Audit rows:

| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| docs/adr/ (26 records), docs/arc42/09-architecture-decisions.md | wrong storage root: records live at `docs/adr/` instead of `docs/arc42/adr/`; `docs/arc42/adr/README.md` and `docs/arc42/adr/adr-template.md` are missing; chapter 09 register lacks Status and Date columns and links to the old root | move the records to `docs/arc42/adr/`, create the ADR directory index, scaffold the template from the 0.8.0 binary, rebuild the chapter-09 register with ADR / Title / Status / Date / File rows | migrate-adr-storage |
| docs/arc42/09-architecture-decisions.md | register embeds a long ADR-0011 decision summary beyond link/status/date register information | trim to register rows; keep any still-current rationale once in the owning capability spec | migrate-adr-storage |
| docs/arc42/adr/adr-template.md | absent; when scaffolded it must not carry a pre-passed verdict | scaffold via `givn addons enable arc42`; confirm no `ADR-QUALIFICATION: PASS` or `qualification: QUALIFIED` remains | migrate-adr-storage |
| docs/adr/ (26 records) | missing YAML frontmatter (status/date/decision-makers/consulted/informed), no `## Qualification` verdict and checklist, missing `## Pros and Cons of the Options` and `## More Information`; nonstandard section names in 0008 and 0025; nonconforming superseded status text in 0007, 0008, 0011, 0013 | retrofit every record to the 0.8.0 MADR template; normalize status values to the allowed set | migrate-adr-schema |
| docs/adr/0005-execution-with-confirmation.md | re-qualification fails: product-behavior exclusion (F3) — the `-x` opt-in and confirmation gate is workflow authorization owned by Gherkin | move the rationale once to `givn/specs/fragments/ask.feature`; remove the record from the register and both indexes; keep the audit note in `arc42.md`; never copy it into `docs/arc42/adr/archive/` | retire-nonqualifying-adrs |
| docs/adr/0010-ratatui-model-picker.md | proposed record whose owning change (`ratatui-model-picker`) has archived; re-qualification fails single-change reversibility (F1) and product-behavior exclusion (F3) | move the rationale once to `givn/specs/configure-model/ratatui-model-picker.feature`; remove the record from the register and both indexes; keep the audit note | retire-nonqualifying-adrs |
| docs/adr/0012-structured-widget-composition-for-terminal-setup-views.md | proposed record whose owning change (`provider-setup-widget-layout`) has archived; re-qualification fails single-change reversibility (F1) and product-behavior exclusion (F3) | move the rationale once to `givn/specs/configure-provider/provider-setup-widget-layout.feature`; remove the record from the register and both indexes; keep the audit note | retire-nonqualifying-adrs |
| docs/adr/0006, 0007, 0008, 0011, 0013, 0014 | superseded records remain in the active register at the wrong root; 0007 and 0008 are only partially superseded and their remaining current rationale is unrecorded; 0008 leaves stale risk R-008 in chapter 11 | normalize status to `superseded by ADR-NNNN`, move to `docs/arc42/adr/archive/`, update both indexes, move any still-current rationale once to its capability spec, clear the stale chapter-11 risk | archive-superseded-adrs |

Observations (no row): the design contains no new Technology Decision that passes the falsification tests, so no new MADR is created in this change (Phase 3 step 6). No project-local `adr-template.md` copy with a pre-passed verdict exists.

## Phase 4 — Smaller adoptions

| Artifact | Deviation (criterion or test) | Disposition | Follow-up change id |
|---|---|---|---|
| givn/ideation/terminal-wow-factor/changes/use-explanatory-shell-shortcut.md | no valid seed status line (`givn lint` `[SEED ]` advisory); the seed was consumed by the archived `use-explanatory-shell-shortcut` change; confirmed decisions carry no option treatment and the open questions are not in the full content schema | stamp `Status: implemented in use-explanatory-shell-shortcut`; add option treatment to the accepted decisions; record the open questions in the full content schema | stamp-use-explanatory-shell-shortcut-seed |
| givn/ideation/terminal-wow-factor/domain-terms.md | term records lack the term/status/definition/contrast/first-use/collision-check schema; `candidate`, `confirmed`, and `rejected` statuses are not recorded | migrate the eight entries to the 0.8.0 term-record schema; rejected terms keep their definition and reason and are never deleted | migrate-terminal-wow-factor-term-records |
| givn/ideation/terminal-wow-factor/domain-terms.md | confirmed terms carried no re-run collision evidence | **edited in this change**: `command flow` and `candidate` searches re-run 2026-09-17 with commands and hit counts in the collision-check cells; no competing usage found | (this change) |

Observations (no row): no use-case rule or guarantee carries an unresolved prerequisite — no `(open: Q<N>)` marker exists and the single parked ideation question (Q8 decision transport) was resolved by the archived implementing change. The only active proposal (this change) has no open questions; archived proposals are historical and are not rewritten, so no `D<n>` ledger row exists. Dialog policy and the severity/routing output were re-read, not repaired.

## Phase 5 — Ejected overrides

No ejected override carries pre-0.8.0 rule text or a stale default, so no row exists:

- `givn/config.yaml`: the uncommented `addons:` block matches the 0.8.0 addon set (all eight addons enabled) and `skills.targets: [opencode, agents, claude]` is a genuine project setting. No rule text is present.
- `givn/artifacts/` and `givn/skills/`: no override files exist.

## Validation

1. This change edited no corpus artifact except the term collision-check cells in `givn/ideation/terminal-wow-factor/domain-terms.md`.
2. Every row names a follow-up change; no archived change is named as a pending follow-up.
3. `givn lint` reports 27 files checked with advisory findings only (no blocking finding); `givn check review` is run by the review artifact.
4. The inventory is committed with this change; the maintenance change archives once every row has a disposition and a follow-up.

## Follow-up changes

Phase 1: `split-configure-interactive`, `rename-configure-interactive-capabilities`, `split-configure-model`, `rename-configure-model-capabilities`, `refocus-configure-provider`, `rename-configure-provider-capabilities`, `split-use-shell`, `rename-use-shell-capabilities`, `rename-corpus-infra-capabilities`.

Phase 2: `reformat-terminal-developer-persona`, `refresh-terminal-wow-factor-persona-record`.

Phase 3: `migrate-adr-storage`, `migrate-adr-schema`, `retire-nonqualifying-adrs`, `archive-superseded-adrs`.

Phase 4: `stamp-use-explanatory-shell-shortcut-seed`, `migrate-terminal-wow-factor-term-records`.

Suggested order: storage before schema before retirement/archive for the ADRs; per-use-case splits before their capability renames; persona and term/seed follow-ups are independent. Each follow-up runs as a normal change with its own specification deltas, review, and archive.
