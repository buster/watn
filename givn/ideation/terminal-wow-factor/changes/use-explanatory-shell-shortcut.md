# Change Seed: use-explanatory-shell-shortcut

## Stable use-case ID

`use-explanatory-shell-shortcut`

## Topic source

`terminal-wow-factor`

## Permanent owner

`use-shell`

## Handoff status

Ready for the normal proposal flow. Handoff does not start the change.

## Goal

Extend Watn's native shell shortcut with a small, transient explanatory review
panel that makes a generated command's command flow and model-written stage
purposes visible, supports refinement and comparison, and replaces the shell
buffer only after explicit acceptance.

## Confirmed persona

- `terminal-developer--interactive`

## Confirmed decisions

- Existing progress feedback remains first; the panel opens after a candidate is
  available and stage purposes may continue loading.
- The panel is a small transient inline ANSI panel on the controlling terminal,
  not a full-screen UI.
- The panel is enabled by default, configurable persistently, overridable per
  invocation, and may use automatically selected enhanced renderers.
- Enhanced renderer failure falls back to the portable inline panel; portable
  panel failure preserves the original input and releases no command.
- The panel applies to all eligible interactive paths; non-TTY and redirected
  paths preserve existing behavior.
- Ctrl-W records the original prompt in shell history and replaces the prompt
  with the accepted candidate, without executing it.
- Direct command editing preserves original intent, refreshes command-flow and
  explanation state, and requires final acceptance.
- Rephrase, higher-tier generation, explicit model selection, regeneration,
  rejection, cancellation, comparison, and candidate selection are supported.
- Active `-x` requires `-x` opt-in and final review acceptance; non-review `-x`
  retains existing confirmation.
- The explanation is advisory and never a semantic command-safety verdict.

## Open Design questions

- Decision/result transport that keeps review UI out of command stdout.
- Controlling-terminal descriptor, shell-specific redraw, resize, and repaint.
- Renderer adapter discovery and optional Kitty/tmux/zellij enhancements.
- Adaptive provider response shape and explanation fallback operation.
- Exact command-flow grammar and unsupported syntax marking.

## Suggested start order

1. Establish the portable inline panel and preserve the current progress line,
   stdout, shell history, and buffer contracts.
2. Add command-flow representation with visible degradation.
3. Add live stage-purpose loading and failure state.
4. Add candidate editing, rephrasing, comparison, escalation, and acceptance.
5. Integrate direct positional, interactive-stdin, and active `-x` consumers.
6. Add renderer selection, inline fallback, and optional enhanced adapters.
