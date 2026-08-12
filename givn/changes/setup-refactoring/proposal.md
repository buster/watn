# Proposal: Setup Refactoring

## Problem / Opportunity

The current setup experience is organized around seven implementation fields:
endpoint, credential, three separate model tiers, shell completion, and the
Ctrl-W shortcut. It creates configuration while reading a missing config path,
can skip onboarding solely because an environment credential is available, and
persists a provider before the user has reviewed the complete setup.

The command surface has also split one configuration task across `watn setup`,
`watn provider`, and `watn models`, plus provider/model selection flags and
environment overrides. These parallel entry points have different save and
validation boundaries. They make it harder to tell which values are loaded,
detected, recommended, or about to be persisted.

## Proposed Solution

Replace the field-oriented flow with one `watn setup` wizard containing four
topics:

1. Provider
2. Model roles
3. Shell integration
4. Review

The wizard starts on Provider, including when an existing config is being
edited. It loads supported saved configuration into an in-memory draft and
labels each prefixed value as loaded from config, detected from the environment,
recommended by watn, or entered by the user. A contextual-help pane remains
visible beside settings on wide terminals and below them on narrow terminals.

First use is determined by physical config-path absence, not provider
readiness. An interactive `watn "question"` invocation with no config opens
setup even when a recognized credential is present. A non-interactive first use
prints actionable `watn setup` guidance to stderr, exits with status 1, creates
no config, and sends no request. A pre-existing file, including a legacy
comment-only template, counts as existing configuration.

Discovery reads only an explicit credential-variable allowlist. Detected
credentials remain variable names and presence information; their values are
never rendered, logged, or persisted. Users may intentionally enter another
valid environment-variable name, which persists only as `${NAME}`. Provider
choices are OpenRouter, OpenAI, and Custom.

Model roles show Small / fast, Balanced / normal, and Thinking together. Catalog
results produce labeled suggestions rather than silently selected hard-coded
models. If discovery fails or cannot produce reliable candidates, users can
enter all required model IDs manually. The review carries an unverified-catalog
warning and manual/metadata-unknown roles use `Reasoning: off`. A provider
change makes existing model assignments require review before Finish.

Nothing writes configuration before Review's Finish action. Finish validates the
draft, writes supported configuration fields once, then applies selected shell
changes. Cancelling or interrupting leaves the original config byte-for-byte
unchanged; first-run cancellation leaves no config file. Shell startup files,
identified by their existing watn marker blocks, are the source of truth for
completion and shortcut selections. Checking installs a block and unchecking
removes it on Finish. If a shell operation fails after config is saved, watn
reports the partial result and exits nonzero.

Remove `watn provider`, `watn models`, `--provider`, `--model`,
`--set-small`, `--set-normal`, `--set-thinking`, `WATN_PROVIDER`, and
`WATN_MODEL`. Retain `watn completions` and the request-time tier selectors
`-1`, `-2`, and `-3`.

For automatic onboarding, Finish writes no command text to stdout, prints
`Setup complete. Retry your command.` to stderr, exits 0, and does not replay
the original request.

## Out of Scope

- Changing persisted provider, tier, reasoning, or credential-reference
  formats.
- Preserving unknown TOML fields, comments, formatting, or key order. Supported
  semantic configuration remains the preservation boundary.
- Automatically scanning arbitrary environment variables or copying any
  detected secret into TOML.
- Inventing a model identifier when catalog data is unavailable or insufficient.
- Changing chat request semantics for the retained `-1`, `-2`, and `-3` tier
  selectors.
- Adding an uninstall command beyond unchecking an integration in `watn setup`.
- Keeping compatibility aliases for removed configuration commands or
  provider/model-selection overrides.

## Open Questions

None. The branch handover records the resolved product decisions above.
