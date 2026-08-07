Feature: Reasoning Support

  @e2e
  Scenario: Thinking tier sends reasoning without printing it
    Given  a configured default provider "openai"
    And  a model "o3-mini" assigned to the thinking tier
    When  I run `watn -3 "design a fault-tolerant message queue"`
    Then  the exit status should be 0
    And  the API request should include reasoning with effort "high"
    And  stderr should not contain "reasoning:"

  @e2e
  Scenario: Thinking tier with verbose flag prints reasoning to stderr
    Given  a configured default provider "openai"
    And  a model "o3-mini" assigned to the thinking tier
    And  the mock returns reasoning "We need a distributed commit log"
    When  I run `watn -3 -v "design a fault-tolerant message queue"`
    Then  the exit status should be 0
    And  stderr should contain "reasoning:"
    And  stderr should contain "We need a distributed commit log"

  @e2e
  Scenario: Verbose flag with small tier prints reasoning if present
    Given  a configured default provider "openai"
    And  a model "gpt-4o-mini" assigned to the small/fast tier
    And  the mock returns reasoning "This is a simple command"
    When  I run `watn -1 -v "list go files"`
    Then  the exit status should be 0
    And  stderr should contain "reasoning:"
    And  stderr should contain "This is a simple command"

  @e2e
  Scenario: Small tier without verbose flag does not print reasoning
    Given  a configured default provider "openai"
    And  a model "gpt-4o-mini" assigned to the small/fast tier
    And  the mock returns reasoning "Hidden reasoning text"
    When  I run `watn -1 "list go files"`
    Then  the exit status should be 0
    And  stderr should not contain "reasoning:"

  @e2e
  Scenario: Verbose flag with default tier does not alter existing model behavior
    Given  a configured default provider "openai"
    And  a model "gpt-4o-mini" assigned to the small/fast tier
    When  I run `watn -v "show disk usage"`
    Then  the exit status should be 0
    And  the output should contain a model name

  @e2e
  Scenario: Help output includes verbose flag
    When  I run `watn --help`
    Then  the exit status should be 0
    And  the output should contain "--verbose"

  @e2e
  Scenario: Thinking tier with verbose and execute flags
    Given  a configured default provider "openai"
    And  a model "o3-mini" assigned to the thinking tier
    And  the mock returns reasoning "We need to use find"
    And  the mock returns command "find . -name '*.log'"
    When  I run `watn -3 -v -x "find log files"` and answer with "n"
    Then  the exit status should be 0
    And  stderr should contain "reasoning:"
    And  stderr should contain "We need to use find"
    And  the output should contain "find"
