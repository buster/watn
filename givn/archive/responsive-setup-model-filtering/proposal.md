# Proposal: {{CHANGE_ID}}

## Problem / Opportunity

> Describe the problem or opportunity in domain terms. What is currently broken,
> missing, or could be better? Explain the user impact.

## Proposed Solution

> What should the system do, in observable-behaviour terms?
> Write from the user's perspective. Do NOT mention implementation details
> (classes, functions, frameworks, routes, DB schemas, step mechanics).

## Out of Scope

> What is explicitly not changing? Clarify boundaries.

## Open Questions

> List any unresolved decisions or unknowns before moving to specs.
# Proposal: Responsive Setup Model Filtering

## Problem / Opportunity

When a user types a model name while assigning a setup tier, the filter should
feel like an active input rather than a request that pauses the wizard. The
current experience can clear the visible choices while a search is pending,
does not consistently keep the typed query visible, and can perform a remote
search even when the available catalog is already sufficient for local
filtering. An older, slower search must never replace the results for the
newest query.

## Proposed Solution

Make model filtering responsive while the user continues typing:

- Keep the complete query visible in the active model-filter input.
- Wait briefly after typing stops before applying a filter update, so a burst
  of keystrokes produces one coherent update.
- Update the visible model choices continuously without preventing further
  keyboard input while a search is in progress.
- Filter the already-loaded catalog locally when it contains the complete set
  of available models.
- Use provider-backed searching when the complete catalog is not available
  locally.
- Ensure results are always associated with the newest query; late results
  from an older query are ignored.
- Keep existing model selection, reasoning choices, validation, and wizard
  navigation behavior unchanged.

## Out of Scope

- Changing provider model-catalog data or search semantics.
- Changing setup pages, keyboard shortcuts, model selection rules, or reasoning
  policy outside the filtering interaction.
- Changing provider credentials, configuration persistence, shell integration,
  or generated shell widgets.
- Adding a new user-visible search mode or a manual refresh control.

## Open Questions

None. The debounce interval is 200 milliseconds, local filtering is preferred
when the complete catalog is available, and the newest query is authoritative.
