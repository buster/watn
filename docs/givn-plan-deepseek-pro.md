# Plan: Making givn produce non-redundant specs with reusable steps and clear boundaries

## 1. Root-cause analysis — why the overlap happens

The overlap report (`docs/feature-step-overlap-report.md`) and the current givn code/skills together point to five structural causes. Every improvement below traces back to one of them.

**Cause A — The merge engine only sees one file, and only exact titles.**
`merge.rs:42-65` blocks `@givn.added` only when the title already exists *in the same permanent file*. The two exact duplicates the report found (F1 credential precedence in `credential-sources` vs `provider-setup`, F2 newest-search-result in `model-autosuggest` vs `search-concurrency`) are in different files, so archive never noticed. There is no cross-capability check, no semantic fingerprint, no scenario statistics anywhere in `status`/`archive`/`lint`. Archive is purely additive (Process Cause #2) — the merge engine has `Added/Modified/Removed` (`delta.rs:4-12`) but no *supersedes* operation, so a stronger scenario can never atomically retire a weaker one, which is exactly how the Bash-widget subsets (F4/F5) survived.

**Cause B — Skills never tell the agent to look at the whole active tree.**
`givn-spec` says "check permanent specs" but only as *baseline for the same capability* (`.opencode/commands/givn-spec.md` step 3). Nothing instructs the agent to search all active specs for the intended invariant, classify it (new/variant/layered/supersedes/duplicate), or apply length rules. The report's own "Scenario Decision Tree" and "Behavior Ledger" are recommendations for a tool and workflow that do not exist — the skill literally cannot comply with them.

**Cause C — Step reuse is instructed but untooled.**
`givn-steps` says "check for a matching step definition before writing a new one" but offers no mechanism: no normalized-binding search, no shared-helper location convention, no alias detection. In watn, bindings are registered globally (`tests/steps/mod.rs`, 858 attribute declarations), so agents invented new prose to dodge expression collisions instead of reusing — producing S1–S5's duplicated page-pollers, PTY drivers, suggestion assertions, and config parsers.

**Cause D — Three prose layers repeat the rules and drift.**
The same rules live in `assets/instructions/*.md` (served by `givn instructions`), `assets/skills/*/SKILL.md.tmpl` (rendered to `.opencode/skills/…`), and `assets/commands/*.md` (slash commands). Drift is already visible: `commands/givn-spec.md` says "one `@e2e` scenario per distinct happy-path action", while `instructions/specs.md` and `skills/givn-spec` now say "one per User Interaction Inventory entry". Whatever normative rule is added in one layer is not automatically in the others.

**Cause E — No gate exists, so nothing fails.**
`lint.rs` checks only parse errors and `@wip` (plus a tasks.md e2e cross-check). Worse, `givn lint` with no `--change` recurses into `givn/archive` (`parse.rs:216-237` has no archive skip), contradicting the report's requirement that archived files be evidence, not active failures. `givn check review`/`archive` run verify, verify-e2e, integrity — but nothing about scenario redundancy. A green suite with a larger scenario count is treated as progress because no counter-signal exists.

---

## 2. Design principles for the fix

1. **Merge examples, not invariants** (the report's key rule): a scenario becomes an Examples row only when the user action, production boundary, and assertion shape are unchanged.
2. **Detection in the binary, discipline in the skills.** Rules that must hold mechanically (duplicate titles, fingerprint dispositions, length, net deltas) belong in `givn`; the skills orchestrate the tool instead of repeating prose.
3. **One canonical owner per invariant per boundary.** The ledger is the source of truth; scenario tags reference it; archive updates it.
4. **Supersession is a first-class merge operation.** Replacing a weaker scenario must be expressible and atomic, and reported as a net delta.
5. **Normative text lives in one place** (the embedded instructions); skills and commands become thin choreography layers.
6. **False positives must be cheap to dispose of.** Semantic matches warn and require a disposition (an explicit tag/ledger entry), not a hard failure; exact duplicates hard-fail.

---

## 3. Plan — binary (`givn`) improvements

### 3.1 `givn lint` becomes an overlap gate (extends `parse.rs`/`lint.rs`)

Add a third lint family beyond `ParseError`/`Wip`: `Overlap`, with a normalized comparison pipeline:

- **Step normalization**: replace quoted values, numbers, model/provider/shell names, and enum-like tokens with `<value>`; drop `Given/When/Then/And/But` keywords. Produces the normalized step-expression fingerprints the report's "Automated Checks" section asks for.
- **Scenario fingerprint**: command/entry point + interaction verbs + primary assertion shape (from normalized steps), independent of prose wording.
- **Checks, split by confidence:**
  - *Error*: exact duplicate scenario titles anywhere in the active tree (all of `givn/specs` + active change specs).
  - *Error*: `@givn.added` scenario whose normalized fingerprint already exists at the same boundary without a new-boundary declaration (see 3.3).
  - *Warning + required disposition*: semantic fingerprint match without a tag stating the boundary difference (`@boundary.cli`, `@boundary.pty`, `@boundary.resolver`, …) or a `@behavior.<id>` reference.
  - *Length gates* (configurable thresholds, default from the report): >14 steps → warn; >19 steps → error unless the scenario is tagged `@long.rationale` (or carries a design-review reference).
- **Scope fix**: `givn lint` (repo-wide mode) must skip `givn/archive`, with `--include-archive` producing the *historical* report (archive→active matches shown as historical duplication, exactly as the report demands). A new `givn lint --all` covers active changes + permanent specs.
- **Step-binding alias report** (new `givn steps report`, language-aware via `givn/config.yaml`): parse registered bindings (`#[given/when/then("…")]`, `@given(...)`, behave `@given(...)`, etc. — configurable regex per `verify.framework`), normalize expressions, and report (a) two different expressions whose normalized forms collide, (b) different expressions with identical bodies (alias suspicion, S3/S5), (c) bindings with no scenario usage. This makes "reuse existing steps; never duplicate" mechanically checkable instead of aspirational.

### 3.2 Behavior ledger (new `givn/behavior.yaml`, optional addon `behavior_ledger`)

The report's ledger as a first-class artifact:

```yaml
- id: model-search/newest-query-wins
  action: user types a query while a search is running
  outcome: the newest completed result stays visible
  boundary: model-picker-state
  canonical: model-autosuggest.feature → "The newest search result stays visible…"
  variants: [completion-order: newer-first, older-first]
  supersedes: []
```

- Scenarios may carry `@behavior.<id>`. `givn lint` cross-references: a `@givn.added` scenario without a ledger row → warning requiring either a ledger entry or an explicit `@boundary.*` declaration.
- `givn spec ownership <query>` — the search command the spec skill needs: given a phrase, returns ledger rows and active scenarios whose fingerprints match, with their boundary and canonical owner. This is the concrete answer to the report's "Search all active feature files for the intended behavior".

### 3.3 Supersession in the merge engine (`delta.rs`/`merge.rs`)

- New tag `@givn.supersedes <title|@behavior.<id>>`: on archive, the merge validates the target exists anywhere in the active tree (cross-file lookup, fixing Cause A), removes it, appends the replacement, and records the supersession.
- Archive still fails on `@givn.added` exact-title collisions — now across *all* permanent specs, not just the same file.
- Merge output gains the report's net-delta block (see 3.5). Merge remains text-level and all-or-nothing; it must not attempt automatic Scenario Outline rewrites (too risky for a text engine) — consolidation stays an authoring-time activity driven by lint suggestions.

### 3.4 Archive gate and net-delta report

`givn archive` runs the overlap lint against the *merged* tree before the final move (same all-or-nothing/rollback pattern as the existing post-merge verify). On success it prints:

```
new invariants:        N
new examples:          N
superseded scenarios:  N
removed duplicate steps: N
net scenario delta:    N
net source-step delta: N
```

so "a green suite with a larger count" is no longer automatically progress (report §Feature Merging Workflow). `givn status` also gains scenario/step counts per change and for the permanent tree.

### 3.5 Console/UX consolidation of the three prose layers

Make `assets/instructions/*.md` the single normative source (they already serve `givn instructions`, which the skills are supposed to run anyway). Then:

- `SKILL.md.tmpl` files become choreography: "run `givn instructions specs --change <id>` and obey it; here is the workflow order and the tool commands". Remove duplicated rule text.
- `assets/commands/*.md` shrink to entry points that delegate to the skill.
- Resolve the `@e2e` scope contradiction while doing this: adopt the skill/instruction wording (one `@e2e` per inventory entry) everywhere, and state the variant rule identically.
- `givn upgrade`/`skills sync` re-renders all three from the one source, so future rule changes can't drift.

---

## 4. Plan — skill rewrites

### `givn-spec` (highest priority)

1. **Pre-write ownership check (mandatory step 0):** run `givn spec ownership <summary-of-behavior>` (3.2) before writing any scenario; report matches to the user; classify the new scenario with the report's decision tree (6 questions, embedded in the skill).
2. **Boundary naming rule:** layered scenarios must name the boundary in the title ("… through provider resolution", "… through the CLI") and/or carry `@boundary.*`; same title across boundaries is forbidden (F1/F2 prevention).
3. **Supersede-not-add:** when a stronger scenario replaces a weaker one, use `@givn.supersedes` in the same change; never add both.
4. **Matrix rules:** value variants and completion-order variants become Scenario Outlines/Examples rows (F2, F8, F13); one canonical happy path per action per boundary; extend the canonical scenario with one focused assertion instead of re-driving the full flow (F10/F12).
5. **Length rules:** >14 steps requires a split-or-keep decision recorded in the scenario's docstring or design-review; >19 requires explicit design-review approval.

### `givn-steps`

1. **Reuse-first via the tool:** before writing a binding, run `givn steps report` and reuse any normalized match (RED step: "reused: keep as-is" becomes checkable). New prose that normalizes to an existing binding is a finding, not wording variation (report: "treat any new global step-expression alias as a design-review item").
2. **Shared-helper convention:** mandate where cross-capability helpers live (e.g. `tests/steps/mod.rs` in Rust/cucumber-rs) and when delegation is correct (the S6 delegation model as the positive pattern); duplicate bodies across modules are a review-time finding.
3. **Scenario targeting stays, plus boundary assertion:** keep RED/GREEN/REFACTOR unchanged; add: before GREEN, state which boundary (state-level vs PTY vs subprocess) this scenario owns, so a regular-vs-E2E repeat is intentional, not accidental (F9/F12).

### `givn-review`

Add **step 0 overlap disposition** (before the fabrication audit): run the overlap lint, and for every `variant`/`supersedes`/`layered` match, record the disposition in review.md — a change that adds no new behavior IDs but adds scenarios fails review unless it records a replacement or boundary change (report's gate list). Superseded scenarios must be removed by the same change, not marked "future cleanup".

### `givn-design`

Require the Step Definitions table to include, per binding: *reused (from where) / new* and the shared helper it uses — so S1–S5 duplication is visible at design time. Require the design to name the production boundary each non-E2E scenario tests (picker state vs coordinator path vs resolver), mirroring the spec's boundary tags.

### `givn-archive` / `givn-tasks` / `givn-propose` / `givn-explore`

- `givn-archive`: mention the merged-tree overlap lint and the net-delta report in the gate description.
- `givn-tasks`: each scenario task gains a "boundary" line; the reuse-vs-new decision from design is checked off per scenario.
- `givn-propose`: proposal gets an optional "behaviors affected" section referencing ledger IDs.
- `givn-explore`: when an insight is a requirement change, offer `givn spec ownership` as the discovery command alongside `/givn-spec`.

### `givn-dev-principles`

Add one line: "Fix overlap at the source — supersede and delete, never accumulate; a second scenario for an existing invariant is a bug in the spec."

---

## 5. Harness and rollout

- **Harness evaluator** (`harness/src/givn_harness/evaluator.py`): add checks that `givn lint` exits 0 with no Overlap errors, that archive output contains the net-delta block, and that a change introducing a fingerprint duplicate either supersedes or declares a boundary. This dogfoods the gates so givn itself can't regress on the very problem it now prevents.
- **watn rollout:** the report's "Recommended Consolidation Order" (items 1–9) becomes executable givn changes, each now *gated*: e.g. the F1/F2 dedup change uses `@givn.supersedes`; the S1–S5 helper extraction is driven by `givn steps report`; the shell-completion outline conversion (F13) is verified by lint's Examples-suggestion. The first change should also seed `givn/behavior.yaml` from the report's semantic-groups tables, turning a static document into the live ledger.

## 6. Sequencing

| Phase | Work | Rationale |
|---|---|---|
| 0 | Lint scope fix (skip archive) + scenario stats in `status` | One-line-adjacent changes; immediately makes lint trustworthy for gating. |
| 1 | Overlap lint (normalization, fingerprints, length gates, `steps report`) | Everything downstream depends on detection. |
| 2 | `@givn.supersedes` + cross-file merge checks + archive net-delta | Makes consolidation atomic and visible; retires Cause A. |
| 3 | Behavior ledger + `spec ownership` | Provides the search-and-dispose workflow the skills need. |
| 4 | Skill rewrites + instructions/skills/commands consolidation | Choreography can only be rewritten after the tools exist; kills Cause D. |
| 5 | Harness checks + watn consolidation changes | Dogfooding + applying the gates to the report's findings. |

## 7. Risks and mitigations

- **Normalization false positives** → only exact title matches hard-fail; semantic matches warn and require a one-tag disposition (`@boundary.*`/`@behavior.<id>`), keeping the loop fast.
- **Over-blocking agents** → thresholds configurable per project in `givn/config.yaml`; gates degrade to warnings when the ledger addon is disabled.
- **Ledger maintenance burden** → ledger is optional; `givn spec ownership` and archive's net-delta keep it cheap to maintain; missing rows only warn.
- **Text-level merge engine limits** → no automated Scenario-Outline rewrites; lint only *suggests* consolidation; rewrites stay authoring-time and are reviewed like any other change.

The single biggest win is Phase 1+2: without cross-file detection and atomic supersession, every skill-level rule remains unenforceable prose — which is precisely how the current 25-feature/858-binding tree accumulated.
