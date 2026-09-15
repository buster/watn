# Proposal: robust-review-response-recovery

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: none
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

In review mode the provider is asked for one structured review response. In
practice the response frequently cannot be read: it contains literal line
breaks or tabs inside string values, unescaped quotation marks, or it is cut
off before the closing brace. When that happens the review surface stops
showing a command at all — it shows the raw provider payload as if it were the
command. The developer sees JSON punctuation where a command should be, cannot
judge or edit it, and the only explanation is the status `purpose-unavailable`,
which does not say what went wrong.

The failure is also invisible to a bug report: the Ctrl-W path names no reason
and keeps no copy of what the provider sent, so "the model answered badly" and
"watn misread a usable answer" cannot be told apart after the fact.

## Proposed Solution

When the review surface is enabled and eligible:

- A provider response that is shaped like a structured review response but is
  not strictly valid is never shown as the command. Watn recovers the command
  it contains whenever the command can be delimited unambiguously and keeps it
  reviewable with `purpose-unavailable`; Stage purposes are adopted only under
  the existing exact-match rules. Watn never invents a command or a Stage
  purpose, and nothing is released without explicit acceptance.
  - A response whose string values contain literal line breaks, carriage
    returns, or tabs is read like a valid response.
  - A response that is cut off after a complete command still shows that
    command with `purpose-unavailable` and names that the response was
    incomplete.
  - A response from which no command can be recovered releases nothing and
    preserves the original input.
- The review surface names why the Stage purposes are unavailable in plain
  language instead of only the status word: for example "response was
  incomplete", "response was not valid JSON", or "stages did not match the
  command". The name is advisory; it never changes what can be accepted.
- `-v`/`--verbose` prints the raw provider response on stderr after the review
  surface closes, for both the review path and `watn explain`.
- Every unusable provider response is additionally written to
  `$XDG_STATE_HOME/watn/last-unusable-response.txt` (default
  `~/.local/state/watn/last-unusable-response.txt`), overwritten per
  invocation, so a bug report can attach it without `-v`. Watn names the path
  on stderr after the review surface closes.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | `EXTEND interactive-shell-shortcut` | The failure and the reason display live in the Ctrl-W/direct review surface; recovery extends the existing review-response contract. |
| `explain-command` | `EXTEND explain-command` | `EXTEND explain-command` | The Explained command path shares the same response reading and gains the same raw-response capture. |

## Out of Scope

- No automatic retry or repair request to the provider when a response is
  unusable.
- No change to the structured review response contract itself
  (`review_version` stays 1).
- No semantic command-risk validation.
- No history of unusable responses; the state file holds the most recent one
  only.
- No new review decision or key; recovery changes what the existing surface
  shows, not how it is operated.
- Disabled, non-review, and non-terminal behavior is unchanged.

## Open Questions

- None. The raw-response capture is decided: `-v` on stderr plus one
  overwritten state file for unusable responses.
