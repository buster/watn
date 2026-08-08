# Proposal: One keyboard-driven dialog for choosing models and reasoning per level

## Problem / Opportunity

Setting up the three levels — small, normal, and thinking — is a slow, clumsy,
one-at-a-time process. When you open the model settings screen (`models`), it
asks you three separate questions in sequence, one per level. The experience
has several concrete flaws:

- You cannot move through the list of suggested models. The up/down arrow
  keys do nothing, so pressing Enter always picks the first suggestion. On a
  long list you can never reach the model you actually want.
- There is no page up / page down, so even knowing a model exists does not
  help you browse to it.
- You cannot see what the list is currently filtered on. The filter is
  invisible, so you cannot tell why a model is shown, or fix a typo to widen
  the search.
- Matching is weak: it only finds results that contain your whole search text
  in order. Typing "dee flash" will not find "DeepSeek V4 Flash", even though
  the intent is obvious.
- Reasoning is not under your control. The thinking level always reasons at a
  fixed effort, and the other two levels never do. There is no way to say, per
  level, whether reasoning is on and how much of it you want — for example, to
  run reasoning on the normal level or light reasoning on the thinking level.

The effect: most people settle for the first suggestion, keep every default,
and never set the levels they actually want.

## Proposed Solution

Replace the three sequential questions with a keyboard-driven dialog, built
with the ratatui crate, for choosing model settings. The dialog walks the
three levels one at a time in a guided sequence — small, then normal, then
thinking — with a way to go back to a previous level and change it before
confirming. In this dialog:

- For each level you pick a model **and** choose a graduated reasoning
  strength: off, low, medium, or high. The choice you make per level is saved
  and used on subsequent runs, so you set it once and it sticks until you
  change it.
- The up and down arrow keys move the selection through the list. The selected
  entry is always clearly highlighted.
- The page up and page down keys move through the list a page at a time, so
  even a catalogue of hundreds of models can be walked quickly.
- The dialog always shows what the list is currently filtered on, so you can
  see exactly what you typed and why the shown models match.
- Filtering is a per-word, order-independent match against the model's
  identifier: "dee flash" finds the model "DeepSeek V4 Flash", because each
  word is matched separately anywhere in the identifier.
- Each model entry shows additional information about the model when it is
  available — for example, its pricing — so you can compare models in the
  dialog.
- The suggested list updates within about 200 milliseconds after you stop
  typing — responsive enough to feel immediate, without the list jumping
  around mid-keystroke.
- If nothing matches your filter, the dialog says so plainly and lets you
  adjust, instead of silently showing an empty list.
- Matching still works when the catalogue cannot be searched remotely; the
  dialog can always fall back to matching against the models it has locally.
- The reasoning choice takes effect on the requests that level makes: a level
  marked "no reasoning" never reasons, and a level with a reasoning strength
  uses that strength.

## Out of Scope

- The plain non-interactive listing of models (when the dialog is not used)
  stays exactly as it is today.
- Nothing changes about browsing or exploring the catalogue beyond picking a
  model in this dialog.
- Nothing changes about how the model catalogue is obtained or updated, or
  how individual requests are wired up internally — only that the saved
  settings are honoured.

## Open Questions

None — the dialog shape (guided per-level sequence), the reasoning scale
(graduated: off, low, medium, high), and the filter scope (identifier only)
are settled as stated above.