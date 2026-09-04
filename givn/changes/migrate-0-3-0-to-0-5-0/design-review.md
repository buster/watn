# Design Review: migrate-0-3-0-to-0-5-0

Maintenance change (no product spec — `.givn-skip: specs`). Grilling was
performed by a fresh-context subagent; all branches walked; findings recorded
below. All read-only verification steps from design.md Prompt 5 that can run
before review/archive were executed during grilling and re-confirmed.

## Branch outcomes

- **Scope** — no issue found. design.md covers Phase 1 (verify-only) and
  Phase 2 (reconciliation) exactly as proposed; Completion Definition is
  testable. Commit A `8cd0d581` stages exactly the managed allowlist
  (30 files: generated `.opencode/` skills + commands, `givn/commands.yaml`,
  `givn/config.yaml` version bump), nothing user-owned.
  Hardening applied: corrected `migrate-0.3.0-to-0.5.0` →
  `migrate-0-3-0-to-0-5-0` (dots vs hyphens) at design.md:181 and :283 so the
  recorded archive command is runnable; removed empty untracked residue
  directory `givn/artifacts/arc42-docs/`.
- **Tech choices** — no issue found. Remaining work is commits + gates +
  archive. Corpus already grouped (pre-dates this change), no semantic
  capabilities to retire, no disposition tables to convert, no flat-layout
  regroup. `givn spec route` output for this change is advisory only and was
  not acted on. Generated skill/command targets reconciled: `skills.targets`
  set to `[opencode, agents, claude]` and re-synced; `.claude/skills/` and
  `.agents/skills/` verified byte-identical to `.opencode/skills/`.
- **Missing scenarios** — no product scenarios exist by design. Two
  process findings, both resolved: (1) dirty worktree from Phase-2 edits —
  resolved by separate commits (skills retarget; run scaffold; migration
  evidence) before sign-off; (2) empty `givn/artifacts/arc42-docs/` residue —
  removed. Verified clean: no references to removed givn commands in
  run-tests.sh, measure-coverage.sh, merge-coverages.sh, README.md, AGENTS.md,
  or docs/arc42/; config loads (`givn status` healthy); effective manifest
  still contains `review`.
- **Testability / verification** — no issue found. Executed: `givn lint`
  (26 files, clean; warnings non-blocking), `./run-tests.sh` (155 passed),
  `./run-tests.sh --e2e` (77 passed), `givn spec tree` (5 groups, Actor/Goal
  present), `givn spec find provider` (hits returned), `givn spec duplicates`
  (new output format, model-free), `givn spec route --change` (advisory).
  `givn check review` runs at the review gate once review.md exists.
- **Risk** — most likely failure was commit-granularity drift mixing managed
  and project concerns; mitigated by the separate commits listed above and by
  using the literal change id (now corrected in design.md).
- **ADR qualification** — the semantic-features decision (feature-free
  default, user-confirmed) routes to change evidence, not an ADR: alternatives
  existed and were decided; zero architectural impact on watn (touches no
  boundary, contract, or dependency direction); no durable product
  consequence; canonical lower-level home is this change's evidence plus
  `givn/config.yaml`; existing-ADR check — all records in docs/arc42/adr/ are
  watn product decisions. NOT_QUALIFIED → `CANONICAL_ARTIFACT`. Correct.
- **arc42** — independent 12-row re-derivation returned all "No" and matched
  `arc42.md` exactly (zero diff rows). Chapter file integrity: all 12
  `docs/arc42/` files exist with real content (53–889 lines); zero ASCII-art
  violations (no box-drawing characters, no pseudo-diagram blocks). No
  qualified ADR candidates in this change, so no chapter-09/chapter-11
  additions required.
- **Interaction coverage** — N/A: `.givn-skip: specs` confirmed; zero delta
  `.feature` files in the change directory.

## Hardening applied

1. design.md: fixed change-id references (`migrate-0-3-0-to-0-5-0`).
2. Removed empty untracked `givn/artifacts/` residue.
3. `givn/commands.yaml` `run:` scaffold filled (`cargo run --release -- --help`,
   `product_type: cli`) — archive blocker cleared.
4. `givn/config.yaml` skills targets set to `[opencode, agents, claude]`;
   `givn skills sync` re-run; all three targets verified current.
5. Semantic-features decision recorded: feature-free default (user-confirmed).
6. `givn/changes/migrate-0-3-0-to-0-5-0/arc42.md` written (all 12 rows No,
   STATUS: DONE).
7. Worktree committed one concern per commit (see git history).

## Sign-off

All branches walked. All questions resolved from evidence — none required user
escalation beyond the already-recorded semantic-features decision.

DESIGN-REVIEW: PASS
