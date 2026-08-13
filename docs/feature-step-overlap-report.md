# Feature and Step Overlap Report

Date: 2026-08-13

Branch: `report/gherkin-step-overlaps`

Baseline: `5f88af7` (`main`)

## Scope and Method

The review covered:

- All active feature files under `givn/specs/`.
- All historical feature files under `givn/archive/` for comparison and origin tracing.
- All registered step modules under `tests/steps/`.
- Relevant Gherkin history, archived design reviews, task evidence, and commits.

The runner only executes active specs. `tests/features_runner.rs:97-118` skips
directories named `archive`, and `tests/features_runner.rs:150-165` collects
`givn/specs` plus active change specs. Archived files are therefore evidence,
not active coverage.

Inventory at the baseline:

| Surface | Count | Execution status |
|---|---:|---|
| Active feature files | 25 | Executed |
| Active scenarios | 223 | Executed by the runner, subject to tags |
| Archived feature files | 26 | Not executed |
| Archived scenarios | 216 | Historical only |
| Step source files | 30, including `mod.rs` | 29 modules registered |
| Step binding attribute declarations | 858 | Registered globally |
| Comment-only compatibility modules | 3 | `config_steps.rs`, `models_steps.rs`, `providers_steps.rs` |

The classifications used below are:

- **Exact duplicate:** same scenario title or same observable contract with no meaningful boundary difference.
- **Subsumed:** one scenario asserts everything the other asserts, plus more.
- **Layered overlap:** the same product invariant is tested through different entry points or test seams. This can be valid, but the boundary must be explicit.
- **Helper duplication:** step bodies or support helpers repeat mechanics and should share implementation without necessarily changing Gherkin wording.

## Executive Findings

1. Two exact duplicate scenario titles exist in active specs: the literal credential precedence scenario and the newest-search-result scenario.
2. Three active scenarios are clear subsets of stronger scenarios: missing-config guidance, empty picker state, and the Bash widget E2E flow. The shortcut suite also contains a second duplicate failed/empty-output flow.
3. Model selection, catalog filtering, provider setup, and reasoning are covered repeatedly because each historical change added its own end-to-end path without a repository-wide behavior ownership check.
4. The same setup page polling, ANSI cleanup, PTY startup, model selection, and suggestion assertion mechanics are implemented in several step modules.
5. The current suite is green, but that only proves that the global bindings are currently registerable and the scenarios pass. It does not prove that the scenarios are independent or that the concurrency steps exercise a real race.
6. The regular `search-concurrency` scenario is particularly weak: its steps mutate `picker_suggestions` directly and assert that the option is present rather than starting workers and observing completion order (`tests/steps/search_concurrency_steps.rs:9-46`). Its E2E variant also does not assert exact result exclusion and does not configure delayed responses (`tests/steps/search_concurrency_steps.rs:49-88`).

## Active Scenario Findings

### F1. Duplicate Literal Credential Precedence Scenario

Severity: **High**

Evidence:

- `givn/specs/credential-sources/credential-sources.feature:17-26`
- `givn/specs/provider-setup/provider-setup.feature:187-195`
- Both scenarios are titled `A literal saved credential is authoritative over environment fallback`.
- Both configure the same provider, literal key, provider-specific fallback, generic fallback, and default model, then assert that the literal key wins.

The only meaningful difference is the `When` step. The credential-source scenario runs the actual binary (`I run watn "hello"`). The provider-setup scenario calls a test step that resolves the configured key (`tests/steps/provider_setup_steps.rs:186-200`). The duplicate title hides the fact that one is a user-facing request test and the other is a lower-level provider-resolution test.

Recommendation:

- Keep the real request scenario in `credential-sources.feature` as the canonical user-visible contract.
- Rename the provider step scenario to state its lower-level boundary, or remove it if the direct resolver assertion adds no branch coverage beyond the real request.
- Do not retain identical titles across boundaries.

### F2. Duplicate Newest-Search-Result Scenario

Severity: **High**

Evidence:

- `givn/specs/model-autosuggest/model-autosuggest.feature:28-33`
- `givn/specs/search-concurrency/search-concurrency.feature:3-8`
- Both scenarios are titled `The newest search result stays visible when an older result arrives later`.
- Both assert that the newer `o3` result remains visible and the older `gpt` result cannot replace it.

The intended difference is response ordering:

- `model-autosuggest`: `gpt` is slower than `o3`.
- `search-concurrency`: `gpt` is faster than `o3`, but both searches are started before application.

That is a valid two-row test matrix, not a reason for two independently named scenarios. It should be one scenario outline or one canonical concurrency feature with explicit completion-order examples.

The current implementations do not fully justify the split:

- `tests/steps/ask_steps.rs:1064-1090` and `:1164-1196` complete each blocking search in its own `When` step, so the model-autosuggest Gherkin does not itself create overlapping workers.
- `tests/steps/search_concurrency_steps.rs:18-31` assigns the expected newer result directly instead of coordinating two workers.
- `tests/steps/search_concurrency_steps.rs:44-46` treats the presence of an option as proof of worker cleanup.

Recommendation:

- Define one concurrency contract with two completion-order examples.
- Drive actual worker handles or the production generation seam.
- Assert exact final IDs, explicit exclusion of stale IDs, and actual join/cleanup.
- Retain one separate PTY scenario only if the terminal rendering boundary is being tested; give it a distinct title.

### F3. Missing-Config Guidance Is a Subset

Severity: **High**

Evidence:

- `givn/specs/auto-init-config/auto-init-config.feature:3-8`
- `givn/specs/config/config.feature:45-50`
- Both run `watn "hello"` with no config and assert exit status `1` plus provider setup guidance.
- The auto-init scenario additionally asserts that no config file is created.

The auto-init scenario subsumes the config scenario. The product invariant is one behavior: a first non-TTY request reports setup guidance and does not create configuration.

Recommendation:

- Keep one scenario in the auto-init capability with all assertions.
- Remove or rename the config scenario to a distinct configuration-parser or config-loader contract if it is needed for ownership reasons.

### F4. Bash Widget E2E Scenario Is Duplicated by a Stronger Scenario

Severity: **High**

Evidence:

- `givn/specs/interactive-shell-shortcut/interactive-shell-shortcut.feature:13-18`
- `givn/specs/interactive-shell-shortcut/interactive-shell-shortcut.feature:203-209`
- Both use the same Given, When, command-line assertion, and non-execution assertion.
- The later scenario adds `the Bash process should preserve the request as a comment`.

The later scenario subsumes the earlier one. Git blame shows the first flow came from `f4c1b46` and the stronger flow was added by `d23068e9` during the request-preservation change.

Recommendation:

- Keep the later scenario and delete the earlier duplicate, or add the comment assertion to the original and remove the later scenario.
- Keep the shared `run_bash_widget` helper in `interactive_shell_shortcut_steps.rs`; the E2E module already delegates to it (`tests/steps/interactive_shell_shortcut_e2e_steps.rs:53-58`).

### F5. Failed or Empty Bash Output Is Tested Twice

Severity: **Medium**

Evidence:

- `givn/specs/interactive-shell-shortcut/interactive-shell-shortcut.feature:131-137`
- `givn/specs/interactive-shell-shortcut/interactive-shell-shortcut.feature:188-194`
- Both run a failing fake `watn`, then empty output, and assert that the original input remains unchanged.

The second scenario uses stronger exact-buffer wording, but it covers the same two cases. The corresponding step definitions are separate patterns in `tests/steps/interactive_shell_shortcut_steps.rs:831-836` and the generic current-buffer assertion around `:744-755`.

Recommendation:

- Keep one scenario with exact buffer assertions for both failure modes.
- Use a scenario outline only if failure and empty output must have different fixtures or messages.

### F6. Empty Model Search State Is a Subset

Severity: **Medium**

Evidence:

- `givn/specs/model-autosuggest/model-autosuggest.feature:23-26`
- `givn/specs/ratatui-model-picker/ratatui-model-picker.feature:59-63`
- Both type `does-not-exist` with the same two-model catalog and assert `no models were found`.
- The Ratatui scenario additionally asserts that the filter text remains visible.

Recommendation:

- Keep the Ratatui scenario as the stronger picker contract.
- Remove the autosuggest duplicate or rename it to a distinct lower-level search-state contract if it intentionally targets `picker::execute_search` rather than the dialog.
- The non-E2E boundary must be stated in the title or design; otherwise the same feature behavior is being counted twice.

### F7. Version Flag Has a Deliberate but Unclear Layered Overlap

Severity: **Medium**

Evidence:

- `givn/specs/ask/ask.feature:96-100`: debug binary, generic `watn` output, logo/name, and any version number.
- `givn/specs/release-truth/release-truth.feature:4-10`: release binary, package-derived exact version, and exit status.

These are not exact duplicates because the binary and assertion strength differ. They test the same CLI flag, however, and the generic scenario contributes little once the release scenario exists.

Recommendation:

- Keep the release scenario for release truth.
- Either remove the generic ask scenario or rename it to make the debug smoke boundary explicit.
- Historical review already recognized this overlap at the step-expression level and deliberately used unique wording (`givn/archive/release-truth-and-repository-cleanup/review.md:5-8`). The scenario-level distinction should be equally explicit.

### F8. Model Tier Assignment Is Repeated Across Seven Feature Families

Severity: **Medium**

The same core interaction, discover models, select small/normal/thinking, and persist tiers, appears in:

- `givn/specs/models/models.feature:6-11` and `:20-25`
- `givn/specs/credential-sources/credential-sources.feature:4-14`
- `givn/specs/catalog-source/catalog-source.feature:4-28`
- `givn/specs/setup-persistence/setup-persistence.feature:36-43`
- `givn/specs/ratatui-model-picker/ratatui-model-picker.feature:4-8`
- `givn/specs/streamlined-setup/streamlined-setup.feature:103-115`
- `givn/specs/model-autosuggest/model-autosuggest.feature:4-12`

The variants add real assertions: environment credential selection, LiteLLM versus provider catalog source, preservation of provider settings, reasoning values, pagination, or PTY search. The problem is that most variants repeat the complete tier-selection flow instead of adding one focused assertion to a canonical flow.

Recommendation:

- Own the basic interactive tier assignment in one model-picker scenario.
- Keep one catalog-source scenario for source and authorization routing.
- Keep one persistence-boundary scenario for preservation of provider/catalog settings.
- Keep one reasoning scenario for per-tier reasoning persistence and request construction.
- Keep pagination/search as a separate interaction because it changes the model-selection mechanism.
- Convert provider or credential variants into scenario outlines or focused Given/Then additions rather than full copies of the selection flow.

### F9. Model Filtering and Stale Results Have Four Overlapping Families

Evidence:

- `model-autosuggest.feature:14-38`: text replacement, empty state, stale result, unsupported search.
- `ratatui-model-picker.feature:19-75`: filtering, empty state, metadata, and remote-search fallback.
- `responsive-setup-model-filtering.feature:3-36`: local filtering, remote filtering, stale-result authority, and delayed PTY input.
- `search-concurrency.feature:3-16`: stale-result authority at regular and PTY boundaries.

Some overlap is legitimate because the old model picker and the newer setup coordinator are different production paths. The local-filter, remote-filter, stale-generation, fallback, and PTY responsiveness contracts must be separated explicitly. At present, the repeated `newer query wins` behavior is not separated cleanly and is duplicated by title and intent.

Recommendation:

- Create a behavior matrix with rows for local filtering, provider-backed filtering, stale-result rejection, unsupported-search fallback, and PTY responsiveness.
- Assign each row one canonical scenario per production boundary.
- Do not add another stale-result scenario without naming the distinct production path or test seam.

### F10. Setup Wizard Entry and Page Navigation Are Layered but Overbuilt

Evidence:

- `unified-setup-wizard.feature:4-29` drives the complete five-page setup flow.
- `streamlined-setup.feature:4-28` drives the newer coordinated provider/model/reasoning/review flow.
- `provider-setup-widget-layout.feature:4-12` checks provider page transitions.
- `highlight-active-setup-input.feature:4-55` repeats the same provider, endpoint, credential, model, reasoning, and shell navigation to inspect border focus.
- `provider-setup.feature:43-58` drives first-use provider then model setup.
- `setup-persistence.feature:4-34` drives setup cancellation and credential/catalog persistence boundaries.

These are not all removable. Focus colors, layout, save boundaries, and first-use routing are separate contracts. The full setup happy path in `streamlined-setup` is newer and stronger than the older full flow in `unified-setup-wizard`, though. The older scenario is now mostly a second page-navigation smoke test.

Recommendation:

- Keep one full coordinated setup smoke scenario.
- Keep focused layout/focus scenarios only for assertions not present in the smoke flow.
- Remove or narrow the older unified happy path after checking that its unique compatibility, cursor, and discard assertions are covered elsewhere.
- Use shared interaction helpers rather than duplicating page-driving code.

### F11. Provider Setup Persistence Scenarios Need Invariant Ownership

Evidence:

- `provider-setup.feature:131-165` covers rerun preservation, Escape/Ctrl-C cancellation, and catalog failure after provider setup.
- `setup-persistence.feature:4-34` covers catalog failure before final confirmation and cancellation before/after credential confirmation.
- `streamlined-setup.feature:52-57` and `:222-246` cover coordinated cancellation and catalog failure.
- `unified-setup-wizard.feature:45-51` covers Escape/discard.

The boundaries differ, but titles and assertions repeatedly use generic phrases such as `config file should be byte-for-byte unchanged`, `no provider entry should be persisted`, and `no selected tier assignments`. This makes it difficult to tell whether a scenario protects provider confirmation, catalog confirmation, final review atomicity, or cancellation UX.

Recommendation:

- Name scenarios after the persistence invariant: provider-before-catalog, draft-before-final-review, final-write atomicity, or cancellation UX.
- Keep one scenario per persistence boundary.
- Reuse the existing generic unchanged-config step instead of introducing boundary-specific aliases with equivalent bodies.

### F12. Reasoning Is Tested at Four Boundaries Without a Single Canonical Round Trip

Evidence:

- `reasoning.feature:4-66` covers request inclusion and verbose display.
- `reasoning-policy.feature:4-43` covers default resolution and persistence policy.
- `ratatui-model-picker.feature:39-51` covers per-level UI/request reasoning.
- `streamlined-setup.feature:135-157` and `:366-394` covers catalog choices, free-form values, off, unknown values, and persistence.

The contracts are meaningfully different, but there are same-function pairs:

- `reasoning-policy.feature:26-30` and `streamlined-setup.feature:374-379` both prove an unknown reasoning value remains active and is sent.
- `reasoning-policy.feature:4-8` and `streamlined-setup.feature:366-372` both prove a persisted reasoning value survives into a request, with different value shapes.
- `reasoning.feature:4-10` and `ratatui-model-picker.feature:39-45` both prove a selected tier sends reasoning without printing it.

Recommendation:

- Make the policy feature the canonical resolver/persistence contract.
- Make one CLI request scenario the canonical round trip.
- Keep UI scenarios only for selection/default presentation and verbose-output scenarios only for channel behavior.
- Use examples for effort values instead of repeating full request flows for `low`, `minimal`, `bogus`, and free-form values.

### F13. Shell Completion Scenarios Are a Scenario-Outline Candidate

Severity: **Low**

Evidence:

- `shell-completions.feature:3-155` repeats the same command-tree, stdout-only, deterministic-output, and syntax-check contract for Bash, Zsh, Fish, Elvish, and PowerShell.
- `tests/steps/shell_completions_steps.rs:75-148` has one syntax assertion per shell.
- `tests/steps/shell_completions_steps.rs:227-247` has one byte-for-byte repeat assertion per shell.

The Bash option table differs slightly from the other shells, so this is not a blind text merge. It is still a strong Scenario Outline or parameterized-step candidate with shell-specific examples and an optional option-table row.

Recommendation:

- Use one outline for common generation properties.
- Keep the built-Bash scenario separate because it tests the built binary boundary.
- Keep unsupported-shell and help scenarios separate because they test different contracts.

## Step Definition Findings

The current runner passed without a duplicate-binding panic, so there is no active exact binding collision comparable to the historical `selected_tiers` collision removed by `e0dff35`. The merge opportunities are semantic and implementation-level.

### S1. Duplicate Page Polling and ANSI Helpers

Merge target: shared PTY helpers in `tests/steps/mod.rs`.

Current implementations:

- `tests/steps/setup_wizard_steps.rs:15-43`: `latest_page` and `wait_for_page`.
- `tests/steps/streamlined_setup_steps.rs:6-61`: `latest_page`, `visible_output`, `wait_for_active_page`, and related polling.
- `tests/steps/streamlined_setup_e2e_steps.rs:6-25`: another `visible_output` and `wait_for_page`.
- `tests/steps/highlight_active_setup_input_steps.rs:310-330`: another active-page poller.
- `tests/steps/model_picker_layout_steps.rs:7-21`: another page poller.
- `tests/steps/ask_steps.rs:1607-1622`: another model-picker page poller.

These helpers all strip or inspect terminal output, locate the latest `Page`
marker, and poll with a deadline. Differences such as color-aware screen
reconstruction should remain specialized, but the ANSI cleaner, latest-page
selector, timeout, and generic label polling belong in one shared helper.

### S2. Duplicate PTY Startup and Model Selection Drivers

Merge target: shared low-level operations, with a small number of canonical
Gherkin bindings.

Current implementations include:

- `setup_wizard_steps.rs:45-63` for setup/models startup.
- `provider_setup_steps.rs:1001-1013` for provider and interactive request startup.
- `model_picker_layout_steps.rs:33-42` for `watn models` startup.
- `responsive_setup_model_filtering_steps.rs:347-352` for setup/model startup.
- `ask_steps.rs:1514-1554` for tier-by-tier model selection.
- `ask_steps.rs:1645-1754` for model/reasoning selection and back-navigation.
- `streamlined_setup_e2e_steps.rs:266-325` for role selection and reasoning navigation.
- `provider_setup_steps.rs:1041-1069` for first-use model selection.

The repeated sequence is: type model, wait for result, press Enter, wait for a
reasoning page, select reasoning, and wait for the next role. The page labels
and entry points vary, but the mechanics should be one helper accepting role,
model, reasoning, and next page. This is the highest-value step refactor.

### S3. Duplicate Suggestion Assertions

Merge target: shared ID-level assertion helpers plus separate terminal-screen
assertions.

Current implementations:

- `tests/steps/ask_steps.rs:1092-1162` checks included and excluded model IDs.
- `tests/steps/responsive_setup_model_filtering_steps.rs:408-448` repeats included, excluded, and provider-request checks for rendered setup state.
- `tests/steps/ask_steps.rs:1256-1290` checks current and stale result sets.
- `tests/steps/search_concurrency_steps.rs:33-46` reimplements the same intent with a weaker state check.

The direct picker state assertions can share a helper that receives expected
and forbidden IDs. PTY tests should retain a screen-specific helper. The
Gherkin bindings should not multiply merely because the feature file uses
different prose for the same assertion.

### S4. Duplicate Shell Completion Wrappers

Merge target: one parameterized binding or one common helper.

`shell_completions_steps.rs` has separate wrappers for each shell's syntax and
repeatability assertions. The helper should accept a shell name and expected
syntax marker. The feature can then use a Scenario Outline. Do not merge this
with shell shortcut assertions; completion generation and shortcut installation
are different product boundaries.

### S5. Duplicate Config/Tier Assertions

Merge target: one TOML parser/helper, not necessarily one Gherkin phrase.

The following definitions assert selected tiers with varying strength:

- `tests/steps/ask_steps.rs:816-838` checks selected tier assignments.
- `tests/steps/ask_steps.rs:1820-1874` checks tiers and reasoning strengths.
- `tests/steps/setup_wizard_steps.rs:310-326` parses all three wizard tiers.
- `tests/steps/streamlined_setup_e2e_steps.rs:350-365` checks the three selected role names.
- `tests/steps/streamlined_setup_e2e_steps.rs:394-420` parses role and reasoning pairs.

The strongest helper should parse the config once and expose model/reasoning
lookups. Weaker wrappers should either call it or be deleted when their feature
assertion is a subset.

### S6. Shared Streaming and Shell Helpers Are Mostly Correct

Not every repeated module is a problem. These are good existing patterns:

- `incremental_sse_rendering_e2e_steps.rs` delegates provider setup and release to `incremental_sse_rendering_steps.rs:119-124` and `:164-169`.
- `cancel_completion_steps.rs` reuses streaming request-header/setup helpers.
- `interactive_shell_shortcut_e2e_steps.rs:53-58` delegates Bash widget execution to the regular shortcut module.
- `preserve_ctrl_w_requests_steps.rs:21-28` delegates escaped-input execution to the shared Bash helper.

Preserve this delegation model. The problem is the duplicated scenario
contracts and local polling helpers, not the existence of capability modules.

## Historical Cause

### Timeline

| Date | Evidence | Effect |
|---|---|---|
| 2026-08-07 | `90da800`, `4503c6b` | Initial CLI features and most generic bindings were placed in `ask_steps.rs`. |
| 2026-08-08 | `f95674a`, `233f1d9`, `5a8893a` | Autosuggest and Ratatui picker changes each added model-filter and tier-selection coverage. Both were locally reasonable, but they established overlapping ownership. |
| 2026-08-09 | `07d46f3` | Provider setup added the literal credential precedence scenario. |
| 2026-08-10 | `57688f6`, `eb328dd`, `a0dd716` | Model discovery added the same credential title in a new capability and added a second newest-search scenario. The later archive review described only the new change's interaction matrix. |
| 2026-08-11 | `529615d`, `0590d2d` | Responsive setup added another stale-query scenario and a PTY variant. This tests the newer coordinator path, but reused the same behavior wording. |
| 2026-08-11 to 2026-08-12 | `f4c1b46`, `d23068e`, `e7b7bb6` | Shell shortcut request-preservation work added a stronger Bash E2E scenario without removing the earlier subset. |
| 2026-08-12 to 2026-08-13 | `9f92a2b`, `e3e1b02`, `d116acd` | The setup refactor and streamlined flow added a large, newer setup matrix while older unified/provider/model scenarios remained active. |

### Process Causes

1. **Change-local planning.** Archived reviews enforce one-to-one mapping between a change's inventory and its own scenarios. They do not establish ownership against every permanent feature. For example, `givn/archive/model-discovery-and-setup-correctness/review.md:24-32` lists the new overlapping-search scenario as coverage without comparing it to the existing autosuggest scenario.
2. **Archive merge is additive from the behavior perspective.** Archive commits move change artifacts and add permanent specs. Examples include `57688f6`, `c8f50b7`, and `e3e1b02`. The runner skips archived copies, but the permanent spec remains alongside older permanent specs.
3. **Entry-point slicing.** Authors organized scenarios around `watn setup`, `watn provider`, `watn models`, provider setup, catalog source, and reasoning policy. The same invariant was therefore reintroduced for each command rather than referenced from a behavior ownership matrix.
4. **Global Cucumber registration.** The repository correctly recognized that bindings are global. The initial design put generic steps in `ask_steps.rs`, while later capability modules added new wording to avoid expression collisions. This prevented some hard failures but encouraged aliases and repeated helper bodies.
5. **Superset additions were not cleanup events.** The shell and setup histories show stronger scenarios added later. The old subsets were left active because the new work was treated as additive coverage.
6. **Existing historical findings were not turned into a permanent gate.** `implement-empty-step-assertions` caught a duplicate base/delta scenario in its design review, and `improve-model-selection-autosuggest` removed two placeholder scenarios that duplicated one E2E interaction. Those were change-specific fixes, not repository-wide checks.

## Prevention

### Required Before Adding a Scenario

1. Search all active feature files for the intended behavior, not only the feature directory being changed.
2. Record the invariant, command entry point, production module, and test seam in a repository-wide behavior matrix.
3. Decide whether the new scenario is a new contract, an example row for an existing contract, a stronger replacement, or a distinct boundary test.
4. If it is a stronger replacement, delete or rename the weaker scenario in the same change and record the supersession.

### Automated Checks

Add a lightweight active-spec lint step that:

- Fails on duplicate scenario titles under `givn/specs`.
- Warns on duplicate feature titles.
- Normalizes step text by replacing quoted values and numbers, then reports repeated Given/When/Then shapes across files.
- Reports repeated scenario fingerprints based on command, key interaction verbs, and observable assertions.
- Compares new permanent specs with the current permanent tree during archive so archive cannot introduce an already-owned scenario silently.
- Reports binding expressions and implementation files together, making aliases visible even when their prose differs.

The check must exclude `givn/archive` from active failures while still reporting an archive-to-active match as historical duplication.

### Naming and Ownership Rules

- Give each behavior one canonical feature owner.
- Name layered tests after the boundary: `... through provider resolution`, `... through the CLI`, or `... at final confirmation`.
- Use Scenario Outlines for shell/provider/model value variants when the workflow and assertions are the same.
- Keep one `@e2e` scenario per distinct user interaction unless a second scenario exercises a genuinely different interface boundary.
- Keep regular scenarios at the smallest useful seam; do not repeat a full PTY flow to assert one additional field.
- Put polling, ANSI parsing, PTY startup, model selection, and config parsing in shared helpers.
- Treat any new global step-expression alias as a design-review item, not as harmless wording variation.

### Review Checklist

Every feature review should answer:

- Which existing scenario would fail if this scenario were deleted?
- What exact invariant is new?
- Is the new assertion stronger than an existing scenario?
- Does the test use a different production module or only different prose?
- Can the variant be an Examples row?
- Are any old scenarios or step definitions removed because they are now subsumed?
- Are the regular and E2E steps asserting different boundaries, or just repeating the same interaction?

## Recommended Consolidation Order

1. Remove or rename the two exact duplicate scenario titles: credential precedence and newest search result.
2. Merge the missing-config, empty-search, Bash widget, and failed/empty-output subsets.
3. Replace the fake `search-concurrency` regular steps with a real race test while consolidating stale-result coverage.
4. Extract shared PTY/page/model-selection/config helpers.
5. Reduce the model tier assignment matrix to canonical core, catalog-source, persistence, reasoning, and pagination/search contracts.
6. Reassess the older unified setup happy path after the streamlined setup flow is the canonical coordinated smoke test.
7. Convert shell completion variants to an outline and add active-spec duplicate lint.

## Verification Evidence

The report branch was checked against the unchanged baseline:

```text
./run-tests.sh
19 features
148 scenarios (148 passed)
851 steps (851 passed)

./run-tests.sh --e2e
24 features
75 scenarios (75 passed)
555 steps (555 passed)
```

The E2E run emitted environment limitations for unavailable optional shell
executables (Zsh, Elvish, and PowerShell); the corresponding generated-output
checks passed and syntax checks were skipped only for those unavailable tools.

The current green suite confirms that the active global step registry is
registerable and executable. It does not invalidate the semantic overlap
findings above.
