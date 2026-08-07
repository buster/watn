## Grilling log
| # | Branch | Question | Recommended | Outcome |
|---|---|---|---|---|
| 1 | tech-choices | Blocking reqwest with no per-chunk rendering mechanism contradicts "shows tokens as they generate" | Spawn blocking fetch thread, pipe chunks through mpsc channel for progressive rendering | AGREED |
| 2 | e2e-fidelity | -x scenarios have no Given step configuring mock response; can't fail in RED | Add explicit Given step "the mock returns command ..." before each -x scenario | AGREED |
| 3 | missing-scenarios | No --model flag standalone usage scenario | Add @wip scenario: `watn --model some-model "question"` uses that model | AGREED |
| 4 | missing-scenarios | No --version flag scenario | Add @wip scenario for `watn --version` output | AGREED |
| 5 | missing-scenarios | No fallback model resolution path scenario | Add @wip scenario: provider.default_model used when no tiers configured | AGREED |
| 6 | arch-docs | arc42 ch.04 says "async streaming SSE", ch.05 says "RUSK_*" | Fix ch.04 to "blocking reqwest", ch.05 "RUSK_*" -> "WATN_*" | AGREED |
| 7 | risk | Most likely failure: blocking-reqwest-vs-progressive-output contradiction | Mitigated via Q1 decision (threaded mpsc) | AGREED |

## Resolved by codebase exploration
| Branch | Finding |
|---|---|
| e2e-fidelity | Interaction Coverage Matrix: exactly 1 @e2e scenario per distinct happy-path action |
| risk | Greenfield project — no legacy patterns |
| arch-docs | All 6 MADR files exist under docs/adr/ with proper status/options/decision/consequences |
| arch-docs | ch.11 includes R-006; all decisions have ADRs and risk entries |
| arch-docs | No chapter claimed as "updated" is still scaffold placeholder |
| missing-scenarios | Ctrl+C, unknown provider, missing API key, malformed config all covered |
| e2e-fidelity | --model flag tested in config.feature but not as standalone scenario (added per Q3) |

## Open questions
None.

## Architecture documentation (arc42) check
| # | Chapter | Subagent's own Yes/No | arc42.md's Yes/No | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | Introduction and Goals | YES | YES | YES | YES |
| 2 | Architecture Constraints | YES | YES | YES | YES |
| 3 | Context and Scope | YES | YES | YES | YES |
| 4 | Solution Strategy | YES | YES | YES | PARTIAL — ch.04 line 17 said "async streaming", fixed per Q6 |
| 5 | Building Block View | YES | YES | YES | PARTIAL — ch.05 line 52 said "RUSK_*", fixed per Q6 |
| 6 | Runtime View | YES | YES | YES | YES |
| 7 | Deployment View | YES | YES | YES | YES |
| 8 | Cross-cutting Concepts | YES | YES | YES | YES |
| 9 | Architecture Decisions | YES | YES | YES | YES — references standalone MADR files |
| 10 | Quality Requirements | YES | YES | YES | YES |
| 11 | Risks and Technical Debt | YES | YES | YES | YES |
| 12 | Glossary | YES | YES | YES | YES |

- [x] All 12 rows independently assessed against proposal.md/design.md before opening arc42.md.
- [x] No row where the subagent said "Yes" and arc42.md said "No"/omitted it.
- [x] Every non-trivial architecture/technology decision in design.md has a MADR entry in chapter 09.
- [x] Every MADR "Bad, because..." consequence has a counterpart in chapter 11.
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening
| Artifact | Change summary |
|---|---|
| `design.md` | Added threaded mpsc approach for progressive streaming rendering; documented mock-return convention |
| `specs/ask.feature` | Added Given mock-return steps to -x scenarios; added 3 new @wip scenarios (--model, --version, fallback model) |
| `docs/arc42/04-solution-strategy.md` | Fixed "async streaming SSE" -> "blocking reqwest with mpsc channel" |
| `docs/arc42/05-building-block-view.md` | Fixed "RUSK_*" -> "WATN_*" |

## Sign-off
- [x] All branches walked.
- [x] All open questions resolved.
- [x] design.md reflects decisions reached.
- [x] specs/*.feature updated for any missing scenarios.
- [x] givn lint exits 0 or 2.
- [x] Architecture documentation (arc42) check completed above.

DESIGN-REVIEW: PASS
