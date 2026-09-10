# Open Questions: terminal-wow-factor

## Q1. Safety boundary

Does “safe” mean Watn preserves user control and prevents unintended
execution, or must Watn also understand and validate the semantic risk of a
generated command?

Confirmed decision: Control and non-execution are mandatory in this topic.
Semantic risk assessment is staged as a separate later capability unless
Event Storming demonstrates that it belongs in the same value-producing
sequence.

Status: Resolved

## Decision record

- Confirmed by the user: control and non-execution are mandatory; semantic risk
  assessment is staged separately unless the event chain proves otherwise.

## Q2. Action boundary

Which outcome belongs to the value-producing boundary: visible stdout,
confirmation, shell-buffer insertion, actual execution, or more than one?

Vision received: Ctrl-W should open an explanatory review surface over or near
the current shell prompt. The surface would visualize a generated command's
pipeline and control flow, then return an explicit accept, edit, reject, or
cancel decision to the shell widget. Only acceptance may replace the shell
buffer; execution remains the shell's responsibility.

Technical boundary: Kitty can provide a terminal-specific overlay through a
kitten. Alacritty does not provide a generic overlay/split API; portable
fallbacks are an alternate-screen TUI, a new terminal window, or a
multiplexer integration.

Confirmed decision: Make the explanatory review surface launched by Ctrl-W the
primary value boundary, with shell-buffer replacement only after explicit
acceptance. Treat Kitty overlay as an enhanced presentation and retain a
portable fallback.

Confirmed continuation capabilities:

- Rephrase the original intent from the review surface.
- Request a higher-tier model from the review surface.

Confirmed decision: The review surface makes proposal retention user-selectable:
replacement is the default for focus, with an explicit option to retain prior
proposals for comparison.

Status: Resolved

## Decision record

- Confirmed by the user: replacement is the default; comparison is explicit.

## Q3. First-time meaning

Does “first-time user” mean first-ever Watn invocation, first interactive
request in a session, first use of a command pattern, or a novice terminal
developer?

Confirmed decision: Remove the first-time framing. This is a repeatable expert
workflow invoked through Ctrl-W, not onboarding.

Status: Resolved

## Decision record

- Confirmed by the user: the topic defines a repeatable workflow rather than a
  first-use moment.

## Q4. Ownership shape

Is the outcome a new user goal with a new permanent use-case owner, or a
coherent improvement across the existing `ask` and `use-shell` boundaries?

Confirmed decision: Extend the existing `use-shell` user goal with an
explanatory review-surface capability attached to the interactive shortcut.

Confirmed ownership split: `use-shell` owns the Ctrl-W review surface and
shell-buffer replacement. Existing asking/execution behavior consumes the same
review interaction for positional questions, interactive stdin, and `-x`.
No new permanent user-goal use case is created.

Status: Resolved

## Decision record

- Confirmed by the user: retain `use-shell` as the permanent user-goal owner
  and model the other entry paths as explicit consumers.

## Q5. Surface boundary

Are positional questions, stdin, `-x`, and Ctrl-W all in scope, or is this topic
interactive-only and explicitly excludes scripted/CI output contracts?

Confirmed decision: The explanatory review interaction is in scope for
positional questions, interactive stdin, and `-x`, in addition to Ctrl-W.

Status: Resolved

## Q6. Non-TTY stdin

Should non-TTY piped stdin retain the existing raw, script-safe output contract,
or should a pipe also receive an explanatory review surface?

Confirmed decision: Preserve the existing raw, script-safe output contract for
non-TTY pipes. Reserve the explanatory review surface for interactive terminal
contexts.

Status: Resolved

## Decision record

- Confirmed by the user: all listed entry paths are in scope for the interactive
  review interaction.
- Confirmed by the user: non-TTY pipes remain raw and script-safe.

## Q7. Review eligibility

What exact terminal condition allows a review surface to open for positional
questions, interactive stdin, and `-x`? The current candidates are all
interactive standard streams, a controlling terminal, or a shell-widget-only
condition for Ctrl-W with separate rules for direct invocation.

Confirmed decision: Direct positional questions, interactive stdin, and `-x`
open the review surface only when stdin, stdout, and stderr are all terminals.
If any standard stream is redirected, the existing raw/script-safe behavior
remains in force. Ctrl-W remains an interactive shell-widget entry point.

Status: Resolved

## Decision record

- Confirmed by the user: all-three-stream TTY eligibility for direct invocation.

## Q8. Decision transport

How does the review surface return accept/edit/reject/cancel and the selected
proposal without contaminating command stdout captured by the shell widget?

Status: Parked for Design

Confirmed domain invariant: explanatory UI output must never be captured as
command output. The shell widget must receive a distinct decision result and,
when applicable, the selected command. Transport mechanics are deferred to
Design.

## Q9. Explanation depth

Should the command flow explain only the syntactic structure of the proposal,
or should each stage also carry a plain-language purpose explaining why it is
present and what it contributes to the requested intent?

Confirmed decision: Each command-flow stage receives a concise model-written
purpose explaining why it is present and what it contributes to the intent.
Structural parsing supplies the visual layout. The explanation is advisory and
must not be presented as a semantic safety verdict.

Status: Resolved

## Decision record

- Confirmed by the user: use model-written explanations for each stage.

## Q10. Explanation request

Should stage purposes be returned as part of the original proposal generation,
or requested as a separate explanation operation after the command exists?

Confirmed decision: Use an adaptive explanation operation. Prefer one coherent
proposal response containing the command and stage purposes when the provider
supports it; otherwise generate the command first and request stage purposes as
a separate operation. The command remains reviewable if explanation generation
fails.

Status: Resolved

## Decision record

- Confirmed by the user: adaptive explanation with command review preserved on
  explanation failure.

## Q11. Unsupported command flow

If Watn cannot derive a complete command flow for generated shell syntax, what
should the review surface do?

Confirmed decision: Degrade visibly. Show the raw command, mark unsupported
portions clearly, and keep accept/edit/reject/cancel available.

Status: Resolved

## Decision record

- Confirmed by the user: incomplete structural derivation must remain visible
  and must not remove the developer's review choices.

## Q12. Edit meaning

When the developer chooses `edit`, does that mean directly editing the proposed
command, rephrasing the original intent, or both as distinct actions?

Confirmed decision: Support both actions as distinct. `Edit command` changes the
proposed command directly without model regeneration. `Rephrase intent` changes
the request and starts a new proposal cycle.

Status: Resolved

## Decision record

- Confirmed by the user: direct command editing and intent rephrasing are
  separate review actions.

## Q13. Edited proposal acceptance

Must a directly edited command be explicitly accepted again before it can be
returned to the shell buffer or the direct interactive caller?

Confirmed decision: Every candidate, including a directly edited command,
requires an explicit final acceptance before it can be returned to the shell
buffer or direct interactive caller.

Status: Resolved

## Decision record

- Confirmed by the user: final acceptance is mandatory for directly edited
  commands.

## Q14. Rejection and cancellation

What is the domain distinction between rejecting a proposal and cancelling the
review surface?

Confirmed decision: `Reject` discards the current proposal but keeps the review
cycle active for another proposal. `Cancel` exits the review surface and
preserves the original shell/input state.

Status: Resolved

## Decision record

- Confirmed by the user: reject and cancel are separate outcomes.

## Q15. Rejection input

After rejecting a proposal, does the review surface ask for a revised intent,
or does it return to the original intent with the developer choosing the next
action?

Confirmed decision: After rejection, return to the original intent and let the
developer choose rephrase, higher-tier request, or regeneration.

Status: Resolved

## Decision record

- Confirmed by the user: rejection returns to a developer-selected next action.

## Q16. Comparison history

When the developer explicitly retains proposals for comparison, how much review
history is retained?

Confirmed decision: Retain every generated or edited candidate from the current
review when comparison is explicitly enabled. Do not persist review history
across sessions.

Status: Resolved

## Decision record

- Confirmed by the user: comparison history covers the current review only.

## Q17. Candidate selection

When multiple candidates are retained, how does final acceptance identify the
candidate to return or replace into the shell buffer?

Confirmed decision: The review surface always has one selected candidate, and
explicit final acceptance returns that selected candidate.

Status: Resolved

## Decision record

- Confirmed by the user: accept the currently selected candidate.

## Q18. Execute continuation

For `-x`, after a proposal is reviewed and explicitly accepted, should the
existing separate execution confirmation still be required?

Confirmed decision: With `-x`, explicit final acceptance in the review surface
authorizes execution of the selected candidate, so the existing separate
`Execute now?` confirmation is not shown when the review surface is active.
This is not implicit execution: the `-x` opt-in and final review acceptance are
both required. Redirected/non-review `-x` paths retain the existing confirmation
behavior.

Status: Resolved

## Decision record

- Confirmed by the user: active review plus `-x` plus final acceptance authorizes
  execution; non-review paths retain the existing prompt.

## Q19. Proposal output timing

When the explanatory review surface is eligible, should Watn buffer the
generated proposal away from command stdout until final acceptance, then emit
or return only the selected command? Should reject, cancel, failure, or empty
output emit no command?

Confirmed decision: Buffer the proposal until final acceptance. The review
surface owns the proposal while open. Only final acceptance releases the
selected command to the direct caller or shell widget. Reject, cancel,
generation failure, explanation failure without a reviewable command, and empty
output release no command.

Status: Resolved

## Decision record

- Confirmed by the user: eligible review paths buffer proposals and release
  commands only after final acceptance.

## Q20. Candidate vocabulary

What single domain term should name each generated or directly edited command
that can be selected and finally accepted during the current review?

Confirmed decision: Use `candidate` for each selectable generated or edited
command. `Proposal` names the overall command offer or review subject; a review
may retain multiple candidates for comparison.

Status: Resolved

## Decision record

- Confirmed by the user: `candidate` is the canonical selectable command term.

## Q21. Direct edit explanation

After the developer directly edits a candidate command, should Watn derive a new
command flow and request or retain stage purposes for the edited candidate?

Confirmed decision: A direct command edit creates a refreshed candidate state.
Watn re-derives its command flow and refreshes or requests stage purposes for
the edited candidate.

Status: Resolved

## Decision record

- Confirmed by the user: direct edits refresh structural and explanatory state.

## Q22. Edited explanation failure

If flow derivation or stage-purpose generation fails after a direct edit, should
the edited candidate remain reviewable under the visible-degradation rule?

Confirmed decision: If refreshed flow or stage purposes fail after a direct
edit, keep the edited candidate reviewable. Show the raw edited command and an
explicit unavailable or unsupported explanation state.

Status: Resolved

## Decision record

- Confirmed by the user: explanation refresh failure does not remove an edited
  candidate from review.

## Q23. Stage information

What minimum information must the review surface show for each command-flow
stage?

Confirmed decision: Show the exact stage text and a concise model-written
purpose tied to the original intent. Inputs, outputs, environment, and detailed
success/failure effects remain outside this first elaboration boundary unless a
later event proves they are necessary.

Status: Resolved

## Decision record

- Confirmed by the user: command text plus purpose is the minimum stage
  information.

## Q24. Highest-tier escalation

What should the review surface do when the current candidate already uses the
highest configured model tier and the developer requests a higher tier?

Confirmed decision: If the current candidate already uses the highest
configured tier, a higher-tier request opens explicit model selection rather
than silently regenerating at the same tier.

Status: Resolved

## Decision record

- User selected explicit model escalation.

## Q25. Explicit model selection

How should the review surface let the developer choose the explicit model for
the next candidate?

Confirmed decision: Use the existing provider catalog picker for explicit model
selection. Apply the selected model only to the next candidate.

Status: Resolved

## Decision record

- User selected the catalog picker.

## Q26. Candidate comparison metadata

When candidates from different tiers or explicit models are retained for
comparison, what model context should the review surface show with each
candidate?

Confirmed decision: Show the configured tier when applicable and the concrete
provider/model identifier beside each retained candidate.

Status: Resolved

## Decision record

- User selected tier and concrete model metadata.

## Q27. Intent visibility

How should the review surface represent the original intent while a candidate
is directly edited or while multiple candidates are compared?

Confirmed decision: Keep the original intent in a collapsible context panel
that remains available while candidates are edited or compared.

Status: Resolved

## Decision record

- User selected a persistent collapsible context panel for the original intent.

## Q28. Rephrased intent visibility

When the developer explicitly rephrases the intent, should the context panel
show both the original intent and the current rephrased intent?

Confirmed decision: A rephrased intent replaces the visible active intent in
the context panel. The prior intent remains internal current-review history.
This differs from direct command editing, which preserves the original intent.

Status: Resolved

## Decision record

- User selected replacement of the visible intent after explicit rephrasing.

## Q29. Review interruption

If the developer interrupts a new generation, explanation request, or model
selection while review is active, should the current selected candidate remain
available for acceptance?

Confirmed decision: Interrupting generation, explanation, or model selection
cancels only the in-progress operation and preserves the current selected
candidate and review state.

Status: Resolved

## Decision record

- User selected preservation of the current candidate on interruption.

## Q30. Presentation fallback

What portable presentation should be the required fallback when a Kitty overlay
is unavailable or fails?

Options discussed:

- Alternate-screen TUI: portable baseline for iTerm2, Alacritty, Kitty, SSH,
  and multiplexers; supports the complete review interaction without an
  emulator-specific result channel.
- Existing tmux/zellij pane: cross-terminal adjacent review surface when a
  multiplexer is already present; should remain optional.
- Alacritty new window: possible through Alacritty IPC when enabled, but it is a
  separate window and requires explicit result transport.
- iTerm2 new tab/window: possible through macOS/iTerm automation, but it is
  platform-specific and permission-sensitive.
- Inline redraw above the prompt: visually close to the request but fragile
  across Readline, ZLE, Fish, multiline prompts, resizing, and multiplexers.
- Graphics protocols: possible as an enhancement, not a portable baseline.

Recommended hierarchy: a small transient inline ANSI panel on the controlling
terminal as the required portable baseline. Kitty overlay and an optional
tmux/zellij pane may enhance presentation. Alacritty and iTerm2 new-window
integrations remain later enhancements.

Universal terminal capability identified: a child process can render a bounded
interactive UI through ordinary ANSI/ECMA-48 control sequences on the
controlling terminal. The portable form is a small transient inline panel with
raw keypress input, cursor movement, redraw, and timer animation. The UI must use
the controlling terminal (`/dev/tty` or an equivalent terminal descriptor),
while stdout remains reserved for the accepted command because the Ctrl-W
widget captures stdout.

This provides a portable menu and animated review surface in Alacritty, iTerm2,
Kitty, SSH sessions, and multiplexers without requiring a native overlay API.
It is a transient inline region, not a universal floating overlay. The shell
integration must preserve and repaint the current line because prompt wrapping,
Readline, ZLE, Fish, resize, and multiplexer behavior differ.

Confirmed decision: Use a small transient inline ANSI panel on the controlling
terminal as the required portable baseline. The panel must not switch to or
occupy the full alternate screen. Kitty overlays and opted-in tmux/zellij panes
may enhance presentation, but the review behavior must work without them.
Alacritty and iTerm2 native windows remain later enhancements.

Status: Resolved

## Decision record

- The user's requirement for a small non-intrusive menu/UI that works across
  modern terminal emulators establishes a transient inline panel as the
  portable-first hierarchy. Full-screen alternate-screen UI is rejected.

## Q31. Review timing

When generation begins, should the transient inline panel open immediately with
animated progress and elapsed time, before a candidate exists, or should it open
only after the first candidate has been generated?

Working UI concept discussed with the user: the portable review surface is a
small transient inline panel, not a full-screen viewport. A typical panel has a
compact status line with Watn state, model/tier, and elapsed time; a short
command-flow view; the selected stage's exact command text and model-written
purpose; and a one-line action menu. Arrow keys can navigate stages, candidates,
and menus. Tab and Shift-Tab can move focus between compact regions, with Enter
activating the focused action and Escape cancelling. The panel disappears when
the review ends and the shell widget repaints the original or accepted buffer.

Confirmed timing/presentation decision: Keep the existing progress line as the
universal generation feedback. The explanatory review panel is an optional
enhancement, enabled by default, that appears after the progress phase when a
candidate is ready. It can be disabled, and a terminal-specific renderer such
as a Kitty presentation can replace the default panel. The progress line
remains reusable across all renderers.

Status: Resolved

## Decision record

- User selected the existing progress line first, with an optional default-on
  panel enhancement and renderer replacement support.

## Q32. Panel renderer selection

How should a developer disable the default panel or select a Kitty-specific
renderer?

Confirmed decision: Persist the panel default in Watn configuration, allow a
per-invocation CLI override, and auto-select enhanced renderers when available.

Status: Resolved

## Decision record

- User selected CLI, config, and automatic renderer selection.

## Q33. Disabled-panel behavior

When the panel is disabled by configuration or a per-invocation override,
should Ctrl-W retain the existing direct replacement behavior without command
flow explanation?

Confirmed decision: When the panel is disabled, Ctrl-W retains the existing
direct replacement behavior and skips command-flow review.

Status: Resolved

## Decision record

- User selected compatibility with the existing direct replacement behavior.

## Q34. Compact flow density

When the compact panel is too narrow or short to show the entire command flow,
should it show all stages in a horizontally or vertically compressed view, or
focus one selected stage at a time?

Confirmed decision: Use an adaptive layout. Show all stages when they fit;
otherwise show a compact overview plus one fully readable selected stage, with
arrow navigation between stages.

Status: Resolved

## Decision record

- User selected adaptive command-flow density.

## Q35. Primary action

When the explanatory panel opens with a candidate ready, should final acceptance
be the default focused action, or should the panel focus the command-flow view
first?

Confirmed decision: Final acceptance is the initial focused action when a
candidate is ready. The command-flow view remains visible and navigable before
the developer commits.

Status: Resolved

## Decision record

- User selected acceptance as the initial focus.

## Q36. Panel dismissal

After final acceptance or cancellation, should the transient panel disappear
immediately and restore the shell prompt, or remain briefly as a completion
status before disappearing?

Confirmed decision: Remove the transient panel immediately after final
acceptance or cancellation and restore the shell prompt without a lingering
status view.

Status: Resolved

## Decision record

- User selected immediate panel dismissal and shell restoration.

## Q37. Direct-path default

Should the default-on explanatory panel apply to direct positional questions,
interactive stdin, and eligible `-x` paths as well as Ctrl-W, or should those
paths require a separate opt-in?

Confirmed decision: Apply the default-on explanatory panel to Ctrl-W, direct
positional questions, interactive stdin, and eligible `-x` paths. The persisted
setting and per-invocation override apply consistently across all eligible
paths.

Status: Resolved

## Decision record

- User selected the panel for all eligible interactive paths.

## Q38. Direct-path accepted output

After final acceptance on a direct positional or interactive-stdin path, should
the panel return only the selected command through the normal command output
channel, with no flow or explanation text emitted there?

Confirmed decision: For Ctrl-W, restore the shell prompt with the accepted
candidate in the buffer and record the original natural-language prompt in shell
history as a history comment, matching the existing shortcut behavior. The
candidate is not executed by review acceptance.

Status: Resolved

## Q39. Direct output contract

For direct positional and interactive-stdin paths, should final acceptance emit
the selected command through the existing normal output channel while keeping
the explanatory panel content on the controlling terminal?

Confirmed decision: For direct positional and interactive-stdin paths, preserve
the existing command-output contract: emit the accepted candidate through the
normal output channel and keep the review panel's flow and explanation text on
the controlling terminal. No shell history or prompt replacement is involved.

Status: Resolved

## Decision record

- User reiterated that Ctrl-W records the original prompt in history and replaces
  the prompt with the accepted candidate, matching today.
- Direct paths retain their existing output shape rather than adopting shell
  prompt behavior.

## Q40. Disabled direct paths

When the explanatory panel is disabled for a direct positional or
interactive-stdin path, should Watn return the generated command through the
existing output path immediately? When disabled for `-x`, should it retain the
existing `Execute now?` confirmation?

Confirmed decision: When the panel is disabled, direct positional and
interactive-stdin paths preserve existing command output, and `-x` preserves
the existing `Execute now?` confirmation.

Status: Resolved

## Decision record

- User selected preservation of existing behavior for every disabled direct
  path.

## Q41. Keyboard contract

What should the compact panel's focus model be for stage navigation, candidate
comparison, actions, and direct command editing?

Confirmed decision: Use three focus regions. Tab cycles `Flow`, `Candidates`,
and `Actions`; arrow keys navigate within the focused region; Enter activates;
Escape cancels review. Direct command editing opens a separate editor where
Enter commits and Escape discards the edit.

Status: Resolved

## Decision record

- User selected the three-region focus model.

## Q42. Renderer fallback

If a configured or automatically selected enhanced renderer is unavailable or
fails after selection, should Watn fall back to the portable inline panel or
abort the review while preserving the original input state?

Confirmed decision: If an automatically selected or configured enhanced
renderer is unavailable or fails, fall back to the portable inline panel and
preserve the review state. Abort safely only if the portable panel also cannot
open.

Status: Resolved

## Decision record

- User selected inline fallback for enhanced-renderer failure.

## Q43. Explanation loading

After a candidate is generated but its model-written stage purposes are still
loading, should the inline panel open immediately with an explanation-loading
state, or wait until purposes are available?

Confirmed decision: Open the inline panel as soon as a candidate exists. Show
stage purposes as loading while they are unavailable, then update the panel when
they arrive. Explanation failure leaves the candidate reviewable.

Status: Resolved

## Decision record

- User selected immediate panel opening while stage purposes load.
