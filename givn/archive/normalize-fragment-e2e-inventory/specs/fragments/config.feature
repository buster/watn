@givn.delta @config

Feature: Configuration management
  The tool reads configuration from multiple layered sources.

  @givn.modified
  Scenario: Environment variable overrides config file
    Given a user config file with provider "openai"
    And environment variable WATN_PROVIDER is set to "custom"
    When I run `watn "hello"`
    Then the request should be sent to provider "custom"

  @givn.modified
  Scenario: CLI flag overrides environment variable
    Given environment variable WATN_MODEL is set to "gpt-4"
    When I run `watn --model gpt-4o "hello"`
    Then the request should use model "gpt-4o"
