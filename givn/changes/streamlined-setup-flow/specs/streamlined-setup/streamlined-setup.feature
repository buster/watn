# User Interaction Inventory:
# - invoke `watn setup` and complete the coordinated configuration flow
# - invoke `watn provider` and configure a provider independently
# - invoke `watn models` and configure the three model roles independently
# - invoke `watn shell` and configure shell integrations independently
# - invoke an interactive `watn "question"` request when setup is incomplete

@givn.delta @streamlined-setup

Feature: Streamlined setup flow

  @givn.added @e2e @wip
  Scenario: Coordinated setup completes provider models reasoning and shell choices
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["small-model", "normal-model", "thinking-model"] for "/models"
    When I start `watn setup` in a terminal
    Then the setup coordinator should show the provider question first
    When I choose provider "OpenRouter"
    And accept the default completion endpoint
    And choose to paste an API key
    And enter API key "sk-coordinated-key"
    And accept the derived catalog endpoint
    And choose "small-model" for the small role
    And choose reasoning "low" for the small role
    And choose "normal-model" for the normal role
    And choose reasoning "medium" for the normal role
    And choose "thinking-model" for the thinking role
    And choose reasoning "high" for the thinking role
    And choose no shell completion integrations
    And choose no Ctrl-W shortcut integrations
    And confirm the setup review
    Then setup should exit successfully
    And the config file should contain provider "openrouter"
    And the config file should contain small model "small-model" with reasoning "low"
    And the config file should contain normal model "normal-model" with reasoning "medium"
    And the config file should contain thinking model "thinking-model" with reasoning "high"

  @givn.added @wip
  Scenario: Coordinated setup displays one separate reasoning question after each model
    Given a configured provider with catalog models "alpha", "beta", and "gamma"
    When I start `watn setup` in a terminal
    And advance to the small model question
    Then the small model question should not contain the reasoning choices
    When I choose model "alpha" for the small role
    Then the small reasoning question should identify model "alpha"
    When I choose reasoning "low" for the small role
    Then the normal model question should be active

  @givn.added @wip
  Scenario: Rerunning coordinated setup prefills current values and preserves a masked literal credential
    Given a config file contains provider "custom" with endpoint "https://llm.example/v1"
    And the provider credential is the literal "sk-existing-key"
    And the config file contains models "small-old", "normal-old", and "thinking-old"
    When I start `watn setup` in a terminal
    Then the provider question should show "custom" selected
    And the completion endpoint input should show "https://llm.example/v1"
    And the credential input should remain masked
    And the small model input should show "small-old"
    And the normal model input should show "normal-old"
    And the thinking model input should show "thinking-old"

  @givn.added @wip
  Scenario: Cancelling coordinated setup leaves an existing configuration unchanged
    Given an existing config contains provider "legacy" with credential "sk-old-key"
    And the existing config content is recorded
    When I start `watn setup` in a terminal
    And cancel setup before final confirmation
    Then the config file should be byte-for-byte unchanged

  @givn.added @e2e @wip
  Scenario: Provider setup configures an OpenAI provider with an environment credential
    Given no provider is configured
    And environment variable OPENAI_API_KEY is set to "sk-openai-test"
    When I start `watn provider` in a terminal
    Then provider setup should show provider choices "OpenRouter", "OpenAI", and "Custom"
    When I choose provider "OpenAI"
    And accept the default completion endpoint
    And choose environment variable "OPENAI_API_KEY"
    Then provider setup should exit successfully
    And the config file should contain provider "openai"
    And the config file should contain endpoint exactly "https://api.openai.com/v1"
    And the config file should contain credential reference "${OPENAI_API_KEY}"
    And the config file should not contain "sk-openai-test"

  @givn.added @wip
  Scenario: Provider setup requires a custom endpoint
    Given no provider is configured
    When I start `watn provider` in a terminal
    And choose provider "Custom"
    Then provider setup should not allow the empty endpoint
    When I enter endpoint "https://llm.example/v1"
    Then provider setup should allow the credential question

  @givn.added @wip
  Scenario: Provider setup refuses an unresolved environment credential
    Given no provider is configured
    And environment variable MISSING_API_KEY is not set
    When provider setup chooses environment variable "MISSING_API_KEY"
    Then provider setup should show that "MISSING_API_KEY" must contain a non-empty value
    And the config file should not contain a provider entry for the attempted setup

  @givn.added @wip
  Scenario: Provider setup preserves unrelated settings
    Given a config file contains provider "legacy" with endpoint "https://legacy.example/v1"
    And the config file contains pricing and LiteLLM settings
    When provider setup saves provider "custom" with endpoint "https://new.example/v1" and credential "sk-new-key"
    Then provider "legacy" should remain unchanged
    And the pricing and LiteLLM settings should remain unchanged

  @givn.added @wip
  Scenario: Provider setup does not probe the catalog
    Given no provider is configured
    When provider setup saves provider "custom" with endpoint "https://llm.example/v1" and credential "sk-key"
    Then no model catalog request should be sent

  @givn.added @e2e @wip
  Scenario: Models setup configures all three roles from an available catalog
    Given a configured provider with catalog models "small-model", "normal-model", and "thinking-model"
    When I start `watn models` in a terminal
    Then the model setup should begin with the small role
    When I choose "small-model" for the small role
    And choose reasoning "low" for the small role
    And choose "normal-model" for the normal role
    And choose reasoning "medium" for the normal role
    And choose "thinking-model" for the thinking role
    And choose reasoning "high" for the thinking role
    Then models setup should exit successfully
    And the config file should contain the three selected model roles

  @givn.added @wip
  Scenario: Models setup gives guidance when no provider is configured
    Given no provider is configured
    When I run `watn models` without a terminal
    Then the output should instruct me to run `watn provider`
    And no provider question should be shown

  @givn.added @wip
  Scenario: Available catalog restricts model choices
    Given a configured provider with catalog models "catalog-one" and "catalog-two"
    And the config file contains model "not-in-catalog" for the small role
    When I start `watn models` in a terminal
    Then the small role should require a replacement model
    And the model choices should include only "catalog-one" and "catalog-two"

  @givn.added @wip
  Scenario: Unavailable catalog allows manual model identifiers
    Given a configured provider with an unreachable catalog endpoint
    When I start `watn models` in a terminal
    Then model setup should warn that catalog discovery is unavailable
    And model setup should allow a manually entered model identifier

  @givn.added @wip
  Scenario: Catalog metadata selects supported reasoning efforts for the chosen model
    Given a configured provider catalog model "reasoning-model" supports efforts "low", "medium", and "high"
    And the catalog default effort for "reasoning-model" is "medium"
    When I start `watn models` in a terminal
    And choose "reasoning-model" for the small role
    Then the small reasoning question should show only "low", "medium", and "high"
    And "medium" should be selected by default

  @givn.added @wip
  Scenario: Missing reasoning metadata provides generic efforts and free-form input
    Given a configured provider catalog model "plain-model" has no reasoning metadata
    When I start `watn models` in a terminal
    And choose "plain-model" for the small role
    Then the small reasoning question should warn that supported efforts are unavailable
    And the generic reasoning choices should include "off", "low", "minimal", "medium", and "high"
    And the generic reasoning choices should include a custom effort entry
    When I enter custom reasoning effort "x-high"
    Then the small role should use reasoning "x-high"

  @givn.added @wip
  Scenario: Off reasoning omits the reasoning setting from a request
    Given a configured provider with model "plain-model" for the small role
    And the small role reasoning is "off"
    When I send a request through the configured provider
    Then the API request should omit the reasoning effort

  @givn.added @e2e @wip
  Scenario: Shell setup independently configures completion and Ctrl-W integrations
    Given no Watn-managed shell integrations are installed
    When I start `watn shell` in a terminal
    Then shell setup should show independent completion and Ctrl-W questions
    And the shell choices should include only Bash, Fish, and Zsh
    When I choose Bash for completion
    And choose Zsh for the Ctrl-W shortcut
    Then shell setup should exit successfully
    And Bash should contain a Watn-managed completion block
    And Zsh should contain a Watn-managed Ctrl-W block
    And Fish should remain unchanged

  @givn.added @wip
  Scenario: Shell setup prefills installed integrations and removes only managed blocks when deselected
    Given Bash contains a valid Watn-managed completion block
    And Bash contains user-owned shell content
    When I start `watn shell` in a terminal
    Then Bash completion should be selected
    When I deselect Bash completion
    Then the Watn-managed completion block should be removed from Bash
    And the user-owned shell content should remain

  @givn.added @wip
  Scenario: Shell setup refuses malformed managed markers
    Given Bash contains duplicated Watn completion markers
    When I deselect Bash completion in shell setup
    Then shell setup should report a malformed managed block
    And the Bash file should remain unchanged

  @givn.added @wip
  Scenario: Shell failure does not discard successful shell changes or configuration
    Given Bash can accept a Watn completion block
    And Zsh cannot be modified
    When coordinated setup is confirmed with Bash completion and Zsh Ctrl-W selected
    Then the Bash completion change should remain
    And the provider and model configuration should be saved
    And setup should report a nonzero result

  @givn.added @e2e @wip
  Scenario: Incomplete interactive request opens setup and does not send the original request
    Given a usable provider credential is configured
    And the normal model role is missing
    And the request transport would return a successful answer
    When I start interactive `watn "hello"` in a terminal
    Then the setup coordinator should open
    And the existing provider values should be prefilled
    When I cancel setup before final confirmation
    Then no original chat completion request should be sent

  @givn.added @wip
  Scenario: Non-interactive incomplete request prints setup guidance without probing
    Given no config file exists
    When I run a non-TTY request for "hello"
    Then the exit status should be nonzero
    And stderr should instruct me to run `watn setup` or `watn provider` in a terminal
    And no model catalog request should be sent
    And no original chat completion request should be sent

  @givn.added @wip
  Scenario: Malformed configuration is reported without modification
    Given the config file contains malformed TOML
    And the malformed config content is recorded
    When I run `watn setup`
    Then setup should exit with a configuration error
    And the config file should be byte-for-byte unchanged
