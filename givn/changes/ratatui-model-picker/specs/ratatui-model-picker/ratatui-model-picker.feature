# User Interaction Inventory:
# - run `watn models` and configure model and reasoning strength for each of the three levels in the keyboard-driven dialog
# - browse the model list with the up/down arrow keys and page up / page down
# - type a search filter into the dialog and see the visible filter and matching suggestions
# - return to a previous level and change its selection before confirming
# - run `watn` so the configured per-level reasoning takes effect on requests

@givn.delta @ratatui-model-picker

Feature: Keyboard-driven model picker

  @givn.added @e2e
  Scenario: Configure model and reasoning for all three levels in the dialog
    Given  a configured provider "test" with models endpoint
    And  the endpoint returns models ["gpt-4o-mini", "o3-pro", "claude-3.7-sonnet"]
    When  I run `watn models` and configure "gpt-4o" with reasoning "off" for small, "o3" with reasoning "low" for normal, and "claude" with reasoning "high" for thinking
    Then  the config file should contain the selected tier assignments with their reasoning strengths

  @givn.added @e2e
  Scenario: Browse the model list with arrow keys and page keys
    Given  a configured provider "test" with a long model list
    When  I run `watn models` and use the down arrow to move the selection to the second model
    And  use the page down key to move the selection by a full page
    Then  the dialog highlights the selected model
    And  the completed setup reports small="model-12"

  @givn.added @e2e @wip
  Scenario: Type a filter and see the matching suggestions
    Given  a configured provider "test" with models endpoint
    And  the endpoint returns models ["deepseek/deepseek-v4-pro", "~deepseek/deepseek-v4-flash-latest", "z-ai/glm-5.2"]
    When  I run `watn models`, type "dee flash" into the small tier picker, and choose "~deepseek/deepseek-v4-flash-latest"
    And  choose "~deepseek/deepseek-v4-flash-latest" for the normal tier
    And  choose "~deepseek/deepseek-v4-flash-latest" for the thinking tier
    Then  the picker displays "~deepseek/deepseek-v4-flash-latest" as a matching suggestion
    And  the dialog shows the filter text "dee flash"

  @givn.added @e2e @wip
  Scenario: Return to a previous level and change its selection before confirming
    Given  a configured provider "test" with models endpoint
    And  the endpoint returns models ["gpt-4o-mini", "o3-pro", "claude-3.7-sonnet"]
    When  I run `watn models` and configure "gpt-4o" with reasoning "off" for small
    And  advance to the normal tier and back to the small tier
    And  change the small tier model to "o3" with reasoning "off"
    And  configure "gpt-4o-mini" with reasoning "low" for normal and "claude" with reasoning "high" for thinking
    Then  the completed setup reports small="o3-pro", normal="gpt-4o-mini", thinking="claude-3.7-sonnet"

  @givn.added @e2e
  Scenario: Configured per-level reasoning takes effect on a request
    Given  a configured default provider "openai"
    And  a model "gpt-4o" assigned to the normal tier with reasoning "low"
    When  I run `watn -2 "summarise the changes"`
    Then  the exit status should be 0
    And  the API request should include reasoning with effort "low"
    And  stderr should not contain "reasoning:"

  @givn.added
  Scenario: Level with reasoning off never sends a reasoning request
    Given  a configured default provider "openai"
    And  a model "gpt-4o-mini" assigned to the small tier with reasoning "off"
    When  I run `watn -1 "list files"`
    Then  the API request should not include reasoning

  @givn.added
  Scenario: Per-word order-independent filter matches any identifier word
    Given  a provider with models "deepseek/deepseek-v4-pro", "~deepseek/deepseek-v4-flash-latest", and "z-ai/glm-5.2"
    When  I type "dee flash" into the active tier picker
    Then  the suggestions include "~deepseek/deepseek-v4-flash-latest"
    And  the suggestions do not include "z-ai/glm-5.2"

  @givn.added
  Scenario: Empty filter result produces a clear empty state
    Given  a provider with models "gpt-4o-mini" and "gpt-4o"
    When  I type "does-not-exist" into the active tier picker
    Then  the picker says that no models were found
    And  the dialog shows the filter text "does-not-exist"

  @givn.added
  Scenario: Model entry shows additional metadata when available
    Given  the catalog has models "model-a" and "model-b" where "model-a" has pricing
    When  I format the model list for display
    Then  the entry for "model-a" shows a price
    And  the entry for "model-b" shows no price

  @givn.added
  Scenario: Remote search failure falls back to local matching
    Given  a provider that does not support searching its model catalog with models "gpt-4o-mini" and "gpt-4o"
    When  I type "gpt" into the active tier picker
    Then  the suggestions include "gpt-4o"
    And  the picker reports that model search is unavailable