# ADR-0012: Structured widget composition for terminal setup views

- **Status:** proposed
- **Date:** 2026-08-10
- **Decision-makers:** architect

## Context and Problem Statement

The provider setup and model settings dialogs already use Ratatui for frame
rendering, but most visible content is assembled as paragraph text or manually
positioned terminal output. Provider choices, provider details, model metadata,
active tier, and list position are therefore difficult to scan. A long model
catalog also provides no persistent indication of the user's position.

## Decision Drivers

- Make setup state legible without changing the existing keyboard contract.
- Keep provider choices visibly distinct from provider details and guidance.
- Align model metadata into columns rather than relying on spacing in a sentence.
- Show the active model-assignment tier and overflow position.
- Keep the single-binary, TTY-gated terminal architecture and existing state
  transitions.
- Keep terminal output testable through the existing PTY harness.
- Keep search responsive while a provider request is in flight and prevent an
  older result from replacing a newer query.

## Considered Options

- **Keep paragraph-only rendering:** smallest code change, but it preserves the
  ambiguity and scanning problems that motivated this change.
- **Continue hand-positioned terminal output:** allows custom formatting, but
  requires manual cursor math, repaint handling, and scrolling behavior for
  every new field.
- **Compose native Ratatui widgets:** use bordered blocks, lists, tables,
  paragraphs, tabs, and stateful scrollbars around the existing state machine.
  This delegates layout and selection presentation to the existing TUI library
  without introducing a second interaction model.
- **Run searches on the draw loop:** simplest control flow, but a slow provider
  blocks keyboard input and makes a visual picker feel unresponsive.

## Decision Outcome

Chosen: compose the provider setup and model picker with native Ratatui widgets.
Provider setup uses an outer border, credential-source list, provider-details
table, and guidance paragraph. The model picker uses an outer border, small /
normal / thinking tabs, filter and status paragraphs, an aligned metadata table,
and a vertical scrollbar driven by the existing selection index. Typed searches
use a 200 ms worker debounce and generation check so the draw loop remains
responsive and only the newest result is applied. Existing keyboard events,
validation, persistence, and terminal restoration remain unchanged.

## Consequences

- **Good:** users can identify the active setup screen and its regions quickly.
- **Good:** model metadata is aligned and the selected row remains visible while
  browsing a long catalog.
- **Good:** the rendering code no longer needs direct cursor movement for setup
  state that Ratatui can represent.
- **Good:** existing PTY tests can assert stable titles and headings through the
  actual terminal interface.
- **Bad:** the layout has more visible regions and can become cramped on narrow
  terminals; constraints, wrapping, and truncation are required.
- **Bad:** terminal snapshots contain renderer control sequences, so tests must
  normalize them before asserting visible text.
- **Good:** a slow search no longer blocks keyboard input, and stale search
  results cannot overwrite a newer query.
- **Bad:** debounced worker lifecycle and channel handling add concurrency and
  timing paths to the dialog.
- **Bad:** the tabs reflect the existing guided sequence rather than adding a
  freely switchable navigation model, avoiding a second persistence contract at
  the cost of less direct tab navigation.

## Confirmation

The `provider-setup-widget-layout` Gherkin E2E scenarios drive `watn provider`
and `watn models` through a PTY and assert the bordered panels, source list,
detail/table headings, tier tabs, and long-list scrollbar presentation.
