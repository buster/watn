@givn.delta @streamlined-setup

Feature: Streamlined setup flow

  @givn.modified @e2e @wip
  Scenario: Coordinated setup completes provider models reasoning and shell choices
    Given  no config file exists
    And  no supported provider environment variable is set
    And  the ephemeral E2E transport returns priced models:
      | model          | input | output |
      | small-model    | 0.15  | 0.60   |
      | normal-model   | 2.50  | 10.00  |
      | thinking-model | 1.10  | 4.40   |
    When  I start `watn setup` in a terminal
    Then  the setup coordinator should show the provider question first
    When  I choose provider "OpenRouter"
    And  accept the default completion endpoint
    And  choose to paste an API key
    And  enter API key "sk-coordinated-key"
    And  accept the derived catalog endpoint
    And  configure all model roles with their reasoning choices
    And  choose no shell completion integrations
    And  choose no Ctrl-W shortcut integrations
    And  confirm the setup review
    Then  setup should exit successfully
    And  the config file should contain provider "openrouter"
    And  the config file should contain small model "small-model" with reasoning "low"
    And  the config file should contain normal model "normal-model" with reasoning "medium"
    And  the config file should contain thinking model "thinking-model" with reasoning "high"
    And  the config file should record pricing for "small-model" at 0.15 input and 0.60 output per million tokens
    And  the config file should record pricing for "normal-model" at 2.50 input and 10.00 output per million tokens
    And  the config file should record pricing for "thinking-model" at 1.10 input and 4.40 output per million tokens

  @givn.modified @e2e @wip
  Scenario: Models setup configures all three roles from an available catalog
    Given  a configured provider with a priced catalog:
      | model          | input | output |
      | small-model    | 0.15  | 0.60   |
      | normal-model   | 2.50  | 10.00  |
      | thinking-model | 1.10  | 4.40   |
    When  I start `watn models` in a terminal
    Then  the model setup should begin with the small role
    When  I choose "small-model" for the small role
    And  choose reasoning "low" for the small role
    And  choose "normal-model" for the normal role
    And  choose reasoning "medium" for the normal role
    And  choose "thinking-model" for the thinking role
    And  choose reasoning "high" for the thinking role
    Then  models setup should exit successfully
    And  the config file should contain the three selected model roles
    And  the config file should record pricing for "small-model" at 0.15 input and 0.60 output per million tokens
    And  the config file should record pricing for "normal-model" at 2.50 input and 10.00 output per million tokens
    And  the config file should record pricing for "thinking-model" at 1.10 input and 4.40 output per million tokens
