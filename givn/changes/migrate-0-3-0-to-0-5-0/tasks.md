# Migration Tasks: Givn 0.3.0 to 0.5.0 (aggregate)

This change has no product scenarios (`.givn-skip: specs`). The proposal
forbids a synthetic feature, so the tasks below record evidence-based
migration concerns rather than RED/GREEN changes to watn. Ordered per
`migration.yaml`: phase 1 (0.3.0 → 0.4.0, verify-only) then phase 2
(0.4.0 → 0.5.0 reconciliation).

## Phase 1: managed 0.4.0 boundary (verify-only)

- [x] **Evidence:** `givn/config.yaml` carries `givn_config_version: "0.5.0"`.
  Managed upgrade Commit A `8cd0d581` stages exactly 30 managed paths
  (generated `.opencode/commands/*` and `.opencode/skills/*`,
  `givn/commands.yaml` scaffold, `givn/config.yaml` version bump) and nothing
  user-owned; aggregate plan Commit B `4384bb0` records the migration plan.
  0.4.0 introduces no domain, configuration, specification, or architecture
  migration for consumers — delivery-pipeline content (`.github/workflows/`,
  `cliff.toml`, `deny.toml`) was correctly NOT copied into this project.
- [x] **Verification:** `git show --stat 8cd0d581` matches the allowlist;
  `git status --porcelain --untracked-files=all` clean before migration-change
  commits; `givn --version` = 0.5.0. No project-owned surface required a
  0.4.0 edit.
- [x] **COMMIT:** Managed preparation is recorded by `8cd0d581` and `4384bb0`.
  No consumer migration commit was required for phase 1.

## Phase 2: generated guidance and overrides

- [x] **Evidence:** `givn/config.yaml` overrides preserved (all eight addons
  enabled). Skills targets set to `[opencode, agents, claude]` and re-synced;
  `.claude/skills/` and `.agents/skills/` verified byte-identical to
  `.opencode/skills/`; no retired semantic section
  (`## Semantic review classifications` / `## Semantic remediation
  verification` / `## Semantic Review and Remediation`) exists in any
  generated target. `givn/commands.yaml` product-start scaffold filled
  (`cargo run --release -- --help`, `product_type: cli`).
- [x] **Verification:** resolved review guidance contains the deterministic
  disposition contract (`## Overlap dispositions`, `## Split-or-keep`,
  `duplicate` / `variant` / `boundary` — 3 section hits) and zero
  `## Semantic` classification sections; resolved specs instruction carries
  the `group.md` interaction-inventory policy (17 hits).
- [x] **COMMIT:** `9a6e95c` (skills targets + resync), `e656609` (run
  declaration). No hand-patched override was needed beyond the target list.

## Phase 2: project-specific decisions

- [x] **Evidence:** Semantic features: feature-free default accepted
  (user-confirmed; retrieval was never functional in this project — recorded
  unavailable in the 0.2.0→0.3.0 migration evidence). Corpus layout: already
  grouped — 5 groups (`corpus-infra`, `interactive-setup`, `model-setup`,
  `provider`, `shell`), each with a real `group.md` (Actor, Goal, Main flow,
  Interactions, Includes, Extends); no flat directories remain, no
  `givn spec regroup` proposal needed. No capability describes semantic-engine
  behaviour, so no `@givn.retired` delta is required. No active change other
  than this maintenance change exists, so no stale disposition table or
  missing Capability Routing table needs conversion; `givn spec route`
  output for this change is advisory only and was not acted on. No live
  script or doc (run-tests.sh, measure-coverage.sh, merge-coverages.sh,
  README.md, AGENTS.md) references removed commands or the old
  duplicate-audit output format.
- [x] **Verification:** `givn spec tree` lists all 5 groups with Actor/Goal;
  `givn spec find provider` returns expected hits (151); `givn spec
  duplicates` runs model-free printing `scanned permanent corpus: 115
  exact/structural match(es)`; `givn status` confirms `givn/config.yaml`
  loads and the effective manifest still contains `review`.
- [x] **COMMIT:** Decisions and evidence recorded in this change's artifacts
  (`arc42.md`, `design-review.md`, `tasks.md`) — commit recorded in the
  archive task below.

## Phase 2: arc42 and ADR reconciliation

- [x] **Evidence:** All twelve arc42 chapter-impact rows walked against the
  migration proposal/design — all "No"; independent re-derivation during
  design-review matched with zero diff. All 12 `docs/arc42/` chapter files
  exist with real content; zero ASCII-art violations. Semantic-features
  candidate fails the ADR qualification gate (no watn boundary, contract, or
  dependency direction affected) → routed to change evidence; active and
  archived ADR registers searched (all records are watn product decisions).
- [x] **Verification:** `arc42.md` contains the full 12-row table and
  `STATUS: DONE`; design-review cross-checked it independently.
- [x] **COMMIT:** `9f5fe2b` (arc42.md + design-review PASS + design.md id
  correction). No durable chapter or ADR file modified.

## Full verification and archive

- [x] **Evidence:** `./run-tests.sh` passed: 20 features, 155 scenarios, 913
  steps. `./run-tests.sh --e2e` passed: 24 features, 77 scenarios, 568 steps
  (strict subset: 77 < 155). `givn lint` clean (26 files; warnings
  non-blocking). Coverage measured via `./measure-coverage.sh` +
  `./merge-coverages.sh`: merged `coverage/cobertura-coverage.xml`
  `line-rate="0.9149426846639083"` = 91.49% line coverage.
- [x] **Verification:** `review.md` written with real evidence;
  `givn check review --change migrate-0-3-0-to-0-5-0` passed all gates
  (verify, verify-e2e, integrity, run declaration, overlap dispositions;
  net delta 0/0/0, zero warnings). `givn status` shows all artifacts
  complete.
- [x] **COMMIT:** Evidence commits `cf05e3a` (tasks + review PASS) and
  `d4344d3` (clean disposition tables). Archive follows immediately.
