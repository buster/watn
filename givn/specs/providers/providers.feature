Feature: Provider configuration
  A user can configure any OpenAI-compatible API as a provider.

  @e2e
  Scenario: Custom OpenAI-compatible provider from config
    Given a user config file with a provider definition:
      """
      [providers.custom]
      endpoint = "https://custom-llm.example.com/v1"
      api_key = "sk-custom-key"
      default_model = "custom-model-1"
      """
    When I run `watn --provider custom "hello"`
    Then the request should be sent to "https://custom-llm.example.com/v1"

  @e2e
  Scenario: LiteLLM endpoint in config for model discovery
    Given a user config file with:
      """
      [litellm]
      endpoint = "http://localhost:4000"
      api_key = "sk-litellm-key"
      """
    When I run `watn models`
    Then it should query the model list at "http://localhost:4000/models"

  @e2e
  Scenario: Provider API key from environment variable
    Given a provider "openai" configured without an api_key
    And environment variable WATN_OPENAI_API_KEY is set to "sk-env-key"
    When I run `watn --provider openai "hello"`
    Then the request should include the Authorization header with "sk-env-key"

  Scenario: Unknown provider produces error
    When I run `watn --provider nonexistent "hello"`
    Then the exit status should be 1
    And stderr should contain "unknown provider"

  Scenario: Missing API key produces error
    Given a provider "openai" with no api_key configured and no env var set
    When I run `watn --provider openai "hello"`
    Then the exit status should be 2
    And stderr should contain "api key"
