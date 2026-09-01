Feature: Quick setup first run

  @e2e
  Scenario: First run without a configuration starts the quick setup
    Given  no watn configuration exists
    And  provider requests are captured by a sentinel
    When  I start interactive `watn "hello"` in a terminal
    Then  the quick setup should announce that no configuration was found
    And  the quick setup should ask for the completion endpoint
    And  the endpoint question should suggest "https://openrouter.ai/api/v1"
    And  no original chat completion request should be sent

  @e2e
  Scenario: Quick setup stores answers and installs integrations
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    And  bash, zsh, and fish are available on the path
    When  I start `watn quicksetup` in a terminal
    And  I accept the suggested endpoint, credential, and models
    And  I keep the pre-selected shell integrations and confirm
    Then  quick setup should persist the selected configuration and integrations

  @e2e
  Scenario: Explicit quick setup overwrites an existing configuration
    Given  an existing watn configuration contains provider "openai" with credential "sk-old-key"
    And  the config file contains models "old-small", "old-normal", and "old-thinking"
    And  bash, zsh, and fish are available on the path
    And  provider requests are captured by a sentinel
    When  I start `watn quicksetup` in a terminal
    And  I answer the endpoint with "https://llm.example/v1"
    And  I answer the credential with "sk-new-key"
    And  I answer the small model with "new-small"
    And  I accept the pre-filled normal model
    And  I accept the pre-filled thinking model
    And  I deselect all shell integrations and confirm
    Then  quick setup should exit successfully
    And  the config file should contain provider "custom"
    And  the config file should contain endpoint exactly "https://llm.example/v1"
    And  the config file should contain credential "sk-new-key"
    And  the config file should contain small model "new-small"
    And  the config file should contain normal model "new-small"
    And  the config file should contain thinking model "new-small"
    And  no model catalog request should be sent
    And  no shell target file should change

  @e2e
  Scenario: Aborting quick setup with Ctrl-C on the first run leaves no configuration
    Given  no watn configuration exists
    And  provider requests are captured by a sentinel
    When  I start interactive `watn "hello"` in a terminal
    And  I accept the suggested endpoint
    And  I abort the quick setup with Ctrl-C
    Then  no config file should exist
    And  no shell target file should change
    And  no original chat completion request should be sent

  Scenario: Quick setup does not ask reasoning questions and stores no reasoning
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    When  I complete the quick setup with the suggested answers and no shell integrations
    Then  no reasoning question should have been shown
    And  the config file should contain small model "google/gemma-4-flash" without reasoning
    And  the config file should contain normal model "google/gemma-4-flash" without reasoning
    And  the config file should contain thinking model "google/gemma-4-flash" without reasoning

  Scenario: Shell integrations are pre-selected only for shells available on the path
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    And  bash and zsh are available on the path but fish is not
    When  I start `watn quicksetup` in a terminal
    And  I accept the suggested endpoint, credential, and models
    Then  the shell integration list should mark Bash as selected
    And  the shell integration list should mark Zsh as selected
    And  the shell integration list should mark Fish as not selected

  Scenario: An unknown shell name shows an error and keeps the list open
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    When  I start `watn quicksetup` in a terminal
    And  I accept the suggested endpoint, credential, and models
    And  I type an unknown shell name
    Then  the shell integration list should show an error for the unknown shell
    When  I keep the pre-selected shell integrations and confirm
    Then  quick setup should exit successfully

  Scenario: An empty model answer is rejected before configuration
    Given  no watn configuration exists
    When  I start `watn quicksetup` in a terminal
    And  I answer the endpoint with "https://llm.example/v1"
    And  I answer the credential with "sk-key"
    And  I answer the small model question with an empty input
    Then  quick setup should still ask for the small model
    And  no config file should exist
    When  I answer the small model with "my-small"
    And  I finish quick setup with the remaining suggestions and confirm
    Then  quick setup should exit successfully
    And  the config file should contain small model "my-small"

  Scenario: An invalid endpoint is rejected before setup
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    When  I start `watn quicksetup` in a terminal
    And  I answer the endpoint with an invalid value
    Then  quick setup should still ask for the endpoint
    When  I accept the suggested endpoint, credential, and models
    And  I keep the pre-selected shell integrations and confirm
    Then  quick setup should exit successfully
    And  the config file should contain provider "openrouter"

  Scenario: An OpenAI endpoint suggests the OpenAI credential and no model
    Given  no watn configuration exists
    And  environment variable OPENAI_API_KEY is set to "sk-openai-test"
    When  I start `watn quicksetup` in a terminal
    And  I answer the endpoint with "https://api.openai.com/v1"
    Then  the credential question should suggest "${OPENAI_API_KEY}"
    When  I accept the suggested credential reference
    Then  the small model question should show no suggestion

  Scenario: Explicit provider selection skips the first-run quick setup
    Given  no watn configuration exists
    And  environment variable WATN_PROVIDER is set to "openai"
    When  I run a request for "hello" without a terminal
    Then  the exit status should be nonzero
    And  the output should not mention the quick setup
    And  no config file should exist

  Scenario: Aborting explicit quick setup leaves the previous configuration unchanged
    Given  an existing watn configuration contains provider "openai" with credential "sk-old-key"
    And  the existing config content is recorded
    When  I start `watn quicksetup` in a terminal
    And  I abort the quick setup with Ctrl-C
    Then  the config file should be byte-for-byte unchanged
    And  no shell target file should change

  Scenario: Quick setup without a terminal prints guidance instead of asking
    Given  no watn configuration exists
    When  I run `watn quicksetup` without a terminal
    Then  the exit status should be nonzero
    And  the output should instruct me to run `watn quicksetup` in a terminal
    And  no config file should exist

  Scenario: A failed configuration write installs no shell integration
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    And  the configuration write is forced to fail
    When  I complete the quick setup with the suggested answers and shell integrations selected
    Then  quick setup should report a configuration error
    And  no config file should exist
    And  no shell target file should change

  Scenario: A failed shell installation keeps the saved configuration
    Given  no watn configuration exists
    And  environment variable OPENROUTER_API_KEY is set to "sk-quick-key"
    And  the fish target path cannot be written
    When  I complete the quick setup with the suggested answers and shell integrations selected
    Then  quick setup should report a nonzero result
    And  the config file should contain provider "openrouter"
    And  the config file should contain small model "google/gemma-4-flash"
    And  Bash should contain a Watn-managed completion block
    And  Zsh should contain a Watn-managed Ctrl-W block
