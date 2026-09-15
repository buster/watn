@givn.delta @interactive-shell-shortcut

Feature: Review response recovery and diagnostics

  @givn.added
  Scenario: A review response with literal line breaks and tabs in its values is read as a structured response
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a structured review response with literal line breaks and tabs inside its values
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show the command "df -h"
    And  the review surface should show the stage purpose "Show local disk usage."
    And  the review surface should not claim that purposes are loading

  @givn.added
  Scenario: A review response cut off after a complete command stays reviewable
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a structured review response cut off after the command "df -h"
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show the command "df -h"
    And  the review surface should show purpose-unavailable
    And  the review surface should name that the provider response was incomplete
    And  final acceptance should still be required

  @givn.added
  Scenario: A review response whose values contain unescaped quotation marks is still read
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a structured review response whose values contain unescaped quotation marks
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show the command "df -h"
    And  the review surface should not show the raw provider payload
    And  the review surface should show purpose-unavailable
    And  the review surface should name that the provider response was not valid JSON

  @givn.added
  Scenario: A review response cut off inside the command releases nothing
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a structured review response cut off inside the command
    When  I invoke Ctrl-W with the current input
    Then  the original Bash command line should remain unchanged
    And  no candidate should be released to the shell

  @givn.added
  Scenario: The review surface names why stage purposes are unavailable
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a review response with mismatched stage text
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show purpose-unavailable
    And  the review surface should name that the response stages did not match the command

  @givn.added
  Scenario: Verbose review prints the raw provider response after the surface closes
    Given  a configured provider that serves this structured review response:
      """
      {"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h","purpose":"Show local disk usage."}],"purpose_status":"ready"}
      """
    When  I run `watn -v` for "show disk usage" in an eligible terminal
    And  I accept the candidate in the review surface
    Then  the review invocation should report the raw provider response
    And  normal command output should contain only "df -h"

  @givn.added @wip
  Scenario: An unusable provider response is saved for bug reports
    Given  a configured provider that serves this review response:
      """
      {"review_version":1,"command":"df -h","stages":[{"stage_text":"not the derived stage","purpose":"Wrong stage text."}],"purpose_status":"ready"}
      """
    When  I run `watn` for "show disk usage" in an eligible terminal
    And  I cancel the review surface
    Then  the raw provider response should be saved to the unusable-response state file
    And  the review invocation should name the unusable-response state file path

  @givn.added @wip
  Scenario: An unwritable unusable-response state file does not break the review
    Given  a configured provider that serves this review response:
      """
      {"review_version":1,"command":"df -h","stages":[{"stage_text":"not the derived stage","purpose":"Wrong stage text."}],"purpose_status":"ready"}
      """
    And  the unusable-response state directory cannot be created
    When  I run `watn` for "show disk usage" in an eligible terminal
    And  I cancel the review surface
    Then  the review invocation should show the command "df -h"
    And  the review invocation should warn that the unusable-response state file could not be written

  @givn.added @wip
  Scenario: A usable review response does not overwrite the captured unusable response
    Given  an unusable-response state file that already holds a previous response
    And  a configured provider that serves this structured review response:
      """
      {"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h","purpose":"Show local disk usage."}],"purpose_status":"ready"}
      """
    When  I run `watn` for "show disk usage" in an eligible terminal
    And  I cancel the review surface
    Then  the unusable-response state file should still hold the previous response
