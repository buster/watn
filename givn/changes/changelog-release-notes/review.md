# Review: changelog-release-notes

## Use-case and Persona conformance

Owning contract: fragment `corpus-infra`, capability `release-truth`. The
fragment records no actor goals and no interactions; the change amends the
capability's scenarios and adds no typed relationship.

Persona disposition: `terminal-developer--interactive` is the only confirmed
persona and remains a review lens (reader of the published release notes); it
is not a Gherkin actor, and no persona biography appears in the specs.

Interface classification, opposite tested: a browser-rendered page is a Web UI.
The opposite does not hold here — the change renders no page and adds no
screen; its outputs are a generated Markdown section and a CLI process.

No mandatory check is deferred; no operator deferral is recorded.

## Step 0 — fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in the delta spec | Clean. `git log -p` on the delta feature shows the only tag change was `-@givn.added @e2e @wip` → `+@givn.added @e2e`; the `@e2e` tag was never removed. |
| 1 | Empty or no-op step bodies | Clean. `grep -rn 'unimplemented!\|todo!()' tests/` → 0 matches; every new step asserts, builds a fixture, or runs the real tool. |
| 2 | Every checked task has a commit touching production source | `ef6f197` (`.github/workflows/release.yml`), `9bcec21` (`cliff.toml`, `.github/workflows/ci.yml`, `tests/steps/release_truth_steps.rs`), `3657bc4` (`cliff.toml`, `tests/steps/release_truth_e2e_steps.rs`), `837a4f2` (full-suite evidence and the real-range preview). T1 is the setup task; it records the strict-mode proof and claims no production change. No `.feature`-only or stub commit. |
| 3 | Components `design.md` promised exist | `ChangelogFixture`, `changelog_fixture()`, `find_git_cliff()`, and the assertion steps in `tests/steps/release_truth_steps.rs`; the e2e fixture and assertions in `tests/steps/release_truth_e2e_steps.rs`; the release guard in both `release.yml` steps. |
| 4 | Proof of strictness present with non-zero exit | Present in `tasks.md` T1 and both REDs: undefined step → `1 scenario (1 failed)`, `Step doesn't match any function`, exit 1; each RED → `Step panicked. Captured output: not implemented`, exit 1. |
| 5 | Every `@e2e` scenario asserts on the real interface | Clean. The Then steps read git-cliff's stdout for the release range; no repository- or database-only assertion. |
| 6 | Browser-UI step fidelity | Not applicable: the interface is a CLI subprocess; no browser or HTTP client is used as a driver. |
| 7 | `verify.e2e_command` target and duplicate implementations | `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags '@e2e and not @wip'`. One implementation per step; no second, weaker `@e2e` implementation exists in the tree. |
| 8 | E2E scope: one e2e scenario per normalized action | The fragment records no interactions; the capability keeps exactly one `@e2e` scenario, matching the design's matrix row. No excess e2e action. |
| 9 | Local run command starts the stack including twins | `./run-tests.sh` and `./run-tests.sh --e2e`; `git-cliff` 2.13.1 on `PATH` and a temporary git fixture; no container, server, or network, so no digital twin exists. |
| 10 | `verify.command` vs `verify.e2e_command`, isolation proven | Distinct strings; the receipt counts prove isolation: 274 regular scenarios vs 92 e2e scenarios, strictly fewer. |
| 11 | Implementation vs `design.md` deviations | Conformant. The named files, the five user-visible groups, the `unique(attribute="message")` template change, and the two-step release guard are as designed; the design-review's move of the e2e assertions into the e2e step file is recorded in `design.md`. No silent deviation. |
| 12 | Findings reopened as work | None at review time. The design-review findings (recovery-path guard, e2e step separation, fallback fixture, `fix(e2e)` clarification, unique limitation, arc42 verdict, persona disposition, step captures) were hardened in the planning artifacts before tasks and are reflected in the implementation. |
| 13 | Interaction coverage cross-reference | See the table below. |
| 14 | Coverage measurement validity | Valid. `./measure-coverage.sh` (exit 0) instrumented the default and test-support binaries and the runner, wrote collision-safe profiles, and ran both scopes (274 and 92 passed); `./merge-coverages.sh` (exit 0) freshly wrote `coverage/cobertura-coverage.xml`; overall 92.89% (21191/22812 lines); `src/amount.rs` reads 100%, a known exercised production path. |

### Interaction coverage cross-reference

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Verified |
|---|---|---|---|---|
| none (fragment `corpus-infra` records no interactions) | Repeated revisions of one change yield one changelog entry | CLI | Real `git-cliff` subprocess with the project's `cliff.toml` over a temporary git fixture; stdout is read | Clean — `tests/steps/release_truth_e2e_steps.rs` |

## Arc42 implementation conformance

| Chapter / fact | Durable source | Implementation evidence | Result |
|---|---|---|---|
| 11 R-095 — a range without user-visible change stops the release | `docs/arc42/11-risks-and-technical-debt.md` | `release.yml` guard in prepare and publish (`ef6f197`) | Match |
| 11 R-096 — the acceptance suite depends on the pinned git-cliff | `docs/arc42/11-risks-and-technical-debt.md` | `ci.yml` installs 2.13.1 (`9bcec21`); the step fails with the pinned install command when absent | Match |
| 12 `Release note` and `Changelog` | `docs/arc42/12-glossary.md` | proposal, spec, design, and this change's revisions reuse the terms | Match |
| 01-10 unchanged | the corresponding chapters | no Watn component, runtime, interface, quality, or deployment change; release tooling only | Match |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term | Glossary | Specs / design | Implementation |
|---|---|---|---|
| Release note | present (added in this change) | the proposal's release note, the spec scenarios, the design's subject rule | every revision of this change carries it as the message subject |
| Changelog | present (added in this change) | the capability's scenario titles and the design's five groups | `cliff.toml` groups and collapse rule; the preview under `evidence/` |

No term is used that the glossary does not hold; no drift between spec, design,
and configuration.

## Visual review

Not applicable: the change declares no visual interface and introduces no
screen. No screenshot or transcript evidence is required for release tooling.

## Deterministic gate dispositions

`givn lint --change changelog-release-notes` reports `1 file(s) checked —
clean`: no shape match, no structural subset, no long scenario. No overlap
disposition is required, and the two scenarios are short.

## README impact decision

README-IMPACT: none — the change adjusts release tooling and the generated
release notes; no consumer-visible command, flag, configuration key, or
terminal output changes.

## Coverage classification

This change adds and modifies no production Rust lines, so it introduces no
uncovered region. The merged report is fresh and valid (check 14); the overall
source coverage is unchanged at 92.89%, and no gap is reclassified as
hard-to-test.

## Sign-off

- [x] `./run-tests.sh` green: 274 scenarios (274 passed)
- [x] `./run-tests.sh --e2e` green: 92 scenarios (92 passed)
- [x] Fabrication audit clean; `@e2e` tag intact
- [x] Arc42 conformance clean
- [x] Ubiquitous language clean
- [x] No deferred mandatory check without an operator decision
- [x] README impact decided

REVIEW: PASS
