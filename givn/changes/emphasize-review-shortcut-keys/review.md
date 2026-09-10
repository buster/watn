# Review: emphasize-review-shortcut-keys

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — no `@e2e` scenario was added, modified, or stripped; the styling change adds no interaction |
| 1 | Empty/no-op step bodies | PASS — 0 `unimplemented!()`/`todo!()` bodies remain across the two capability step files |
| 2 | Checked tasks have commits touching production | PASS — `b382fe1` changes `src/review/card.rs` and the step assertions; `057c9cc` rewrites the reworded permanent scenario and the E2E wait labels |
| 3 | design.md components exist | PASS — `Ink::key`, `Ink::accept_key`, the review/editor/chooser hint builders, and raw-SGR assertions |
| 4 | Strict-mode proof present | PASS — tasks.md RED records the targeted run at exit 1 with the stub panic |
| 5 | E2E Then steps assert the real interface | PASS — unchanged E2E scenarios still assert PTY content and the command-output channel; only the card wait label moved to the emphasized `⏎` token because `accept` is now split by SGR codes |
| 6 | Browser-UI driver check | N/A — CLI capability |
| 7 | `verify.e2e_command` binding and duplicate implementations | PASS — `./run-tests.sh --e2e` selects `@e2e`; one E2E implementation per capability |
| 8 | E2E action normalization | PASS — no new action; styling is not a distinct user action |
| 9 | `verify.command` vs `verify.e2e_command` | PASS — distinct strings; 210 regular versus 82 e2e scenarios in the instrumented runs |
| 10 | Implementation vs design.md | PASS — the review hint emphasizes the acting letter inside the word, the editor and chooser hints emphasize their key tokens, and the design now records the `⏎` E2E wait label |
| 11 | Interaction coverage cross-reference | PASS — inventory and matrix are unchanged from `simplify-review-card-controls`; the same nine actions map to the same `@e2e` scenarios, and the PTY drivers are untouched |
| 12 | Coverage measurement validity | PASS — see Coverage classification |
| 13 | Local run command and twins | PASS — `cargo run --release -- --help`; in-process provider twin, no external service |
| 14 | README impact | `README-IMPACT: none` — the README does not document card styling or key bindings |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Emphasized hint keys | `docs/arc42/05-building-block-view.md` | Affected | Hint Rendering | Scenario | `card.rs` hint builders | Yes |
| QS-067 decision hints | `docs/arc42/10-quality-requirements.md` | Affected | Hint Rendering | Scenario | QS-067 bullets and matrix row updated | Yes |
| Chapters 1-4, 6-9, 11, 12 | n/a | Unaffected | ADR `NOT_QUALIFIED` | n/a | No flow, term, risk, or decision change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

No term is added, renamed, or redefined. The hint words use the existing
`Review decision` vocabulary; `Model chooser` is reused as recorded.

UBIQUITOUS LANGUAGE: CLEAN

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The review card exposes the direct decision shortcuts | The review card exposes the decision shortcuts | variant |

The old permanent scenario asserted the removed `a accept`/`e edit` tokens; the
delta removes it and adds the reworded scenario asserting the same decisions
with the letter-in-word hints. The new behavior is covered by that scenario and
by "The review card emphasizes the decision shortcut keys".

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.2%** (instrumented runs: 210 regular and 82
e2e scenarios, all passed).

| Region | Coverage |
|---|---|
| `src/review/card.rs` | 542/543 (99.8%) |
| `src/review/panel.rs` | 842/878 (95.9%) |
| `src/review/session.rs` | 69/79 (87.3%) |
| `src/main.rs` | 534/732 (73.0%) |

This change adds no uncovered region: the emphasized hint lines and both
rewritten scenario steps are fully covered. The residual gaps are the ones
classified and justified in `simplify-review-card-controls`: terminal failure
paths, defensive chooser guards, the truncation block terminator, interruption
timing, and pre-existing subcommand/error branches (bucket 3 in each case).
No dead code and no unclassified missing coverage.

## Verification runs

- `./run-tests.sh` (instrumented) → exit 0; 21 features, 210 scenarios, 1271 steps passed
- `./run-tests.sh --e2e` (instrumented) → exit 0; 24 features, 82 scenarios, 599 steps passed
- `./measure-coverage.sh` → both source reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change emphasize-review-shortcut-keys` → exit 0, clean; one advisory subset notice is dispositioned above
- `cargo check --locked` → clean
- `cargo test --locked --lib` → 77 passed
- Commits: `b382fe10a35b903718d04c53d7a4858fa08f6fba`, `057c9cc22b3dc60f9d7e914b3fcfa150676f8273`

## README impact decision

README-IMPACT: none

The README does not document the card's hint styling, and no CLI flag,
configuration key, or output contract changed.

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0; e2e scope is a strict
  subset (82 < 210).
- Coverage measured across the runner, both binaries, and the unit tests; no new
  gap and all residual gaps remain classified.
- No `@wip` tags remain.
- No interaction changed, so the existing `@e2e` coverage stays valid.
- Durable arc42 chapters match the implementation.
- Local run command starts the CLI with the in-process provider twin.

REVIEW: PASS
