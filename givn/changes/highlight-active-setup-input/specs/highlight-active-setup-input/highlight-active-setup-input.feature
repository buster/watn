# User Interaction Inventory:
# - launch `watn setup` and inspect the active URL input
# - advance to the API key page and move between its credential locations
# - advance to a model page and switch between model and reasoning input
# - complete model setup and move between the optional shortcut question and shell list
#
@givn.delta @highlight-active-setup-input
Feature: Highlight the active setup input

  @givn.added @e2e
  Scenario: The initial URL input has a green border
    Given no config file exists
    And no supported provider environment variable is set
    When I start `watn setup` in a terminal
    Then the setup wizard should show the active URL input with a green border

  @givn.added @e2e
  Scenario: The green border follows API key focus
    Given no config file exists
    And no supported provider environment variable is set
    When I start `watn setup` in a terminal
    And I enter the default endpoint and advance to the API key page
    Then the setup wizard should show the active credential location with a green border
    And the inactive API key input should retain its default border styling
    When choose to store the API key in the configuration
    Then the setup wizard should show the API key input with a green border
    And the inactive credential location should retain its default border styling

  @givn.added @e2e @wip
  Scenario: The green border follows model focus
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    When I start `watn setup` in a terminal
    And I enter the default endpoint and advance to the API key page
    And choose to store the API key in the configuration
    And enter API key "sk-focus-key" and advance to Small Model
    Then the setup wizard should show the model input with a green border
    And the inactive reasoning input should retain its default border styling
    When I toggle reasoning focus in the setup wizard
    Then the setup wizard should show the reasoning input with a green border
    And the inactive model input should retain its default border styling

  @givn.added @e2e @wip
  Scenario: The green border follows optional shortcut focus
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    When I start `watn setup` in a terminal
    And I enter the default endpoint and advance to the API key page
    And choose to store the API key in the configuration
    And enter API key "sk-shortcut-focus-key" and advance to Small Model
    And choose "model-small" and "model-middle" with Enter
    And I type "model-large" on the Large Model page
    And I confirm the Large Model selection and configure the shortcut
    Then the setup wizard should show the shortcut question with a green border
    And the inactive shell selection should retain its default border styling
    When I enable shortcut configuration
    Then the setup wizard should show shell selection with a green border
    And the inactive shortcut question should retain its default border styling
