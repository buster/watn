@givn.delta @explain-command

Feature: Explanation response recovery and diagnostics

  @givn.added
  Scenario: An explanation response with literal line breaks in its values is read as a structured response
    Given  a configured provider whose explanation response contains literal line breaks inside its values
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the explanation card should show the stage purpose "List the commits."

  @givn.added
  Scenario: An explanation response cut off after a complete command keeps the command reviewable
    Given  a configured provider whose explanation response is cut off after the command
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the explanation card should show purpose-unavailable
    And  the explanation card should name that the provider response was incomplete

  @givn.added
  Scenario: The explanation card names why stage purposes are unavailable
    Given  a configured provider whose explanation does not cover the command
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show purpose-unavailable
    And  the explanation card should name that the response stages did not match the command

  @givn.added
  Scenario: Verbose explain prints the raw provider response after the card closes
    Given  a configured provider whose explanation covers the command:
      """
      git log --oneline | head -5
      """
    When  I run `watn explain -v` with that command as one argument in a terminal and close the card
    Then  the explain invocation should report the raw provider response
    And  the command-output channel should contain no command

  @givn.added
  Scenario: An unusable explanation response is saved for bug reports
    Given  a configured provider whose explanation does not cover the command
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the raw provider response should be saved to the unusable-response state file
    And  the explain invocation should name the unusable-response state file path
