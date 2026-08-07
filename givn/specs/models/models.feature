Feature: Model explorer

  @e2e
  Scenario: Discover models and select tiers interactively
    Given  a LiteLLM endpoint at "http://localhost:4000"
    And  the endpoint returns models ["gpt-4o-mini", "gpt-4o", "o3-mini", "claude-3-haiku", "claude-3-sonnet"]
    When  I run `watn models` and select "gpt-4o-mini" for small, "gpt-4o" for normal, and "o3-mini" for thinking
    Then  the config file should contain the selected tier assignments
    And  running `watn "hello"` should use "gpt-4o-mini"

  @e2e
  Scenario: Model explorer without LiteLLM endpoint configured
    Given  no LiteLLM endpoint is configured
    When  I run `watn models`
    Then  the exit status should be 0
    And  the output should contain instructions for configuring providers manually
