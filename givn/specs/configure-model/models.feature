Feature: Model explorer

  @e2e
  @e2e
  Scenario: Discover models and select tiers interactively
    Given  a configured provider "test" with models endpoint
    And  the endpoint returns models ["gpt-4o-mini", "gpt-4o", "o3-mini", "claude-3-haiku"]
    When  I run `watn models` and select "gpt-4o-mini" for small, "gpt-4o" for normal, and "o3-mini" for thinking
    Then  the config file should contain the selected tier assignments
    And  running `watn "hello"` should use "gpt-4o-mini"

  @e2e
  Scenario: Model explorer without LiteLLM endpoint configured
    Given  no provider is configured
    When  I run `watn models`
    Then  the exit status should be 0
    And  the output should contain instructions for configuring providers manually

  Scenario: Model explorer with openrouter default and env var set
    Given  environment variable OPENROUTER_API_KEY is set to "sk-or-v1-test"
    And  a configured provider "test" with models endpoint
    And  the endpoint returns models ["~deepseek/deepseek-v4-flash-latest", "deepseek/deepseek-v4-pro", "z-ai/glm-5.2"]
    When  I run `watn models` and select "~deepseek/deepseek-v4-flash-latest" for small, "deepseek/deepseek-v4-pro" for normal, and "z-ai/glm-5.2" for thinking
    Then  the config file should contain the selected tier assignments

  Scenario: Model explorer api call fails
    Given  a configured provider "test" with failing models endpoint
    When  I run `watn models`
    Then  the exit status should be non-zero
    And  the output should contain an error message

  Scenario: Model picker shows metadata when available
    Given  a configured provider "test" with models endpoint returning rich metadata
    When  I run `watn models` and select "model-a" for small, "model-a" for normal, and "model-a" for thinking
    Then  the output should contain model metadata
    And  stderr should contain "$0.15/1M in, $0.60/1M out"
    And  stderr should not contain "$-"

  Scenario: Model picker shows model IDs when no metadata available
    Given  a configured provider "test" with models endpoint returning bare model IDs
    When  I run `watn models` and select "model-a" for small, "model-a" for normal, and "model-a" for thinking
    Then  the output should not contain pricing information
  Scenario: Non-terminal model assignment records catalog prices
    Given  a configured provider "test" with models endpoint returning rich metadata
    And  the config file records pricing for "model-a" at 9.99 input and 9.99 output per million tokens
    And  the config file records pricing for "model-b" at 1.23 input and 4.56 output per million tokens
    And  the config file records pricing for "model-plain" at 1.50 input and 2.50 output per million tokens
    When  I run `watn models` and select "model-a" for small, "model-sentinel" for normal, and "model-plain" for thinking
    Then  the config file should record pricing for "model-a" at 0.15 input and 0.60 output per million tokens
    And  the config file should record pricing for "model-b" at 1.23 input and 4.56 output per million tokens
    And  the config file should record pricing for "model-plain" at 1.50 input and 2.50 output per million tokens
    And  the config file should not record pricing for "model-sentinel"

  Scenario: A partial catalog price is not recorded
    Given  a configured provider "test" with models endpoint publishing a partial price
    When  I run `watn models` and select "model-partial" for small, "model-partial" for normal, and "model-partial" for thinking
    Then  the output should not contain pricing information
    And  the config file should not record pricing for "model-partial"
