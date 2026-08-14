# Givn Feature-Spec Creation and Merge Improvement Plan

## Objective

Make Givn behavior-first rather than feature-file-first.

A feature file is an executable presentation of behavior. It is not the ownership boundary for that behavior. One invariant may have multiple valid test layers, but each layer must be intentional, named, and non-subsumed. Variants belong in Examples rows or existing scenarios. Stronger scenarios replace weaker ones. Deltas must be tested and merged as their projected final state, not as additive raw files.

## Diagnosis

| Failure | Evidence | Consequence |
|---|---|---|
| Planning is local to a change | `docs/feature-step-overlap-report.md:694-701` | Each change can satisfy its own interaction inventory while reintroducing an existing invariant. |
| Feature/capability names are treated as behavior ownership | The report identifies repeated model selection, filtering, setup, reasoning, and credential contracts across feature families | Entry-point slicing creates a new feature instead of extending, layering, or superseding an existing contract. |
| Scenario titles are merge identity | Givn `merge.rs` replaces and deletes by title | Renames are unsafe, duplicate titles are ambiguous, and concurrent modifications cannot be resolved safely. |
| Delta semantics do not define the test view | `tests/features_runner.rs:153-164` runs permanent specs plus raw active deltas | A modified scenario can require both old and new behavior before archive. A removed scenario remains executable until archive. |
| Archive guarantees are overstated | Default manifest hooks are empty; `archive` runs configured post-hooks but does not invoke the full mandatory `check review` path | A skill can claim that integrity, task evidence, and review checks block archive when the binary has not actually enforced them. |
| Step reuse is phrased as file-local reuse | `givn-steps` requires one file per capability and permits immediate GREEN for all-reused steps | Global Cucumber bindings avoid exact collisions but still permit semantic aliases and repeated helper implementations. |
| Fuzzy similarity has no disposition model | The report's behavior ledger and merge-reconciliation proposal are documentation only | The system has no way to distinguish a legitimate layered test from a duplicate or a candidate Scenario Outline. |
| Source-of-truth drift exists | Generated commands, skills, config comments, and binary behavior disagree on runner paths, archive hooks, and workflow boundaries | Agents act on false operational guarantees. |

The history confirms this is structural, not incidental. `eb328dd` introduced a `@givn.modified` stale-search scenario, while archive commit `57688f6` left the original autosuggest scenario unchanged and added a separate permanent feature. The resulting duplication was accepted because review evaluated local coverage, not repository behavior ownership. `c0a3fcf` made `givn check review` stronger while leaving archive dependent on empty default post-hooks, widening the gap between the generated guidance and executable enforcement.

## Target Model

Separate four concepts that are currently conflated.

| Concept | Meaning | Identity rule |
|---|---|---|
| Domain behavior | The durable invariant the product promises, such as "the newest query wins." | Stable behavior ID. Does not include test layer or file location. |
| Coverage cell | Evidence for a behavior through a defined interface, boundary, execution tier, and fault domain. | Behavior ID plus declared boundary metadata. |
| Scenario | One executable Gherkin proof for a coverage cell. | Immutable scenario anchor, independent of title and path. |
| Feature file | A navigational container for related scenarios. | Never the behavior identity or merge target. |

Use a versioned, machine-readable behavior registry as the durable ownership source. Each record must contain:

- Stable behavior ID.
- Stable scenario anchor.
- Current source location.
- Coverage cell: interface, production boundary, execution tier, primary observable outcome, and declared fault domain.
- Canonical, layered, E2E-smoke, variant, supersession, retirement, relocation, split, or merge relationships.
- Registry revision and semantic digest.
- Active or retired state.
- Links to the scenarios or Examples rows that prove the behavior.

Persist a Gherkin-safe scenario anchor alongside the registry. Do not use a current `@givn.*` tag or a comment until archive preserves it losslessly. Current archive behavior strips all `@givn.*` tags and has already lost feature-level comments during merge. The V2 merge contract must preserve identity directives verbatim and validate a one-to-one registry-to-scenario-anchor relationship.

Treat `new`, `variant`, `layered`, `supersedes`, and `duplicate` as relationships between proposed and existing evidence, not as a single scenario property. A scenario can be a new coverage cell for an existing behavior while also superseding weaker evidence.

Use this decision sequence before authoring:

1. Identify the domain behavior.
2. Search the active registry and effective active tree.
3. Decide whether the proposed evidence is a new behavior, a value variant, a new coverage cell, a stronger replacement, or a duplicate.
4. Record the disposition against the matched behavior and registry revision.
5. Write Gherkin only after that disposition is accepted.

A title match is a high-confidence candidate, not automatic proof that one scenario must be deleted. The literal credential scenarios demonstrate why: resolver and CLI evidence can both be valid. They must be classified as layered, named by boundary, and made unambiguous. An unresolved exact-title collision blocks archive. A valid layered disposition normally resolves it by renaming the scenarios to state their distinct boundary.

An `@e2e` tag is not a new behavior by itself. It is evidence at a different execution tier. An E2E scenario may intentionally assert the same user outcome as a lower-level scenario to expose wiring, packaging, terminal, process, browser, or deployment faults. Require a declared non-subsumption or fault-domain rationale. Do not force artificial behavioral differences merely to justify an E2E test.

## Phase 0: Define the V2 Contract

Write the semantic and merge contract before changing skills or gates.

- Define the behavior registry schema, persistent scenario anchor syntax, coverage-cell vocabulary, relationship semantics, and revision rules.
- Define default forward-flow behavior impact as a manifest role, not a hard-coded artifact ID. The default graph should require a behavior-impact artifact between proposal and specs.
- Keep custom manifests supported. A project may use a differently named artifact if it declares the behavior-impact role.
- Define a separate reverse-engineering profile. It begins with a provisional observed-behavior inventory and reconciles ownership after characterization. It must not require a prescriptive proposal or known behavior owner before baseline discovery.
- Define V2 delta operations against stable scenario anchors and registry revisions. Do not default an untagged V2 scenario to `added`.
- Support explicit amend, retire, relocate, rename, split, merge, and supersede semantics. Retirement must name either a replacement behavior/scenario or an intentional coverage retirement rationale.
- Define Scenario Outline identity separately from Examples-row identity.
- Define Rule, Background, Feature, Scenario Outline, Examples, tags, descriptions, doc strings, data tables, comments, dialects, and ordering as lossless merge inputs.
- Define semantic digests over inherited context. A scenario's effective contract includes relevant Feature, Rule, and Background context, not only its local steps.
- Define a versioned normalizer for candidate detection. Preserve ordering, negation, cardinality, temporal terms, and assertion polarity.

Build the regression corpus before enforcement. It must include the report's credential precedence pair, stale-search pair, Bash subset pair, missing-config subset pair, valid E2E layering, Scenario Outlines, Rules, Backgrounds, comments, doc strings, tables, renames, relocations, concurrent edits, and archive failures mid-batch.

## Phase 1: Correct Binary Semantics

Fix execution and archive correctness before asking agents to make semantic decisions.

- Introduce a runner adapter contract. It must receive an explicit effective-spec root, selected change stack, source map, targeted scenario selector, and execution tier.
- Stop treating `GIVN_FEATURES` as an implied contract. The current Watn runner ignores it and hard-codes `givn/specs`.
- Route targeted RED, GREEN, REFACTOR, `check`, review, and archive verification through the same binary-owned effective-spec execution path.
- Build an effective projection for each change. It overlays the chosen delta over its declared baseline so amended scenarios replace prior evidence and retired scenarios disappear before execution.
- Do not union every active change's raw features into one runner invocation. Either select an explicit compatible stack or reject unresolved claims between active changes.
- Make V2 delta validation a real binary gate. Validate identity anchors, operation cardinality, target existence, registry revision, semantic digest, placement, removal/replacement rules, and duplicate operations before any write.
- Reject conflicting active changes early. Two changes may not silently amend, retire, relocate, or claim the same scenario or coverage cell without an explicit dependency and rebase.
- Replace title-based merge targeting with scenario anchors and registry revisions.
- Stage a complete candidate permanent tree outside the live tree.
- Parse and validate every delta before mutating permanent specs.
- Run mandatory task, review, integrity, normal verification, E2E verification, behavior analysis, and configured project checks against the staged candidate.
- Promote only after a compare-and-swap against the expected registry revision and Git baseline.
- Use a transaction journal with resumable or abortable states. A local filesystem lock is insufficient across worktrees or clones.
- Treat CI or a merge queue as the cross-clone authority for final compare-and-swap validation.
- Isolate hooks that may modify README, coverage, or other tracked outputs. Promote only declared outputs from a successful staged transaction.
- Preserve source text losslessly except for intentional V2 operation directives. The current scenario-only renderer is not sufficient.

Archive must derive required artifacts from the selected workflow profile's transitive dependency closure. A skipped mandatory ancestor cannot satisfy archive. Reverse-engineering remains valid through its own explicit profile, not through unrestricted `--skip` holes.

## Phase 2: Add Repository-Wide Behavior Analysis

Make the binary produce evidence. Do not rely on agents to perform whole-repository comparison manually.

Add a behavior analysis service with stable machine-readable output for:

- Permanent registry entries.
- Effective active changes.
- Exact scenario-anchor conflicts.
- Exact title collisions.
- Same behavior and same coverage-cell collisions.
- Normalized Gherkin fingerprints.
- Candidate Scenario Outline groups.
- Potential subsumption.
- Candidate layered evidence.
- Active-change claims and conflicts.
- Scenario length and justification requirements.
- Net behavior, scenario, binding, and helper deltas.

Use severity correctly:

| Finding | Default action |
|---|---|
| Duplicate scenario anchor | Hard failure |
| Stale registry revision or semantic digest | Hard failure |
| Conflicting active claim | Hard failure |
| Missing behavior-impact disposition | Hard failure for V2 |
| Same behavior in same coverage cell | Hard failure unless explicitly converted to replacement or variant |
| Exact duplicate title | Blocking unresolved candidate; resolve by classification and clearer naming |
| Fuzzy normalized match | Candidate only; requires a recorded disposition, never automatic deletion |
| Potential Scenario Outline | Advisory, then reviewer must explain retain-versus-merge |
| Helper similarity | Advisory provenance, not a generic binary hard gate |
| Long scenario | Rationale required, not automatic splitting |

The binary should compare affected nodes against the active registry, not perform an unbounded lexical scan on every command. Every candidate disposition must name the compared registry revision so that an upstream change reopens stale decisions.

Generate an archive receipt containing:

- New behaviors.
- New coverage cells.
- New Examples rows.
- Superseded scenarios.
- Retired scenarios.
- Relocated scenarios.
- Unresolved candidate count.
- Net scenario delta.
- Net source-step delta.
- Binding-index changes.
- Helper-reuse advisories.
- Baseline and resulting registry revisions.

Do not use raw scenario-count reduction as a goal. The target is minimum duplicated setup and bindings per covered behavior/cell, not minimum test count.

## Phase 3: Replace Capability-Local Step Rules

Separate scenario reuse, binding reuse, and helper reuse.

| Reuse level | Required policy |
|---|---|
| Scenario reuse | Extend the canonical scenario, add an Examples row, add a declared layered cell, or supersede prior evidence. Do not create a parallel scenario merely because a different feature module changed. |
| Binding reuse | Reuse the same global step expression when it expresses the same domain action or assertion. Do not invent aliases to avoid global registration collisions. |
| Helper reuse | Shared polling, PTY setup, config parsing, model selection, assertion utilities, and fixtures belong in explicit shared helpers. Capability-specific bindings delegate to them. |

Replace "one step-definition file per capability" with a Step Ownership and Reuse Matrix in design:

| Step or scenario action | Existing binding candidate | Decision | Binding owner | Helper owner | Boundary reason |
|---|---|---|---|---|---|

Keep E2E adapters separate where their infrastructure differs. Do not force separate E2E domain actions or duplicate assertions merely because the adapter is separate.

Make the binding index project-provided and versioned. A generic CLI must not attempt to parse Rust, Java, Python, JavaScript, generated glue, and runner internals itself. The project adapter should emit:

- Binding expression.
- Source location.
- Scope and tag filters.
- Parameter types.
- Generated or handwritten state.
- Index build digest.

The runner remains responsible for runtime ambiguity detection. Helper reuse remains advisory because generic cross-language source analysis cannot prove it safely.

Remove the normal-flow "all steps reused means immediate GREEN" rule. For a forward change, a scenario that passes immediately with reused bindings usually proves one of four things: the behavior exists already, the change is a variant, the scenario is duplicate evidence, or the work is characterization. Route it through behavior reconciliation. Permit immediate GREEN only in the reverse/characterization profile, or under an explicit behavior-impact exception proving a new coverage cell.

Treat retirement differently from runnable scenario work. A removed scenario's placeholder is metadata, not an implementation target. Its task proves the projected absence, replacement coverage where required, and absence of obsolete bindings or helpers.

## Phase 4: Rewrite Every Givn Skill

| Skill | Required revision |
|---|---|
| `givn-explore` | Start with behavior hypotheses, affected user actions, existing owners, and possible boundaries. Do not choose a capability/file layout during exploration. Keep the no-artifact stance. |
| `givn-propose` | Describe user outcomes and scope in domain language. Hand off explicit affected-behavior hypotheses to the behavior-impact artifact without embedding implementation details. |
| `givn-spec` | Run binary behavior analysis before authoring. Record a disposition for every matched candidate. Build features around canonical behavior ownership, not changed source modules. Require examples for value-only variation. |
| `givn-domain-modeling` | Make invariant and boundary vocabulary durable input to behavior impact. Keep aggregate and lifecycle detail optional for simple changes. |
| `givn-design` | Replace one-file-per-capability rules with the Step Ownership and Reuse Matrix. Document the runner adapter, effective-spec command, binding-index command, E2E fault-domain rationale, and projected test boundary. |
| `givn-design-review` | Add a mandatory repository-wide semantic review branch. The fresh reviewer must challenge each new behavior, coverage cell, layer, supersession, retirement, outline decision, and long-scenario rationale. |
| `givn-tasks` | Create tasks by behavior operation and scenario anchor, not only by prose scenario title. Distinguish add, amend, variant, supersede, retire, and characterization tasks. |
| `givn-steps` | Require binding-index search before a new binding. Ban semantic aliases. Direct repeated mechanics to shared helpers. Remove the generic immediate-GREEN exception. |
| `givn-implement` | Use the effective-spec runner for every targeted phase. A forward scenario that passes before behavior changes is a classification problem, not a valid RED/GREEN shortcut. |
| `givn-dev-principles` | Add a rule that a local alias, helper copy, or capability-specific wrapper is not a harmless convention when a canonical owner already exists. |
| `givn-review` | Run binary semantic analysis before coverage review. Require clean dispositions, boundary/fault-domain rationale for layers, and binding-index evidence for every new expression. |
| `givn-archive` | Run binary preflight, display the receipt, then let archive rerun the same preflight internally. Remove claims about checks that binary archive has not executed. |
| `givn-reverse-engineer` | Begin with a provisional observed-behavior inventory against the active registry. Do not assume the permanent capability spec is empty. Reconcile discovered evidence after characterization. |
| `givn-characterize` | Allow observed behavior to be recorded without production changes, but require registry ownership and a reconciliation decision before archive. |
| Slash commands and generated instructions | Become thin renderings over the binary's resolved manifest, behavior report, and structured instruction output. They must not independently restate operational claims. |

Move E2E inventory ownership from "the same feature file must contain a matching E2E scenario" to "the registry must show one adequate E2E evidence cell per distinct real user action." This prevents every new feature slice from adding another full E2E path for the same action.

## Phase 5: Establish One Authoritative Workflow Contract

Eliminate duplicated policy across assets, generated skills, slash commands, config comments, and runtime behavior.

- Make the manifest and binary gate model authoritative for artifact requirements, workflow roles, archive requirements, command locations, and validation semantics.
- Extend `givn instructions --json` with artifact role, generated paths, dependencies, current state, mandatory checks, configured verification commands, and effective-spec execution details.
- Generate human instructions, skills, slash commands, and config comments from shared fragments tied to that schema.
- Include a policy/config hash in generated skill and command output.
- Make `givn skills sync` validate stale generated outputs.
- Add contract tests comparing CLI help, JSON schemas, default manifest, generated config comments, generated skills, generated commands, and runtime behavior.
- Remove all claims that configuration lives in `givn/config.yaml` when it actually resolves from `givn/commands.yaml`.
- Remove all claims that archive runs review integrity checks until archive actually invokes those checks.

## Phase 6: Migrate Without Blocking the Existing Baseline

Use a V1/V2 transition.

- Keep existing permanent specs readable under V1 compatibility.
- Require V2 metadata and explicit operations for new scenarios and any scenario touched by a new change.
- Do not hard-block the current baseline merely because `givn lint` is clean while the overlap report identifies semantic debt.
- Build the first registry from the current active tree and manually classify behavior families.
- Use the report's exact duplicate, subset, layered, helper-duplication, and Outline candidates as golden migration fixtures.
- Preserve valid layers while renaming them by boundary. The credential resolver and CLI request scenarios are the model case.
- Consolidate value-only variations into Scenario Outlines only after confirming the user action, primary assertion shape, and coverage cell remain constant.
- Retire or supersede subset scenarios in the same change that introduces the stronger evidence.
- Migrate one behavior family at a time. Do not run an automatic global deduplication pass.
- Run semantic analysis in advisory mode first and record false positives, accepted layers, and normalization gaps.
- Promote only stable structural checks to hard errors first: anchors, revisions, invalid operations, active conflicts, unowned V2 scenarios, and unsafe archive state.
- Promote unresolved semantic candidates to archive blockers after the registry is sufficiently populated and reviewers have calibrated dispositions.
- Enforce V2-only archives for new behavior families after the migration window closes.

## Completion Criteria

The work is complete when all of these are true:

- A changed or retired scenario is never executed beside its obsolete permanent version.
- A delta targets a stable anchor and expected revision, never only a title.
- An archive cannot partially mutate permanent specs, registry state, README, coverage outputs, or archive directories.
- Archive runs the same mandatory checks promised by its generated guidance.
- Every V2 scenario maps to one behavior and one declared coverage cell.
- Every semantic match has a recorded disposition against the active registry revision.
- A new E2E scenario proves a repository-wide user-action coverage need, not merely a feature-local inventory requirement.
- New bindings are checked through the project binding index before creation.
- Repeated helpers are visible as advisories and migrate toward shared ownership without false language-agnostic claims.
- Feature files preserve all supported Gherkin structure and source content through projection and archive.
- Generated skills, commands, config comments, binary JSON, and actual gate behavior pass contract tests.
- The overlap report's known duplicate and subset groups are either consolidated, explicitly layered, or explicitly retired with registry evidence.

## Ordering Rationale

Correct the executable delta model and archive transaction first. Add behavior analysis second. Rewrite agent guidance only after the binary can supply reliable evidence. Enforce semantic quality last, after legacy migration and false-positive calibration.
