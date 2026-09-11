# Proposal: quicksetup-parameters

## Use-Case Context

- Use-Case ID: configure-interactive
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

`watn quicksetup` only works as a fully interactive question flow. A user who
already knows the endpoint, credential, and model must still type every answer
or accept the defaults, which makes scripted or documented setups needlessly
slow and error-prone. The README shows the interactive command but no complete
one-line setup, so a new user cannot copy a working example.

## Proposed Solution

`watn quicksetup` accepts optional parameters that prefill the dialog instead of
replacing it:

- `--url <URL>` prefills the completion endpoint.
- `--key <KEY>` prefills the credential; a `${ENV_VAR}` reference (written in
  single quotes by the caller) is stored as an environment-backed credential.
- `--model <MODEL>` prefills the small, normal, and thinking tiers.
- `--model-small`, `--model-normal`, and `--model-thinking` prefill one tier and
  take precedence over `--model` for that tier.

The dialog still runs: every prefilled value appears as the suggestion, an
empty answer accepts it, and the shell-integration question is always asked, so
the user still decides what gets installed. Without parameters, the flow is
unchanged.

The README Quick setup section gains the install command and two complete
examples plus a test query:

```sh
cargo install watn
watn quicksetup --url https://openrouter.ai/api/v1 --key '${OPENROUTER_API_KEY}' --model '~anthropic/claude-haiku-latest:nitro'
watn quicksetup --url https://openrouter.ai/api/v1 --key 'sk-abc123' --model-small 'google/gemini-3.7-flash'
watn "find the 5 largest files in the commit history"
```

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `quicksetup` | `EXTEND quicksetup` | `EXTEND quicksetup` | The parameters prefill the existing quick setup capability. |

## Out of Scope

- No new provider, network, or catalog behavior.
- No new flags for `watn setup`, `watn provider`, `watn models`, or `watn shell`.
- No non-interactive mode: the dialog always runs.

## Open Questions

- None.
