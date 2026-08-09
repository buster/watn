# User Interaction Inventory:
# - run `watn provider` and complete the interactive provider setup
# - run a normal `watn` command with no recognized provider and complete automatic provider and model setup

@givn.delta @provider-setup

Feature: Interactive provider setup
  A user can configure an OpenAI-compatible provider and continue to model setup
  without manually editing the configuration file.

  @givn.added @e2e @wip
  Scenario: Configure OpenRouter with an environment-backed credential
    Given no provider is configured
    And environment variable OPENROUTER_API_KEY is set to "sk-or-v1-test"
    And the ephemeral E2E transport returns a successful chat completion for "/chat/completions"
    When I start `watn provider` in a terminal
    Then the setup terminal should show endpoint prompt default "https://openrouter.ai/api/v1"
    And the setup terminal should show pasted and environment credential choices
    When I accept the OpenRouter endpoint
    And choose environment variable "OPENROUTER_API_KEY" for the credential
    Then the config file should contain default provider "openrouter"
    And the config file should contain endpoint exactly "https://openrouter.ai/api/v1"
    And the config file should contain api_key exactly "${OPENROUTER_API_KEY}"
    And the config file should not contain "sk-or-v1-test"
    When I run `watn "hello"`
    Then the request should hit the ephemeral E2E transport path "/chat/completions"
    And the API request should use API key "sk-or-v1-test"
    And the persisted provider endpoint should still be exactly "https://openrouter.ai/api/v1"

  @givn.added
  Scenario: Configure a custom endpoint with a pasted credential
    Given no provider is configured
    When provider setup accepts endpoint "https://llm.example.com/v1"
    And provider setup accepts pasted credential "sk-custom-key"
    Then provider setup should return configured provider "custom"
    And the config file should contain default provider "custom"
    And the config file should contain endpoint exactly "https://llm.example.com/v1"
    And the config file should contain api_key exactly "sk-custom-key"

  @givn.added
  Scenario: Configure a custom provider with the generic environment variable
    Given no provider is configured
    And environment variable WATN_API_KEY is set to "sk-watn-test"
    When provider setup accepts endpoint "https://llm.example.com/v1"
    Then provider setup should suggest environment variable "WATN_API_KEY"
    When provider setup chooses environment variable "WATN_API_KEY"
    Then the config file should contain default provider "custom"
    And the config file should contain api_key exactly "${WATN_API_KEY}"
    And the config file should not contain "sk-watn-test"

  @givn.added @e2e @wip
  Scenario: First normal use starts provider setup and then model setup
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["model-small", "model-normal", "model-thinking"] for "/models"
    When I start interactive `watn "hello"` in a terminal
    And accept the default endpoint in provider setup
    And paste credential "sk-first-run"
    Then the terminal should show model setup after provider setup
    When I select "model-small" for small, "model-normal" for normal, and "model-thinking" for thinking
    Then automatic onboarding should exit successfully after model selection
    And the config file should contain default provider "openrouter"
    And the config file should contain endpoint exactly "https://openrouter.ai/api/v1"
    And the config file should contain api_key exactly "sk-first-run"
    And the config file should contain the selected tier assignments
    And the model catalog request should hit ephemeral path "/models"
    And no original chat completion request should be sent

  @givn.added
  Scenario: A recognized environment credential skips automatic provider setup
    Given no config file exists
    And environment variable OPENROUTER_API_KEY is set to "sk-or-v1-test"
    And the request transport returns a successful response for the implicit OpenRouter request
    When I run `watn "hello"`
    Then provider setup should not start
    And the request should use the implicit OpenRouter endpoint
    And the API request should use API key "sk-or-v1-test"
    And the process should not initialize ratatui

  @givn.added
  Scenario: A saved provider with a default model skips automatic provider setup
    Given a configured default provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved credential is "sk-custom-key"
    And its saved default model is "custom-model"
    When I run `watn "hello"`
    Then provider setup should not start
    And the API request should be sent to "https://llm.example.com/v1/chat/completions"
    And the request should use model "custom-model"

  @givn.added
  Scenario: Invalid endpoint remains in provider setup for correction
    Given no provider is configured
    When provider setup receives endpoint "not a URL"
    Then provider setup should show validation error "endpoint must be an HTTP or HTTPS URL"
    And provider setup should not return a configured provider
    And the config file should not contain a provider entry for the attempted setup

  @givn.added
  Scenario: Empty credential remains in provider setup for correction
    Given no provider is configured
    When provider setup receives endpoint "https://llm.example.com/v1"
    And provider setup receives an empty pasted credential
    Then provider setup should show validation error "credential cannot be empty"
    And provider setup should not return a configured provider
    And the config file should not contain a provider entry for the attempted setup

  @givn.added
  Scenario: A missing saved environment reference fails authentication without a request
    Given a configured provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved api_key is "${MISSING_WATN_KEY}"
    And environment variable MISSING_WATN_KEY is not set
    And its saved default model is "custom-model"
    When I run `watn --provider custom "hello"`
    Then the exit status should be 2
    And stderr should contain "authentication"
    And stderr should contain "MISSING_WATN_KEY"
    And no request should be sent to "/chat/completions"

  @givn.added
  Scenario: A saved OpenRouter endpoint takes precedence over the built-in endpoint
    Given a saved default provider "openrouter" with endpoint "https://saved-openrouter.example/v1"
    And its saved credential is "sk-saved-openrouter"
    And its saved default model is "saved-model"
    When I resolve the saved OpenRouter provider for a request
    Then the selected endpoint should be exactly "https://saved-openrouter.example/v1"
    And the built-in endpoint "https://openrouter.ai/api/v1" should not be selected

  @givn.added
  Scenario: An explicitly named environment variable is persisted and expanded at use time
    Given no provider is configured
    And environment variable CUSTOM_LLM_TOKEN is set to "sk-explicit-test"
    When provider setup accepts endpoint "https://llm.example.com/v1"
    And provider setup chooses explicitly named environment variable "CUSTOM_LLM_TOKEN"
    Then the config file should contain api_key exactly "${CUSTOM_LLM_TOKEN}"
    And the config file should not contain "sk-explicit-test"
    When I send a request through the configured provider
    Then the API request should use API key "sk-explicit-test"

  @givn.added
  Scenario: Trailing slashes are normalized before persistence and requests
    Given no provider is configured
    When provider setup accepts endpoint "https://llm.example.com/v1///"
    And provider setup accepts pasted credential "sk-custom-key"
    Then the config file should contain endpoint exactly "https://llm.example.com/v1"
    And the model catalog URL should be exactly "https://llm.example.com/v1/models"
    And the chat completion URL should be exactly "https://llm.example.com/v1/chat/completions"

  @givn.added
  Scenario: Rerunning provider setup preserves unrelated configuration
    Given a config file contains provider "legacy" with endpoint "https://legacy.example/v1"
    And the config file contains tiers, pricing, and LiteLLM settings
    When provider setup accepts endpoint "https://new.example/v1"
    And provider setup accepts pasted credential "sk-new-key"
    Then the default provider should be "custom"
    And provider "legacy" should remain unchanged
    And the existing tiers, pricing, and LiteLLM settings should remain unchanged
    And only the fixed provider entry "custom" should be replaced or created

  @givn.added
  Scenario: Escape cancellation preserves the existing provider configuration
    Given an existing config contains provider "legacy" with credential "sk-old-key"
    When provider setup is cancelled with Escape
    Then the exit status should be 1
    And the config file should be byte-for-byte unchanged
    And provider "legacy" should still contain credential "sk-old-key"
    And no request should be sent to "/chat/completions"

  @givn.added
  Scenario: Ctrl-C cancellation preserves the existing provider configuration
    Given an existing config contains provider "legacy" with credential "sk-old-key"
    When provider setup is cancelled with Ctrl-C
    Then the exit status should be 130
    And the config file should be byte-for-byte unchanged
    And provider "legacy" should still contain credential "sk-old-key"
    And no request should be sent to "/chat/completions"

  @givn.added
  Scenario: Model catalog failure after provider setup preserves the provider and sends no request
    Given no config file exists
    And the model catalog transport returns HTTP 500 for "/models"
    When automatic onboarding saves provider endpoint "https://llm.example.com/v1" and credential "sk-first-run"
    And automatic model setup attempts catalog discovery
    Then the exit status should be 2
    And the config file should contain provider "custom" with endpoint "https://llm.example.com/v1"
    And the config file should not contain selected tier assignments
    And no original chat completion request should be sent

  @givn.added
  Scenario: The explicit provider command ends without model setup
    Given no provider is configured
    When the explicit provider setup command saves endpoint "https://llm.example.com/v1" and credential "sk-custom-key"
    Then the exit status should be 0
    And the config file should contain provider "custom"
    And model setup should not start
    And no model catalog request should be sent to "/models"

  @givn.added @wip
  Scenario: Non-TTY first use prints setup guidance instead of starting ratatui
    Given no config file exists
    And no recognized provider environment variable is set
    When I run a non-TTY request for "hello"
    Then the exit status should be 1
    And stderr should contain actionable guidance to run "watn provider" in a terminal
    And stderr should contain the configuration path "config.toml"
    And stderr should not contain ANSI escape sequences
    And ratatui should not be initialized
    And no model catalog request should be sent to "/models"
    And no original chat completion request should be sent

  @givn.added @wip
  Scenario: A literal saved credential is authoritative over environment fallback
    Given a configured provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved api_key is "sk-saved-literal"
    And environment variable WATN_CUSTOM_API_KEY is set to "sk-env-different"
    And environment variable WATN_API_KEY is set to "sk-generic-different"
    And its saved default model is "custom-model"
    When I send a request through the configured provider
    Then the API request should use API key "sk-saved-literal"
    And the environment fallback values should not be used

  @givn.added @wip
  Scenario: Explicit provider selection from the environment preserves missing-key errors
    Given environment variable WATN_PROVIDER is set to "custom"
    And provider "custom" has endpoint "https://llm.example.com/v1" and no api_key
    And environment variable WATN_CUSTOM_API_KEY is not set
    And environment variable WATN_API_KEY is not set
    When I run `watn "hello"`
    Then provider setup should not start
    And the exit status should be 2
    And stderr should contain "api key"
    And no original chat completion request should be sent

  @givn.added @wip
  Scenario: Saving provider configuration secures a world-readable file
    Given an existing provider config file has Unix mode "0644"
    When provider setup saves endpoint "https://llm.example.com/v1" and credential "sk-new-key"
    Then the config file should have Unix mode "0600"
    And the saved provider endpoint should be "https://llm.example.com/v1"
    And the save should use the existing direct-write behavior without an atomic-file promise
