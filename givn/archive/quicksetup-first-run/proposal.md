# Proposal: quicksetup-first-run

## Problem / Opportunity

The first run of `watn` without a configuration launches the full-screen setup
wizard. It asks about the provider, endpoint, API key, model catalog, three
models with three reasoning levels, and shell integrations before anything
works. A new user who only wants to ask a question has to walk through the
entire interactive experience. There is no fast path from "installed watn" to
"first answer".

## Proposed Solution

Add a quick setup that runs when `watn` is started and no configuration file
exists (and is available as an explicit `watn quicksetup` command). It replaces
the full wizard on first run and asks a minimal set of plain questions:

1. Before the questions start, watn states that no configuration was found and
   that the quick setup is starting.
2. Which OpenAI-compatible endpoint to use. The suggested answer is the
   OpenRouter endpoint.
3. The API key for that endpoint. If a known API-key environment variable is
   set for the suggested endpoint, the suggested answer references that
   environment variable instead of pasting the secret (e.g. `${OPENROUTER_API_KEY}`).
4. The model for the small strength. When OpenRouter is the endpoint, the
   model `google/gemma-4-flash` is suggested; otherwise the suggestion is
   empty. The model for the
   normal and the thinking strength are then asked, each pre-filled with the
   small model answer. No reasoning-level questions are asked; reasoning stays
   unset.
5. Which shells get the watn integrations (completion and shortcut together),
   as a single multiple-choice list with `[ ]` and `[x]` entries. Shells that
   are available on this system (bash, zsh, fish found on the path) are
   pre-selected.

The quick setup never contacts the endpoint to validate answers, and it asks no
reasoning questions. On confirmation the configuration is saved and the chosen
shells receive both integrations. watn then states where the configuration
file lives and that it can be changed later with `watn setup`. The run ends
there; an original request that triggered the first-run quick setup is not
sent automatically.

Abandoning the quick setup (Ctrl-C) leaves no configuration file (first run)
or the previous configuration (explicit run) and installs nothing. An empty
answer accepts the suggested value. The explicit `watn quicksetup` command may
run when a configuration already exists and overwrites it with the quick setup
answers; the automatic first-run quick setup never touches an existing
configuration.

## Out of Scope

- The full setup wizard and all its questions (catalog choice, reasoning
  levels, review page) remain unchanged and reachable via `watn setup`.
- Behaviour when a configuration file already exists (automatic quick setup
  does not run).
- Non-interactive use: without a terminal, watn keeps printing setup guidance
  instead of asking questions.
- Provider catalog selection, reasoning configuration, and editing an existing
  configuration beyond the quick setup questions.

## Open Questions

None — resolved: OpenRouter suggests `google/gemma-4-flash`; the explicit
command overwrites an existing configuration; only Ctrl-C aborts, an empty
answer accepts the suggestion.
