@givn.delta @quicksetup
Feature: Quick setup parameters

  @e2e @givn.added
  Scenario: Quick setup parameters prefill the dialog with an environment credential
    Given no watn configuration exists
    And provider requests are captured by a sentinel
    And bash, zsh, and fish are available on the path
    When I start `watn quicksetup` with these parameters:
      """
      --url https://openrouter.ai/api/v1
      --key ${OPENROUTER_API_KEY}
      --model ~anthropic/claude-haiku-latest:nitro
      """
    Then the endpoint question should suggest "https://openrouter.ai/api/v1"
    When I accept the suggested endpoint, credential, and models
    And I deselect all shell integrations and confirm
    Then quick setup should exit successfully
    And the config file should contain provider "openrouter"
    And the config file should contain endpoint exactly "https://openrouter.ai/api/v1"
    And the config file should contain credential "${OPENROUTER_API_KEY}"
    And the config file should contain small model "~anthropic/claude-haiku-latest:nitro"
    And the config file should contain normal model "~anthropic/claude-haiku-latest:nitro"
    And the config file should contain thinking model "~anthropic/claude-haiku-latest:nitro"
    And no model catalog request should be sent

  @e2e @givn.added
  Scenario: A small-model parameter seeds the remaining tiers
    Given no watn configuration exists
    And provider requests are captured by a sentinel
    And bash, zsh, and fish are available on the path
    When I start `watn quicksetup` with these parameters:
      """
      --url https://openrouter.ai/api/v1
      --key sk-abc123
      --model-small google/gemini-3.7-flash
      """
    When I accept the suggested endpoint, credential, and models
    And I deselect all shell integrations and confirm
    Then quick setup should exit successfully
    And the config file should contain small model "google/gemini-3.7-flash"
    And the config file should contain normal model "google/gemini-3.7-flash"
    And the config file should contain thinking model "google/gemini-3.7-flash"

  @e2e @givn.added
  Scenario: Tier parameters override the shared model prefill
    Given no watn configuration exists
    And provider requests are captured by a sentinel
    And bash, zsh, and fish are available on the path
    When I start `watn quicksetup` with these parameters:
      """
      --url https://openrouter.ai/api/v1
      --key sk-abc123
      --model ~anthropic/claude-haiku-latest:nitro
      --model-small google/gemini-3.7-flash
      --model-thinking deepseek/deepseek-v4-pro
      """
    When I accept the suggested endpoint, credential, and models
    And I deselect all shell integrations and confirm
    Then quick setup should exit successfully
    And the config file should contain small model "google/gemini-3.7-flash"
    And the config file should contain normal model "~anthropic/claude-haiku-latest:nitro"
    And the config file should contain thinking model "deepseek/deepseek-v4-pro"

  @givn.added
  Scenario: Quick setup documents its parameters in help
    When I run `watn quicksetup --help`
    Then the exit status should be 0
    And the output should contain "--url"
    And the output should contain "--key"
    And the output should contain "--model-small"
    And the output should contain "--model-normal"
    And the output should contain "--model-thinking"
