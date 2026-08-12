@wip
Feature: Interactive provider setup

  @e2e
  Scenario: Interactive model catalog failure after provider setup preserves the provider and sends no request
    Given  no config file exists
    And  the model catalog transport returns HTTP 500 for "/models"
    When  I start `watn setup` in a terminal
    And  accept the default endpoint in provider setup
    And  paste credential "sk-first-run"
    And  confirm the credential before loading models
    Then  the setup wizard should report the catalog failure
    And  the config file should contain provider "openrouter" with endpoint "https://openrouter.ai/api/v1"
    And  the config file should not contain selected tier assignments
    And  no original chat completion request should be sent

  @e2e
  Scenario: Cancelling before credential confirmation does not save a provider
    Given  no config file exists
    When  I start `watn setup` in a terminal
    And  cancel setup before confirming the credential
    Then  setup should exit with cancellation
    And  the setup wizard should ask whether to save current settings
    And  the config file should not contain a provider entry for the attempted setup

  @e2e
  Scenario: Cancelling after credential confirmation preserves the provider
    Given  no config file exists
    When  I confirm provider endpoint "https://llm.example.com/v1" and credential "sk-confirmed"
    And  the credential confirmation is persisted
    And  cancel model setup
    Then  the config file should contain provider "custom" with endpoint "https://llm.example.com/v1"
    And  the config file should contain api_key exactly "sk-confirmed"
    And  the config file should not contain selected tier assignments

  @e2e
  Scenario: Assigning tiers does not replace the active provider or catalog settings
    Given  a configured provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog returns models ["model-a", "model-b", "model-c"]
    When  I run `watn models` and select "model-a" for small, "model-b" for normal, and "model-c" for thinking
    Then  the output should contain "Tiers configured"
    And  the default provider should remain "custom"
    And  the LiteLLM settings should remain unchanged
    And  the config file should contain the selected tier assignments
