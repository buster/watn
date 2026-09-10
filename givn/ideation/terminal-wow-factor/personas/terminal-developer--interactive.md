# Persona: terminal-developer--interactive

## Role

Developer working interactively in a terminal and using Watn for commands that
are not yet fully known or remembered.

## Interests

- Preserve control over what reaches the shell.
- Understand enough of a generated command to trust or reject it.
- Move from intent to useful action without breaking concentration or changing
  tools.
- Keep the terminal history useful for later inspection and repetition.

## Goals

- Express an operational or development intent in ordinary language.
- Receive a concrete next action quickly.
- Distinguish harmless inspection from commands with meaningful side effects.
- Correct or refine the action without losing the original intent.

## Pain points

- Shell syntax and flags are easy to misremember.
- A generated command can be technically valid while still being surprising,
  overly broad, or unsafe.
- Explanations that require leaving the terminal interrupt the work context.
- A command preview alone does not always make its effect obvious.

## Success criterion

The developer can express intent, understand the proposed terminal action,
decide whether it is safe, and continue with the intended work without leaving
the terminal or surrendering control to an opaque automation step.

## Vocabulary

- Intent: the task expressed in ordinary language.
- Proposed action: the command or terminal action Watn presents in response.
- Trust: enough understanding and control to decide whether to accept the
  proposed action.
- Terminal context: the current shell session, prompt, working directory, and
  visible command history relevant to the request.

## Decision record

- Candidate created from the confirmed `terminal developer` stakeholder type.
- Event Storming reaction recorded: the chain preserves control, supports
  refining intent without losing it, enables comparison and explicit
  acceptance, and makes unavailable or unsupported explanation state visible.
- Persona interpretation recorded: “safe” in this topic means retained control
  and informed acceptance, not semantic validation of command risk.

## Open demands

- Direct command editing must never silently alter the original intent.
- Cancellation or failure must preserve the original shell/input state.

## Confirmation status

Confirmed by the user as the review lens for this topic.
