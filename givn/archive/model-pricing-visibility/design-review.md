# Design Review: model-pricing-visibility

## Grilling (fresh-context subagent)

The plan (proposal, delta specs, design, arc42 impact, glossary, relevant
code, and test harness) was grilled in a fresh context. Findings and
dispositions:

| # | Severity | Finding | Disposition |
|---|---|---|---|
| 1 | Blocker | The exact-price Then in the models delta used `the output should contain ...`, which reads stdout only; the piped model list is written to stderr (`src/models/mod.rs:158-161`). The scenario could never go GREEN, and a second same-prose step is an ambiguous match. | Fixed: the delta now uses the existing `stderr should contain {string}` step and an added `stderr should not contain "$-"` assertion. |
| 2 | Major | Priced-table unit semantics were undefined: table values are $/1M while the mock catalog must publish per-token values. The likely implementation (embedding the table value as per-token) would render `$150000.00/1M`. | Fixed: `design.md` now states that Gherkin tables are $/1M and the mock helper emits `value / 1_000_000.0`; rich fixture values are per-token strings. |
| 3 | Major | The negative sentinel was never chosen in any scenario, so the guard could be missing and all assertions still pass; sentinel display was unasserted. | Fixed: the capture scenario selects `model-sentinel` for the normal tier, asserts no config entry, and the display scenario asserts stderr contains no `$-`. |
| 4 | Major | A chosen model with no catalog price and an existing recorded price was never asserted to keep that price. | Fixed: the capture scenario seeds `model-plain`, chooses it for the thinking tier, and asserts the seeded values survive. |
| 5 | Major | Coordinated setup (`apply_result`) capture was designed and documented in arc42 but had no scenario. | Fixed: the coordinated `@e2e` scenario now uses a priced transport table and asserts all three captured prices; `design.md` maps all three write paths to evidence. |
| 6 | Minor | Capture→cost was not exercised end to end; the traceability table overclaimed the permanent cost scenario as evidence. | Fixed: the traceability row now states the composition explicitly and adds no new cost-formula scenario; the cost formula and read side are unchanged. |
| 7 | Minor | The separate `CatalogPrice` type was defensible but the stated justification was wrong; parse is the single origin, so parse-time normalization is simpler and removes fixture unit ambiguity. | Fixed: the design now normalizes once during parsing and keeps `ModelPricing` ($/1M) as the only type; the glossary keeps `Catalog price` as the provider-unit concept. |
| 8 | Minor | Float round-trip can persist `0.15000000000000002` and make textual equality brittle. | Fixed: parse rounds to six decimals per 1M; assertion steps parse TOML and compare values, not text. |
| 9 | Minor | "published price" vs "Catalog price" drift, and mixed-component semantics were pinned nowhere. | Fixed: glossary defines `Catalog price` and `Price capture`; specs use catalog-price wording; the design requires both components present and non-negative, else no price. |
| 10 | Minor | Step-file wording; streamlined delta Feature title mismatched the permanent title; two-decimal display could theoretically show `$0.00` for sub-half-cent prices. | Fixed: wording corrected, Feature title aligned to `Streamlined setup flow`, and the design records live-catalog evidence (smallest positive price `$0.017/1M`, renders `$0.02`; no live entry rounds to zero). |

### Human questions raised

1. Mixed/partial price metadata — resolved: both components required and
   non-negative; otherwise the model has no catalog price. A partial object can
   never overwrite a recorded price with a fake zero.
2. Display precision — resolved: two decimals per 1M, justified against the
   live catalog minimum.
3. Coordinated setup scope — resolved: Price capture applies to every model
   choice write path, including coordinated setup; a scenario was added.

No question remains open.

## E2E Fidelity and Interaction Coverage

- The Interaction Coverage Matrix contains all 25 inventory entries from
  `givn/specs/configure-model/usecase.md`, each with a real interface and a
  non-empty driving mechanism (piped subprocess, PTY, or `httpmock` twin).
- The change adds no inventory entry and no new consumer action; it modifies
  two existing `@e2e` scenarios in place, preserving the one-E2E-per-action
  rule. `@e2e` tags are retained.
- The e2e runner (`./run-tests.sh --e2e`) is configured; strict mode is
  `.fail_on_skipped()` plus the non-zero exit on skipped/failed scenarios.

## Domain and Ubiquitous Language

- Use case `configure-model`, Actor `Watn user`, confirmed Personas: none. No
  Persona was invented or promoted; capability ownership matches the proposal
  routing (`models`, `ratatui-model-picker`, `streamlined-setup`).
- `Catalog price`, `Price capture`, and `Pricing` are recorded in
  `docs/arc42/12-glossary.md` and used consistently in the proposal, specs, and
  design.

## Arc42 Cross-check

The 12-row assessment in `givn/changes/model-pricing-visibility/arc42.md` was
independently re-derived and matches: chapters 5, 6, 8, 11, and 12 affected;
1, 2, 3, 4, 7, 9, and 10 unaffected. The affected chapter files contain the
promised changes; all 12 chapter files exist, contain real content, and use no
ASCII-art diagrams. No ADR candidate passes all mandatory qualification
dimensions; each non-qualifying choice names exactly one canonical destination
(design.md or the Gherkin spec), and the existing-ADR check (ADR-0020,
ADR-0021, ADR-0024) found no amendment or supersession.

## Hardening Applied

- `proposal.md`: partial/invalid price rule explicit.
- `design.md`: parse-time normalization, both-components rule, rounding,
  fixture units, coordinated scenario, traceability corrections, step-file
  corrections.
- `specs/models/models.feature`: stderr assertions, sentinel selection, bare
  chosen model preservation.
- `specs/streamlined-setup/streamlined-setup.feature`: priced coordinated e2e,
  Feature title aligned.
- `arc42.md` and `docs/arc42/{05,06,08,12}`: normalize-at-parse wording.

`givn lint --change model-pricing-visibility` exits with only `@wip` findings
(expected before implementation) and one long-scenario warning inherited from
the coordinated e2e.

DESIGN-REVIEW: PASS
