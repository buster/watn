# Ideation: Terminal Wow Factor

## Outcome

Watn should make natural-language intent visibly become understandable terminal
action without forcing the terminal developer to leave the terminal or surrender
control to opaque automation.

The topic deliberately does not claim semantic command-risk validation. Its
safety boundary is visible proposed action, explicit acceptance or rejection,
non-evaluation during review and shell-buffer replacement, and preserved input
on cancellation or failure.

## Confirmed stakeholder and persona

- Stakeholder: terminal developer.
- Persona: `terminal-developer--interactive`.
- The persona is promoted to `givn/personas/terminal-developer--interactive.md`.

## Permanent ownership

The permanent user-goal owner is `use-shell`. The topic adds one change-sized
capability under that owner:

`use-explanatory-shell-shortcut`

The shared review lifecycle is consumed by direct positional questions,
interactive stdin, and eligible `-x` paths. These are not new permanent
user-goal roots. Non-TTY and redirected paths preserve their existing behavior.

## Change seed

`givn/ideation/terminal-wow-factor/changes/use-explanatory-shell-shortcut.md`

The seed is ready for `/givn-propose`. Handoff does not start the change.

## Confirmed behavior

- The existing progress line remains the universal generation feedback.
- After a candidate exists, a small transient inline panel opens by default.
- The panel uses ordinary ANSI/ECMA-48 redraw through the controlling terminal;
  it is not a full-screen alternate-screen interface.
- Kitty overlays and optional tmux/zellij panes may replace the baseline renderer;
  enhanced-renderer failure falls back to the portable inline panel.
- Panel preference is persistent, overridable per invocation, and supports
  automatic enhanced-renderer selection.
- The panel can be disabled. Disabled behavior preserves existing direct
  replacement, output, and `-x` confirmation behavior.
- The panel shows a command flow, exact stage text, and concise model-written
  stage purposes. Purposes may load after the panel opens.
- Incomplete flow derivation and explanation failure remain visibly reviewable.
- Tab cycles Flow, Candidates, and Actions. Arrow keys navigate. Enter activates.
  Escape cancels. Direct editing has separate commit/discard behavior.
- Candidates can be edited, rephrased, regenerated, compared, rejected, or
  escalated to higher tiers or an explicit model from the provider catalog.
- Final acceptance selects exactly one candidate.
- Ctrl-W records the original prompt in shell history and replaces the prompt
  with the accepted candidate without executing it.
- Direct positional and interactive-stdin paths preserve existing output.
- Active `-x` requires `-x` plus final review acceptance and does not show a
  second confirmation. Non-review `-x` retains the existing confirmation.

## Open Design questions

- Result transport that keeps review UI out of command stdout.
- Controlling-terminal descriptor, shell-specific redraw, resize, and repaint.
- Renderer discovery and adapter implementation.
- Adaptive provider response shape and explanation fallback operation.
- Exact command-flow grammar and unsupported syntax marking.

## Durable terms promoted

- `Candidate`: selectable generated or directly edited command in the current
  review. Anti-terms: `result`, `answer`.
- `Command flow`: visible syntactic structure of a candidate. Anti-term:
  `pipeline map`.
- `Review surface`: transient terminal presentation for reviewing a proposal.
  Anti-term: `overlay` as the domain boundary.
- `Review decision`: explicit developer action during review. Anti-term:
  `action` when the review outcome is meant.
- `Presentation adapter`: terminal-specific renderer of the review surface.
  Anti-term: `handler`.

## Next step

Run the normal proposal flow for `use-explanatory-shell-shortcut` when ready.
