# Proposal: migrate-usecases-watn

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: none; this is an aggregate migration
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

Watn's active specification corpus still uses group directories and
`group.md` narratives. The support collection is reusable infrastructure rather
than an independent user goal, while setup, provider, model, and shell behavior
each have distinct user-facing goals. The current layout does not make those
boundaries or capability ownership machine-checkable.

## Proposed Solution

Migrate the active corpus to explicit use-case documents and a fragment
collection without changing scenario behavior. Preserve stable scenario
identity, all E2E evidence, and interaction coverage. Keep historical archive
content untouched.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| corpus-infra capabilities | NEW in fragments | NEW in fragments | Reusable infrastructure has no independent user goal. |
| setup capabilities | NEW in configure-interactive | NEW in configure-interactive | Guided setup is a user-goal use case. |
| provider capabilities | NEW in configure-provider | NEW in configure-provider | Provider configuration is an independent goal. |
| model capabilities | NEW in configure-model | NEW in configure-model | Model selection and reasoning are an independent goal. |
| shell capabilities | NEW in use-shell | NEW in use-shell | Shell integration is an independent goal. |

## Out of Scope

No scenario text, step definition, or product implementation is changed. No
ideation state exists in Watn, so no Persona or Handoff is promoted.

## Open Questions

None for the current clean Watn corpus.
