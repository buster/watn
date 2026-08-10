# ADR-0014: Independent catalog source and provider confirmation

- **Status:** accepted
- **Date:** 2026-08-10
- **Decision-makers:** architect

## Context and Problem Statement

Model discovery currently derives its endpoint from the active provider even
when a LiteLLM catalog is configured. The setup flow also waits until complete
model selection before saving the provider, so a catalog failure can lose a
credential that the user already confirmed. Resolving credentials too early can
also replace an environment-backed source with its secret value.

## Decision Drivers

- LiteLLM must remain discovery-only and must never receive chat completions.
- List, pagination, and search must use one exact catalog source and credential
  policy.
- Optional LiteLLM authentication must omit Authorization when absent.
- Literal credentials and exact environment references must remain authoritative.
- A valid provider must survive catalog failure or post-confirmation
  cancellation without changing unconfirmed tiers.
- The smallest change should preserve the blocking HTTP and shared wizard
  architecture.

## Considered Options

- **Keep using the active provider for all requests:** minimal code, but ignores
  configured LiteLLM and couples catalog and chat behavior.
- **Resolve and persist every credential as a literal:** simple request code,
  but leaks secrets into TOML and violates environment-backed configuration.
- **Resolve an explicit runtime catalog source and save the provider at
  credential confirmation:** keeps endpoint and credential policies separate,
  preserves source representations, and creates a clear partial-save boundary.

## Decision Outcome

Chosen: resolve a runtime catalog source with LiteLLM precedence and an optional
raw credential source. Expand that source only when a catalog request is built.
Keep active-provider resolution separate for chat. In the shared wizard, validate
and resolve the credential when the user confirms the API-key page, save the
provider draft at that point, then perform the first catalog request. Model-only
entry points return no provider replacement and persist tiers separately.

## Consequences

- **Good:** configured LiteLLM is consumed for all discovery operations without
  changing chat routing.
- **Good:** environment references remain references and missing explicit
  variables fail without fallback or a request.
- **Good:** catalog failure after confirmation leaves a usable provider and old
  tiers intact.
- **Good:** exact endpoint and Authorization assertions can distinguish source
  crossover from a generic successful mock.
- **Bad:** catalog and provider resolution have separate runtime paths that must
  be kept consistent for fallback behavior.
- **Bad:** a provider can be saved without model tiers after catalog failure;
  the UI must report the failure and make rerunning model setup clear.
- **Bad:** direct config writes remain non-atomic, so a process interruption can
  still leave a partial file.

## Confirmation

The change features drive TTY and subprocess setup flows, assert the raw
credential reference and absence of its secret, exercise exact LiteLLM/provider
request twins, and verify provider persistence after catalog failure.
