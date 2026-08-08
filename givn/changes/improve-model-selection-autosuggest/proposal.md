# Proposal: improve-model-selection-autosuggest

## Problem / Opportunity

The model picker is difficult to use when a provider exposes a large catalog.
Users must scan a long list to find a model, and a model that is not on the
currently visible page cannot be found by narrowing the visible choices. This
makes assigning the small, normal, and thinking tiers slow and error-prone.

The existing model catalog already belongs to the configured provider and may
be divided into pages. The picker needs to help the user find a model in the
provider's complete catalog rather than treating the currently visible page as
the complete set of choices.

## Proposed Solution

When the user opens a tier's model picker and types, the visible suggestions
update to match the text as it changes. Matching is performed against the
provider's catalog, including models that are not on the initially visible
page.

The picker keeps the current suggestions consistent with the most recent text
the user entered. If there are no matches, it clearly says that no models were
found. Clearing the text restores the available suggestions. While a new
catalog result is being obtained, the picker shows that it is updating and
does not replace newer suggestions with an older result.

The user can choose a displayed suggestion for the active tier. The chosen
model is retained when the next tier is presented, and completing all three
choices continues to save the selected tier assignments as it does today.
Existing model metadata, provider configuration, manual tier assignment, and
non-interactive use remain available.

## Out of Scope

- Changing how providers authenticate or how model metadata is represented.
- Changing the model catalog returned by a provider.
- Adding model caching, ranking, fuzzy matching, or heuristic model
  recommendations beyond matching the user's entered text.
- Changing the names or meaning of the small, normal, and thinking tiers.
- Replacing the interactive picker with a graphical interface.
- Changing the existing manual tier-setting flags or the behavior of ordinary
  model requests.

## Open Questions

No product decision blocks planning. The design must bind the picker to the
provider's existing server-side search and pagination contract, preserve the
provider's model identifiers exactly, and define how a provider response that
does not support server-side filtering is reported rather than silently
filtering only the currently visible page.
