# Watn Consolidation Ledger

This ledger records the evidence-backed removal decisions for
`watn-consolidation`. It is intentionally separate from the executable
Gherkin delta so the retained contract remains reviewable after archive.

| Finding | Removed scenario | Canonical retained contract | Decision |
|---|---|---|---|
| F1 | `provider-setup`: A literal saved credential is authoritative over environment fallback | `credential-sources`: same title, real request boundary | Remove duplicate |
| F2 | `search-concurrency`: The newest search result stays visible when an older result arrives later | `model-autosuggest`: same title plus retained terminal stale-result boundary | Remove regular duplicate seam |
| F3 | `config`: Missing config file prints provider setup guidance | `auto-init-config`: guidance plus no-config invariant | Remove subset |
| F4 | `interactive-shell-shortcut`: The generated Bash widget runs through Bash without evaluating its result | Later Bash E2E: same behavior plus request preservation | Remove subset |
| F5 | `interactive-shell-shortcut`: Failed or empty generation preserves the original buffer | Same-capability exact-buffer failure/empty contract | Remove subset |
| F6 | `model-autosuggest`: No matching model produces a clear empty state | `ratatui-model-picker`: empty state plus retained filter | Remove subset |

Each row is represented by a `@givn.removed` delta in the original capability
and is covered by the retained permanent scenario named above.
