# User Interaction Inventory:
# - run `watn <question>` with a selected model tier and observe the request

@givn.delta @reasoning-policy
Feature: Reasoning default and persistence policy

  @givn.added @e2e @wip
  Scenario: Minimal reasoning is persisted and sent
    Given a model "gpt-4o" assigned to the normal tier with reasoning "minimal"
    When I run `watn -2 "summarise the changes"`
    Then the exit status should be 0
    And the API request should include reasoning with effort "minimal"

  @givn.added
  Scenario: A disabled model default selects off even when a default effort is present
    Given model reasoning metadata has default effort "high", default enabled false, and supported efforts "low", "high"
    When I resolve the model reasoning default
    Then the selected reasoning should be "off"

  @givn.added
  Scenario: Mandatory reasoning excludes off
    Given model reasoning metadata is mandatory with supported efforts "low", "high"
    When I resolve the model reasoning default
    Then the selected reasoning should be the first valid supported effort "low"

  @givn.added
  Scenario: Mandatory reasoning with no usable metadata returns a policy error
    Given model reasoning metadata is mandatory with supported efforts "bogus"
    And no existing non-off reasoning value is configured
    When I resolve the model reasoning default
    Then the resolver should return a reasoning policy error

  @givn.added @wip
  Scenario: Unknown persisted reasoning sends no reasoning request
    Given a model "gpt-4o" assigned to the normal tier with reasoning "bogus"
    When I run `watn -2 "summarise the changes"`
    Then the exit status should be 0
    And the API request should not include reasoning

  @givn.added @wip
  Scenario: Non-TTY model assignment never persists empty reasoning values
    Given a configured provider "test" with models endpoint
    And the endpoint returns models ["model-a", "model-b", "model-c"]
    When I run `watn models` and select "model-a" for small, "model-b" for normal, and "model-c" for thinking
    Then the config file should not contain an empty reasoning value

  @givn.added @wip
  Scenario: Existing reasoning survives selection without a valid replacement
    Given a configured provider "test" with models that have no reasoning metadata
    And the existing small tier reasoning is "medium"
    When I select a new model for the small tier through the non-TTY model assignment
    Then the saved small tier reasoning should remain "medium"
    And the new small-tier model should be persisted
