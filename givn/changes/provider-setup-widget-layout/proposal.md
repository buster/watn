# Proposal: provider-setup-widget-layout

## Problem / Opportunity

The provider setup and model selection screens present important information as
mostly undifferentiated terminal content. Users have to infer which values are
editable, which provider is selected, and where they are in a long model list.
This makes setup slower and makes the model picker difficult to scan when a
provider exposes many models.

## Proposed Solution

When configuring a provider, the user sees a clearly bordered, titled setup
screen with a selectable credential-source list, the endpoint and current
credential details in aligned rows, and supporting instructions or validation
status in readable paragraphs. The selected credential source remains visually
distinct while the user moves through setup; the endpoint and provider naming
rules remain unchanged.

When choosing a model, the user sees a bordered picker with tabs for the
available model groups. The active group displays models in aligned columns,
the current model is visibly selected, and a scrollbar indicates the user's
position whenever the model list is longer than the available space. Keyboard
navigation continues to move the selection and advance or return between model
groups without losing the current visual context. Typing a filter updates the
catalog after a short debounce in the background; an older search result cannot
replace a newer result.

## Out of Scope

- Provider credentials, provider naming, catalog contents, and persistence
  behavior are not changing.
- The available providers and models are not changing.
- Keyboard commands unrelated to navigating or displaying these screens are not
  changing.

## Open Questions

- Resolved: provider details are the endpoint, credential source, and masked or
  environment-backed current value.
- Resolved: model tabs are the existing `small`, `normal`, and `thinking` tiers.
- Resolved: the screens use a fully stacked layout at the existing 120-column by
  40-row PTY size; long cells are truncated, model IDs remain the first column,
  and the scrollbar is hidden for exact-fit or empty lists.
