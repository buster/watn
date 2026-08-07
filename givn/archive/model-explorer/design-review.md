# Design Review: model-explorer

## Phase 1: Grilling

**Scope**: Matches proposal exactly. No gaps.

**Tech choices**: `dialoguer::Select` for interactive prompts. In test subprocesses with piped stdin, `Select::interact()` reads from the pipe (not /dev/tty), so stdin input works. Verified: dialoguer 0.11 falls back to stdin when not a tty.

**Missing scenarios**: None identified. 6 scenarios cover: interactive success, no-provider, env-var-based provider, API failure, rich metadata, bare IDs.

**Testability**: Every scenario can genuinely fail. RED steps use `unimplemented!()`.

**E2E fidelity**: CLI interface. Driving mechanism is `std::process::Command` with piped stdin. Real interface assertion: CLI stdout/stderr and config file written to disk.

**Interaction Coverage**: All 4 inventory entries mapped to @e2e scenarios.

**Risk**: Most likely failure mode is `dialoguer::Select` failing when stdin is not a tty. Mitigation: test this explicitly with a piped-stdin subprocess call.

**Architecture documentation (arc42)**: Verified 12-row assessment matches. New `src/models/list.rs` module and interactive model selection flow need minimal updates to chapters 5 and 6. Not blockers.

## Phase 2: Hardening

No changes needed to design or specs. Plan is solid.

## Sign-off

DESIGN-REVIEW: PASS
