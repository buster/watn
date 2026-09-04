Feature: Cancelling a running completion

  @e2e
  Scenario: One Ctrl+C cancels a completion waiting for streamed output
    Given  a streaming provider flushes content "printf first" and holds the stream open without `[DONE]`
    When  I start watn with the invocation `watn "output first"` in a terminal
    Then  the first streamed content "printf first" is visible
    When  I press Ctrl+C
    Then  the exit status should be 130
    And  the terminal output contains "printf first"
    And  stderr should not contain a reported error
    And  stderr should not contain final metadata

  @e2e
  Scenario: One Ctrl+C cancels a completion waiting for a connection
    Given  a provider accepts a connection and never sends a response
    When  I start watn with the invocation `watn "unresponsive provider"` in a terminal
    Then  the progress indicator is visible while the connection is pending
    When  I press Ctrl+C
    Then  the exit status should be 130
    And  stderr should not contain a reported error
