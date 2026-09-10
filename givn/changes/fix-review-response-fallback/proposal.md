# Proposal: fix-review-response-fallback

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to the archived use-explanatory-shell-shortcut change)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Real provider responses sometimes arrive wrapped in a markdown code fence,
with surrounding prose, or with a status value outside the reviewed review
contract. Today such a response is treated as the command itself: the review
surface shows raw provider payload, the command flow is derived from JSON
punctuation, and line breaks inside the payload break the bounded inline panel
so it repaints over itself.

The developer cannot understand, edit, or accept a candidate from that state,
even when the payload contains a complete, usable command.

## Proposed Solution

When the review surface is enabled and eligible:

- A structured review response is recognized even when the provider wraps it in
  a markdown code fence around otherwise valid content.
- When the response is not a valid structured response but does contain a
  complete non-empty command, the candidate shows that command with
  `purpose-unavailable`. The developer can still inspect, edit, or accept it.
  Watn never invents a stage purpose.
- When no usable command can be recovered, the existing unavailable behavior
  applies: the original input is preserved and nothing is released.
- Every rendered candidate, intent, stage, and purpose value occupies a single
  inline row. Line-breaking and tab characters are shown as spaces so the
  bounded panel never corrupts its own layout or row accounting.
- Disabled, non-review, and non-terminal behavior is unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | No new interaction exists; this extends the already-owned review capability in `use-shell`. |

## Out of Scope

- No new interaction or command-surface change; Ctrl-W, direct positional,
  interactive stdin, and `-x` routing stay as reviewed.
- No change to disabled or non-review behavior.
- No semantic command-risk validation.
- No change to the structured response contract itself.

## Open Questions

- None.
