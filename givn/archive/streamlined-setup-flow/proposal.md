# Proposal: streamlined-setup-flow

## Problem / Opportunity

The current setup screen presents provider settings, model discovery, model
roles, reasoning effort, shell completion, and the Ctrl-W shortcut together.
These are different decisions, but they compete for space and attention. Model
selection is already complex because it needs filtering, search, pagination, and
model information. Selecting reasoning effort on the same screen makes that
flow difficult to understand.

Users also need to rerun setup safely. Existing settings should be visible and
reusable without forcing users to re-enter masked credentials or revisit values
they do not want to change. A user who wants to change only the provider, model
roles, or shell integration should not have to pass through unrelated setup.

## Proposed Solution

Provide three focused setup commands and one coordinated setup flow:

- `watn provider` configures the provider and its completion service address.
- `watn models` configures the `small`, `normal`, and `thinking` model roles.
- `watn shell` configures shell completion and the optional Ctrl-W shortcut.
- `watn setup` runs the complete coordinated flow every time it is invoked.

The three focused commands save only their own settings when their flow
completes. They do not open questions belonging to another setup area.

### Coordinated Setup

`watn setup` presents one question at a time in this order:

1. Provider choice.
2. Completion service address.
3. Credential source and value.
4. Model catalog address and reachability status.
5. `small` model selection.
6. `small` reasoning effort.
7. `normal` model selection.
8. `normal` reasoning effort.
9. `thinking` model selection.
10. `thinking` reasoning effort.
11. Shell completion selection.
12. Ctrl-W shortcut selection.
13. Compact review and final confirmation.

Every current value is prefilled when setup is rerun. Moving forward without
editing a valid value keeps that value. Back navigation keeps the in-progress
values. A question with no current value or usable default cannot be skipped.

The complete coordinated draft remains in memory until final confirmation. A
cancelled setup leaves an existing configuration unchanged and does not create
a missing configuration file. The review shows the provider, catalog status,
three model/effort pairs, shell choices, and credential status without exposing
secret values.

### Provider Setup

When no provider or supported environment credential is available, provider
selection is an explicit Up/Down list with these choices:

1. OpenRouter
2. OpenAI
3. Custom

OpenRouter and OpenAI prefill editable standard completion service addresses.
Custom requires the user to enter an address. When an existing configuration or
supported environment credential identifies a provider, that provider and its
values are prefilled, but the user can change them.

Credential source is a separate two-option question:

- `Environment variable` shows or accepts a variable name. The variable must
  exist and contain a non-empty value before setup can continue.
- `Paste API key` accepts a masked literal value. An existing literal remains
  masked and preserved unless the user explicitly chooses to replace it.

The model catalog uses the same credential as the completion service. Its
initial address is derived from the accepted completion service address and is
probed before model selection. The user may edit the derived address and it is
probed again. A replacement address is saved only after a successful probe.

If an existing catalog address fails, setup warns the user, preserves the
existing address and model roles, and permits editing the address. If a new or
edited address is unreachable, setup keeps the previous reachable address when
one exists; otherwise the catalog address remains unset. Provider setup remains
usable and model selection switches to manual entry. Setup never sends a test
chat request to validate the completion service.

Changing provider preserves the current values in the draft while they are
revalidated against the newly selected provider. Values that fail validation or
catalog probing must be replaced before final confirmation.

### Model Setup

The existing role names and command selection remain unchanged: `small`,
`normal`, and `thinking`, selected by `-1`, `-2`, and `-3` respectively.

Model selection and reasoning selection are separate questions. The model
question contains the catalog interaction: filtering, search, pagination, and
concise model information. The reasoning question is a simpler screen that
identifies the selected model and asks for its effort.

When the catalog is available, model identifiers must come from that catalog.
An existing model not present in the catalog cannot be kept and must be
replaced. Each required role must have a model before later setup can be
completed.

When the catalog is empty, invalid, or unavailable, setup displays a notice and
allows plain-text model identifiers. When the catalog does not provide
reasoning efforts, setup displays a notice, defaults reasoning to `off`, and
offers the generic effort list. The generic list includes `off`, `low`,
`minimal`, `medium`, `high`, and a final free-form entry. A free-form value must
be non-empty, is persisted as entered, and is sent unchanged. `off` omits the
reasoning setting from the request.

When catalog metadata provides reasoning efforts, only the selected model's
supported efforts are shown and its catalog default is preselected. A saved
effort that is not supported by the available catalog must be replaced.

`watn models` requires a configured provider. If none exists, it remains a
model-only command and displays concise guidance to run `watn provider` rather
than opening provider questions.

### Shell Setup

Shell completion is asked before the Ctrl-W shortcut, and they are always
separate questions. The setup flow surfaces only Bash, Fish, and Zsh for both
choices. Other shells may be supported by `watn completions` but are not
presented by setup.

Completion and shortcut selections are independent. Each is prefilled from the
current filesystem state and allows different shell selections. Deselecting an
installed integration removes only its Watn-managed block. Missing, duplicated,
malformed, or out-of-order managed markers cause setup to refuse that file
change rather than risk user-owned content.

Shell operations are reported independently. A successful completion
installation remains when shortcut installation fails. During coordinated
setup, provider and model configuration is still written after final
confirmation, successful shell changes remain, and the overall result reports
nonzero when any selected shell operation fails.

### First-Run Behavior

An interactive request opens the coordinated setup when no configuration file
exists, when no usable provider or credential is available, or when the
configuration is otherwise incomplete enough to require setup, including when
one or more model roles are missing. Existing valid partial values and
supported environment credentials are prefilled.

When setup completes from an implicit first-use request, Watn exits
successfully without sending the original request. The user reruns the request.
Non-interactive use does not open the terminal flow or probe the network; it
prints concise guidance to run `watn setup` or `watn provider` from a terminal.

Malformed or unreadable configuration stops with an error and remains untouched.
Setup does not attempt recovery or overwrite it.

## Out of Scope

- OpenAI-compatible chat request, response, streaming, and execution behavior.
- Existing model metadata formats and provider response formats.
- Renaming the `small`, `normal`, or `thinking` configuration roles.
- Changing the `-1`, `-2`, or `-3` command-line role selection behavior.
- Adding provider-specific APIs beyond the existing provider choices.
- Exposing additional shells in setup beyond Bash, Fish, and Zsh.
- Changing the shell-generated completion or Ctrl-W widget content beyond the
  setup selection and safe managed-block removal described above.
- Automatic replay of the original request after implicit setup.
- Non-interactive configuration flags and direct configuration mechanisms.

## Open Questions

None.
