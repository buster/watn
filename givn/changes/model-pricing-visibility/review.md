# Review: model-pricing-visibility

## Fabrication audit

- **@e2e tag integrity**: both modified e2e scenarios in
  `givn/changes/model-pricing-visibility/specs/configure-model/streamlined-setup.feature`
  carry `@givn.modified @e2e`; no `@e2e` tag was removed or downgraded.
- **Empty/no-op step bodies**: 0 found. Scanned `tests/steps/*.rs` (31
  files) and `src/**/*.rs` for `{}`/whitespace-only bodies and
  `unimplemented!`/`todo!`; no matches.
- **Task commits touch implementation**: 5366fff (models parse/display),
  4557110 (piped capture), 3aadc73 (formatter scenario), 188ea65 (table
  header + priced given + config steps), 10667bf (focused apply capture),
  c613209 (coordinated apply capture), 835accf (partial-price coverage).
  3aadc73 and 835accf are test-surface commits whose production behavior
  was delivered by 5366fff; recorded as cross-references, not fabricated
  tasks. No spec-only commit exists.
- **Promised components exist**: `parse_pricing`, `parse_pricing_value`,
  `per_million_tokens` (`src/models/list.rs`); `capture_catalog_price`
  (`src/setup.rs`); call sites in `apply_result`, `apply_models_result`,
  `run_models_result`; `Pricing ($/1M)` header and per-1M cell in
  `draw_model`; priced-catalog given, priced ephemeral-transport given,
  table steps, and config assertion steps in `tests/steps/`.
- **Strict-mode proof**: present in `tasks.md` setup task — targeted run of
  `Non-terminal model assignment records catalog prices` failed with
  `Step doesn't match any function`, `exit status: 1`.
- **E2E Then assertions**: the modified e2e scenarios assert CLI exit
  status and the config file written by the real PTY-driven binary; they do
  not assert only against repository state.
- **Browser-UI checks**: N/A — CLI capability.
- **E2E scope isolation**: `verify.command` is `./run-tests.sh`
  (`@wip`-and-`@e2e`-excluded), `verify.e2e_command` is
  `./run-tests.sh --e2e` (`@e2e`-only). Counts: regular `22 features, 222
  scenarios`, e2e `25 features, 90 scenarios`; 90 < 222, so the filter is
  real. No parallel `@e2e` implementation exists; the e2e step files are
  the ones `design.md` named.
- **Design conformance**: step files and commands match `design.md`
  (ask_steps for models/formatter/config assertions,
  model_picker_layout_steps for the table, streamlined_setup_steps for the
  priced given, provider_setup_steps for the priced transport,
  streamlined_setup_e2e_steps for choose-role steps). No deviation.
- **Interaction coverage**: 25 inventory entries in
  `givn/specs/configure-model/usecase.md`, 25 rows in the design matrix,
  all 25 e2e titles present in the permanent corpus. The change modifies
  two existing e2e scenarios in place and adds none, so the
  one-E2E-per-action rule holds.

| Capability · action | @e2e scenario exists | Driving mechanism in step file | Match |
|---|---|---|---|
| reasoning-policy · persist minimal reasoning | yes | `run_binary_with_state` + `httpmock` body assertion | yes |
| streamlined-setup · coordinate all setup choices | yes (modified) | PTY `start_pty_session` + `pty_write` | yes |
| streamlined-setup · configure provider from environment | yes | PTY + env credential | yes |
| streamlined-setup · configure all model roles | yes (modified) | PTY + typed filter/Enter | yes |
| streamlined-setup · configure shell integration | yes | PTY + shell choices | yes |
| streamlined-setup · reject incomplete request | yes | PTY + request-count mock | yes |
| reasoning · send thinking reasoning | yes | subprocess + body assertion | yes |
| reasoning · print verbose thinking reasoning | yes | subprocess + stderr assertion | yes |
| reasoning · print small-tier reasoning | yes | subprocess + stderr assertion | yes |
| reasoning · suppress small-tier reasoning | yes | subprocess + stderr assertion | yes |
| reasoning · preserve default-tier behavior | yes | subprocess + body assertion | yes |
| reasoning · inspect verbose help | yes | subprocess + stdout assertion | yes |
| reasoning · combine verbose and execute | yes | subprocess + stdout/stderr | yes |
| ratatui-model-picker · configure three levels | yes | PTY + typed filter/reasoning | yes |
| ratatui-model-picker · browse model list | yes | PTY arrow/page keys | yes |
| ratatui-model-picker · filter model suggestions | yes | PTY typed filter | yes |
| ratatui-model-picker · revise previous level | yes | PTY back navigation | yes |
| ratatui-model-picker · apply per-level reasoning | yes | subprocess + body assertion | yes |
| credential-sources · discover with environment credential | yes | PTY + env resolution | yes |
| credential-sources · prefer saved credential | yes | PTY + auth-header mock | yes |
| models · discover and assign tiers | yes | piped subprocess + catalog mock | yes |
| models · browse without LiteLLM | yes | subprocess + guidance assertion | yes |
| catalog-source · use configured LiteLLM catalog | yes | subprocess + mock hits | yes |
| catalog-source · preserve chat provider | yes | subprocess + saved config | yes |
| model-autosuggest · find a model outside first page | yes | PTY search + mock | yes |

**FABRICATION AUDIT: CLEAN**

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Catalog price normalized to $/1M, invalid component yields no price | 05 `ModelExplorer`, 08 cost tracking, 12 glossary | rows 5, 8, 12 = Yes | Data Model, Catalog parsing | Scenario 1 GREEN, partial follow-up | `src/models/list.rs::parse_pricing`, `parse_pricing_value`, `per_million_tokens` | yes |
| Capture on every model-choice write path, insert-only by id | 06 catalog/coordinated flows | rows 6 = Yes | Capture section | Scenarios 2, 5, 6 GREEN | `src/setup.rs::capture_catalog_price` + 3 call sites | yes |
| Display in per-million units in list and table | 05 `ConfigWriter`/`ModelExplorer`, README | rows 5, 8 = Yes | Display section | Scenarios 1, 3, 4 GREEN | `format_model_entry`, `draw_model` header/cell | yes |
| Non-OpenRouter pricing-shape risk | 11 R-079 | row 11 = Yes | Justification 2–4 | Scenario 1/partial GREEN | strict both-components and negative guard | yes |
| No new ADR; choices routed to design/spec | 09 register unchanged | ADR qualification = none | Justifications | n/a | no new `docs/adr` file | yes |
| Glossary terms `Catalog price`, `Price capture`, `Pricing` | 12 glossary | row 12 = Yes | terminology used consistently | all scenarios | proposal/spec/design/glossary wording | yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

`Catalog price` (provider per-token quote, invalid component means none),
`Price capture` (recording chosen catalog prices as Pricing), and `Pricing`
($/1M persisted configuration) are recorded in
`docs/arc42/12-glossary.md` and used consistently in the proposal, delta
specs, design, and code. No unrecorded or conflicting term found.

## Coverage classification

No archive verification receipt exists yet for this change; classification
is from the completed runs and static inspection. The archive receipt will
carry the measured merged result.

| Region | Disposition |
|---|---|
| Per-token → $/1M conversion, six-decimal rounding | Covered (scenarios 1, 3, 4, 5, 6) |
| Negative-component guard | Covered (scenario 2, sentinel chosen for normal tier) |
| Absent `pricing` object | Covered (scenario 2, `model-plain`) |
| Partial `pricing` object | Bucket 2 — resolved by adding `A partial catalog price is not recorded` (835accf) |
| Capture insert/replace, unchosen preservation | Covered (scenarios 2, 5, 6) |
| TUI header and cell | Covered (scenario 4) |
| Old duplicated parse loop | Deleting during scenario 1 removed dead duplication |
| Hard-to-test regions | None |

## E2E coverage

Both `@e2e` scenarios drive the real CLI in a PTY; primary assertions are
the CLI exit status and the config file written by the binary. No scenario
was downgraded to repository-only or HTTP-in-place-of-interface. No
inventory action lost or gained an `@e2e` scenario.

## Use-case and Persona conformance

Use case `configure-model`, Actor `Watn user`, confirmed Personas: none.
Guarantees and rules are preserved; no Persona was invented or promoted;
capability ownership matches the proposal routing.

## Overlap dispositions

No shape-match findings were reported by `givn lint` for this change.

## Split-or-keep

| Scenario | Decision |
|---|---|
| `Coordinated setup completes provider models reasoning and shell choices` | keep |

Justification: it is the canonical coordinated-setup e2e; the change extends
its existing assertions for captured prices instead of adding a parallel
scenario, preserving one E2E per action.

## README impact decision

README-IMPACT: updated - Setup, Usage

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching implementation
      (cross-referenced for test-surface commits).
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (222 regular, 90 e2e).
- [x] Coverage classified; partial-component gap resolved with a scenario.
- [x] Dead code deleted (duplicated parse loop); missing tests added;
      no hard-to-test exceptions needed.
- [x] Redundant unit tests removed — none existed for the changed behavior.
- [x] No `@wip` tags remain; no implementation detail in the specs.
- [x] Canonical E2E policy applied: one `@e2e` per inventory action.
- [x] Every E2E scenario has a real-interface primary assertion.
- [x] Local run command starts the stack cleanly; all provider twins are
      in-process `httpmock` servers with no external dependency.
- [x] `verify.e2e_command` is not identical to `verify.command`; counts
      prove the e2e scope is strictly smaller.
- [x] Implementation matches `design.md`'s commands, file layout, and
      framework; no undocumented deviation.
- [x] Interaction coverage verified: 25/25 inventory entries map to matrix
      rows and existing e2e scenarios with the promised drivers.
- [x] No finding excused outside the three coverage buckets.

REVIEW: PASS
