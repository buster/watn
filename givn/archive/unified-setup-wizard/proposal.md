# Proposal: unified-setup-wizard

## Problem / Opportunity

The current setup screens show endpoint input, credential choices, provider
details, model lists, instructions, and status at the same time. It is not
obvious which step is active, what the user is expected to enter, or where the
cursor is. Provider setup and model setup also feel like separate tools even
though they are one onboarding task.

## Proposed Solution

Provide one setup wizard with five pages shown as tabs across the top:

1. URL
2. API key
3. Small Model
4. Middle Model
5. Large Model

Only the active page presents its prompt and editable value. The active tab and
current page are visibly highlighted, and the cursor is visible on the line
being edited.

The URL page explains that the endpoint must be OpenAI/LiteLLM compatible. The
API key page first asks whether the credential should be stored directly in the
configuration or referenced through an environment variable. An environment
choice asks for the variable name; a configuration choice asks for the key.

The three model pages present available models in aligned tables and clearly
highlight the current selection. When a model reports reasoning capabilities,
the page shows only that model's supported reasoning efforts, defaults, and
whether reasoning is mandatory. A dedicated reasoning focus lets the user
change the effort without confusing it with page navigation. Enter/Return
advances to the next page. Tab moves forward and Shift-Tab moves backward.
Escape asks whether the current settings should be saved or discarded. Saving
persists the validated provider and completed model selections.

`watn setup` opens the wizard at the URL page. `watn provider` opens the same
wizard at the URL page, while `watn models` opens it directly at Small Model
when provider information is already configured.

## Out of Scope

- Provider endpoint compatibility rules and credential resolution rules are not
  changing.
- The available providers and model catalog contents are not changing.
- Persisted tier names and request behavior are not changing.
- Non-interactive configuration and piped command execution are not changing.

## Open Questions

- Resolved: the existing `small`, `normal`, and `thinking` tiers are presented
  to users as Small Model, Middle Model, and Large Model.
- Resolved: existing `provider` and `models` commands remain entry points into
  the shared wizard, with `setup` as the unified entry point.
