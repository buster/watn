# Design Review: isolate-test-transport

This rerun follows the explicit scope correction: debug verification is the
focus, release-profile runtime verification is deferred, the fallback case is
one parser-safe scenario, and the bootstrap builds and copies two debug
variants from Cargo's shared default target cache. The normal scenario invokes
only the default-feature debug copy; the test-support copy is reserved for
isolated routing and fallback. The proposal, all change specs, design,
change-level Arc42 assessment, durable Arc42 chapters, ADR-0011, source
transport seam, and Cucumber harness were reviewed before this record was
updated.

## Ranked Findings

| Rank | ID | Severity | Finding | Decision / resolution |
|---|---|---|---|---|
| 1 | F1 | Critical | The endpoint override must not exist in a release-profile binary built with `test-support`; a feature-only guard is insufficient. | Keep the exact `cfg(all(feature = "test-support", debug_assertions))` override branch and its negated configured-endpoint branch. The source guard is in this change; release-profile runtime verification is explicitly deferred to `release-truth-and-repository-cleanup`. |
| 2 | F2 | Major | The previous four-build, per-variant target-directory bootstrap was inefficient and no longer matched the corrected debug scope. | Build `cargo build --bin watn`, copy `target/debug/watn` to one temporary `default-debug` path, build `cargo build --features test-support --bin watn`, and copy it to one temporary `test-support-debug` path. Export only those two absolute paths; do not use `--target-dir`, build in scenarios, discover `target/debug/watn`, or build release variants here. |
| 3 | F3 | Major | Broad or aggregate mock hits alone could hide a wrong endpoint, path, credential, response source, or an incorrectly accounted child invocation. | Use separate loopback twins and exact method/path/Authorization matchers. Record the single normal invocation and both fallback invocations independently; assert one configured hit for the normal child, an aggregate fallback count of exactly two, competing/unused twins at zero, output from the expected twin, and TOML retaining only the configured endpoint. |
| 4 | F4 | Major | Unreachable or incomplete provider fixtures could fail during readiness or model resolution before transport routing is exercised. | Use reachable `127.0.0.1` twins, configured endpoints ending in `/v1`, `sk-configured`, and `default_model = "test-model"`. |
| 5 | F5 | Major | Shared optional world fields or one server could make transport state conditional and allow route ambiguity. | Keep concrete transport-specific state with separate configured, competing, and isolated servers, exact endpoints, mock identifiers, config path/snapshot, credential/model, invocation records, and readiness result. |
| 6 | F6 | Major | A `Scenario Outline` would not reliably execute the missing and whitespace cases through the custom feature parser. | Keep one `@e2e` Scenario with one literal When step that runs two explicit children: one with the override removed and one with whitespace. Then steps assert both child results and exact aggregate counts. |
| 7 | F7 | Major | Fallback and readiness independence are different observable contracts. | Keep the corrected fallback as CLI E2E and readiness as a focused non-`@e2e` predicate scenario asserting readiness, configured endpoint retention, and zero requests. |
| 8 | F8 | Major | Stale-search behavior is a separate model-discovery concurrency defect and would add an unrelated implementation obligation. | Defer it to `model-discovery-and-setup-correctness`; do not change stale-search state or `search_query_delays` here. |
| 9 | F9 | Major | The delta must identify the real interface and prevent skipped, unbound, concurrent, or stale-binary execution from appearing green. | Use real CLI subprocesses, `.fail_on_skipped()`, `max_concurrent_scenarios(1)`, required named binary paths, and prebuilt binaries before Cucumber starts. |
| 10 | F10 | Major | The interaction inventory and matrix must cover the corrected fallback action without multiplying its two input states into separate user interactions. | Keep three inventory entries, three matching `@e2e` scenarios, and three non-empty CLI matrix rows. The fallback row names both explicit child invocations. |
| 11 | F11 | Major | Arc42 and ADR text still described the obsolete four-variant bootstrap and release smoke execution. | Re-derive all 12 rows, mark 1, 2, 4 through 12 affected and 3 unaffected, update the change marker, revise false durable build claims, and align ADR-0011 with chapter 11. |

## Grilling Log

| # | Branch | Question | Decision | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the proposal contain only transport isolation, exact transport evidence, fallback, readiness, and the release safety rule? | Keep those behaviors; defer release runtime proof and stale search. | Resolved. The proposal's normal/release endpoint rule is a product invariant, not a claim that this scenario runs release binaries. This change verifies normal/debug behavior and names `release-truth-and-repository-cleanup` as the later runtime-verification owner. |
| 2 | Tech choices | Is a compile-time feature plus the existing blocking client simpler and safer than a process-local injection rewrite? | Keep one guarded resolution seam, pure URL builders, and the existing client. | Resolved. The debug-plus-feature guard limits the test capability without changing the external protocol or persisted model. |
| 3 | Missing scenarios | Are normal debug, isolated debug, missing/whitespace fallback, persistence, and readiness observable? | Use the three CLI scenarios and the focused readiness predicate. | Resolved. Existing model/provider setup specifications exercise model transport paths through the same seam; no new user interaction is added. Release runtime execution is deliberately not a scenario in this change. |
| 4 | Testability | Can every change scenario fail for wrong endpoint, path, credential, response source, count, persistence, or networked readiness? | Use reachable twins, default model, exact matchers, per-invocation records, child exit/output assertions, aggregate fallback counts, raw TOML checks, and explicit paths. | Resolved at the design level. Existing implementation tasks must finish per-invocation accounting before `@wip` is removed; the normal single-child assertion and the fallback two-child aggregate are distinct evidence requirements. |
| 5 | E2E fidelity | Does each user-facing interaction use the real interface and mechanism? | Use real `watn` CLI subprocesses for the three `@e2e` scenarios; use the public readiness predicate for the local non-E2E contract. | Resolved. No direct provider call substitutes for the CLI scenarios, and no browser or HTTP-client interface is implied. |
| 6 | Interaction Coverage | Does every inventory comment have one matching scenario and one valid, non-empty driving mechanism? | Map each inventory entry to one CLI row. | Resolved. The fallback remains one row despite two explicit child invocations. |
| 7 | Strict mode and runner | Can skipped, outline, stale-binary, or concurrent execution hide a failure? | Use direct feature parsing, `.fail_on_skipped()`, serial execution, required binary paths, and prebuilt copies. | Resolved. The current runner and corrected verify/e2e bootstrap enforce these controls. |
| 8 | Risk | What is the most likely implementation failure? | A child selects the wrong binary or the resolver reads the override in the wrong compilation branch. | Resolved for the current debug scope by distinct copied paths, exact twin counts, and the source guard. Release runtime proof remains an explicit later-change obligation rather than a hidden claim here. |
| 9 | Architecture documentation | Which chapters are affected after independently applying all 12 Arc42 selection rules? | Mark 1, 2, 4 through 12 Yes; mark 3 No because test twins and the override are test architecture, not a product boundary or user-facing surface. | Resolved in the Arc42 check below. |

## Resolved by Codebase Exploration

| Branch | Finding |
|---|---|
| Parser behavior | `tests/features_runner.rs` parses files with `Feature::parse_path` through `VecParser`; the corrected feature has no outline or example placeholders. The fallback phrase is bound literally in `tests/steps/transport_steps.rs` and runs the two explicit children. |
| Strict runner | `tests/features_runner.rs` calls `.fail_on_skipped()` and `.max_concurrent_scenarios(1)`, so an unbound step fails and local-server counts cannot race between scenarios. |
| Binary selection | `tests/steps/mod.rs::binary_from_env` requires a named environment path and asserts that it is a file. It does not discover `target/debug/watn`; the normal step selects only `WATN_DEFAULT_DEBUG_BIN`, while isolated routing and fallback select `WATN_TEST_SUPPORT_DEBUG_BIN`. |
| Build bootstrap | `givn/commands.yaml`'s verify and E2E commands build the default-feature debug binary, copy it, build the debug `test-support` binary from the same Cargo target cache, copy it, and pass only the two absolute paths before Cucumber starts. No scenario builds or discovers a binary. |
| Compile boundary | `src/provider/transport.rs` contains the exact debug-plus-feature override branch and the negated configured-endpoint branch. This supports the release safety invariant without pretending that release runtime smoke testing is part of this change. |
| Request construction | Chat and model request paths currently use the transport seam. The design still requires resolution once before pure URL builders and forbids a second environment lookup or use during config, readiness, persistence, or display. |
| Fixture correctness | The transport steps create separate loopback `httpmock` servers, configure `/v1`, `sk-configured`, and `test-model`, match exact chat method/path/header, and capture child output and status. Independent invocation records remain a completion condition because the current partial implementation still has aggregate hit assertions. |
| Readiness | `provider_ready` resolves provider configuration and credentials without constructing an HTTP URL. The readiness scenario asserts `true`, exact configured endpoint retention, and zero hits on both local servers while the competing override is present. |
| Scope | The stale-search implementation and `search_query_delays` behavior are outside this transport delta and remain assigned to `model-discovery-and-setup-correctness`. |

## Architecture Documentation (arc42) Check

The independent assessment was made against the 12-row selection table from
`givn instructions arc42-docs --change isolate-test-transport` before relying
on `givn/changes/isolate-test-transport/arc42.md`. The marker and all durable
chapter contents were then compared row by row.

| # | Chapter | Independent assessment | `arc42.md` assessment | Match? | Content / integrity result |
|---|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Yes | Yes | Test isolation, configured-endpoint authority, and release safety are recorded. |
| 2 | Architecture Constraints | Yes | Yes | Yes | The compile guard, debug verification scope, and two explicit copied paths are recorded. |
| 3 | Context and Scope | No | No | Yes | No product boundary, external provider contract, or user-facing surface changes; loopback twins are test fixtures. |
| 4 | Solution Strategy | Yes | Yes | Yes | The guarded seam, pure builders, shared-cache bootstrap, and deferred release proof are recorded. |
| 5 | Building Block View | Yes | Yes | Yes | The transport boundary and configured/readiness split are recorded. |
| 6 | Runtime View | Yes | Yes | Yes | Debug normal, isolated, fallback, and zero-network readiness flows are recorded; release runtime proof is not falsely shown as current. |
| 7 | Deployment View | Yes | Yes | Yes | The two debug verification copies and unchanged product release deployment are recorded. |
| 8 | Cross-cutting Concepts | Yes | Yes | Yes | Environment, Authorization, persistence, readiness, and security rules are recorded. |
| 9 | Architecture Decisions | Yes | Yes | Yes | ADR-0011 records the debug-plus-feature seam, pure builders, efficient debug bootstrap, and deferred release verification. |
| 10 | Quality Requirements | Yes | Yes | Yes | Debug exact-transport, fallback, readiness, and source-level release-safety criteria are recorded without claiming a current release smoke run. |
| 11 | Risks and Technical Debt | Yes | Yes | Yes | Release proof deferral, stale binary selection, broad mocks, and per-child evidence obligations are recorded. |
| 12 | Glossary | Yes | Yes | Yes | Transport, debug/test-support binary, release-profile binary, endpoint, and competing-twin terms are defined. |

- [x] All 12 rows were independently assessed before relying on the marker.
- [x] No independent Yes row is omitted or marked No in `arc42.md`.
- [x] All 12 durable chapter files exist and contain project-specific content.
- [x] No Unicode box-drawing or non-table pseudo-diagram was found; diagrams use Mermaid fenced blocks.
- [x] ADR-0011 contains the non-trivial transport decision and its consequences are reflected in chapter 11.

## Correction and Hardening Summary

| Artifact | Verified correction or hardening |
|---|---|
| `specs/transport/transport.feature` | The fallback case is one `@e2e` Scenario, not an outline. Its literal When step runs one missing and one whitespace child invocation, and Then steps assert both results plus exact aggregate counts. |
| `design.md` | The bootstrap uses two sequential shared-cache builds and copies, but the normal scenario invokes exactly one default-feature debug child. The test-support debug copy is reserved for isolated routing and fallback; no scenario invokes both copies as a pair, and no release build is part of this change. The source release guard and later verification deferral are explicit. |
| `givn/commands.yaml` | Verify and E2E bootstrap commands export only the two copied debug paths and do not build release variants. |
| `arc42.md` and durable docs | The 12-row assessment, two-copy build topology, release deferral, ADR-0011, and chapter 11 consequences now match the corrected design. |
| `tasks.md` | Not edited. Existing task progress remains the implementation source of truth. |

No proposal, production source, test source, or task-progress edit was made by
this rerun.

## Verification

- `givn lint --change isolate-test-transport`: clean, exit 0.
- The feature inventory has exactly three entries, each maps to one `@e2e` scenario and one non-empty real-subprocess matrix row.
- The fallback mechanism is parser-safe and RED-testable without outline substitution.
- The runner is strict and serial; exactly two copied debug paths are required before scenarios run.
- Arc42 is enabled; all twelve chapter files are present and substantive, and ADR-0011/chapter 11 align with the corrected design.
- Existing `tasks.md` and its progress were left untouched.
- No production or test source file was edited by this rerun.

## Sign-off

- [x] Scope, technology, missing scenarios, RED testability, E2E fidelity, interaction coverage, risk, runner strictness, efficient bootstrap, and release deferral were reviewed.
- [x] The parser-specific fallback correction is recorded and consistent across feature, design, and runner binding.
- [x] The source release guard is retained, while release-profile runtime verification is explicitly deferred to a named later change.
- [x] All findings are resolved or explicitly deferred to a named later change.
- [x] `givn lint` is clean.
- [x] Arc42 impact assessment and ADR consequence coverage are complete.
- [x] Tasks and task progress were not changed.

DESIGN-REVIEW: PASS
