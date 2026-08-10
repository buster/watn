Feature: Unified setup wizard

  @e2e
  Scenario: Setup wizard guides provider and model configuration page by page
    Given  no config file exists
    And  no supported provider environment variable is set
    And  the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    When  I start `watn setup` in a terminal
    Then  the setup wizard should show tabs "URL", "API key", "Small Model", "Middle Model", "Large Model"
    And  the setup wizard should show the URL page as active
    And  the setup wizard should explain OpenAI and LiteLLM compatibility
    And  the setup wizard should show a visible cursor on the active input
    When  I enter the default endpoint and advance to the API key page
    And  choose to store the API key in the configuration
    And  enter API key "sk-wizard-key" and advance to Small Model
    And  choose "model-small" and "model-middle" with Enter
    When  I type "model-large" on the Large Model page
    Then  the setup wizard should show the Large Model page as active
    When  I confirm the Large Model selection with Enter
    Then  setup should exit successfully
    And  the config file should contain api_key exactly "sk-wizard-key"
    And  the config file should contain small tier "model-small", middle tier "model-middle", and large tier "model-large"

  @e2e
  Scenario: Models command opens the shared wizard on Small Model
    Given  a configured provider "test" with a long model list
    When  I start the shared `watn models` wizard in a terminal
    Then  the setup wizard should show the Small Model page as active
    And  the setup wizard should show the URL and API key tabs
    And  the setup wizard should show model choices in a table
    And  the setup wizard should show model-specific reasoning options
    When  I choose the second model and advance with Enter
    Then  the setup wizard should show the Middle Model page as active

  @e2e
  Scenario: Escape asks whether to save or discard current setup
    Given  an existing config contains provider "legacy" with credential "sk-old-key"
    When  I start `watn setup` in a terminal
    And  press Escape in the setup wizard
    Then  the setup wizard should ask whether to save current settings
    When  I choose to discard current setup
    Then  the config file should be byte-for-byte unchanged
