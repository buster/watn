# Review: setup-shell-and-style

## Fabrication audit

- **@e2e tag integrity**: the retired shortcut-focus scenario is marked
  `@givn.removed @e2e` (tag retained on the removal record), and its
  replacement, `The green border follows the shell lists`, is
  `@givn.added @e2e`. No `@e2e` tag was stripped to bypass the e2e gate.
- **Empty/no-op step bodies**: 0 found. Scanned `tests/steps/*.rs` and
  `src/**/*.rs` for empty bodies and `unimplemented!`/`todo!`; no matches.
- **Checked tasks carry commits**: b6ba08e (core shell behavior + most step
  glue), 20d1e59 (bookkeeping), ed39095 (visual language + amber + plain
  fallback), df520cb (shell e2e), 8bb02b9 (shell-lists focus e2e, removals,
  dead-step cleanup). Scenarios 2–5 share the core production commit because
  the list-first behavior, the `enabled` rule, and the preselection sources
  were introduced together in b6ba08e; their task evidence records
  mutation-proven RED and targeted GREEN runs rather than separate code
  commits.
- **Promised components exist**: `SetupInk`, list-first shell key handling,
  PATH + managed-block preselection, the `enabled` rule, `●`/`○` markers,
  `watn · setup` frame, `◆`/`▶`/`⚠` markers, dim default-foreground borders,
  bold-key/dim-label hints, `@givn.removed` filtering, harness HOME/XDG/
  `NO_COLOR` handling.
- **Strict-mode proof**: present in `tasks.md`; targeted run with an
  undefined step exited non-zero.
- **E2E Then assertions**: both e2e scenarios drive the real CLI in a PTY and
  assert visible shell-list focus borders and resulting startup-file blocks;
  no repository-only or downgraded scenario exists.
- **E2E scope isolation**: `verify.command` `./run-tests.sh` (224 scenarios)
  and `verify.e2e_command` `./run-tests.sh --e2e` (89 scenarios); 89 < 224, so
  the `@e2e` filter is real. The e2e step files are those `design.md` named.
- **Design conformance**: step files match `design.md`
  (`streamlined_setup_steps.rs`, `streamlined_setup_e2e_steps.rs`,
  `setup_wizard_steps.rs`, `ask_steps.rs`,
  `highlight_active_setup_input_steps.rs`); `ShellEnvironment::detected_shells`
  was removed as designed.
- **Interaction coverage**: 25 configure-model + 19 configure-interactive
  inventory entries map to matrix rows and `@e2e` scenarios. The replacement
  `The green border follows the shell lists` exists in the delta and merges
  into the permanent corpus at archive; the permanent title is retired via
  `@givn.removed`. Drivers are real PTY/subprocess invocations.

| Capability · action | @e2e scenario | Driving mechanism | Match |
|---|---|---|---|
| configure-model: 25 entries (matrix in `design.md`) | all present in permanent corpus except the shell/e2e titles modified in this change | PTY or piped subprocess + `httpmock` | yes |
| configure-interactive: 17 existing entries | all present | PTY or piped subprocess | yes |
| highlight-active-setup-input · move focus to shell pages | `The green border follows the shell lists` (delta `@e2e`) | PTY, SGR border parse | yes |
| unified-setup-wizard · style aspects | regular scenarios under the existing navigate action | PTY, raw snapshot | yes |

**FABRICATION AUDIT: CLEAN**

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Setup Wizard owns the shared visual language and shell preselection | 05 Setup Wizard row | row 5 = Yes | Architecture Impact | Scenarios 1, 6 | `src/setup.rs` `SetupInk`, preselection | yes |
| List-first shell flow with no-write empty path | 06 optional-shell-shortcut scenario | row 6 = Yes | Shell Interaction Contract | Scenarios 1–5 | `handle_shell_install_key`, `enabled` rule | yes |
| Setup visual language and color fallback | 08 setup visual language section | row 8 = Yes | Visual Language | Scenarios 6–8 | `SetupInk` styles, `terminal_supports_color` | yes |
| ADR-0018 amended (opt-in/default-decline/`$SHELL` superseded) | 09 ADR register, ADR-0018 amendment | row 9 = Yes | ADR Qualification | Scenarios 1, 5 | `docs/adr/0018-...md` amendment | yes |
| R-048 and ADR-0018 consequence coverage updated | 11 R-048 + coverage section | row 11 = Yes | Risk section | Scenarios 1, 4 | `docs/arc42/11` edits | yes |
| `Shell preselection`, `Setup visual language` | 12 glossary | row 12 = Yes | terminology | scenarios | glossary entries | yes |
| arc42 README summary | README index | noted outside the 12 rows | — | — | README summary updated | yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

`Shell preselection` (PATH binaries plus managed blocks; shown state is the
desired state applied on Enter) and `Setup visual language` (shared symbols,
emphasis, color roles, and key-hint markup; disabled as a whole without color)
are recorded in `docs/arc42/12-glossary.md` and used consistently in the
proposal, delta specs, design, and code. No unrecorded or conflicting term.

## Coverage classification

No archive verification receipt exists yet; classification is from the
completed runs and static inspection.

| Region | Disposition |
|---|---|
| `SetupInk` palette methods and default fallback | Covered (scenarios 6, 7, 8) |
| List-first key handling, space toggle, advance, back | Covered (scenarios 1–5, 9, 10) |
| PATH + managed-block preselection | Covered (scenarios 1, 5) |
| `enabled = preselected_any || any_selected` | Covered (scenarios 2, 4; mutation-proven) |
| `●`/`○` markers and selected/dim spans | Covered (scenarios 1, 5) |
| Frame/tabs/header/footer markup | Covered (scenario 6) |
| Amber warnings | Covered (scenario 7) |
| Plain-terminal fallback | Covered (scenario 8) |
| `ShellEnvironment::detected_shells` and retired step bindings | Deleted (dead code) |
| Hard-to-test regions | None identified |

## E2E coverage

Two `@e2e` scenarios are modified in place and one is removed and replaced by
an equivalent real-interface scenario; the one-E2E-per-action rule holds.
Primary assertions are visible terminal focus borders and startup-file blocks
produced through the real binary. No scenario was downgraded.

## Use-case and Persona conformance

Use cases `configure-model` and `configure-interactive`, Actor `Watn user`,
confirmed Personas: none. Guarantees and rules are preserved; capability
ownership matches the proposal routing; no Persona was invented or promoted.

## Overlap dispositions

| Finding pair | Decision | Rationale |
|---|---|---|
| `Shell setup independently configures completion and Ctrl-W integrations` ↔ `Detected shells are preselected on the shell pages` | variant | The first proves independent install outcomes; the second proves PATH preselection markers and accept-all behavior on the same action. |
| `Declining shell setup performs no target inspection or write` (removed) ↔ `Declining shell setup writes no shell target` (added) | boundary | Supersession: the contract is corrected from "no inspection" (preselection always reads managed blocks) to "no write or creation", and the new scenario drives the real wizard. |
| `The green border follows optional shortcut focus` (removed) ↔ `The green border follows the shell lists` (added) | boundary | Supersession: the question-to-list transition no longer exists; the replacement asserts focus on both shell lists. |
| `A managed shell without a binary stays selected` ↔ `Shell setup independently configures completion and Ctrl-W integrations` | variant | Distinct invariant: managed-block preselection when `PATH` detection is empty. |
| Permanent and delta scenarios sharing a title | duplicate | The `@givn.modified` merge mechanism runs both during the change; archive replaces the permanent body. |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## README impact decision

README-IMPACT: updated - Setup, Shell shortcut

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit or a recorded shared commit.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (224 regular, 89 e2e).
- [x] Coverage classified; dead code deleted; no hard-to-test exceptions.
- [x] No `@wip` tags remain; no implementation detail in the specs.
- [x] Canonical E2E policy applied: one `@e2e` per inventory action.
- [x] Every E2E scenario has a real-interface primary assertion.
- [x] Local run command starts the stack cleanly; all provider twins are
      in-process `httpmock` servers; shell targets are temp files.
- [x] `verify.e2e_command` is a strict subset of `verify.command`; counts
      prove it (89 < 224).
- [x] Implementation matches `design.md`'s commands, file layout, and
      framework; no undocumented deviation.
- [x] Interaction coverage verified: 44 inventory entries map to matrix rows
      and `@e2e` scenarios with the promised drivers.
- [x] No finding was excused outside the three coverage buckets.

REVIEW: PASS
