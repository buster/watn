@givn.delta @ratatui-model-picker

Feature: Keyboard-driven model picker

  @givn.modified
  Scenario: Model entry shows additional metadata when available
    Given  the catalog has models "model-a" and "model-b" where "model-a" has pricing
    When  I format the model list for display
    Then  the entry for "model-a" shows a price
    And  the displayed price for "model-a" is "$0.15/1M in, $0.60/1M out"
    And  the entry for "model-b" shows no price

  @givn.added
  Scenario: Interactive model table shows published prices per million tokens
    Given  a configured provider with a priced catalog:
      | model        | input | output |
      | priced-model | 0.15  | 0.60   |
    When  I start `watn models` in a terminal
    Then  the model table should show "$0.15/$0.60"
    And  the model table header should show "$/1M"
