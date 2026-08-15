# Review: watn-consolidation

## Scope

This review covers the six evidence-backed F1-F6 consolidation dispositions,
the isolated review/archive/rollback fixture, the runner and CI filters for
`@givn.removed`, and the durable architecture documentation. The change does
not modify Watn runtime or deployment code.

## Fabrication Audit

### Scenario and tag integrity

Six delta feature files were scanned. They contain six `@givn.removed`
placeholders, two `@givn.added @e2e` CLI scenarios, and one regular rollback
scenario. No `@wip` tags remain. The two E2E tags are present in the tracked
delta and were never removed.

### Step-body scan

The three consolidation step modules were scanned:

- `tests/steps/watn_consolidation_steps.rs`
- `tests/steps/watn_consolidation_e2e_steps.rs`
- `tests/steps/watn_consolidation_support.rs`

Result: `0` empty, pending, or no-op step bodies found. Every step either
performs fixture/subprocess setup or asserts exit status, CLI output, archive
tree contents, duplicate titles, or byte-for-byte rollback preservation.
Strictness is enabled by `.fail_on_skipped()` in
`tests/features_runner.rs`. The recorded proof in `tasks.md` shows a pending
step exiting non-zero with `1 step failed`.

### Implementation and commit audit

All checked tasks have commit evidence and the promised files exist. This is a
specification-ownership change: F1, F3, F4, F5, and F6 intentionally modify
the executable Gherkin delta and consolidation evidence rather than Watn
runtime code. F2 modifies the retained search step support, and the review,
archive, and rollback scenarios modify the Rust fixture implementation.

Relevant commit evidence:

| Work | Evidence |
|---|---|
| F1 | `0269099` |
| F2 | `cc4bdee` |
| F3 | `8fadd07` |
| F4 | `3010e28` |
| F5 | `54d4d97` task record, F5 delta in `3010e28`, retained assertion refactor in `fae8415` |
| F6 | `e593bae` |
| Rollback fixture | `9379c3e` |
| Review fixture | `1034be9` |
| Archive fixture | `c021902` |

The implementation surface promised by `design.md` is present: the six delta
capability directories, the consolidation feature, the regular rollback step
file, the separate CLI E2E step file, the shared fixture module, runner
registration, scripts, CI filters, ADR, durable Arc42 updates, and review
evidence.

### E2E fidelity and scope

Both E2E scenarios drive the real `givn` CLI subprocess with an isolated
temporary project. Their primary assertions are on CLI exit status and
stdout; permanent-tree assertions are secondary archive-result checks. No
browser capability exists, and no HTTP or in-page `fetch` shortcut is used.

The two E2E scenarios are distinct happy-path actions: review receipt
generation and archive publication. The rollback scenario is intentionally
regular because it verifies failure preservation rather than a separate
publication action.

### Runner isolation

The literal commands in `givn/commands.yaml` are:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

`run-tests.sh` maps these to the strict Cucumber tag filters:

```text
not @wip and not @e2e and not @givn.removed
@e2e and not @wip and not @givn.removed
```

The full non-E2E run reported `149` scenarios and the E2E subset reported
`77` scenarios. The subset is strictly smaller. `measure-coverage.sh` applies
the same filters. Named runs reject removed placeholders before invoking
Cucumber.

### Interaction coverage cross-reference

| User Interaction Inventory entry | E2E scenario | Step file | Driving mechanism | Match |
|---|---|---|---|---|
| Run `givn check review --change <fixture-change>` to verify repository-wide dispositions | `Repository-wide review accepts the consolidation dispositions` | `watn_consolidation_e2e_steps.rs` | Real `givn check review` subprocess in a fresh `TempDir`; stdout and exit status asserted | Yes |
| Run `givn archive --change <fixture-change>` to publish consolidated permanent specs | `Archive publishes the consolidated permanent specifications` | `watn_consolidation_e2e_steps.rs` | Real `givn archive` subprocess in a fresh `TempDir`; stdout, exit status, and resulting tree asserted | Yes |

The regular rollback interaction is additionally covered by
`watn_consolidation_steps.rs` and does not appear in the E2E inventory because
it is a failure-preservation check, not a separate happy-path action.

### Local runnability and isolation

The local commands are `./run-tests.sh` and `./run-tests.sh --e2e`; they build
the Watn debug binaries and run the complete applicable Gherkin suite. The
consolidation fixture starts no service and needs no digital twin: it creates a
fresh `TempDir`, initializes a fixture with `givn init --no-addons`, and invokes
the real installed `givn` executable with the fixture as current directory.
`GIVN_BIN` is required to be an absolute existing file when supplied. The
failed-archive scenario confirms that the permanent fixture tree remains
byte-for-byte unchanged.

## Verification Evidence

| Command | Result |
|---|---|
| `givn lint --change watn-consolidation` | Clean; six files checked |
| `givn check design-review --change watn-consolidation` | Passed |
| `GIVN_BIN=/home/buster/.cargo/bin/givn ./run-tests.sh` | `20` features, `149` scenarios, `855` steps passed |
| `GIVN_BIN=/home/buster/.cargo/bin/givn ./run-tests.sh --e2e` | `25` features, `77` scenarios, `567` steps passed |
| `GIVN_BIN=/home/buster/.cargo/bin/givn ./measure-coverage.sh` | Non-E2E and E2E reports freshly generated; both suites passed |
| `GIVN_BIN=/home/buster/.cargo/bin/givn ./merge-coverages.sh` | Fresh merged Cobertura report written |
| `cargo fmt --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `givn check review --change watn-consolidation` | Passed verify, verify-e2e, integrity, and overlap dispositions; net delta `3 added, 0 modified, 6 removed` |

## Coverage Measurement

Coverage is valid. `measure-coverage.sh` instruments the Watn library,
`tests/features_runner`, and the two debug Watn binaries used by subprocess
steps. `LLVM_PROFILE_FILE=coverage/profraw/%p-%m.profraw` gives each process a
collision-safe profile. The two source reports are merged by
`merge-coverages.sh` into `coverage/cobertura-coverage.xml`.

The fresh merged report contains:

- Lines: `13042/14210` covered, `91.7804%`.
- Branches: `0/0`, therefore `n/a` rather than a fabricated percentage.
- A known exercised production path is `src/provider/transport.rs`, reported
  at `20/20` lines; `src/models/picker.rs` is also non-zero at `195/196`
  reported executable lines in the merged report.

Uncovered regions are in pre-existing, unchanged Watn runtime modules. This
change adds no runtime path and removes no runtime test boundary. They are
classified as bucket 3, legitimately hard to test for this change: adding
duplicate runtime tests solely to raise the aggregate percentage would not
exercise a new consolidation behavior and would violate the repository's
Gherkin-as-source-of-truth boundary. No new consolidation implementation line
is uncovered; all consolidation Rust steps are exercised by the regular or
E2E runner.

## Arc42 Implementation Conformance

| Arc42 chapter/fact | Durable source | `arc42.md` claim | Design/tasks mapping | Implementation evidence | Match |
|---|---|---|---|---|---|
| 1. Canonical specification ownership is a quality goal | `docs/arc42/01-introduction-and-goals.md` | Affected | Scope and F1-F6 matrix; final gate | Repository-wide delta and archive verification | Yes |
| 2. Permanent scenario titles are repository-wide unique | `docs/arc42/02-architecture-constraints.md` | Affected | Data invariants and F1-F6 dispositions | Duplicate-title review and archive fixture | Yes |
| 3. Maintainer/givn workflow boundary | `docs/arc42/03-context-and-scope.md` | Affected | Review/archive flow and CLI fixture | Real `givn` subprocess in `TempDir` | Yes |
| 4. Deterministic ownership findings precede archive | `docs/arc42/04-solution-strategy.md` | Affected | Consolidation procedure and commands | `givn check review` passes overlap dispositions | Yes |
| 5. Specification ownership and evidence workflow blocks | `docs/arc42/05-building-block-view.md` | Affected | Component table and step modules | Registered regular, E2E, and support modules | Yes |
| 6. Review, archive, rollback, and post-archive flow | `docs/arc42/06-runtime-view.md` | Affected | Sequence and rollback invariants | Review/archive/rollback scenarios pass | Yes |
| 7. No deployment topology change | `docs/arc42/07-deployment-view.md` | Not affected | Explicitly marked No | No runtime, release, or deployment file changed | Yes |
| 8. Ownership, atomic archive, and no-provider fixture rules | `docs/arc42/08-crosscutting-concepts.md` | Affected | Strict filters, TempDir, and archive semantics | Scripts and fixture enforce these boundaries | Yes |
| 9. ADR-0025 repository-wide ownership decision | `docs/arc42/09-architecture-decisions.md`, `docs/adr/0025-repository-wide-specification-ownership.md` | Affected | ADR and design-review conformance | Full MADR, durable summary, and design decision agree | Yes |
| 10. QS-063 through QS-065 | `docs/arc42/10-quality-requirements.md` | Affected | Verification and interaction matrix | Net delta, duplicate-free archive, and retained contracts verified | Yes |
| 11. R-062 through R-064 and TD-010 | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Risks and mitigations in design | Retained-contract table, rollback, and binding scan | Yes |
| 12. Scenario ownership vocabulary | `docs/arc42/12-glossary.md` | Affected | Proposal/design terminology | Canonical, retained, removed, and disposition terms used consistently | Yes |

ARC42 CONFORMANCE: CLEAN

## Sign-Off

- [x] Fabrication audit is clean.
- [x] Every checked task has commit evidence and an implementation artifact;
  this change intentionally has no Watn runtime source modification.
- [x] Every promised component exists.
- [x] Strict-mode proof is present and non-zero.
- [x] `verify.command` and `verify.e2e_command` both exit 0.
- [x] Coverage is freshly measured across the runner and Watn subprocesses.
- [x] Coverage gaps are classified under the three allowed buckets.
- [x] No dead consolidation code remains and no redundant unit test was added.
- [x] No `@wip` tags remain and the specs contain no implementation-layer detail.
- [x] Each distinct happy-path CLI action has exactly one E2E scenario with
  primary assertions on the real CLI interface.
- [x] The local run commands require no external service for this capability.
- [x] The exact E2E command and all three consolidation step modules were read;
  no parallel implementation exists.
- [x] E2E scope is a strict subset of the non-E2E command.
- [x] Implementation matches the reviewed design after the documented step-file
  and fixture adjustments; design-review was re-run successfully.
- [x] Interaction inventory, coverage matrix, feature scenarios, and step
  definitions cross-reference completely.
- [x] No gap was excused with an unapproved fourth classification.

REVIEW: PASS
