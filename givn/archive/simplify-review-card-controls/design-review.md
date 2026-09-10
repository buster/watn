# Design Review: simplify-review-card-controls

## Review Scope

Grilling covered `proposal.md`, the change feature delta, `design.md`, the
change-side `usecase.md`, the permanent `use-shell` contract and capability,
the glossary, the review panel/renderer/session code, the generation path in
`src/main.rs`, the mock harness, and the affected Arc42 chapters. The grilling
subagent ran in a fresh context and its findings are dispositioned below.

## Findings And Dispositions

### Scope and normalization

**Finding (blocker):** the proposal claimed "no new inventory action" while the
delta adds an inventory row and a dedicated `@e2e` for reject-and-regenerate;
`givn lint` flags the new E2E as a shape subset of the existing accept E2E.

**Disposition:** Resolved in favour of a distinct action. Reject-and-regenerate
is the only review decision that issues a second provider request and replaces
the candidate under the same Intent; it has a different side effect and a
different observable result. `proposal.md` now states the new inventory action,
and the E2E waits for the replacement-specific candidate and asserts the output
channel contains only the replacement, so it is no longer a subset.

### Testability

**Finding (blocker):** the regeneration scenarios would pass through harness
state calls (`replace_current`, `select_model`) rather than production code,
and the failure path had no production test.

**Disposition:** Resolved. The generation path moves to
`src/review/session.rs` (`generate_candidate`, `parse_generated_candidate`,
`chooser_tiers`, `fetch_catalog`). Unit steps build a real
`OpenAICompatibleProvider` against the world's in-process `httpmock` twin and
drive the production functions, so the tier, typed-model, and failed
regeneration scenarios can fail in RED for the right reason. The blocking
generation is single-threaded, so an overlapping reject is impossible by
construction.

### Observability of emphasis

**Finding (question):** "accept should be shown as the default decision" had
no concrete observable; the existing render helper cannot assert emphasis.

**Disposition:** Resolved. The design fixes the rendering contract: in color
mode the accept hint is painted green and the focus strip is absent. A styled
substring helper asserts the accent, and the scenario runs on a color-capable
render.

### Catalog fetch UX

**Finding (should-fix):** a synchronous `fetch_models` call can freeze the raw
mode card for up to 30 s with no feedback.

**Disposition:** Resolved. The chooser opens immediately with tiers and the
text field and a loading row; the catalog fetch runs on a worker with a request
id, and the card loop uses `event::poll` to apply a current result when it
arrives. A stale result can never replace newer state, and an unavailable
catalog degrades to no suggestions.

### Harness and shared steps

**Finding (should-fix):** the E2E path wipes the config and builds tiers as
`None`, so the chooser cannot offer real tiers; the mock cannot return a
different replacement body; and the wait label changes break the existing E2E
and unit steps.

**Disposition:** Resolved. `WatnWorld` gains `pending_tiers` and
`pending_mock_replacements`; replacement mocks are registered before the
generic chat mock so `body_includes` routing wins; E2E waits move to
replacement-specific or contiguous labels (`du -sh .`, `accept ·`). The design
enumerates the shared step migration: removed-API steps are deleted or
rewritten, the retained higher-tier and interrupt scenarios keep passing, and
`Focus`/`Accept` literals are replaced.

### Registry ownership

**Finding (should-fix):** the registry is currently moved into the worker
thread, so the planned `generate_candidate(registry, ...)` signature could not
borrow it.

**Disposition:** Resolved. `generate_candidate` takes `&dyn Provider` and
scopes its thread internally, so `run()` keeps ownership of the registry.

### Catalog source

**Finding (should-fix):** the design used the chat endpoint for catalog
discovery; the rest of the system honours `providers.*.catalog_endpoint`.

**Disposition:** Resolved. `fetch_catalog` uses the provider-local
`catalog_endpoint` when set, otherwise the chat endpoint.

### Missing scenarios and trivial passes

**Finding (should-fix):** observable behaviours lacked scenarios (`a` accept,
typed free-form model, catalog unavailability, incomplete chooser choices,
editor insertion, explain-only key map), and the Enter scenario could pass
trivially against the old default-action behaviour.

**Disposition:** Resolved. Added scenarios: accept shortcut; typed model with
unavailable catalog; incomplete choices ignored; explanation card ignores
review decisions; the editor scenario now asserts insertion. The Enter scenario
asserts the flow-first card with no focus region, so it cannot pass against the
old UI. Editor boundary behaviour and the explain-only key map are recorded in
the Failure Outcomes table.

### E2E fidelity

**Finding:** the direct path redirects stdout to a file and has no shell
buffer; the design said "and shell buffer".

**Disposition:** Resolved. The design states the command-output file as the
primary assertion for the direct paths; PTY key driving remains the mechanism.

### ADR qualification

`NOT_QUALIFIED` with routing to this design. No ADR owns review-card controls;
ADR-0015 owns stream completion and is unchanged. `must_be_shared` is
supporting only.

### Arc42 independent walk

| # | Independent result | Disposition |
|---:|---|---|
| 3 | Yes | Developer context gains the in-card model chooser. |
| 4 | Yes | Review decisions and candidate replacement. |
| 5 | Yes | Panel state machine, renderer rows with focus/tabs/actions wording. |
| 6 | Yes | Review/accept sequence and Candidate lifecycle wording. |
| 8 | Yes | "Review decisions are explicit"; tier selection and catalog lookup. |
| 10 | Yes | QS-067/QS-070 and matrix rows name focus regions and actions. |
| 11 | Yes | Regeneration-failure and catalog-fetch risks. |
| 12 | Yes | Glossary updates below. |
| 1, 2, 7, 9 | No | No other chapter changes. |

### Ubiquitous language

**Finding:** `Review history` still includes Candidate retention, `Review
surface` says "candidates", `Review outcome` lists the old typed results, and
`Model picker` means the SetupWizard flow.

**Disposition:** Resolved. `Review history` becomes prior-Intent only;
`Review surface` drops "candidates"; `Review outcome` gains
`RejectRequested`/`RegenerateWith` and drops stale values; `Model chooser` is
added with anti-terms "not the SetupWizard Model picker, not the model table or
tier tabs". The change-side `usecase.md` uses only these terms.

## Hardening Receipt

- `proposal.md`: reject-and-regenerate recorded as one new inventory action;
  non-blocking suggestions and the unchanged explanation card stated.
- feature delta: fifteen `@givn.added` scenarios (one `@e2e`) and six
  `@givn.removed` scenarios.
- `design.md`: panel state, editor cursor, session extraction, async catalog,
  harness state, chapter set, failure table, glossary updates.
- `usecase.md`: main flow, rules, examples, and the interactions row updated.
- `givn lint --change simplify-review-card-controls`: exit 2 with 12 findings
  (WIP notices plus two overlap notices dispositioned in the coverage review).
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
