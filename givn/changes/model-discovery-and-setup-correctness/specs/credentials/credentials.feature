# User Interaction Inventory:
# - start the interactive `watn models` command and confirm model tiers
# - run `watn <question>` through a configured provider

@givn.delta @credential-sources
Feature: Credential source preservation during model discovery

  @givn.added @e2e
  Scenario: Interactive model discovery preserves an OpenRouter environment credential
    Given no config file exists
    And environment variable OPENROUTER_API_KEY is set to "sk-or-v1-test"
    And the ephemeral E2E transport returns models ["model-small", "model-normal", "model-thinking"] for "/models"
    When I start the shared `watn models` wizard in a terminal
    And choose "model-small" and "model-normal" with Enter
    And I type "model-thinking" on the Large Model page
    And I confirm the Large Model selection with Enter
    Then setup should exit successfully
    And the output should contain "Tiers configured"
    And the config file should contain default provider "openrouter"
    And the config file should contain api_key exactly "${OPENROUTER_API_KEY}"
    And the config file should not contain "sk-or-v1-test"
    And the config file should contain small tier "model-small", middle tier "model-normal", and large tier "model-thinking"

  @givn.modified @e2e
  Scenario: A literal saved credential is authoritative over environment fallback
    Given a configured provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved api_key is "sk-saved-literal"
    And environment variable WATN_CUSTOM_API_KEY is set to "sk-env-different"
    And environment variable WATN_API_KEY is set to "sk-generic-different"
    And its saved default model is "custom-model"
    When I run `watn "hello"`
    Then the exit status should be 0
    And the API request should use API key "sk-saved-literal"
    And the environment fallback values should not be used

  @givn.added
  Scenario: A missing saved environment credential fails before discovery
    Given a configured provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved api_key is "${WATN_CUSTOM_API_KEY}"
    And environment variable WATN_API_KEY is set to "sk-generic-fallback"
    And environment variable WATN_CUSTOM_API_KEY is absent
    When I run `watn models`
    Then the exit status should classify the failure as authentication
    And no model catalog request should be sent
    And the saved api_key should remain exactly "${WATN_CUSTOM_API_KEY}"

  @givn.added
  Scenario: Provider-specific environment fallback precedes generic fallback
    Given a configured provider "custom" with endpoint "https://llm.example.com/v1"
    And its saved api_key is absent
    And environment variable WATN_CUSTOM_API_KEY is set to "sk-provider-fallback"
    And environment variable WATN_API_KEY is set to "sk-generic-fallback"
    And its saved default model is "custom-model"
    When I run `watn "hello"`
    Then the exit status should be 0
    And the API request should use API key "sk-provider-fallback"
    And the generic environment fallback should not be used
