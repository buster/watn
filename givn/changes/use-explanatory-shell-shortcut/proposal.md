# Proposal: use-explanatory-shell-shortcut

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: use-explanatory-shell-shortcut
- Ideation topic: terminal-wow-factor
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Watn's Ctrl-W shortcut currently replaces the shell buffer with a generated
command, but the developer must judge a complex pipeline from the command text
alone. This makes chained commands, pipes, `xargs`, redirections, and success
branches difficult to understand without leaving the terminal. The shortcut
also offers no focused way to refine the command, rephrase the intent, compare
model candidates, or ask for a stronger model before returning a command.

The missing experience is a visible transition from intent to understandable
terminal action that preserves the developer's control and existing shell
history behavior.

## Proposed Solution

When the developer invokes an enabled Watn shortcut in an eligible interactive
terminal, Watn first keeps the existing progress line, then opens a small
transient review surface after a complete candidate is available. The review
surface shows the candidate's command flow, exact stage text, model-written
stage purposes, and the candidate's tier and provider/model context.

The developer can navigate the Flow, Candidates, and Actions regions, inspect
the selected stage, edit the command, rephrase the intent, regenerate, request
a higher tier, choose an explicit model from the provider catalog, retain
candidates for comparison, reject a candidate, or cancel. Direct command edits
preserve the original intent and refresh the command-flow and purpose state.
The review surface opens with purposes loading only when the provider returned
the structured review response needed for a delayed purpose operation. A
command-only or invalid structured response shows purpose-unavailable instead;
it never substitutes model-written purposes with locally invented text. The
review remains usable when purpose generation or flow derivation is unavailable;
unsupported portions are shown visibly.

Every candidate requires explicit final acceptance. The complete candidate is
buffered until the provider's `[DONE]` marker before review, and no candidate is
released before final acceptance. On Ctrl-W acceptance, Watn records the
original prompt in shell history and replaces the shell buffer with the
accepted candidate without executing it. On direct positional and interactive
stdin paths, the existing command-output contract remains unchanged when a
candidate is accepted. On eligible `-x`, the developer's explicit `-x` choice
plus final acceptance is the sole execution authorization; disabled, redirected,
or otherwise non-review `-x` paths keep their existing confirmation.

The review surface is a small transient inline terminal interaction, enabled by
default, configurable persistently, overridable per invocation, and rendered by
an automatically selected presentation adapter. Enhanced presentation-adapter
failure falls back to the portable inline panel. Portable-panel failure
preserves the original input and releases no candidate. Disabling the panel
preserves the existing behavior for Ctrl-W, direct positional input, interactive
stdin, and `-x`.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND quicksetup` (advisory misroute) | `EXTEND interactive-shell-shortcut` in `use-shell` | The behavior is a shell-shortcut interaction, not setup or onboarding. |
| Shared review interaction for direct interactive and `-x` consumers | No direct capability recommendation; ranked setup capabilities are irrelevant | `EXTEND interactive-shell-shortcut` in `use-shell`; keep consumers as interactions | The confirmed permanent owner is `use-shell`; no new user-goal root or review capability is created. |

## Out of Scope

- No full-screen alternate-screen interface.
- No semantic command-risk validation or claim that a command is safe because it
  was explained.
- No change to non-TTY or redirected output behavior.
- No change to disabled-review-surface Ctrl-W, direct-output, or non-review `-x` behavior.
- No persistent candidate history across reviews or sessions.
- No provider adapter, model-quality, parser-grammar, or terminal-specific
  renderer implementation decision in this proposal.
- No shell-completions behavior change or interaction inventory entry.

## Confirmed Design Boundaries

- The command-output channel is stdout and carries only the accepted candidate
  on a review-eligible accepted direct path; review-surface bytes use the
  controlling-terminal channel and never stdout.
- The provider response contract is structured in review mode. The design
  specifies the adaptive response and its purpose-unavailable fallback.
- The portable inline panel is mandatory; enhanced presentation adapters are
  optional and failure-safe.
- Exact command-flow grammar remains conservative; unsupported portions remain
  visible and reviewable.
- The permanent owner is `use-shell`, and direct positional, interactive stdin,
  and eligible `-x` consumers remain interactions of that capability.
