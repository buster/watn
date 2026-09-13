# Proposal: explain-command

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: terminal-wow-factor
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

Developers regularly meet a long, complex command they did not write: from a
runbook, a colleague's message, shell history, or a log line. Such commands mix
pipes (`|`), success and failure branches (`&&`, `||`), sequencing (`;`),
redirections (`<`, `>`), globs (`[]`), substitutions (`$`, backticks), and
several kinds of quoting. Understanding them means reading very carefully or
leaving the terminal for a search engine. Running them to see what happens is
the unsafe option.

Watn already owns the explanatory review surface: a command is shown as a stack
of stages with model-written purposes. But it only explains commands it
generated itself, and only through the `-x` execution prompt's Explain choice.
That path never asks the model for stage purposes, and there is no way to hand
watn an existing command and get the explanatory card.

Handing over such a command safely is its own problem. The shell consumes `&`,
`;`, `|`, `[]`, `<>`, `$`, and backticks before any program starts. The
developer needs a small, documented set of invocation forms that deliver the
command to watn exactly as written, without it being expanded, re-split, or
executed.

## Proposed Solution

Add `watn explain` — a command that opens the explanatory review card for a
command the developer already has.

Observable behaviour:

- `watn explain '<command>'` shows the review card for exactly that command in
  explanation-only mode: the command's stages and, when a model is reachable, a
  model-written purpose for every stage. The card closes on Enter or Escape and
  offers no accept, edit, reject, or regenerate decisions.
- The explained command is the command the developer passed. Watn never
  regenerates, edits, evaluates, re-splits, or executes it, and writes no
  command to standard output.
- The command travels as one argument. The canonical form is a single-quoted
  argument; `--` precedes a command that begins with `-`; the argument `-` reads
  the command from standard input, and piped standard input is used when no
  argument is given. A positional argument takes precedence over standard input.
  Empty input is a usage error and no card opens.
- When no model is configured or the model call fails, the card still opens with
  the command's locally derived stages and shows `purpose-unavailable`; the
  command display is never lost.
- `watn explain` never executes a command; combining it with `-x` is an error
  rather than a silent no-op.

The shell performs quoting before watn starts, so watn receives the command as
literal argument bytes; its guarantee begins at that argument. Expansion of
`$`, backticks, and `$(...)` happens in the developer's own shell unless the
command is single-quoted or passed through a quoted heredoc. The design records
the worked invocation examples for each special-character case.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| explain-command | `EXTEND interactive-shell-shortcut` (score 12) | `NEW in use-shell` | The route's top match is driven by shell metacharacters (`|`, `&&`, `;`) in the shortcut capability's command examples, not by domain meaning. Explaining an existing command has a distinct trigger (a direct `watn explain` invocation), a distinct consumer action (hand over a command the developer already has), and a read-only lifecycle with no generation, editing, acceptance, or shell-buffer replacement. The owner stays `use-shell` because the review surface and the terminal-developer persona are already owned there; a separate use case would fragment that contract. |

## Out of Scope

- Executing an explained command or offering it to a shell buffer.
- Editing, accepting, rejecting, or regenerating an explained command; the card
  is read-only.
- Model-written purposes on the existing `-x` explanation card; that path keeps
  its current behaviour.
- Semantic command-risk verdicts.
- Non-terminal explanation surfaces and persistent explanation history.
- Natural-language question behaviour, except that `explain` becomes a reserved
  first token (like `completions`); a question beginning with `explain` must be
  quoted or passed after `--`.

## Open Questions

None. The argument-passing contract, the purpose-fetching fallback, and the
provider-readiness behaviour are fixed above and detailed in the design.
