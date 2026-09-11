@givn.delta @highlight-active-setup-input

Feature: Highlight the active setup input

  @givn.removed @e2e
  Scenario: The green border follows optional shortcut focus
    Given this obsolete scenario is removed

  @givn.added @e2e
  Scenario: The green border follows the shell lists
    Given  no config file exists
    And  no supported provider environment variable is set
    And  the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    When  I start `watn setup` in a terminal
    And  I choose provider "OpenRouter"
    And  I enter the default endpoint and advance to the API key page
    And  choose to store the API key in the configuration
    And  enter API key "sk-shortcut-focus-key" and advance to Small Model
    And  choose "model-small" and "model-middle" with Enter
    And  I type "model-large" on the Thinking Model page
    And  I confirm the Thinking Model selection and configure the shortcut
    Then  the setup wizard should show shell selection with a green border
    When  I advance past the completion selection
    Then  the setup wizard should show shell selection with a green border
