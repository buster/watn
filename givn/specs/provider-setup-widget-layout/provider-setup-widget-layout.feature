@wip
Feature: Usable provider setup and model picker layouts

  @e2e
  Scenario: Provider setup separates choices, details, and guidance
    Given  no config file exists
    When  I start `watn provider` in a terminal
    Then  the setup wizard should show the URL page as active
    And  the setup wizard should show a visible cursor on the active input
    When  I advance to the API key page in provider setup
    Then  the setup wizard should show the API key page as active

  @e2e
  Scenario: Model picker makes tiers and long model lists easy to scan
    Given  a configured provider "test" with a long model list
    When  I start `watn models` in a terminal
    Then  the model picker should show a bordered "Model picker" panel
    And  the model picker should show tabs for the three model tiers
    And  the model picker should show models in aligned columns
    And  the model picker should show a scrollbar for the model list
    When  I move to the next model and advance to the normal tier
    Then  the model picker should show the active tier "normal"
    And  the model picker should keep the selected row visible
