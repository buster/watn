# User Interaction Inventory:
# - `watn --provider openai "hello"` with WATN_OPENAI_API_KEY env var
# - `watn --provider openai "hello"` with no api_key configured and no env var

@givn.delta @providers
Feature: Provider configuration (delta)

  @givn.added @e2e
  Scenario: Provider API key from environment variable
    Given a provider "openai" configured without an api_key
    And environment variable WATN_OPENAI_API_KEY is set to "sk-env-key"
    When I run `watn --provider openai "hello"`
    Then the request should include the Authorization header with "sk-env-key"

  @givn.added
  Scenario: Missing API key produces error
    Given a provider "openai" with no api_key configured and no env var set
    When I run `watn --provider openai "hello"`
    Then the exit status should be 2
    And stderr should contain "api key"
