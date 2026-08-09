# Proposal: watn-provider

## Problem / Opportunity

Watn can use OpenAI-compatible providers, but a new user must understand the
configuration file and credential rules before the first request can work. The
tool does not currently provide a guided way to choose an endpoint or decide
how the provider credential should be supplied. This makes first-run setup
especially difficult when the user already has a credential in their shell
environment.

The model setup depends on a provider being available. Reaching model setup
before a provider is configured leaves the user with an incomplete and
surprising setup path instead of a working watn installation.

## Proposed Solution

Add an interactive `watn provider` command that guides the user through
provider setup in the terminal:

- The setup asks for an OpenAI-compatible endpoint and offers OpenRouter's
  endpoint as the default. The user can accept that default or provide another
  endpoint.
- The setup asks how the provider credential should be supplied. The user can
  paste a credential, which is saved in the configuration, or choose an
  environment variable, which is saved as a reference such as
  `${OPENROUTER_API_KEY}` rather than copying the secret into the
  configuration.
- OpenRouter uses `OPENROUTER_API_KEY` as its suggested environment variable.
  `WATN_API_KEY` is the suggested generic variable for another provider, and
  the user can name a different environment variable when needed.
- The selected provider, endpoint, and credential representation persist in
  the standard watn configuration and are used by subsequent commands. An
  environment-backed credential is resolved from the environment when watn
  makes a request.

When watn starts a normal operation without a provider recognized from either
the configuration or a supported environment variable, it automatically runs
the provider setup. After provider setup succeeds, it automatically starts
model setup so the user can select the models for the three existing tiers.
The automatic flow does not run when a provider is already recognized.

## Out of Scope

- Support for APIs that are not OpenAI-compatible.
- Changes to the model catalogue, model filtering, tier selection, or
  per-level reasoning behavior. Those remain governed by the existing model
  setup.
- Encryption, rotation, or remote management of saved credentials.
- Changes to request generation, streaming, or command execution after a
  provider and models are configured.

## Open Questions

None. OpenRouter's endpoint and `OPENROUTER_API_KEY` are the defaults; other
providers use `WATN_API_KEY` as the generic suggestion while permitting an
explicit environment variable name. A pasted credential is stored literally,
and an environment-backed credential is stored as `${VARIABLE}` and resolved
when used.
