# Setup Refactoring

## Idea: guided setup wizard with first-run discovery

**Status:** idea to implement

Refactor setup into a topic-oriented wizard that teaches the user what each
setting does, detects safe and useful defaults, and gives the user one clear
review point before writing configuration.

The full `watn setup` command should be an educational first-run path. The
existing `watn provider` and `watn models` commands should remain focused entry
points into the same wizard state and rendering model.

## Problem

The current setup implementation already has a wizard shell, but its pages are
organized around individual implementation fields:

```text
URL -> API key -> Small Model -> Middle Model -> Large Model
     -> Shell Completion -> Shell Shortcut
```

This creates seven repetitive pages and makes the page sequence more important
than the user's decisions. The current explanations mostly describe the
control or its keyboard shortcut. They do not consistently explain what a
setting enables, when it should be used, or what tradeoffs it introduces.

First-run behavior is also not fully aligned with the desired experience:

- Automatic setup is currently triggered by provider readiness, not by the
  absence of a configuration file.
- A no-config environment with a recognized API-key variable can skip setup,
  even though the user has never confirmed the provider or model choices.
- `load_config()` currently creates a commented template before setup begins,
  so the wizard does not operate against a genuinely absent configuration.
- The provider draft is persisted before the complete setup flow reaches its
  final review point.
- Environment-variable suggestions are not presented as clearly labeled
  detected values versus recommended defaults.

## Product decisions

The refactoring should implement these decisions:

- If no config file exists and the process has an interactive terminal, open
  first-run setup even when a recognized environment credential is already
  present.
- The full wizard has four user-facing topics: Provider, Model roles, Shell
  integration, and Review.
- The three model assignments appear together on one Model roles page.
- Contextual documentation is always visible beside the settings on wide
  terminals and moves below them on narrow terminals.
- Setup edits an in-memory draft and writes configuration only after the user
  chooses Finish setup.
- Successful automatic onboarding does not replay the original request. It
  saves the configuration, exits with status 0, and prints a message such as
  `Setup complete. Retry your command.`
- Cancelling first-run setup leaves no config file behind.
- Existing persisted configuration remains authoritative unless the user
  explicitly changes it in the wizard.

## Wizard information architecture

### 1. Provider

Group the provider settings that answer one question: "Where should watn send
requests and how should it authenticate?"

Settings:

- OpenAI-compatible endpoint
- Credential source: configuration value or environment variable
- API key or environment-variable name
- Connection and model-catalog status
- Test connection action

The page should show the first-run discovery banner when the config file is
absent. It should not add a separate welcome page because that would add
friction without adding a decision.

### 2. Model roles

Show the relationship between the three model assignments on one page:

```text
Model roles

> Small / fast       <model>       Reasoning: off
  Balanced/normal    <model>       Reasoning: medium
  Thinking           <model>       Reasoning: high
```

The active row drives the contextual help. Entering the model field opens a
focused searchable picker that can show model metadata such as context length,
pricing, features, and reasoning support. The picker should not force the main
wizard page to become a full-screen model browser permanently.

The help for each role should explain the runtime behavior:

- Small / fast is the default model tier and is also selected explicitly by
  `watn -1`; it is intended for lower cost or quicker responses.
- Balanced/normal is selected by `watn -2` and is the general-purpose
  alternative when the user wants a different model tier.
- Thinking is used by `watn -3` and is intended for more complex requests; it
  may use more time or cost more.
- Reasoning choices are model-specific and must reflect catalog metadata.

When no prior tier assignments exist, model suggestions should come from the
fetched catalog and be marked as suggestions. Do not silently depend on a
hardcoded model identifier that may disappear from a provider catalog. If a
reliable candidate cannot be identified for a role, show `Needs selection`
instead of inventing a value.

### 3. Shell integration

Combine shell completion and the `Ctrl-W` shortcut into one optional topic.
Each feature should have its own short description, enable/skip control, and
shell checklist. The page should make the distinction clear:

- Completion adds generated command and subcommand completion to the selected
  shell startup files.
- The shortcut adds a `Ctrl-W` widget that turns a natural-language request
  into a command for review; it does not execute the generated command
  automatically.

The page should be skippable as a whole and should mark the step as Optional in
the progress rail when no integration is selected.

### 4. Review

The final page should summarize the complete draft before persistence:

- Endpoint and provider name
- Credential storage choice, without displaying a resolved secret
- Environment-variable name, when applicable
- Small, balanced, and thinking model assignments
- Reasoning strengths
- Shell integrations and selected shells
- Connection/catalog status
- Warnings or settings that still need attention

The primary action is `Finish setup`. A user can return to any prior step to
edit the draft, but Finish must remain blocked while required settings are
invalid or incomplete.

## Contextual documentation

Use a persistent two-column layout on sufficiently wide terminals:

```text
+ watn setup ----------------------------------------------------------+
| 1 Provider [complete]  2 Model roles [current]  3 Shell  4 Review    |
+------------------------------+--------------------------------------+
| Provider                     | About this setting                   |
|                              |                                      |
| Endpoint                     | What it is                           |
| > https://...                | The API URL watn will call.           |
|                              |                                      |
| Credential storage           | What it enables                      |
| > Environment variable       | Provider requests and model discovery.|
|                              |                                      |
| API key / variable name      | Recommendation and tradeoffs         |
| > OPENROUTER_API_KEY         | Keep secrets outside the config file.|
+------------------------------+--------------------------------------+
| Back                                      Continue to Model roles   |
+---------------------------------------------------------------------+
```

The help pane must update with the active setting and answer four questions:

- What is this?
- What does it enable?
- What do you recommend?
- What tradeoff or requirement should I know?

Example endpoint documentation:

```text
What it is
The OpenAI-compatible API URL that watn uses for requests and model discovery.

What it enables
Connection testing, model discovery, and generated answers.

Recommendation
Use the OpenRouter default unless another compatible service is intended.

Requirement
The endpoint must expose the APIs watn uses, including model discovery and
chat completions.
```

On narrow terminals, stack the help pane below the settings while preserving
the same content. Help must not disappear solely because the terminal is
smaller.

The progress rail should distinguish these states:

- Complete
- Current
- Needs attention
- Optional

Previous completed steps should be revisitable. Changing the provider should
invalidate or mark the model catalog and model assignments as needing review.

## First-run discovery

Detect whether the config path exists before loading a configuration object.
The absence of the file is a first-run signal and must take precedence over
provider readiness for implicit interactive use.

The Provider page should display a discovery banner such as:

```text
No watn configuration found.

Suggested setup
  Endpoint       OpenRouter default
  Credential     OPENROUTER_API_KEY (detected)

Suggested values are ready for review. Nothing is saved until Finish setup.
```

When no credential variable is found:

```text
No API credential was detected.

Using the OpenRouter-compatible default endpoint.
Suggested variable: OPENROUTER_API_KEY (not found)
```

Prefilled values must carry an origin label:

- `Detected from environment`
- `Recommended default`
- `Loaded from config`
- `Entered by you`

A detected variable should be selected as an environment-backed credential,
which persists a reference such as `${OPENROUTER_API_KEY}` rather than the
resolved secret. A recommended but absent variable should be clearly marked as
missing; it must not be represented as a working credential.

### Detection allowlist

Inspect only a deliberate allowlist of variable names. Do not scan all
environment values or display their contents.

The initial candidates should include:

| Variable | Suggested provider behavior |
|---|---|
| `OPENROUTER_API_KEY` | Select the OpenRouter default endpoint. |
| `WATN_API_KEY` | Offer a generic credential; do not infer a custom endpoint from it. |
| `WATN_OPENAI_API_KEY` | Select the built-in OpenAI endpoint when OpenAI is selected. |
| `WATN_<PROVIDER>_API_KEY` | Use only when the provider identity and endpoint are known. |
| `OPENAI_API_KEY` | Add as a standard alias only with an explicit OpenAI mapping. |

The detection check should only establish that a variable is present and
non-empty. It must never log, render, persist, or send the variable's value
outside the normal authenticated request boundary.

If multiple recognized variables are present, show them as choices and require
the user to select one. Do not silently select based on environment iteration
order.

### Suggestion precedence

Use this precedence for first-run suggestions:

1. An explicit provider selection, if one is supplied by the command or
   environment.
2. A recognized provider-specific environment variable.
3. A recognized generic environment variable, with an explicit warning that
   the endpoint still needs confirmation.
4. The OpenRouter endpoint and `OPENROUTER_API_KEY` as recommended defaults.

For an existing config, preserve the existing precedence rules. A saved literal
credential or saved environment reference is authoritative. A missing saved
reference must not silently fall through to another environment variable.

If the user changes the endpoint after accepting an automatic suggestion, the
wizard may recalculate a still-automatic credential suggestion. It must not
overwrite a value the user has edited or explicitly selected.

## Persistence and cancellation

The entire wizard should operate on a draft copy of configuration:

- Provider validation and catalog requests use the draft endpoint and
  credential in memory.
- Moving from Provider to Model roles must not write the provider early.
- Finish validates the full draft, saves the provider, model tiers, reasoning
  settings, and shell choices, and then returns success.
- An environment-backed credential saves only its `${VARIABLE}` reference.
- Literal credentials remain masked in the UI and are written only when the
  user explicitly chooses configuration storage.
- The existing secure config-file permissions must be preserved.
- Escape or cancellation before Finish leaves the existing file unchanged.
- On first run, cancellation leaves the config path absent rather than leaving
  an auto-generated template behind.

This may require separating config loading from template initialization. A
read-only load should be able to return an empty/default configuration plus an
`exists` signal without writing a file. Template generation, if still desired,
should be an explicit action rather than a side effect that occurs before the
wizard can be shown.

## Command behavior

### Implicit first use

For an interactive `watn "question"` invocation:

- No config file: always open first-run setup, even if an environment key is
  detected.
- Existing config with an incomplete provider: open setup or repair flow.
- Existing config with a ready provider: proceed normally.
- After successful setup: print `Setup complete. Retry your command.`, exit 0,
  and do not replay the original request.
- Non-interactive stdin: print setup guidance and do not initialize the TUI.

Explicit provider selection through `--provider` or `WATN_PROVIDER` should keep
its existing error behavior unless a separate product decision changes that
contract. The automatic first-run branch is for implicit onboarding.

### Explicit commands

- `watn setup` opens the complete Provider -> Model roles -> Shell integration
  -> Review flow.
- `watn provider` opens at Provider and finishes after the provider portion,
  while still using the shared contextual-help layout and save behavior.
- `watn models` opens at Model roles and finishes after model configuration,
  without requiring unrelated shell integration choices.

## Navigation contract

Keep navigation predictable and make the active control unambiguous:

- Tab and Shift-Tab move between settings; crossing a page boundary moves to
  the next or previous topic.
- Enter confirms the active choice, opens a model picker, or advances when the
  current page is valid.
- The step rail may allow direct navigation to completed steps.
- Back never discards the draft.
- Escape opens a leave/discard prompt rather than silently losing changes.
- Ctrl-C exits with the existing interrupt status and does not save.

The exact key labels should remain visible in the footer, but the help pane is
the primary explanation of what the active setting means.

## Implementation outline

1. Add a config-existence/read result so first-run detection happens before any
   template file is created.
2. Add a pure environment-discovery function that returns provider suggestions,
   variable names, origin labels, and ambiguity information without returning
   secret values to the renderer.
3. Refactor the setup page model from individual URL, API key, model-tier, and
   shell pages into the four topic pages while preserving focused entry points.
4. Add a shared contextual-help model keyed by the active setting and render it
   beside or below the settings based on terminal width.
5. Make provider and model catalog operations consume the in-memory draft.
6. Move all persistence to the final Finish boundary and preserve existing
   unrelated configuration.
7. Add first-run discovery, precedence, cancellation, and no-secret-leak
   acceptance coverage.
8. Update README and setup ADRs after the implementation is accepted so they
   describe the final page count and first-run behavior.

## Acceptance criteria

The implementation is complete when these behaviors are covered by tests and
visible in the real terminal UI:

- With no config file and `OPENROUTER_API_KEY` set, an interactive implicit
  request opens setup, shows OpenRouter and the detected variable as suggested
  values, and does not bypass the wizard.
- With no config file and no recognized variable, setup opens with the
  OpenRouter endpoint and `OPENROUTER_API_KEY` recommendation marked as
  missing.
- A recognized environment variable is persisted as a reference, never as its
  resolved secret value.
- Multiple detected variables are presented as explicit choices.
- An existing saved credential or environment reference is not replaced by
  fallback discovery.
- Changing the endpoint updates only an untouched automatic suggestion and
  marks model selections for review.
- The Provider, Model roles, Shell integration, and Review topics are visible
  with a clear current-step indicator.
- The active setting has documentation that explains what it is, what it
  enables, the recommendation, and the relevant tradeoff or requirement.
- Wide and narrow terminal layouts both keep contextual help available.
- The Review page prevents Finish when required settings are invalid.
- Cancelling first-run setup leaves no config file; cancelling an existing setup
  leaves the existing file byte-for-byte unchanged.
- Successful automatic setup writes the reviewed configuration, prints the
  retry guidance, exits 0, and sends no original chat request.
- Non-interactive first use prints actionable setup guidance and does not start
  Ratatui.

## Non-goals

- Do not scan arbitrary environment variables for possible secrets.
- Do not silently infer an unsupported provider or custom endpoint.
- Do not automatically copy a detected secret into the config file.
- Do not replay the original command after first-run setup.
- Do not change the persisted provider, model-tier, or reasoning formats unless
  a separate migration is explicitly approved.
