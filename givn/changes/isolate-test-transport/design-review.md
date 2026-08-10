# Design Review: isolate-test-transport

## Ranked Findings

| Rank | ID | Severity | Finding | Decision / resolution |
|---|---|---|---|---|
| 1 | F1 | Critical | A feature-only guard would leave `WATN_TEST_ENDPOINT_OVERRIDE` available in a release build with `test-support`. | Use `cfg(all(feature = "test-support", debug_assertions))` and its negation. Build and run the release binary with the feature enabled against a competing local twin. |
| 2 | F2 | Critical | The harness could execute stale `target/debug/watn` or overwrite the binary while scenarios run. | Prebuild four feature/profile rows in isolated target directories and pass four absolute paths to the harness before Cucumber starts. Missing paths fail bootstrap. |
| 3 | F3 | Major | Broad mock hits did not prove endpoint, path, request count, Authorization, or which server returned the response. | Use separate loopback twins and exact method/path/header matchers. Assert full URL, exact counts, competing zero hits, response source, and raw persisted endpoint. |
| 4 | F4 | Major | The original fixtures used unreachable example endpoints and did not guarantee that the CLI could resolve a model. | Generate all provider endpoints from reachable `127.0.0.1` twins, persist `/v1`, `api_key = "sk-configured"`, and `default_model = "test-model"`. |
| 5 | F5 | Major | Shared optional world fields made transport state conditional and allowed one-server ambiguity. | Define concrete `TransportState` with configured, competing, and isolated server handles, exact URLs, per-binary mock ids, config snapshot, expected credential/model, override state, binary paths, and invocation results. |
| 6 | Major | F6 | Missing and whitespace override behavior and readiness independence were not directly observable. | Add one `@e2e` scenario outline with `missing` and `whitespace` examples, plus a focused non-`@e2e` readiness contract asserting `true` and zero network requests. |
| 7 | Major | F7 | The stale-search scenario was a false-green sequential test and the design made it an implementation obligation without a delta or matrix row. | Remove stale-search work from this transport change and record it for `model-discovery-and-setup-correctness`. No stale-search state or implementation is changed here. |
| 8 | Major | F8 | The active delta did not make the exact release/debug interface and strict runner behavior clear. | Name the CLI subprocess interface, `.fail_on_skipped()`, explicit path variables, prebuilt commands, scenario commands, and non-E2E readiness boundary. |
| 9 | Major | F9 | The inventory and matrix did not cover newly required fallback interactions. | Update the feature inventory to three CLI interactions and map each to exactly one `@e2e` scenario declaration and one matrix row. Readiness is not an inventory interaction because it is a local predicate. |
| 10 | Major | F10 | Arc42 impact had not been recorded at change level, and durable chapters did not state the release-feature consequence. | Walk all 12 rows independently, create `arc42.md` with `STATUS: DONE`, update every affected durable chapter and ADR-0011, and leave chapter 03 unchanged because product context is unaffected. |

## Grilling Log

| # | Branch | Question | Recommended | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the proposal cover only transport isolation and have an unambiguous done condition? | Keep endpoint isolation, exact test evidence, fallback, readiness, and release guarantees; defer stale search. | Agreed. Proposal now names stale search as a later model-discovery change. |
| 2 | Tech choices | Is a compile-time feature plus the existing blocking client simpler and safer than a process-local injection rewrite? | Keep the existing client and use a single guarded resolution seam; URL builders remain pure. | Agreed. The release guard is stronger than the former feature-only plan. |
| 3 | Missing scenarios | Are boundary values and readiness independently observable? | Add missing/whitespace outline and a direct readiness contract with zero network hits. | Agreed and applied. |
| 4 | Testability | Can every new scenario fail for the wrong endpoint, wrong path, wrong key, wrong count, persistence leak, or stale binary? | Use reachable local fixtures, default model, separate twins, exact matchers, explicit counts, raw TOML checks, and explicit binary paths. | Agreed and applied. |
| 5 | E2E fidelity | Does each user-facing interaction use the real interface and mechanism? | Use real CLI subprocesses for the three `@e2e` scenarios; keep readiness non-`@e2e` because it is a local predicate. | Agreed and applied. |
| 6 | Interaction Coverage | Does every inventory entry have exactly one matching `@e2e` scenario and a valid mechanism? | Maintain three inventory entries, three matching scenario declarations, and three CLI matrix rows. | Agreed and applied. |
| 7 | Risk | What is the highest-probability implementation failure? | A release-feature binary or stale target path silently routes through the override; the release rows, isolated targets, and competing-server counts catch both. | Resolved. |
| 8 | Architecture documentation | Which chapters are affected after independently walking all 12 rows? | Update the 11 affected chapters/index and ADR-0011; leave context-and-scope unchanged. | Resolved in the arc42 check below. |

## Resolved by Codebase Exploration

| Branch | Finding |
|---|---|
| Testability | `tests/features_runner.rs` already uses `.fail_on_skipped()` and serial scenarios, but `tests/steps/mod.rs::find_binary` falls back to `target/debug/watn`; the design now forbids that fallback and supplies explicit paths. |
| Testability | The current transport resolver unconditionally reads `WATN_TEST_ENDPOINT_OVERRIDE`; the release-feature requirement therefore needs a `debug_assertions` guard, not merely `not(feature)`. |
| Fixture correctness | `resolve_default_model` requires a custom provider's `default_model`; transport fixtures now write `test-model` and reachable loopback endpoints. |
| Readiness | `provider_ready` resolves configuration and credentials without HTTP; the new readiness scenario asserts this remains true and produces zero local-server hits with an override present. |
| Scope | The stale-search implementation uses sequential `spawn_blocking` calls and write-only `search_query_delays`; it is deferred rather than silently claimed by this transport change. |

## Open Questions

| # | Question |
|---|---|
| - | None. |

## Architecture Documentation (arc42) Check

The independent assessment was made against the 12-row selection table from
`givn instructions arc42-docs --change isolate-test-transport` before relying
on the change marker. The marker and durable chapter contents were then
compared row by row.

| # | Chapter | Independent assessment | `arc42.md` assessment | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Yes | Yes | Yes: release isolation and test-isolation quality goals are recorded. |
| 2 | Architecture Constraints | Yes | Yes | Yes | Yes: compile guard and isolated target requirements are recorded. |
| 3 | Context and Scope | No | No | Yes | Yes: no product boundary changed; test twins remain fixtures only. |
| 4 | Solution Strategy | Yes | Yes | Yes | Yes: guarded seam, pure builders, and build matrix are recorded. |
| 5 | Building Block View | Yes | Yes | Yes | Yes: transport boundary is a distinct resolution seam. |
| 6 | Runtime View | Yes | Yes | Yes | Yes: isolated, fallback, release, and readiness flows are recorded. |
| 7 | Deployment View | Yes | Yes | Yes | Yes: four profile/feature binaries and isolated targets are recorded. |
| 8 | Cross-cutting Concepts | Yes | Yes | Yes | Yes: environment, Authorization, persistence, and readiness rules are recorded. |
| 9 | Architecture Decisions | Yes | Yes | Yes | Yes: ADR-0011 records the debug-plus-feature refinement. |
| 10 | Quality Requirements | Yes | Yes | Yes | Yes: QS-023 through QS-026 state measurable exact assertions. |
| 11 | Risks and Technical Debt | Yes | Yes | Yes | Yes: release leakage, stale binary, and broad mock risks are recorded. |
| 12 | Glossary | Yes | Yes | Yes | Yes: transport, binary, endpoint, and competing-twin terms are recorded. |

- [x] All 12 rows were independently assessed before opening the change marker.
- [x] No independent `Yes` row is omitted or marked `No` in `arc42.md`.
- [x] The existing ADR-0011 is the MADR entry for the refined transport
      decision; its bad consequence is reflected in chapter 11.
- [x] No chapter claimed as updated remains a scaffold or placeholder.
- [x] Updated diagrams remain Mermaid; no ASCII-art diagram was added.

## Changes Made During Hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Removed stale-search delivery from this scope, added exact transport/fallback/readiness behavior, and deferred stale search to `model-discovery-and-setup-correctness`. |
| `specs/transport/transport.feature` | Replaced unreachable fixtures with local twin interactions, added default model, exact transport assertions, release matrix interaction, missing/whitespace outline, and non-E2E readiness contract. |
| `design.md` | Added the debug-plus-feature guard, four isolated build rows and absolute path contract, concrete `TransportState`, exact mock/assertion contract, strict commands, inventory matrix, and stale-search deferral. |
| `arc42.md` | Added the independent 12-row assessment, architecture consequences, durable update record, and `STATUS: DONE`. |
| `docs/arc42/*` | Updated all affected durable chapters and the index; chapter 03 was assessed as unchanged. |
| `docs/adr/0011-interactive-provider-onboarding.md` | Refined the existing E2E transport decision with release-profile behavior, exact assertions, and binary isolation. |

## Verification

- `givn lint --change isolate-test-transport`: exit 2 with four expected WIP
  findings; no parse, inventory, or matrix error.
- No `tasks.md` was created.
- No production or test source file was edited.

## Sign-off

- [x] All required review branches walked.
- [x] All findings resolved or explicitly deferred to a named later change.
- [x] `design.md` reflects the release-profile decision.
- [x] Delta scenarios and the interaction matrix are consistent.
- [x] `givn lint` exits 0 or 2; this change exits 2 only because scenarios remain `@wip`.
- [x] Arc42 impact assessment is complete and `arc42.md` is `STATUS: DONE`.
- [x] `tasks.md` was not created.

DESIGN-REVIEW: PASS
