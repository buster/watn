# User Interaction Inventory:
# - start interactive `watn "hello"` with an incomplete configuration in a terminal
#   (this delta modifies the existing first-use onboarding scenario: the quick
#   setup handles a missing config file, while an existing-but-incomplete
#   configuration still opens the full setup coordinator)

@givn.delta @provider-setup

Feature: Interactive provider setup

  @givn.modified @e2e
  Scenario: First normal use starts provider setup and then model setup
    Given an existing openrouter configuration without a credential
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["model-small", "model-normal", "model-thinking"] for "/models"
    When I start interactive `watn "hello"` in a terminal
    And accept the default endpoint in provider setup
    And paste credential "sk-first-run"
    Then the terminal should show model setup after provider setup
    When I select "model-small" for small, "model-normal" for normal, and "model-thinking" for thinking
    Then automatic onboarding should exit successfully after model selection
    And the config file should contain default provider "openrouter"
    And the config file should contain endpoint exactly "https://openrouter.ai/api/v1"
    And the config file should contain api_key exactly "sk-first-run"
    And the config file should contain the selected tier assignments
    And the model catalog request should hit ephemeral path "/models"
    And no original chat completion request should be sent
