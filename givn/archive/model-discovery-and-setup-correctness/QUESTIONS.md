# Questions and Assumptions

The design-review questions were resolved without user input because this
iteration is non-interactive.

| Question | Assumption used | Consequence |
|---|---|---|
| What is the mandatory-reasoning fallback when metadata has no usable effort? | Preserve an existing valid non-`off` value; otherwise return a typed policy error. | The resolver never invents an effort and never emits `off` for mandatory reasoning. |
| What does newest search mean when completion order differs from entry order? | Newest means the latest user-entered generation, regardless of completion order. | A slower newer search replaces an older completed result; late older results are discarded. |
| What is the environment fallback order when no source is saved? | Use `WATN_<PROVIDER>_API_KEY` first, then `WATN_API_KEY`; provider names are uppercased and non-alphanumeric characters become `_`. | Explicitly saved sources are never replaced by fallback discovery. |
| How is a configured but missing environment reference classified? | It is an authentication error before any catalog or chat request. No configured source is treated differently from a configured missing source. | Optional LiteLLM with no key remains unauthenticated; a configured missing reference does not. |
| Which reasoning tiers may send reasoning? | Any tier with a valid persisted non-`off` strength may send it; the old thinking-tier default remains `high` when absent. | Documentation and request construction use the same five-value policy. |
| Which provider name is persisted when setup accepts the built-in default endpoint? | `openrouter`, because the endpoint-to-provider mapping is deterministic and the default endpoint is OpenRouter. | The setup-failure scenario asserts the confirmed default provider and endpoint rather than treating the built-in endpoint as a custom provider. |
