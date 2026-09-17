# arc42 Documentation Update: migrate-0-7-0-to-0-8-0

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Maintenance inventory change: no product goal, stakeholder, or quality goal changes, and this change edits no chapter content. | |
| 2 | 02 architecture-constraints | No | No technical, organisational, or regulatory constraint is added or changed; the follow-ups reformat records only. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface changes. | |
| 4 | 04 solution-strategy | No | No strategy change; retained ADRs keep the recorded strategy and the non-qualifying rationale is already listed in chapter 4 and the capability specs. | |
| 5 | 05 building-block-view | No | No building block, module, or component is added, removed, or reassigned. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence changes. | |
| 7 | 07 deployment-view | No | No deployment topology, artifact, or runtime-requirement change. | |
| 8 | 08 crosscutting-concepts | No | No error handling, security, configuration, or other cross-cutting concept changes. | |
| 9 | 09 architecture-decisions | No | This change creates no ADR and edits no chapter content; the re-qualification verdicts are recorded here and the register, index, storage, and schema repairs run as the named follow-ups. | |
| 10 | 10 quality-requirements | No | No quality scenario is added or changed. | |
| 11 | 11 risks-and-technical-debt | No | No new risk or debt is introduced; the stale R-008 entry identified by the inventory is cleared by the archive follow-up, not by this change. | |
| 12 | 12 glossary | No | The glossary already owns the promoted terms; the term-record schema migration is a follow-up and does not change glossary content. | |

## Notes

This is a maintenance change: its durable output is the migration inventory in
`design.md` and the re-qualification verdicts below. The repairs run as the
named follow-up changes through the normal gates, so no chapter is edited here.

ADR action: CANONICAL_ARTIFACT
ADR target: `givn/specs/fragments/ask.feature` (ADR-0005), `givn/specs/configure-model/ratatui-model-picker.feature` (ADR-0010), `givn/specs/configure-provider/provider-setup-widget-layout.feature` (ADR-0012), `givn/specs/configure-model/reasoning-policy.feature` (ADR-0022)

Verdict convention for this re-qualification: a retained record is `QUALIFIED`
with all five gates `PASS` and is routed `AMEND_ADR` to itself because the
storage and schema follow-ups amend the record to the 0.8.0 MADR contract; the
decision itself is retained unchanged. Superseded records are historical and
are not re-qualified; their archival is tracked in the `design.md` inventory.
ADR-0026 carries a pre-0.8.0 `## Qualification` block; the schema follow-up
replaces it with the 0.8.0 verdict.

### Structured verdicts (accepted and proposed records)

| ADR | qualification | alternatives | architectural_impact | durable_consequence | lower_level_artifact | existing_adr_check | must_be_shared | routing | target / destination | key evidence |
|---|---|---|---|---|---|---|---|---|---|---|
| ADR-0001 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0001 | options: OpenAI-compatible protocol vs provider-specific adapters vs generic templates; protocol and dependency boundary |
| ADR-0002 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0002 | options: streaming vs buffered vs configurable; SSE-only response contract across provider, output, and review |
| ADR-0003 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0003 | options: layered TOML vs single file vs JSON; precedence and XDG contract |
| ADR-0004 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0004 | options: tier flags vs explicit model vs auto-select; public CLI and config-mapping contract |
| ADR-0009 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0009 | options: local-only vs server-only vs hybrid; provider `?search=` contract and responsiveness ceiling |
| ADR-0015 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0015 | options: synchronous callback vs worker channel vs buffered aggregate; provider-to-CLI boundary and mandatory `[DONE]` |
| ADR-0016 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0016 | options: version literal vs package metadata; release-evidence and deployment boundary |
| ADR-0017 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0017 | options: separate scripts vs library enum vs closed selector; CLI contract and side-effect boundary |
| ADR-0018 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0018 | options: clipboard vs portable script vs direct writes vs marker-owned atomic writes; user-owned startup-file boundary |
| ADR-0019 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0019 | options: worker plus bounded grace vs read-timeout polling vs kill vs async; cancellation contract and exit 130 |
| ADR-0020 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0020 | options: save per page vs snapshot until final confirmation; coordinated-write boundary |
| ADR-0021 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0021 | options: retain LiteLLM precedence vs provider-local discovery; catalog and credential authority |
| ADR-0023 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0023 | options: preserve arbitrary keys vs migrate the selected key to `custom`; persisted provider-identity migration contract |
| ADR-0024 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0024 | options: direct replacement vs same-directory temporary write and rename; persistence-safety boundary |
| ADR-0025 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0025 | options: additive tree vs registry and journal vs one behavior inventory; repository-wide specification ownership |
| ADR-0026 | QUALIFIED | PASS | PASS | PASS | PASS | PASS | SUPPORTING | AMEND_ADR | ADR-0026 | options: plain-line quick setup vs slim coordinator mode vs coordinator only; first-run entry contract and dual-surface boundary |
| ADR-0005 | NOT_QUALIFIED | PASS | FAIL | FAIL | FAIL | PASS | NONE | CANONICAL_ARTIFACT | givn/specs/fragments/ask.feature | see verdict block below |
| ADR-0010 | NOT_QUALIFIED | PASS | FAIL | FAIL | FAIL | PASS | NONE | CANONICAL_ARTIFACT | givn/specs/configure-model/ratatui-model-picker.feature | see verdict block below |
| ADR-0012 | NOT_QUALIFIED | PASS | FAIL | FAIL | FAIL | PASS | NONE | CANONICAL_ARTIFACT | givn/specs/configure-provider/provider-setup-widget-layout.feature | see verdict block below |
| ADR-0022 | NOT_QUALIFIED | PASS | FAIL | FAIL | FAIL | PASS | NONE | CANONICAL_ARTIFACT | givn/specs/configure-model/reasoning-policy.feature | see verdict block below |

### Verdict blocks for non-qualifying records

```text
ADR-0005 — Execution mode with confirmation
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NONE
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/fragments/ask.feature
target_adr: null
replacement_adr: null
evidence:
  alternatives: ["direct execution", "clipboard copy", "`-x` flag with confirmation prompt"]
  architectural_impact: ["no component, authority, or deployment boundary beyond the workflow gate; the executor boundary is unchanged by the prompt"]
  durable_consequence: ["the `-x` opt-in plus confirmation is implementable and reversible within one change touching the Exec consumer"]
  lower_level_artifact: ["ask.feature owns the execute-flag confirmation scenarios ('Execute flag prompts for confirmation', 'Execute flag with explicit \"y\" confirmation')"]
  existing_adr_check: ["register search found no other record for the execution-confirmation boundary"]
failing test: F3 product-behavior exclusion (also F1 single-change reversibility)
destination: givn/specs/fragments/ask.feature
audit note: rationale moves once; the record leaves the chapter-09 register and docs/arc42/adr/README.md; it is never copied into docs/arc42/adr/archive/
```

```text
ADR-0010 — SetupWizard model picker and reasoning selection
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NONE
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/configure-model/ratatui-model-picker.feature
target_adr: null
replacement_adr: null
evidence:
  alternatives: ["extend the existing console raw-mode picker", "ratatui SetupWizard model pages"]
  architectural_impact: ["the choice is local to the setup model-picker component; the ratatui/crossterm stack is already recorded as a technical constraint in chapter 2 and a key decision in chapter 4"]
  durable_consequence: ["reversible within one change touching the picker; the guided sequence, per-word filter, and per-level reasoning are observable behavior"]
  lower_level_artifact: ["ratatui-model-picker.feature owns the picker contract; reasoning.feature and reasoning-policy.feature own per-level reasoning; widget mechanics belong to design/code"]
  existing_adr_check: ["adjacent records ADR-0009 (search strategy), ADR-0012 (widget composition), and the superseded ADR-0011/ADR-0013 onboarding records; none owns the picker implementation boundary; owning change `ratatui-model-picker` has archived"]
failing test: F1 single-change reversibility and F3 product-behavior exclusion
destination: givn/specs/configure-model/ratatui-model-picker.feature
audit note: rationale moves once; the record leaves the chapter-09 register and docs/arc42/adr/README.md; it is never copied into docs/arc42/adr/archive/
```

```text
ADR-0012 — Structured widget composition for terminal setup views
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NONE
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/configure-provider/provider-setup-widget-layout.feature
target_adr: null
replacement_adr: null
evidence:
  alternatives: ["paragraph-only rendering", "hand-positioned terminal output", "compose native Ratatui widgets", "run searches on the draw loop"]
  architectural_impact: ["the composition choice is local to the setup presentation component; the stack itself is recorded in chapter 2 and chapter 4"]
  durable_consequence: ["implementable and reversible within one change touching the setup views; the observable layout contract is spec-owned"]
  lower_level_artifact: ["provider-setup-widget-layout.feature owns the observable layout; the widget composition mechanics belong to design/code"]
  existing_adr_check: ["adjacent records ADR-0009 (search strategy) and ADR-0010 (picker); none owns the widget-composition boundary; owning change `provider-setup-widget-layout` has archived"]
failing test: F1 single-change reversibility and F3 product-behavior exclusion
destination: givn/specs/configure-provider/provider-setup-widget-layout.feature
audit note: rationale moves once; the record leaves the chapter-09 register and docs/arc42/adr/README.md; it is never copied into docs/arc42/adr/archive/
```

```text
ADR-0022 — Verbatim reasoning values
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NONE
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/configure-model/reasoning-policy.feature
target_adr: null
replacement_adr: null
evidence:
  alternatives: ["closed set", "non-empty strings with catalog suggestions"]
  architectural_impact: ["no component, authority, or deployment boundary: the decision fixes an accepted value domain and the `off` omission, both state values"]
  durable_consequence: ["reversible within one change touching the reasoning policy; the behavior is asserted by capability scenarios"]
  lower_level_artifact: ["reasoning-policy.feature owns the persisted/sent value contract ('Minimal reasoning is persisted and sent'); request mechanics belong to design/code"]
  existing_adr_check: ["register search: ADR-0007 is the superseded predecessor for the reasoning-value domain; no other record owns this boundary"]
failing test: F3 product-behavior exclusion (also F1 single-change reversibility)
destination: givn/specs/configure-model/reasoning-policy.feature
audit note: rationale moves once; the record leaves the chapter-09 register and docs/arc42/adr/README.md; it is never copied into docs/arc42/adr/archive/
```

### Superseded records (historical, not re-qualified)

ADR-0006, ADR-0007, ADR-0008, ADR-0011, ADR-0013, and ADR-0014 keep their
superseded status and move to `docs/arc42/adr/archive/` in the
`archive-superseded-adrs` follow-up; ADR-0007, ADR-0008, ADR-0011, and
ADR-0014 need their nonconforming status text normalized, and any still-current
rationale (notably ADR-0011's TTY boundary, typed cancellation, credential
precedence, and transport guard, still present in chapters 2, 4, and 11) moved
once to its capability spec.

## Status

STATUS: DONE
