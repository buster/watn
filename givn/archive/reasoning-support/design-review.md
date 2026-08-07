# Design Review: reasoning-support

## Grilling log

| # | Branch | Question | Recommended | Outcome |
|---|---|---|---|---|
| Q1 | Tech choices | Should the reasoning_effort wire format be a nested object (`{"reasoning": {"effort": "high"}}`) or a top-level string (`"reasoning_effort": "high"`)? | Top-level string | **agreed** |
| Q2 | Risk — runner path | Should the test runner scan `givn/specs/` only, or also `givn/changes/*/specs/`? | Scan both: `givn/` recursively | **agreed** |
| Q3 | Missing scenarios | Should reasoning be short-circuit-parsed (skip when verbose off) or always parsed? | Always parse, gate print on verbose | **agreed** |
| Q4 | Architecture docs — ch.9 | Should ADR-0007 be added for reasoning support? | Yes, add MADR entry | **agreed** |
| Q5 | Missing scenarios | Should a warning be emitted when model doesn't return reasoning on thinking tier? | No, silent degradation | **agreed** |
| Q6 | Missing scenarios — request body assertion | How to capture the request body for E2E assertion? | Add `last_request_body` to `WatnWorld`, capture via httpmock | **agreed** |
| Q7 | Testability — SSE mock | How to set up mock SSE responses with reasoning for testing? | Add `pending_mock_reasoning` to `WatnWorld`, interleave in SSE body | **agreed** |
| Q8 | Scope | Does tier-2+verbose need a dedicated scenario beyond the existing two verbose scenarios? | No, existing scenarios cover | **agreed** |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Step `stderr should not contain "{text}"` | Does not exist in `tests/steps/ask_steps.rs` — needs adding during GREEN |
| Runner path | `features_runner.rs` at line 45-47 pointed at `givn/specs/` only — delta spec not found |
| ADR-0007 | No ADR file exists for reasoning support — created |
| arc42.md ch.9 | Marked as "No" — changed to "Yes" |
| arc42.md ch.11 | Marked as "No" — changed to "Yes" |

## Architecture documentation (arc42) check

| # | Chapter | Grilling subagent's Yes/No | arc42.md's Yes/No | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Yes | ✓ | Yes |
| 2 | Architecture Constraints | No | No | ✓ | N/A |
| 3 | Context and Scope | No | No | ✓ | N/A |
| 4 | Solution Strategy | No | No | ✓ | N/A |
| 5 | Building Block View | Yes | Yes | ✓ | Yes |
| 6 | Runtime View | Yes | Yes | ✓ | Yes (updated to top-level wire format) |
| 7 | Deployment View | No | No | ✓ | N/A |
| 8 | Cross-cutting Concepts | Yes | Yes | ✓ | Yes (updated to top-level wire format) |
| 9 | Architecture Decisions | Yes | Yes (was No) | ✓ | Yes — ADR-0007 added |
| 10 | Quality Requirements | Yes | Yes | ✓ | Yes |
| 11 | Risks and Technical Debt | Yes | Yes (was No) | ✓ | Yes — R-007 added |
| 12 | Glossary | Yes | Yes | ✓ | Yes |

- [x] All 12 rows independently assessed against proposal.md/design.md before opening arc42.md.
- [x] No row where the subagent said "Yes" and arc42.md said "No"/omitted it (any such row is a blocker, listed in Open questions).
- [x] Every non-trivial architecture/technology decision in design.md has a MADR entry in chapter 09.
- [x] Every MADR "Bad, because..." consequence has a counterpart in chapter 11.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `tests/features_runner.rs` | Changed spec_dir from `givn/specs` to `givn/` so cucumber recursively discovers both `givn/specs/` and `givn/changes/*/specs/` |
| `givn/changes/reasoning-support/design.md` | Updated wire format from `{"reasoning": {"effort": "high"}}` to top-level `"reasoning_effort": "high"`. Added always-parse semantics. Added `WatnWorld.pending_mock_reasoning` and `last_request_body` field documentation. Removed "silently discarded" reference. |
| `docs/adr/0007-reasoning-support.md` | Created new MADR documenting wire format choice, output stream, and parse strategy |
| `docs/arc42/09-architecture-decisions.md` | Added ADR-0007 row to decision table |
| `docs/arc42/11-risks-and-technical-debt.md` | Added R-007 for empty reasoning risk on thinking tier |
| `docs/arc42/06-runtime-view.md` | Updated sequence diagram and prose from nested `reasoning: {effort: "high"}` to top-level `reasoning_effort: "high"` |
| `docs/arc42/08-crosscutting-concepts.md` | Updated wire format example and removed "silently discarded" language |
| `givn/changes/reasoning-support/arc42.md` | Changed ch.9 from "No" to "Yes" with ADR-0007 reference. Changed ch.11 from "No" to "Yes" with R-007 reference. Added updated files to Updates Applied. |

## Sign-off

- [x] All branches walked.
- [x] All open questions resolved.
- [x] design.md reflects decisions reached.
- [x] specs/*.feature updated for any missing scenarios.
- [x] givn lint exits 0 or 2.
- [x] Architecture documentation (arc42) check completed above, or marked N/A because the addon is not enabled.

DESIGN-REVIEW: PASS
