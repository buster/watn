@givn.delta @cancel-running-completion

Feature: Cancelling a running completion

  @givn.modified
  Scenario: One Ctrl+C cancels a completion waiting for a connection
    Given  a provider accepts a connection and never sends a response
    When  I start watn with the invocation `watn "unresponsive provider"` in a terminal
    Then  the progress indicator is visible while the connection is pending
    When  I press Ctrl+C
    Then  the exit status should be 130
    And  stderr should not contain a reported error
