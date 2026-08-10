# User Interaction Inventory:
# - open the existing provider setup entry point and identify the active wizard page

@givn.delta @unified-setup-wizard
Feature: Existing setup entry points use the wizard

  @givn.modified @e2e
  Scenario: Provider setup separates choices, details, and guidance
    Given no config file exists
    When I start `watn provider` in a terminal
    Then the setup wizard should show the URL page as active
    And the setup wizard should show a visible cursor on the active input
    When I advance to the API key page in provider setup
    Then the setup wizard should show the API key page as active
