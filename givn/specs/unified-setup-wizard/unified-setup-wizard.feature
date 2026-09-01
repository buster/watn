Feature: Unified setup wizard

  @e2e
  Scenario: Setup wizard guides provider and model configuration page by page
    Given  no config file exists
    And  no supported provider environment variable is set
    And  the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    When  I start `watn setup` in a terminal
    And  I choose provider "OpenRouter"
    Then  the setup wizard should show the provider controls and guidance
    When  I configure the provider and models through the wizard
    And  I complete the optional shell pages without integrations
    Then  setup should exit successfully
    And  the config file should contain api_key exactly "sk-wizard-key"
    And  the config file should contain small tier "model-small", normal tier "model-middle", and thinking tier "model-large"

  @e2e
  Scenario: Models command opens the shared wizard on Small Model
    Given  a configured provider "test" with a long model list
    When  I start the shared `watn models` wizard in a terminal
    Then  the setup wizard should show the Small Model page as active
    And  the setup wizard should show the URL and API key tabs
    And  the setup wizard should show model choices in a table
    When  I choose the second model and advance with Enter
    Then  the setup wizard should show the Small Reasoning page as active
    And  the setup wizard should show model-specific reasoning options
    When  I confirm the Small Reasoning selection with Enter
    Then  the setup wizard should show the Normal Model page as active

  @e2e
  Scenario: Escape asks whether to save or discard current setup
    Given  an existing config contains provider "legacy" with credential "sk-old-key"
    When  I start `watn setup` in a terminal
    And  press Escape in the setup wizard
    Then  the setup wizard should ask whether to save current settings
    When  I choose to discard current setup
    Then  the config file should be byte-for-byte unchanged
