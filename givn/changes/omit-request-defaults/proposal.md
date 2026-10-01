# Proposal: omit-request-defaults

## Release note

> Watn no longer sends a sampling temperature or an output-token limit, so every request relies on the provider's own defaults and models that reject those parameters work.

## Use-Case Context

- Use-case ID: none — cross-cutting request contract affecting `configure-model` (generation parameters) and `use-shell` (review and explanation requests)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: terminal-developer--interactive — the developer whose requests currently fail on models that reject a fixed temperature or a fixed output-token limit

## Problem / Opportunity

Some usable models reject a request that carries a sampling temperature or an
output-token limit: they accept only their own default, or they reject the
field name Watn sends. Watn always sends both, so those models fail on every
request even though the model itself works. The developer sees a provider error
instead of a command, and there is no way to use the model without provider
special-casing.

Concrete instance: a thinking model that only accepts its default temperature
returns an error when Watn sends `0.7`; a model that names its output cap
differently rejects `max_tokens`. Both requests fail before any content
streams.

## Proposed Solution

Watn sends only the parameters a request explicitly requires. It never invents
a sampling temperature or an output-token limit. When the user has configured
no explicit value for either parameter, neither parameter appears in the
request and the provider's own default applies.

Observably:

- A request to a model that rejects a non-default temperature succeeds,
  because no temperature is sent.
- A request to a model that rejects an output-token limit succeeds, because no
  output-token limit is sent.
- Every provider-backed path (ask, review, explanation) follows the same rule,
  since they share one request shape.
- When a provider reports usage, cost reporting and the displayed Amount are
  unchanged.

## Capability Routing

> `givn spec route --change omit-request-defaults` reported no signal: the top
> candidates tied at 13.00. The decision below is recorded with its rationale.

| Proposed capability | Decision | Rationale |
|---|---|---|
| ask | EXTEND ask | `ask` is the base chat-completion capability shared by every provider-backed command; `transport` owns endpoint routing and `configure-model` owns model selection and reasoning policy, but neither owns the set of generation parameters Watn sends. Route reported no signal (top candidates tied at 13.00) |

## Out of Scope

- Reasoning effort behavior and its request field.
- Provider and model setup, discovery, and catalog behavior.
- Per-model capability tables and provider-specific parameter mapping.
- Retry or continuation logic for truncated responses.
- Cost, usage, and Amount reporting.

## Open Questions

### D1 — Keep forcing request parameters, or omit them?

- Question: Should Watn keep sending a sampling temperature and an
  output-token limit with invented values, omit them and rely on provider
  defaults, or handle rejection another way?
- Options:
  1. **Omit both always** — never send either parameter unless a value is
     explicitly configured; gain: models that reject either field work with no
     per-model special cases; cost: output length is whatever the provider
     defaults to; in practice: a model that accepts only its default
     temperature stops erroring on every request.
  2. **Omit temperature, keep a 4096 output limit with a model-aware field
     name** — send `max_completion_tokens` for newer OpenAI families and
     `max_tokens` otherwise; gain: structured reviews stay long; cost: needs
     model-family detection and still fails on models that reject the limit; in
     practice: one model gets `max_completion_tokens`, another `max_tokens`.
  3. **Omit by default, retry once without the rejected field** — gain:
     self-healing for unknown models; cost: one failed round-trip per new model
     and a remembered per-model decision; in practice: the first request
     returns a 400, the retry succeeds.
- Disposition: answered — operator selected option 1 ("omit always") on
  2026-10-01.

### D2 — What happens when a provider default truncates a long review?

- Question: Removing the forced output limit means a provider with a low
  default cap can truncate a structured review response; is that acceptable?
- Options:
  1. **Accept provider defaults** — gain: no special cases; cost: a review
     response may be cut mid-object; in practice: the existing
     unusable-response recovery names the truncation and the provider-written
     command stays reviewable.
  2. **Keep a review-only limit** — gain: long reviews; cost: reintroduces the
     per-model field mapping and rejection risk this change removes.
  3. **Add retry or continuation on truncation** — gain: complete reviews;
     cost: extra requests and new failure handling.
- Disposition: out of scope — option 1 is the chosen contract; the existing
  unusable-response recovery already covers a truncated review response, and a
  continuation safeguard is a separate change if it is observed in practice.