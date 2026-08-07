# model-explorer — Proposal

## Problem / Opportunity

The `watn models` subcommand only shows an interactive model picker when a
LiteLLM endpoint is configured. For the two built-in providers (openrouter,
openai), it prints a static message directing the user to manually edit the
config file. All three providers expose an OpenAI-compatible `/v1/models`
endpoint, so the same interactive flow should work for all of them.

A user who sets `OPENROUTER_API_KEY` has no interactive way to discover and
select models — they must know the model IDs ahead of time and pass
`--set-*` flags or edit `config.toml`.

## Proposed Solution

`watn models` fetches the available model list from the active provider's
`/v1/models` endpoint and presents an interactive picker for each of the three
tiers (small, normal, thinking). The user selects one model per tier, and the
selections are written to the config file.

Flow:

1. Resolve the active provider (openrouter, openai, or a custom provider, or
   LiteLLM — same resolution logic as the main query path).
2. Fetch model entries from the provider's `/v1/models` endpoint.
3. Display an interactive selector for each tier — the user picks a model for
   small, then normal, then thinking.
4. Write the three selected model IDs to the config file under `[tiers]`.

The picker shows the model ID plus any available metadata from the API
response: name, context_length, pricing (prompt/completion per token), and
supported features (e.g. reasoning, tools, structured_outputs). All metadata
fields are optional — if the response omits them, the picker shows only what
is present.

The OpenRouter `/api/v1/models` response schema is the richest reference:
`data[].id`, `data[].name`, `data[].context_length`, `data[].pricing.prompt`,
`data[].pricing.completion`, `data[].supported_features` (which includes
`"reasoning"`). The OpenAI `/v1/models` response returns only `id`, `created`,
and `owned_by`. LiteLLM returns its own variant. The parser tolerates all
three shapes and silently drops unknown fields.

The `--set-small`, `--set-normal`, `--set-thinking` flags remain available as
a non-interactive escape hatch. When all three are provided, the existing
direct-write behavior is used (no API call, no interactive prompt).

When no provider is configured and no env var is set, the existing "No provider
endpoint configured" message is shown.

## Out of Scope

- Caching the model list across invocations.
- Filtering or searching models inside the picker.
- Pagination of model lists.
- LiteLLM-specific model detail display beyond what `/v1/models` returns.

## Open Questions

(Resolved — no open questions.)
