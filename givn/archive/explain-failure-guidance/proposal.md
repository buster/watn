# Proposal: explain-failure-guidance

## Use-Case Context

- Use-case ID: use-shell
- Ideation topic: terminal-wow-factor
- Confirmed Personas: terminal-developer--interactive

## Problem / Opportunity

`watn explain '<command>'` explains a command the developer already has. When
no usable model is configured — a credential reference such as
`${OPENROUTER_API_KEY}` whose environment variable is unset, a missing model, or
no configuration at all — the command silently opens the explanation card with
`purpose-unavailable`. Nothing tells the developer that watn never reached a
model, and nothing offers a way to fix it. The card looks the same whether the
model answered badly or was never asked.

A failed explanation request is equally silent: an HTTP, authentication, or
network failure is swallowed and the same `purpose-unavailable` card appears.
The developer cannot distinguish "no model-written purposes exist" from "watn
could not ask the model".

The plain question path already sets the expected behaviour: when no usable
provider or model is available and no provider or model was explicitly
selected, watn starts setup — quick setup on a machine without a configuration,
the setup wizard when a configuration exists — or prints actionable setup
guidance when the invocation cannot ask questions. The explanation entry point
should follow the same rule instead of degrading without a word.

## Proposed Solution

`watn explain` reports or fixes an unusable model instead of degrading
silently:

- When no usable provider or model is configured and no provider or model was
  explicitly selected, `watn explain` does not open the explanation card. On a
  machine without a configuration it starts quick setup; when a configuration
  exists it starts the setup wizard; when the command input is not a terminal
  it prints actionable setup guidance and exits non-zero.
- After a completed setup the command reports that setup is complete and that
  the explanation must be run again. The original command is not explained in
  the same invocation, exactly like the question path.
- An explicitly selected provider or model (`--provider`, `--model`,
  `WATN_PROVIDER`) never starts setup. Its unknown-provider, missing-model, or
  unusable-credential error is reported with the usual exit status, exactly
  like the question path.
- When a usable model is configured but the explanation request fails (HTTP,
  authentication, network), the explanation card still opens with the
  developer's command and `purpose-unavailable`, and the failure is reported on
  stderr with the mapped non-zero exit status after the card closes.
- When the provider answers but the answer is not usable as an explanation, the
  card opens with `purpose-unavailable` and a distinct message names the
  unusable response; the invocation still succeeds.
- The explained command is never generated, replaced, evaluated, or executed
  on any of these paths.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| explain-command | `EXTEND quicksetup` (use case `configure-interactive`, score 4.00) | `EXTEND explain-command` | The route's ranking is driven by shared setup and credential words in the proposal scaffold, not by domain ownership. The changed behaviour belongs to the existing `explain-command` capability owned by `use-shell`: it changes when `watn explain` opens its card and what it reports. Quick setup, the setup wizard, and provider readiness are reused unchanged and keep their `configure-interactive` ownership. |

## Out of Scope

- Model-written purposes on the `-x` execution prompt's Explain choice; that
  path keeps its current best-effort behaviour and never starts setup.
- Provider and model configuration behaviour itself; the setup surfaces are
  reused, not changed.
- Resuming the original explanation after setup; like the question path, the
  developer reruns the command.
- Non-terminal explanation cards.
- Semantic command-risk validation.
- Natural-language question behaviour.

## Open Questions

None. The failure-mode behaviour, the setup delegation, the explicit-selection
errors, and the exit statuses are fixed above and detailed in the design.
