# ADR-0010: SetupWizard model picker and reasoning selection

- **Status:** proposed
- **Date:** 2026-08-08
- **Decision-makers:** architect

## Context and Problem Statement

Assigning the three tiers (small, normal, thinking) through `watn models` is a
slow, one-at-a-time process. The previous interactive picker ran three
separate sequential questions, each offering a plain model list with a hidden
filter and no way to browse. Reasoning effort was hard-coded: the thinking
tier always reasoned at "high", the other two never reasoned.

Users need to: browse large model catalogs with arrow and page keys, see the
active filter, match by any word in a model id regardless of order, inspect a
model's pricing while choosing, and decide separately for each level whether
and how much reasoning to use — then have those choices stick between runs.

## Decision Drivers

- Must keep the guided small → normal → thinking sequence.
- Must let the user go back to a previous level and change it before
  confirming.
- Must support browsing large lists (arrow keys + page up/down).
- Must show the active filter and match per-word, order-independent.
- Must persist per-level reasoning strength (off/low/minimal/medium/high) in config.
- Must preserve existing non-interactive model listing behaviour.

## Considered Options

- **Extend the existing `console` raw-mode picker** — add page keys, a visible
  filter, metadata lines, and reasoning selectors on top of the current loop.
  The growing set of features (two widget areas, focus management, resize-safe
  rendering) becomes increasingly ad-hoc hand-rolled cursor math.
- **Ratatui SetupWizard model pages** — use ratatui's `List`/`ListState`, `Layout`, and
  `crossterm` event loop so widget rendering, selection highlighting, paging,
  and focus are framework-managed. Ratatui is purpose-built for exactly these
  widgets.

## Decision Outcome

Chosen: ratatui-based keyboard-driven SetupWizard model pages that walk the
three levels in a guided sequence. Each level shows the filter line
(always visible), the matching model list with the current selection
highlighted, and a reasoning-strength selector (off/low/medium/high). Arrow
and page keys browse the list, Enter accepts the model and advances, Escape
opens the save/discard prompt, Shift-Tab returns to the previous page, and
confirming on the final level persists the
per-level model and reasoning choices. Reasoning strength is persisted in
config (`[tiers.reasoning]`) and resolved into `reasoning_effort` per request.

Filter matching is per-word and order-independent against the model id, and
falls back to local matching when the remote search endpoint is unavailable.

## Consequences

- Good: framework-managed list rendering, highlighting, and paging remove
  hand-rolled cursor/repaint code.
- Good: a single guided wizard presents model **and** reasoning choices
  together, so levels stop being three disconnected prompts.
- Good: per-word matching makes large catalogs searchable ("dee flash" finds
  "DeepSeek V4 Flash").
- Bad: adds `ratatui` and `crossterm` dependencies to the binary.
- Bad: escape-sequence handling for arrow/page keys still varies across
  terminal emulators; PTY-based E2E tests pin a known `TERM` to keep them
  deterministic.

## Confirmation

E2E scenario: drive `watn models` in a PTY, configure model + reasoning for
all three levels, navigate with arrow/page keys, type a multi-word filter,
return to a previous level to change it, and verify the per-level reasoning
takes effect on a subsequent request.
