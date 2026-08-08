# QUESTIONS

## Working assumptions made for change `ratatui-model-picker` (no user prompts)

The design-review grilling surfaced findings F1–F9 against the initial spec
and design. Per the working loop ("don't ask; open document QUESTIONS.md,
make an assumption, implement"), each was resolved by an assumption during
hardening rather than a user prompt. These are recorded here.

### A1 — Per-word filter must apply on the remote search path too (F3)

The proposal says "dee flash" finds "DeepSeek V4 Flash". Typed filters travel
through `list::search_models` (remote), not only the local fallback. I
assumed the per-word order-independent predicate is a single shared
`word_matches(id, query)` function applied in both `search_models` (as a
secondary client filter) and `local_filter` (fallback). Existing single-word
autosuggest scenarios remain backward compatible.

### A2 — Dialog focus contract and page size (F8)

I assumed default focus on the model list at each level, so "type filter +
Enter selects the top suggestion" — preserving the existing autosuggest e2e
contract. Tab cycles focus to the reasoning selector and back. Page size is a
fixed `PAGE_SIZE = 10` constant for deterministic tests.

### A3 — Reasoning request-body assertions are made real (F1)

The pre-existing `the API request should include reasoning with effort "X"`
step only asserted `mock.hits() > 0`, which is vacuous. I assumed the chat
mock captures the request body into `WatnWorld.last_request_body` (an
existing never-populated field) and both reasoning-effort Then steps assert
on the parsed body. This is required for the new reasoning scenarios to be
able to fail in RED.

### A4 — Local-fallback scenario loads local models (F2)

The fallback scenario passes the world's local models as `all_models` to
`execute_search` so the suggestions assertion can hold. Assumed the Given
loads the listed models into the world.

### A5 — Deterministic model sets and page target (F4, F5)

I assumed: (1) e2e scenarios use model sets with unique prefixes so a typed
filter uniquely identifies the target model (avoids "gpt-4o" matching both
`gpt-4o-mini` and `gpt-4o`); (2) the browse scenario pins a 40-entry list
`model-01..model-40` and asserts exactly `small="model-12"` after down + one
page (PAGE_SIZE=10).

### A6 — Metadata display is part of the configure interaction (F7)

The metadata scenario is a non-@e2e formatting scenario driving the real
model-list display formatter, asserting pricing presence/absence. It does
not add a distinct inventory entry or an extra `@e2e` (the metadata is
visible in the same dialog the other e2e scenarios drive).

### A7 — Reasoning strength default preserves legacy behaviour

`TierReasoning::effort(tier)` maps "off"/absent → `None`; otherwise
`Some(strength)`. When a tier has no explicit reasoning configured, the prior
default is preserved (thinking → "high", others → None) so archived reasoning
scenarios keep passing. Invalid hand-edited strength values are parsed
leniently and fall back to no reasoning (TD-004).

### A8 — Debounce timing not asserted as wall-clock

The proposal's ~200 ms responsiveness is treated as implementation
behaviour rendered as "filter updates"; the spec asserts the observable
outcome, not the timing.

## Status

No unresolved questions. All assumptions are reflected in the final spec,
design.md, and arc42 docs.
