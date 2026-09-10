# Review: clarify-review-card-guidance

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — no `@e2e` scenario was added, modified, or stripped; the change is card copy and default selection |
| 1 | Empty/no-op step bodies | PASS — 0 `unimplemented!()`/`todo!()` bodies remain across the two capability step files |
| 2 | Checked tasks have commits touching production | PASS — `f96b31a` (card chooser rows and hint), `832f659` (panel default highlight), `61b00bb` (marker rename); an accidental `review.cast` inclusion was untracked in the evidence commit |
| 3 | design.md components exist | PASS — keyed tier digits, active-model marker, `Picks`/`Search` labels, placeholder, new hint, `NESTED_SYNTAX_MARKER`, default highlight |
| 4 | Strict-mode proof present | PASS — tasks.md RED records the targeted run at exit 1 with the stub panic for each scenario |
| 5 | E2E Then steps assert the real interface | PASS — unchanged E2E scenarios still assert PTY content and the command-output channel; the reject E2E uses the tier key `2` and does not depend on the default highlight |
| 6 | Browser-UI driver check | N/A — CLI capability |
| 7 | `verify.e2e_command` binding and duplicate implementations | PASS — `./run-tests.sh --e2e` selects `@e2e`; one E2E implementation per capability |
| 8 | E2E action normalization | PASS — no new action; guidance, default selection, and a marker rename are not distinct user actions |
| 9 | `verify.command` vs `verify.e2e_command` | PASS — distinct strings; 212 regular versus 82 e2e scenarios in the instrumented runs |
| 10 | Implementation vs design.md | PASS — exact copy, active-marker rule (`context.model`), highlight precedence (empty query only), marker constant, and the enumerated step/unit-test migration all match |
| 11 | Interaction coverage cross-reference | PASS — inventory and matrix are unchanged from `simplify-review-card-controls`; the same nine actions map to the same `@e2e` scenarios and PTY drivers |
| 12 | Coverage measurement validity | PASS — see Coverage classification |
| 13 | Local run command and twins | PASS — `cargo run --release -- --help`; in-process provider twin, no external service |
| 14 | README impact | `README-IMPACT: none` — the README documents no chooser copy or card marker wording |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Chooser labels, marker, nested-syntax copy | `docs/arc42/05-building-block-view.md` | Affected | Chooser Rendering | Scenarios A-C | `card.rs` rows and marker constant | Yes |
| Chooser guidance and ready-to-choose selection | `docs/arc42/10-quality-requirements.md` | Affected | Default Highlight | Scenarios A-B | QS-074 bullet updated | Yes |
| Chapters 1-4, 6-9, 11, 12 | n/a | Unaffected | ADR `NOT_QUALIFIED` | n/a | Internal `unsupported` concept unchanged | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The user-facing marker is `nested syntax`; the internal `StageSupport`
`unsupported` concept and the glossary entry are unchanged. `Model chooser`
terms (tiers, picks, search) reuse the recorded vocabulary.

UBIQUITOUS LANGUAGE: CLEAN

## Overlap dispositions

No deterministic shape match remains in the current finding set. The
default-selection scenario proves a different invariant from the catalog
scenarios: with an empty query the first pick is selected and Enter uses it,
while the catalog-typing and typed-model scenarios cover query filtering and
typed-text precedence; the highlight precedence rule keeps them distinct.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.2%** (instrumented runs: 212 regular and 82
e2e scenarios, all passed).

| Region | Coverage |
|---|---|
| `src/review/card.rs` | 545/546 (99.8%) |
| `src/review/panel.rs` | 865/898 (96.3%) |

This change adds no uncovered region: the chooser rows, hint, marker paths, and
default-highlight state are covered by the three scenarios and the new unit
assertions. Residual gaps are the same bucket-3 groups justified in
`simplify-review-card-controls`: terminal failure paths, defensive
chooser-invariant guards, the truncation block terminator, interruption timing,
and pre-existing subcommand/error branches. No dead code, no unclassified
missing coverage.

## Verification runs

- `./run-tests.sh` (instrumented) → exit 0; 21 features, 212 scenarios, 1283 steps passed
- `./run-tests.sh --e2e` (instrumented) → exit 0; 24 features, 82 scenarios, 599 steps passed
- `./measure-coverage.sh` → both source reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change clarify-review-card-guidance` → exit 0, clean; two advisory subset notices dispositioned above
- `cargo check --locked` → clean
- `cargo test --locked --lib` → 78 passed
- Commits: `f96b31aee5a84c40be44f6f10b0a86f1b44ddbe5`, `832f659c8ded8e43db591cc48c3b480cc36488a0`, `61b00bb3cf3eb3e6ae49d612e5560eaee1bb572e`

One unrelated setup PTY scenario ("Invalid catalog data switches to manual
model selection") failed once during the change and passed both targeted and on
rerun; it touches no code changed here.

## README impact decision

README-IMPACT: none

The README does not document chooser copy, the active-model marker, or the card
marker wording, and no CLI flag, configuration key, or output contract changed.

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0; e2e scope is a strict
  subset (82 < 212).
- Coverage measured across the runner, both binaries, and the unit tests; no new
  gap and all residual gaps remain classified.
- No `@wip` tags remain.
- No interaction changed, so the existing `@e2e` coverage stays valid.
- Durable arc42 chapters match the implementation.
- Local run command starts the CLI with the in-process provider twin.

REVIEW: PASS
