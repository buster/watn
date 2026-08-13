# ADR-0013: Shared five-page setup wizard

- **Status:** superseded by ADR-0020 and the streamlined setup design
- **Date:** 2026-08-10
- **Decision-makers:** architect

## Context and Problem Statement

Provider setup and model setup currently own separate terminal flows. Their
screens display too much unrelated information at once, do not make the active
editing line explicit, and make the relationship between provider credentials
and model assignments unclear. The user needs one ordered onboarding surface
with a visible current page and predictable save behavior.

## Decision Drivers

- Make only the current task editable and identify it with a highlighted tab,
  page position, and visible cursor.
- Put URL, API key, and the three model assignments in one ordered flow.
- Preserve `watn provider` and `watn models` as useful entry points.
- Keep persisted provider, model-tier, and reasoning formats unchanged.
- Make Escape safe by asking whether valid current settings should be saved.
- Keep model search responsive and table-based.
- Respect model-specific reasoning metadata instead of exposing a global effort
  list; keep reasoning focus separate from page navigation.

## Considered Options

- **Keep separate provider and model dialogs:** minimal change, but preserves the
  confusing split workflow and duplicate event-loop ownership.
- **Render all setup information on one dashboard:** makes every value visible,
  but does not identify the active prompt or editing cursor.
- **Use one five-page wizard with command-specific entry pages:** establishes a
  single navigation and save contract while allowing provider-only and
  model-only entry points.

## Decision Outcome

Chosen: a shared Ratatui setup wizard with URL, API key, Small Model, Middle
Model, and Large Model pages. `watn setup` starts at URL and traverses all pages;
`watn provider` starts at URL and ends after API key; `watn models` starts at
Small Model when provider information is available. Enter and Tab advance,
Shift-Tab returns, Ctrl-R toggles model/reasoning focus, and Escape opens a save/discard prompt. The wizard returns
runtime drafts; callers persist provider and completed model choices through the
existing config writer.

The API key page asks whether to store a literal configuration value or an
environment reference. Model pages use aligned tables, visible row selection,
scrollbars for overflow, and the existing background search contract. Reasoning
options are derived from each model's catalog metadata. Mandatory models cannot
select `off`; disabled models offer `off`; supported efforts are limited to the
model response. Ctrl-R toggles reasoning focus because Tab is reserved for
wizard navigation.

## Consequences

- **Good:** users see one task, one cursor, one active tab, and one page position
  at a time.
- **Good:** provider and model onboarding share validation, cancellation, and
  persistence boundaries.
- **Good:** existing commands remain useful as focused entry points.
- **Good:** partial saves can preserve a valid provider without overwriting
  uncompleted tier assignments.
- **Bad:** the wizard has more state than either former dialog, including page
  range, API storage focus, partial choices, and save prompt.
- **Bad:** command-specific entry pages require careful initialization of
  provider values and model selections.
- **Bad:** a user who expects Escape to go back must use Shift-Tab; Escape now
  has an explicit save/discard meaning.
- **Bad:** the old Tab/Escape model-dialog contract changes; the visible focus
  indicator and migrated command steps reduce the transition cost.
- **Bad:** model-specific reasoning can make adjacent model pages expose
  different effort choices; catalog metadata and default-selection rules keep
  the control valid.

## Confirmation

The `unified-setup-wizard` E2E scenarios drive `watn setup` and `watn models`
through a PTY and assert active tabs, visible cursor/page indicators, URL/API
key prompts, model tables, page navigation, and Escape save/discard behavior.
