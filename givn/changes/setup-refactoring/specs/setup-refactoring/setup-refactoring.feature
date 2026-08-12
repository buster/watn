# User Interaction Inventory:
# - run `watn "question"` for the first time with and without detected credentials
# - run `watn setup` to review and change an existing configuration
# - select all model roles from a catalog or enter them manually after catalog failure
# - inspect contextual help on wide and narrow terminals
# - reconcile completion and Ctrl-W marker blocks in shell startup files

@givn.delta @setup-refactoring
Feature: Reviewed first-run setup

  @givn.added @e2e
  Scenario: Interactive first use reviews a detected credential before saving
    Given no config file exists
    And environment variable OPENROUTER_API_KEY is set to "sk-detected-secret"
    And the ephemeral catalog returns models ["small-model", "normal-model", "thinking-model"] for "/models"
    When I start interactive `watn "show changed files"` in a terminal
    Then the setup wizard should show topics "Provider", "Model roles", "Shell integration", and "Review"
    And the Provider topic should identify "OPENROUTER_API_KEY" as "Detected from environment"
    And the setup terminal should not contain "sk-detected-secret"
    When I accept the detected credential and complete the required model roles
    And I finish setup from Review
    Then the config file should contain api_key exactly "${OPENROUTER_API_KEY}"
    And the config file should not contain "sk-detected-secret"
    And stderr should contain "Setup complete. Retry your command."
    And stdout should be empty
    And the exit status should be 0
    And no original chat completion request should be sent

  @givn.added @e2e
  Scenario: First use without a credential shows a missing recommendation
    Given no config file exists
    And no recognized credential environment variable is set
    When I start `watn setup` in a terminal
    Then the Provider topic should show the OpenRouter endpoint as "Recommended default"
    And the Provider topic should show "OPENROUTER_API_KEY" as "Recommended default" and missing
    And Finish setup should be unavailable until a credential source is supplied
    And no config file should exist before Finish

  @givn.added @e2e
  Scenario: Multiple discovered credentials require an explicit selection
    Given no config file exists
    And environment variable OPENROUTER_API_KEY is set to "sk-openrouter-secret"
    And environment variable WATN_API_KEY is set to "sk-generic-secret"
    When I start `watn setup` in a terminal
    Then the Provider topic should present "OPENROUTER_API_KEY" and "WATN_API_KEY" as separate choices
    And the Provider topic should not select either detected credential automatically
    And the setup terminal should not contain "sk-openrouter-secret" or "sk-generic-secret"

  @givn.added @e2e
  Scenario: A deliberately named credential variable persists only its reference
    Given no config file exists
    And environment variable CUSTOM_LLM_TOKEN is set to "sk-custom-secret"
    When I start `watn setup` in a terminal
    And I choose the Custom provider and enter credential variable "CUSTOM_LLM_TOKEN"
    And I complete the required model roles and finish setup
    Then the config file should contain api_key exactly "${CUSTOM_LLM_TOKEN}"
    And the config file should not contain "sk-custom-secret"

  @givn.added @e2e
  Scenario: A legacy commented template is existing configuration
    Given a legacy commented config template exists
    And environment variable OPENROUTER_API_KEY is set to "sk-existing-secret"
    And the request transport returns a successful response for the implicit OpenRouter request
    When I start interactive `watn "show changed files"` in a terminal
    Then first-run setup should not start solely because the existing file has no active settings
    And the original chat completion request should be sent

  @givn.added
  Scenario: Non-interactive first use requires explicit setup even with a detected credential
    Given no config file exists
    And environment variable OPENROUTER_API_KEY is set to "sk-detected-secret"
    When I run a non-TTY request for "show changed files"
    Then the exit status should be 1
    And stderr should contain actionable guidance to run "watn setup" in a terminal
    And stdout should be empty
    And ratatui should not be initialized
    And no config file should exist
    And no model catalog request should be sent to "/models"
    And no original chat completion request should be sent

  @givn.added
  Scenario: The unified setup command replaces focused commands and selection overrides
    Given a complete configuration exists
    When I run `watn provider`
    Then the command should be rejected as unavailable
    When I run `watn models`
    Then the command should be rejected as unavailable
    When I run `watn --provider custom "show changed files"`
    Then the command should reject the removed provider option
    When I run `watn --model alternate-model "show changed files"`
    Then the command should reject the removed model option
    When I run `watn --set-small alternate-model`
    Then the command should reject the removed model-assignment option
    And generated shell completions should not advertise removed setup commands or options
    And `watn -1`, `watn -2`, and `watn -3` should remain valid request tier selectors

  @givn.added
  Scenario: Removed environment selection variables do not override persisted configuration
    Given a complete persisted configuration exists
    And environment variable WATN_PROVIDER is set to "custom"
    And environment variable WATN_MODEL is set to "alternate-model"
    When I run a request with the complete persisted configuration
    Then the persisted provider and model roles should remain the request selection

  @givn.added @e2e
  Scenario: Contextual help remains visible beside settings on wide terminals
    Given a setup draft with an active Provider endpoint field
    When I render `watn setup` in a wide terminal
    Then the active-setting help should explain what the endpoint is
    And the active-setting help should explain what it enables
    And the active-setting help should include a recommendation
    And the active-setting help should include a requirement or tradeoff
    And the help should appear beside the settings

  @givn.added @e2e
  Scenario: Contextual help remains visible below settings on narrow terminals
    Given a setup draft with an active Provider endpoint field
    When I render `watn setup` in a narrow terminal
    Then the active-setting help should explain what the endpoint is
    And the active-setting help should explain what it enables
    And the active-setting help should include a recommendation
    And the active-setting help should include a requirement or tradeoff
    And the help should appear below the settings

  @givn.added @e2e
  Scenario: Model roles are reviewed together after a provider change
    Given a complete config has model roles "old-small", "old-normal", and "old-thinking"
    And the configured provider catalog returns models ["new-small", "new-normal", "new-thinking"]
    When I start `watn setup` in a terminal
    And I change the Provider endpoint
    Then the Model roles topic should show Small / fast, Balanced / normal, and Thinking together
    And the existing model roles should be marked "Needs attention"
    And Finish setup should be unavailable until the model roles are reviewed
    When I select or explicitly retain each model role
    Then the Model roles topic should be complete

  @givn.added @e2e
  Scenario: Manual roles may finish with an unverified catalog warning
    Given no config file exists
    And the model catalog transport fails for "/models"
    When I start `watn setup` in a terminal
    And I provide a valid custom endpoint and credential source
    And I enter manual model IDs "manual-small", "manual-normal", and "manual-thinking"
    Then each manual model role should show reasoning "off"
    And Review should show an unverified catalog warning
    And Finish setup should be available
    When I finish setup
    Then the config file should contain the three manual model roles
    And the config file should contain reasoning "off" for each manual role

  @givn.added @wip @e2e
  Scenario: Review is the only configuration commit boundary
    Given no config file exists
    And the ephemeral catalog returns models ["small-model", "normal-model", "thinking-model"] for "/models"
    When I complete Provider and Model roles in `watn setup`
    Then no config file should exist before Finish
    And Review should summarize the endpoint, credential source, model roles, reasoning, shell changes, and warnings
    When I discard setup from Review
    Then no config file should exist

  @givn.added @e2e
  Scenario: Cancelling an existing setup keeps its configuration byte-for-byte unchanged
    Given an existing config contains provider "custom" with credential "sk-old-key"
    And the config contains known tiers, reasoning, pricing, and LiteLLM settings
    When I start `watn setup` in a terminal
    Then the Provider topic should prefill supported saved values as "Loaded from config"
    When I edit the draft and cancel setup
    Then the config file should be byte-for-byte unchanged
    And no original chat completion request should be sent

  @givn.added @e2e
  Scenario: Finish reconciles shell marker blocks without persisting shell state in TOML
    Given shell startup files contain user content and existing watn completion and shortcut marker blocks
    When I start `watn setup` in a terminal
    Then Shell integration should derive its selections from the marker blocks
    When I uncheck the existing completion block and check a missing shortcut block
    And I finish setup
    Then the completion marker block should be removed
    And the shortcut marker block should be installed
    And unrelated shell startup-file content should be unchanged
    And the config file should not contain shell integration state

  @givn.added @wip @e2e
  Scenario: Shell failure reports partial completion after configuration commits
    Given a valid reviewed setup draft selects shell integrations
    And one selected shell startup file cannot be reconciled
    When I finish setup
    Then the supported configuration changes should be saved
    And successful shell changes should remain applied
    And stderr should identify the failed shell integration and retry guidance
    And the exit status should be non-zero

  @givn.added @wip
  Scenario: OpenAI setup uses the explicit identity and credential mapping
    Given no config file exists
    And environment variable OPENAI_API_KEY is set to "sk-openai-secret"
    When I choose the OpenAI provider in `watn setup`
    Then the Provider topic should show endpoint "https://api.openai.com/v1"
    And the Provider topic should identify "OPENAI_API_KEY" as "Detected from environment"
    And the config should persist provider "openai" and api_key exactly "${OPENAI_API_KEY}"
    And the config should not contain "sk-openai-secret"

  @givn.added @wip
  Scenario: Finish preserves supported configuration outside the setup draft
    Given an existing config contains provider "custom" with credential "sk-old-key"
    And the config contains known tiers, reasoning, pricing, and LiteLLM settings
    When I finish an otherwise unchanged setup draft
    Then the existing default model, provider default model, pricing, and LiteLLM settings should remain unchanged
    And the config should contain no origin or shell integration fields

  @givn.added @wip @e2e
  Scenario: Setup catalog discovery honors the configured LiteLLM source
    Given an existing config has a custom chat provider and a configured LiteLLM catalog source
    And the LiteLLM catalog returns models ["catalog-small", "catalog-normal", "catalog-thinking"]
    When I start `watn setup` in a terminal
    Then model discovery should request the configured LiteLLM endpoint
    And the custom chat provider should receive no model catalog request
    And Review should identify the catalog source separately from the chat provider

  @givn.added @wip @e2e
  Scenario: Ctrl-C during catalog discovery discards the setup draft
    Given no config file exists
    And the model catalog response is delayed
    When I start `watn setup` in a terminal and press Ctrl-C during discovery
    Then the exit status should be 130
    And no config file should exist
    And no shell startup file should be changed
