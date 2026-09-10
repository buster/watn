# Use Case Definitions: terminal-wow-factor

## Stable ownership

The permanent user-goal use case remains `use-shell`:

> Use Watn as a native shell tool with completions and an interactive shortcut.

The topic extends that goal with an explanatory review capability. Direct
interactive positional questions, interactive stdin, and eligible `-x` consume
the shared review interaction but do not become new permanent user goals.

## Candidate use cases

| ID | Level | Boundary | Value outcome | Affected stakeholder |
|---|---:|---|---|---|
| `review-command-candidate` | `-` | Shared review interaction | Examine a generated candidate and its command flow, refine intent or command, compare candidates, and make an explicit final decision | Terminal developer |
| `use-explanatory-shell-shortcut` | `!` | `use-shell` interactive shortcut extension | Press Ctrl-W, review the candidate explanation, and replace the shell buffer only after explicit acceptance without implicit evaluation | Terminal developer |
| `route-reviewed-command` | `-` | Consumer integration | Deliver an accepted candidate to a direct interactive caller or active `-x` execution boundary | Terminal developer |

## Cut rationale

- `review-command-candidate` is a shared subfunction because the same review
  lifecycle is consumed by Ctrl-W, direct interactive questions, and `-x`.
- `use-explanatory-shell-shortcut` is the change-sized user goal under the
  existing `use-shell` owner because Ctrl-W is the defining user interaction and
  shell-buffer replacement is its unique value boundary.
- `route-reviewed-command` remains a subfunction because delivery differs by
  consumer but does not represent an independent user goal.

## Out of scope for separate use cases

- Semantic command-risk validation.
- Provider adapter implementation and model quality.
- Kitty transport, portable fallback mechanics, and result transport protocol.
- Non-TTY pipe behavior, which remains the existing raw/script-safe contract.
- General onboarding and setup.

## Definition status

Candidate cut confirmed by the user. `use-explanatory-shell-shortcut` is the
single change-sized user goal; the other entries remain subfunctions.
