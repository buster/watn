# Proposal: clarify-review-card-guidance

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none (follow-up to simplify-review-card-controls)
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Two parts of the review card do not explain themselves:

- After rejecting a candidate, the model chooser shows the configured tiers,
  catalog matches, and an empty input row, but nothing says that the tier
  digits are keys, that typing filters, that arrows pick, or that Enter uses
  the choice. The empty input row does not look editable, the first tier is
  drawn highlighted although it is not selectable, and the active model is not
  marked. The developer has to guess.
- The stage marker `⚠ unsupported` reads like an error or a semantic verdict.
  It only means the command flow contains shell constructs that Watn does not
  decompose (compound commands such as `while`/`do`/`done`, subshells,
  redirections, command substitution), which is normal and harmless.

## Proposed Solution

**Model chooser guidance.** The chooser's rows and hint line state what can be
done:

- The tier row shows the tier digit as a key, the tier name, and the model;
  the model currently in use is marked.
- The input row is labelled as a search and shows a dim placeholder
  (`type a model name…`) while it is empty, so it reads as editable.
- The suggestion row is labelled in plain language (`Picks`), not jargon.
- The hint line reads like instructions: `1/2/3 switch tier · type to filter ·
  ↑↓ pick · ⏎ use · esc back`.
- A non-empty catalog selects its first pick when the chooser opens (or when
  suggestions arrive), so `↑↓` and Enter work immediately instead of Enter
  doing nothing on an empty query.

**Clearer stage marker.** The marker for a stage Watn does not decompose is
renamed from `unsupported` to `nested syntax`. It keeps the same meaning and
the same amber styling; only the word changes.

Both changes are presentation and input affordance. The key map, the decisions,
the regeneration behavior, the flow splitter, and the release/acceptance rules
are unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND quicksetup` (advisory misroute) | `EXTEND interactive-shell-shortcut` | The route ranking is generic text overlap; the chooser and the stage marker are the existing use-shell review card. No new inventory action, no behavior change beyond presentation and default selection. |

## Out of Scope

- No change to which keys trigger which decisions, to regeneration, or to the
  catalog/typed-model precedence.
- No change to the flow splitter's decisions; only the marker word changes.
- No new interaction inventory entry; existing actions keep their coverage.

## Open Questions

- None.
