Feature: Model configuration entry points

  Scenario: Focused model command is replaced by the unified setup wizard
    Given a complete configuration exists
    When I run `watn models`
    Then the command should be rejected as unavailable
