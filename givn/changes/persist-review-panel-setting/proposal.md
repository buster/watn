# Proposal: persist-review-panel-setting

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to review-panel-controls)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

The review surface can be disabled permanently from inside the panel, but
enabling it again requires editing the configuration file. The command-line
overrides `--review-panel` / `--no-review-panel` change only the current run, so
the developer cannot recover the surface with a single command.

## Proposed Solution

- `--review-panel` enables the review surface for the invocation and persists
  `[review] panel = true`.
- `--no-review-panel` disables it for the invocation and persists
  `[review] panel = false`.
- With neither flag, the persisted setting decides; the surface stays enabled
  by default when no setting exists.
- The panel's `d` decision keeps persisting the disable through the same
  atomic configuration write.
- Other configuration values are preserved by the write. A failed write is
  reported and does not change the behavior of the current invocation.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND highlight-active-setup-input` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The review surface preference belongs to the existing review capability in `use-shell`, not setup-input highlighting. |

## Out of Scope

- No change to the card, its decisions, or the structured response contract.
- No new interaction inventory entry; the CLI flags already exist.
- No change to disabled or non-review request handling.

## Open Questions

- None.
