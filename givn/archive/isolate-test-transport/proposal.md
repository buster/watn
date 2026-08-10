# Proposal: Isolate Test Transport

## Problem / Opportunity

The endpoint used by a normal watn invocation can currently be replaced by a
variable intended only for tests. A user, deployment, or inherited environment
can therefore send provider credentials and questions to an unintended server.
The saved configuration and readiness checks do not make this routing change
visible.

The test suite also accepts a request as correct when it reaches any matching
mock. It does not consistently prove that the configured endpoint and
credential were used. The stale-search false-green behavior is a separate
model-discovery correctness problem and is tracked for the later
`model-discovery-and-setup-correctness` change.

## Proposed Solution

Normal and release invocations always use the endpoint stored or resolved from
the user's provider configuration. Test routing controls have no effect on
those invocations and cannot change readiness decisions or persisted
configuration.

Test runs may use an isolated routing facility to reach a local provider twin.
The facility is unavailable to normal release builds, is never persisted, and
does not alter the endpoint shown or saved for the user.

Provider and model-discovery transport assertions verify the exact requested
endpoint, path, request count, and credential sent with the request. Missing or
whitespace-only test overrides fall back to the configured endpoint, and
readiness remains independent of the test setting.

## Out of Scope

Provider selection precedence, LiteLLM discovery precedence, model reasoning,
SSE parsing, command rendering, and stale-search concurrency are handled by
later changes.

The external provider protocol and the persisted configuration format do not
change.

## Open Questions

No product decision is open. The implementation may choose the smallest
compile-time test-only boundary that preserves the existing local test flow.
