# =============================================================================
# User Interaction Inventory (reasoning-support change)
# 1. watn -3 "question"                   → reasoning sent, NOT printed
# 2. watn -3 -v "question"                → reasoning sent, printed to stderr
# 3. watn -1 -v "question"                → no reasoning sent, reasoning printed if present
# 4. watn -v "question"                   → no reasoning sent, reasoning printed if present
# 5. watn -1 "question"                   → no reasoning sent, nothing extra
# 6. watn -2 "question"                   → no reasoning sent, nothing extra
# 7. watn "question"                      → no reasoning sent, nothing extra
# 8. watn models                          → no change
# 9. watn -h / watn --help                → help shows --verbose / -v flag
# 10. watn -3 -v -x "question"            → reasoning + verbose + execute
# 11. watn -v --model "custom" "question" → verbose with explicit model, no reasoning sent
# =============================================================================

@givn.delta @reasoning
Feature: Reasoning Support

  When the thinking tier is used, the tool should signal high-effort reasoning
  to the API. When the verbose flag is used, any reasoning content in the API
  response should be printed to stderr.

  @givn.added @e2e
  Scenario: Thinking tier sends reasoning without printing it
    Given a configured default provider "openai"
    And a model "o3-mini" assigned to the thinking tier
    When I run `watn -3 "design a fault-tolerant message queue"`
    Then the exit status should be 0
    And the API request should include reasoning with effort "high"
    And stderr should not contain "reasoning:"

  @givn.added @e2e @wip
  Scenario: Thinking tier with verbose flag prints reasoning to stderr
    Given a configured default provider "openai"
    And a model "o3-mini" assigned to the thinking tier
    And the mock returns reasoning "We need a distributed commit log"
    When I run `watn -3 -v "design a fault-tolerant message queue"`
    Then the exit status should be 0
    And stderr should contain "reasoning:"
    And stderr should contain "We need a distributed commit log"

  @givn.added @e2e @wip
  Scenario: Verbose flag with small tier prints reasoning if present
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    And the mock returns reasoning "This is a simple command"
    When I run `watn -1 -v "list go files"`
    Then the exit status should be 0
    And stderr should contain "reasoning:"
    And stderr should contain "This is a simple command"

  @givn.added @e2e @wip
  Scenario: Small tier without verbose flag does not print reasoning
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    And the mock returns reasoning "Hidden reasoning text"
    When I run `watn -1 "list go files"`
    Then the exit status should be 0
    And stderr should not contain "reasoning:"

  @givn.added @e2e @wip
  Scenario: Verbose flag with default tier does not alter existing model behavior
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    When I run `watn -v "show disk usage"`
    Then the exit status should be 0
    And the output should contain a model name

  @givn.added @e2e @wip
  Scenario: Help output includes verbose flag
    When I run `watn --help`
    Then the exit status should be 0
    And the output should contain "--verbose"

  @givn.added @e2e @wip
  Scenario: Thinking tier with verbose and execute flags
    Given a configured default provider "openai"
    And a model "o3-mini" assigned to the thinking tier
    And the mock returns reasoning "We need to use find"
    And the mock returns command "find . -name '*.log'"
    When I run `watn -3 -v -x "find log files"` and answer with "n"
    Then the exit status should be 0
    And stderr should contain "reasoning:"
    And stderr should contain "We need to use find"
    And the output should contain "find"
