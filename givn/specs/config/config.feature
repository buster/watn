Feature: Configuration management
  The tool reads configuration from multiple layered sources.

  @e2e

  Scenario: Configure model tiers in config file
    Given a user config file at "~/.config/watn/config.toml" with content:
      """
      [defaults]
      provider = "openai"

      [tiers]
      small = "gpt-4o-mini"
      normal = "gpt-4o"
      thinking = "o3-mini"
      """
    When I run `watn "list files"`
    Then the request should use model "gpt-4o-mini"
    When I run `watn -3 "design a paxos implementation"`
    Then the request should use model "o3-mini"

  @e2e
  Scenario: Environment variable overrides config file
    Given a user config file with provider "openai"
    And environment variable WATN_PROVIDER is set to "custom"
    When I run `watn "hello"`
    Then the request should be sent to provider "custom"

  @e2e
  Scenario: CLI flag overrides environment variable
    Given environment variable WATN_MODEL is set to "gpt-4"
    When I run `watn --model gpt-4o "hello"`
    Then the request should use model "gpt-4o"

  @e2e
  Scenario: Model pricing configured for cost display
    Given a user config file with per-model pricing:
      """
      [pricing]
      "gpt-4o-mini" = { input = 0.15, output = 0.60 }
      "gpt-4o" = { input = 2.50, output = 10.00 }
      """
    When I run `watn "hello"`
    Then the output should contain a cost estimate

  Scenario: Config file with syntax error produces diagnostic
    Given a user config file with invalid TOML content
    When I run `watn "hello"`
    Then the exit status should be 1
    And stderr should contain "config"
    And stderr should contain "parse error"
