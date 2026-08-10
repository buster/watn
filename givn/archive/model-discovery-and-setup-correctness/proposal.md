# Proposal: model-discovery-and-setup-correctness

## Problem / Opportunity

Model discovery can fail even when the user has supplied a valid credential
through the environment. The setup flow loses the credential's source, can
discard a confirmed provider when discovery fails, and may overwrite existing
reasoning preferences with empty values. A configured catalog service is also
ignored for discovery, while the model-search test does not prove which source
was used. Finally, an older search result can replace a newer result because the
concurrency scenario does not actually overlap the searches.

These failures make first-use setup unreliable, can silently change future chat
behaviour, expose the wrong service configuration, and present stale model
choices to the user.

## Proposed Solution

When I use model discovery with an environment-backed credential, the command
uses that credential without displaying or saving its secret. Confirming the
credential saves its exact environment reference. A saved literal credential
continues to take precedence over environment fallback, and a saved reference
whose variable is missing reports an authentication error without making a
request.

When a catalog service is configured, model listing, pagination, and search use
that service's exact endpoint. Its optional credential is sent only when
configured, including values read from an environment reference. When no
catalog service is configured, discovery uses the selected provider instead.
Chat requests always continue to use the selected provider, independently of
where models were catalogued.

The provider is saved after valid credential confirmation and before the first
catalog request. If catalog discovery then fails, the confirmed provider
remains saved while model tiers remain unchanged. Cancelling before
confirmation writes nothing; cancelling afterward preserves the confirmed
provider. Setup failure and cancellation do not send the original chat request.
Changing model tiers preserves the selected provider and catalog settings.

Reasoning choices accept only `off`, `low`, `minimal`, `medium`, and `high`.
Unknown or empty saved values disable reasoning. Disabled defaults select
`off` for non-mandatory reasoning, while mandatory reasoning cannot select
`off`. A valid configured default effort is preferred when enabled and
supported; otherwise the first valid supported effort is used. Existing
reasoning survives model selection when no valid replacement exists, and empty
reasoning values are never persisted or sent.

If I perform overlapping model searches, the newest completed search remains
visible. A late result from an older search cannot replace it, and search work
is cleaned up when the operation ends.

## Out of Scope

This change does not alter the active chat provider selection, chat request
format, command-generation behaviour, or the model catalog's user-facing
purpose. It does not add new model providers, new reasoning strengths beyond
the listed values, or a new asynchronous execution model. It does not change
release packaging, CLI version reporting, streaming output, documentation
claims, or shell completion behaviour.

## Open Questions

None. The catalog endpoint precedence, credential precedence, save timing,
reasoning policy, and newest-search-wins behaviour are defined above.
