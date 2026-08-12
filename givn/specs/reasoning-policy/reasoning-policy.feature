Feature: Reasoning default and persistence policy

  @e2e
  Scenario: Minimal reasoning is persisted and sent
    Given  a model "gpt-4o" assigned to the normal tier with reasoning "minimal"
    When  I run `watn -2 "summarise the changes"`
    Then  the exit status should be 0
    And  the API request should include reasoning with effort "minimal"

  Scenario: A disabled model default selects off even when a default effort is present
    Given  model reasoning metadata has default effort "high", default enabled false, and supported efforts "low", "high"
    When  I resolve the model reasoning default
    Then  the selected reasoning should be "off"

  Scenario: Mandatory reasoning excludes off
    Given  model reasoning metadata is mandatory with supported efforts "low", "high"
    When  I resolve the model reasoning default
    Then  the selected reasoning should be the first valid supported effort "low"

  Scenario: Mandatory reasoning with no usable metadata returns a policy error
    Given  model reasoning metadata is mandatory with supported efforts "bogus"
    And  no existing non-off reasoning value is configured
    When  I resolve the model reasoning default
    Then  the resolver should return a reasoning policy error

  Scenario: Unknown persisted reasoning sends no reasoning request
    Given  a model "gpt-4o" assigned to the normal tier with reasoning "bogus"
    When  I run `watn -2 "summarise the changes"`
    Then  the exit status should be 0
    And  the API request should not include reasoning
