# Frame: terminal-wow-factor

## Problem

Watn already turns plain-language requests into shell commands, but the
wow-factor opportunity is not yet defined as a user-goal contract. The confirmed
desired outcome is that a terminal developer experiences natural-language
intent visibly becoming safe, understandable terminal action.

## Vision

Watn makes the transition from intent to terminal action feel immediate,
legible, and safe enough to trust without leaving the terminal.

## Existing context

- Watn generates shell commands from positional questions or stdin.
- Generated content streams incrementally to stdout.
- Optional execution requires explicit confirmation.
- The Ctrl-W integration inserts generated text without evaluating it.
- Existing permanent use cases cover configuration, model/provider setup, shell
  completions, and the interactive shortcut.
- No permanent use case currently owns the broader experience of intent
  becoming understandable, safe terminal action.

## Candidate stakeholder types from the durable architecture

These are research findings, not confirmed topic stakeholders or personas:

- Developer / end user: asks for a command from a terminal.
- Power user: needs control over providers, models, and command behavior.
- CI / scripted user: needs clean output, exit codes, and pipe safety.
- Shell user: works through the line editor and shell integration.

## Confirmed stakeholder

- Terminal developer: owns the wow-factor problem and evaluates whether Watn
  makes interactive terminal work more trustworthy and effective.

## Scope boundary

The topic concerns the user-visible transition from an expressed terminal
intent to a trustworthy next action in an interactive terminal context.

- The explanatory review surface is the primary value boundary.
- It visualizes the generated command's pipeline and control-flow structure.
- It supports accept, edit, reject, and cancel decisions.
- It supports rephrasing the original intent and requesting a higher-tier model.
- Replacing a shell buffer requires explicit acceptance.
- No review path executes a command implicitly.
- Proposal replacement is the default after rephrasing or model escalation;
  retaining earlier proposals for comparison is explicit and user-selectable.
- Kitty overlay is an enhanced presentation when available; portable fallbacks
  remain required.
- Positional questions, interactive stdin, `-x`, and Ctrl-W are in scope.
- Non-TTY piped stdin keeps the existing raw, script-safe output contract.

`use-shell` remains the permanent user-goal owner for the Ctrl-W review surface
and shell-buffer replacement. Existing asking/execution behavior consumes the
same explanatory review interaction for positional questions, interactive
stdin, and `-x`; no new permanent user-goal use case is created. Provider
implementation, model quality, and general setup remain supporting concerns
unless Event Storming proves that they are part of the value-producing
boundary.

## Frame status

The problem framing, vision, primary stakeholder type, and interactive terminal
developer persona are confirmed. The Frame boundary review is complete. Safety,
action, lifecycle, ownership, entry-surface, pipe-behavior, and review
eligibility decisions are confirmed. Decision transport mechanics are parked
for Design under the invariant recorded in `questions.md`. No additional
stakeholder or persona has been confirmed.
