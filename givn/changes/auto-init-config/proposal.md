# auto-init-config

## Problem / Opportunity

When a user runs `watn` for the first time, no config file exists. The binary
currently uses hardcoded defaults silently (provider `openai`, no default
model), which means the user gets no API key error unless they already know to
set environment variables. There is no discoverable configuration file to guide
them. The user has to read external documentation to learn what options exist.

## Proposed Solution

The first time `watn` runs (or any subcommand runs), if no config file exists
at the standard XDG path, the binary writes a template config file there with
every option present but commented out. The template includes:

- The default provider set to `openrouter` (uses `OPENROUTER_API_KEY` from
  the environment).
- The three tier slots (`small`, `normal`, `thinking`).
- An example custom provider stanza.
- An example pricing table.

The file is written silently and does not interrupt the command the user
actually issued. The command proceeds as normal after writing.

If a config file already exists, nothing is written.

## Out of Scope

- Interactive prompts or wizards.
- Writing the template on every run (only when absent).
- Migrating an existing config file to a new format.
- Overwriting user changes.

## Open Questions

None.
