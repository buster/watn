# Watn Terminal Design System

Durable visual contract for watn's terminal surfaces. The review card and the
setup pages share one visual language; a change that introduces or visibly
changes a surface updates this document in the same change.

## Scope

- Interface: terminal (ANSI, no alternate screen).
- Surfaces: the transient inline review card (simple, detailed, and
  explanation-only), the model chooser, the command editor, and the ratatui
  setup pages.
- Color is presentation only. When the terminal does not support color
  (`NO_COLOR` set, `TERM=dumb`, or no color capability), the same surface
  renders monochrome and every symbol and text label still carries the meaning.

## Tokens

| Token | ANSI | Role |
|---|---|---|
| border | `38;5;240` | frame lines, separators, secondary structure |
| label | `38;5;81` | field labels and surface headings |
| value | `97` | primary values and command text |
| dim | `2` | hints, secondary guidance, hidden-window marker |
| key | `1;38;5;81` | decision key in a hint, emphasized |
| flag | `38;5;221` | command options and flags |
| green | `38;5;114` | selected choice, focused input accent |
| amber | `38;5;214` | warning, selected-stage marker, validation |
| purpose | `38;5;252` | model-written stage purpose text |

Spacing and typography are terminal-native: bold keys, dim labels, one blank
row between content blocks, and hints joined by ` · `.

## Symbols

| Symbol | Meaning |
|---|---|
| `◆` | frame/surface marker; active setup page |
| `▶` | selected stage or highlighted list/table row |
| `●` / `○` | selected / unselected choice |
| `↳` | guidance or stage purpose marker |
| `⚠` | warning or validation notice |
| `⋮` | hidden-window marker in a bounded stack |
| `U+250C, U+2510, U+2514, U+2518, U+2500, U+2502` | card and page frame (top-left, top-right, bottom-left, bottom-right, horizontal, vertical) |
| `↑↓` | navigation key hint |
| `⏎` | accept/commit key hint |
| `esc` | close/cancel key hint |

## Primitives and states

- **Review card:** framed, bounded inline panel with a `watn · review` header,
  a stage stack with separators and a selected-stage arrow, a marked purpose
  row below the stack, and a decision-hint footer. States: simple, detailed,
  explanation-only, model chooser, command editor, color, monochrome, bounded
  window, truncated stage.
- **Setup page:** framed page with `watn · setup`, active-page marker, list or
  table with a visible cursor, green border on the focused input, guidance
  paragraph, and status line. States: default, focused, disabled, empty,
  loading, error.
- **Error state:** every error carries a heading label (`Error` or a warning
  symbol) and a visible way back (a key hint or the next decision).

## Motion

Surfaces are static redraws; the terminal owns the cursor and repaint. There
is no animation, so `prefers-reduced-motion` does not apply to a terminal
surface.

## Language

All visible text is English. Internal identifiers, state names, and debug
values never appear in visible text.

## Evidence

Every `@e2e` scenario that touches a terminal surface commits a captured
transcript under
`givn/changes/<change-id>/evidence/visual/<scenario-slug>/transcript.txt`
(ANSI-stripped visible text). Screenshots do not apply to terminal surfaces.
