# User Interaction Inventory:
# - Ask a question with default tier (small/fast): `watn "find files modified today"`
# - Ask with explicit tier -1: `watn -1 "find files modified today"`
# - Ask with tier -2 (normal): `watn -2 "explain TCP handshake"`
# - Ask with tier -3 (thinking): `watn -3 "design a distributed rate limiter"`
# - Execute returned command with confirmation: `watn -x "list all pdfs"`
# - Output includes model name
# - Output includes tokens/second
# - Output includes cost when pricing configured
# - Ask a question via stdin pipe
# - Ask a question as positional argument without flags
# - Cancel a streaming response mid-generation (Ctrl+C)
# - Exit codes: 0 = success, 1 = user error, 2 = API error, 3 = network error
# - No arguments and no stdin: print help, exit 1

@givn.delta @ask

Feature: Asking questions
  A user asks a question and receives a copy-pasteable command.
  The output includes the model name, token speed, and cost alongside the output.

  @givn.added @e2e @wip
  Scenario: Ask with default tier returns a copy-pasteable command
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    When I run `watn "find all files modified in the last 3 days"`
    Then the exit status should be 0
    And the output should contain "find"
    And the output should contain a model name
    And the output should contain a tokens/second value
    And the output should not contain ANSI escape sequences

  @givn.added @e2e @wip
  Scenario: Explicit tier -1 uses the small/fast model
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    When I run `watn -1 "list go files"`
    Then the exit status should be 0
    And the output should match regex "gpt-4o-mini"

  @givn.added @e2e @wip
  Scenario: Tier -2 uses the normal model
    Given a configured default provider "openai"
    And a model "gpt-4o" assigned to the normal tier
    When I run `watn -2 "write a docker-compose for postgres and redis"`
    Then the exit status should be 0
    And the output should match regex "gpt-4o"

  @givn.added @e2e @wip
  Scenario: Tier -3 uses the thinking/reasoning model
    Given a configured default provider "openai"
    And a model "o3-mini" assigned to the thinking tier
    When I run `watn -3 "design a fault-tolerant message queue architecture"`
    Then the exit status should be 0
    And the output should match regex "o3-mini"

  @givn.added @e2e @wip
  Scenario: Execute flag prompts for confirmation
    Given the mock returns command "echo hello"
    When I run `watn -x "echo hello"` and answer with "Enter"
    Then the exit status should be 0
    And "hello" should have been printed to stdout

  @givn.added @e2e @wip
  Scenario: Execute flag with explicit "y" confirmation
    Given the mock returns command "echo hello"
    When I run `watn -x "echo hello"` and answer with "y"
    Then the exit status should be 0
    And "hello" should have been printed to stdout

  @givn.added @e2e @wip
  Scenario: Execute flag with "n" answer skips execution
    Given the mock returns command "echo hello"
    When I run `watn -x "echo hello"` and answer with "n"
    Then the command should not have been executed
    And the exit status should be 0

  @givn.added @e2e @wip
  Scenario: Cost is displayed when pricing is configured
    Given pricing configured at "$2.50/1M input tokens" per model
    When I run `watn "show disk usage"`
    Then the output should contain a cost value

  @givn.added @e2e @wip
  Scenario: Tokens/second is displayed after response completes
    When I run `watn "echo hello"`
    Then the output should match regex "tokens/s:\s+\d+\.?\d*"

  @givn.added @e2e @wip
  Scenario: Ask via stdin pipe
    Given the mock returns command "find . -name '*.pdf'"
    When I run `echo "list all pdfs" | watn`
    Then the exit status should be 0
    And the output should be a command suggestion containing "pdf"

  @givn.added @wip
  Scenario: Cancelling a streaming response with Ctrl+C
    Given a configured default provider "openai"
    And a model "gpt-4o-mini" assigned to the small/fast tier
    When I run `watn "Write a long essay"` and send SIGINT after 500ms
    Then the exit status should be 130
    And partial output should have been printed

  @givn.added @wip
  Scenario: No arguments and no stdin prints help and exits with error
    Given no arguments and no stdin
    When I run `watn`
    Then the exit status should be 1
    And stderr should contain "Usage"

  @givn.added @wip
  Scenario: Non-zero exit code on API authentication failure
    Given a configured provider "openai" with api-key "INVALID_KEY"
    When I run `watn "hello"`
    Then the exit status should be 2
    And stderr should contain "authentication"

  @givn.added @wip
  Scenario: Explicit model flag overrides tier dispatch
    Given a configured default provider "openai"
    And a model "gpt-4o" assigned to the normal tier
    When I run `watn --model "gpt-4o-mini" "list go files"`
    Then the exit status should be 0
    And the output should match regex "gpt-4o-mini"

  @givn.added @wip
  Scenario: Version flag prints logo and version
    When I run `watn --version`
    Then the exit status should be 0
    And the output should contain "watn"
    And the output should contain a version number

  @givn.added @wip
  Scenario: Default model used when no tiers configured
    Given a configured default provider "openai" with default model "gpt-4o-mini"
    When I run `watn "find all modified files"`
    Then the exit status should be 0
    And the output should match regex "gpt-4o-mini"
