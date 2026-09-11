@givn.delta @models

Feature: Model explorer

  @givn.modified
  Scenario: Model picker shows metadata when available
    Given  a configured provider "test" with models endpoint returning rich metadata
    When  I run `watn models` and select "model-a" for small, "model-a" for normal, and "model-a" for thinking
    Then  the output should contain model metadata
    And  stderr should contain "$0.15/1M in, $0.60/1M out"
    And  stderr should not contain "$-"

  @givn.added
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

  @givn.added
  Scenario: A partial catalog price is not recorded
    Given  a configured provider "test" with models endpoint publishing a partial price
    When  I run `watn models` and select "model-partial" for small, "model-partial" for normal, and "model-partial" for thinking
    Then  the output should not contain pricing information
    And  the config file should not record pricing for "model-partial"
