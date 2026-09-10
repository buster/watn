# Ideation Topic: terminal-wow-factor

## Current phase

Handoff

## Status

Complete. Handoff mapping, persona promotion, and durable-term promotion were
confirmed by the user. No proposal, specification, design, implementation, or
normal Givn change was started.

## Confirmed so far

- Desired outcome: Watn should make natural-language intent visibly
  become safe, understandable terminal action.
- Frame confirmed: the transition from natural-language intent to an immediate,
  legible, safe terminal action should feel trustworthy without leaving the
  terminal.
- Stakeholder type confirmed: terminal developer.
- Persona confirmed: `terminal-developer--interactive`.
- Safety answer received: control and non-execution are mandatory; semantic
  risk assessment is staged separately unless the event chain proves otherwise.
- Safety decision confirmed by the user.
- Vision elaborated: Ctrl-W should open an explanatory review surface that
  visualizes command pipelines and returns an explicit decision before any
  shell-buffer replacement.
- Action boundary confirmed: explicit accept/edit/reject/cancel precedes
  buffer replacement; the surface must support rephrasing intent and asking a
  higher-tier model.
- Escalation behavior selected: user-selectable proposal retention, with
  replacement as the default and comparison as an explicit option.
- Escalation decision confirmed by the user.
- First-time framing removed: this is a repeatable expert Ctrl-W workflow, not
  onboarding.
- Ownership confirmed: extend the existing `use-shell` user goal.
- Ownership split confirmed: `use-shell` owns Ctrl-W review and buffer
  replacement; asking/execution behavior consumes the shared review interaction
  for positional questions, interactive stdin, and `-x`.
- Entry scope confirmed: positional questions, stdin, `-x`, and Ctrl-W should
  support the explanatory review interaction.
- Non-TTY boundary confirmed: pipes retain raw script-safe output; review is
  reserved for interactive terminal contexts.
- Review eligibility selected: direct invocation requires stdin, stdout, and
  stderr all to be terminals; redirected paths retain raw behavior.
- Review eligibility decision confirmed by the user.
- Decision transport parked for Design with the invariant that explanatory UI
  never becomes command output and the shell widget receives a distinct review
  result.
- Initial Event Storming chain confirmed by the user.
- Explanation depth selected: stage purposes are model-written and advisory;
  structural parsing supplies layout, and no explanation is a semantic safety
  verdict.
- Explanation request selected: adaptive same-response generation with a
  separate fallback operation; command review remains available if explanation
  generation fails.
- Adaptive explanation decision confirmed by the user.
- Unsupported command-flow behavior selected: degrade visibly, preserve the
  raw command, mark unsupported portions, and retain review decisions.
- Domain term confirmed: `command flow` names the visual syntactic structure of
  a proposal.
- Edit semantics selected: direct command editing and intent rephrasing are
  distinct; direct editing does not regenerate, while rephrasing starts a new
  proposal cycle.
- Final acceptance confirmed as mandatory for every candidate, including direct
  command edits.
- Reject/cancel distinction selected: reject keeps review active without the
  current proposal; cancel exits and preserves original input state.
- Rejection loop selected: return to the original intent and let the developer
  choose rephrase, higher-tier request, or regeneration.
- Comparison history selected: retain every generated or edited candidate in
  the current review only; do not persist history across sessions.
- Candidate selection selected: final acceptance returns the currently selected
  candidate.
- `-x` behavior selected: when review is active, final acceptance authorizes
  execution directly; redirected/non-review `-x` retains the existing prompt.
- `-x` execution-boundary decision confirmed by the user.
- Proposal output timing selected: eligible review paths buffer the proposal
  until final acceptance; non-acceptance and failure outcomes release no
  command.
- Proposal output-timing decision confirmed by the user.
- Candidate vocabulary selected: `candidate` names each selectable generated
  or edited command; `proposal` names the overall offer or review subject.
- Candidate vocabulary confirmed by the user.
- Direct edits refresh candidate command-flow and stage-purpose state.
- Edited candidates remain reviewable with explicit unavailable or unsupported
  explanation state when refresh fails.
- Event Storming continuation events drafted for candidate lifecycle, failure,
  path-specific return, shell replacement, and active/non-review `-x` behavior.
- Persona Event Storming review recorded; direct edits preserve original intent,
  and cancellation/failure preserve original shell/input state.
- Use-case core contract confirmed: Ctrl-W changes the shell buffer only after
  explicit final acceptance; otherwise the original buffer and command release
  state remain unchanged.
- Representative example confirmed: a `git log | xargs git show && printf`
  pipeline with visible stages, purposes, and success branch.
- Stage information selected: exact stage text plus a model-written purpose;
  detailed inputs, outputs, environment, and effects are not required by the
  first elaboration boundary.
- Stage-information decision confirmed by the user.
- Direct-edit behavior recorded from prior confirmed decisions: editing a stage
  preserves original intent, refreshes command-flow and purpose state, and still
  requires final acceptance.
- Highest-tier escalation selected: a higher-tier request opens explicit model
  selection instead of silently regenerating at the same tier.
- Explicit model selection selected: use the existing provider catalog picker
  and apply the selected model only to the next candidate.
- Candidate comparison metadata selected: show configured tier when applicable
  and concrete provider/model identifier beside each candidate.
- Intent visibility selected: keep the original intent in a persistent,
  collapsible context panel during editing and comparison.
- Rephrased intent behavior selected: replace the visible active intent while
  retaining the prior intent only in internal current-review history; direct
  command edits preserve the original intent.
- Review interruption selected: cancel only the in-progress operation and keep
  the current selected candidate and review state.
- Presentation options discussed: small transient inline panel, Kitty overlay,
  optional tmux/zellij pane, and later Alacritty/iTerm2 new-window
  enhancements. Full-screen alternate-screen UI was rejected as intrusive.
- Universal terminal mechanism identified: ordinary ANSI/ECMA-48 redraw on the
  controlling terminal, with stdout reserved for the accepted command. The
  portable target is a small transient inline panel; exact shell repaint
  mechanics remain Design work.
- Presentation hierarchy resolved from the user's non-intrusive requirement:
  a small transient inline ANSI panel on the controlling terminal is the
  portable baseline; Kitty overlays and multiplexer surfaces are optional
  enhancements. Full-screen alternate-screen UI is rejected.
- Working UI concept: compact status/timer, short command-flow view, selected
  stage command and purpose, and one-line action menu. Arrows navigate; Tab and
  Shift-Tab move focus; Enter activates; Escape cancels. The panel disappears
  when review ends and the shell widget repaints the buffer. The existing
  progress line remains visible before the panel opens.
- Timing/presentation decision received: retain the existing progress line
  first; show the new explanatory panel after a candidate is ready, default on
  but disableable; allow Kitty or other renderers to replace it.
- Panel settings selected: persist the default, permit a per-invocation
  override, and auto-select enhanced renderers when available.
- Disabled-panel behavior selected: Ctrl-W retains existing direct replacement
  and skips command-flow review.
- Compact-flow behavior selected: show all stages when they fit; otherwise show
  an overview plus one readable selected stage with arrow navigation.
- Initial focus selected: final acceptance is focused when a candidate is ready;
  the command-flow view remains visible and navigable.
- Panel dismissal selected: remove immediately after acceptance or cancellation
  and restore the shell prompt without lingering status UI.
- Direct-path default selected: the panel applies to all eligible interactive
  paths, including direct positional, interactive stdin, Ctrl-W, and `-x`.
- Ctrl-W output clarified: restore the prompt with the accepted command in the
  buffer and record the original natural-language prompt in shell history, as
  today. Direct positional and interactive-stdin output remain separate because
  they have no shell prompt/history.
- Output decision settled without further confirmation: Ctrl-W records the
  original prompt in history and replaces the prompt with the accepted
  candidate; direct paths preserve existing normal output.
- Panel timing resolved: existing progress line first; optional default-on panel
  after candidate generation.
- Panel renderer settings resolved: persistent default, per-invocation override,
  and automatic enhanced-renderer selection.
- Disabled-panel behavior resolved: preserve current direct replacement for
  Ctrl-W and current direct output/`-x` confirmation for direct paths.
- Compact layout, keyboard focus, renderer fallback, explanation loading,
  initial focus, dismissal, direct-path default, and direct output behavior are
  resolved as recorded in `questions.md`.

## Research protocol

- Existing product and permanent specifications are the source of truth for
  current Watn behavior.
- This topic may identify gaps and new domain behavior, but it must not alter
  permanent specifications during ideation.
- Persona confirmed: `terminal-developer--interactive`.
- Unknowns are recorded in `questions.md` rather than resolved by invention.

## Next step

Start `/givn-propose use-explanatory-shell-shortcut` when implementation work is
wanted.
