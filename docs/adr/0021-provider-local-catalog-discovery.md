# ADR-0021: Provider-local catalog discovery

- **Status:** accepted
- **Date:** 2026-08-13
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

Model setup previously allowed an independent LiteLLM catalog source. The
streamlined setup flow needs one unambiguous provider, credential, and catalog
boundary so the models displayed to the user correspond to the selected
provider.

## Decision Drivers

- Catalog and chat credentials must not silently diverge.
- Setup must probe the endpoint the selected provider owns.
- Existing legacy configuration must remain readable during migration.

## Considered Options

- **Retain LiteLLM precedence** - keeps independent discovery but hides the
  provider/catalog relationship.
- **Use provider-local discovery** - derives or reuses a provider catalog base
  and uses the provider credential.

## Decision Outcome

`watn setup` and `watn models` use the selected provider's saved or derived
catalog endpoint for list, pagination, and search. The legacy `[litellm]`
section is preserved as unrelated configuration but is not contacted, migrated,
or used as a fallback.

## Consequences

### Good

- Catalog requests have one observable endpoint and credential source.
- Exact source and authorization behavior are testable through one boundary.

### Bad

- Users relying on a separate LiteLLM catalog must configure the provider-local
  endpoint instead.
- Existing LiteLLM values remain inert until an explicit future migration.

## Confirmation

Feature scenarios assert provider-local list/page/search URLs, provider
Authorization, and zero requests to a conflicting LiteLLM twin.
