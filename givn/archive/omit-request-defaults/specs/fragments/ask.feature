@givn.delta @ask

Feature: Asking questions

  @givn.added
  Scenario: Ask succeeds against a provider that rejects generation parameters
    Given  a configured default provider "openai"
    And  a provider that rejects a request carrying a sampling temperature or an output-token limit
    When  I run `watn "list go files"`
    Then  the exit status should be 0
    And  the output should contain "find"