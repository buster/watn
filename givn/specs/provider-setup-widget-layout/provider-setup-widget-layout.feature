Feature: Usable provider setup and model picker layouts

  @e2e
  Scenario: Provider setup separates choices, details, and guidance
    Given  no provider is configured
    When  I start `watn provider` in a terminal
    Then  the provider setup should show a bordered "Provider setup" panel
    And  provider setup should show a selectable credential source list
    And  provider setup should show provider details in aligned rows
    And  provider setup should show setup guidance as a paragraph
    When  I enter an invalid endpoint in provider setup
    Then  provider setup should show validation message "endpoint must be an HTTP or HTTPS URL"
    When  I restore the default endpoint and enter pasted credential "sk-layout-secret" in provider setup
    Then  provider setup should mask pasted credentials

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
