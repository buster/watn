# User Interaction Inventory:
# - press Ctrl-W in an installed Bash, Zsh, or Fish widget and observe the original request preserved as a comment above the generated command in the editable buffer
#
@givn.delta @interactive-shell-shortcut
Feature: Interactive shell shortcut for watn

  @givn.added
  Scenario: A successful generation keeps the original request visible as a comment
    Given an installed Bash shortcut and a fake watn that returns "printf 'ready'"
    When I run the Bash widget with current input "show status"
    Then the current command line should be exactly "# show status\nprintf 'ready'"
    And the cursor should be at the end of the current command line

  @givn.added
  Scenario: Only the generated command executes when the buffer is committed
    Given an installed Bash shortcut and a fake watn that returns "touch /tmp/watn-shortcut-executed"
    When I run the Bash widget with current input "run the task; touch /tmp/watn-shortcut-comment-should-not-run"
    And I execute the resulting Bash buffer
    Then the file "/tmp/watn-shortcut-executed" should exist
    And the file "/tmp/watn-shortcut-comment-should-not-run" should not exist

  @givn.added
  Scenario: Requests with metacharacters and embedded newlines remain one comment line
    Given an installed Bash shortcut and a fake watn that returns "ls"
    When I run the Bash widget with current input containing "show files; echo unsafe *\nsecond line"
    Then the current command line should be exactly "# show files; echo unsafe * second line\nls"
    And the preserved request comment should be a single line

  @givn.added
  Scenario: Failed or empty generation preserves the original buffer
    Given an installed Bash shortcut and a fake watn that fails
    When I run the Bash widget with current input "list files"
    Then the current command line should be exactly "list files"
    When the fake watn returns empty output
    And I run the Bash widget with current input "show files"
    Then the current command line should be exactly "show files"

  @givn.added
  Scenario: Zsh and Fish widgets preserve the request as a comment
    Given an installed Zsh and Fish shortcut
    Then the Zsh configuration should keep the request above the generated command
    And the Fish configuration should keep the request above the generated command
    And the generated Zsh configuration should pass a Zsh syntax check
    And the generated Fish configuration should pass a Fish syntax check

  @givn.added @e2e
  Scenario: The generated Bash widget keeps the request visible and does not evaluate the command
    Given an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When I run the generated Bash widget through Bash with current input "find all images"
    Then the Bash process command line should contain "# find all images\nprintf 'hello world'"
    And the Bash process should preserve the request as a comment
    And the Bash process should not execute the replacement text

  @givn.modified
  Scenario: A successful widget inserts one normalized command and moves the cursor to its end
    Given an installed Bash shortcut and a fake watn that returns "printf 'ready'\n\n"
    When I run the Bash widget with current input "show status"
    Then the current command line should be exactly "# show status\nprintf 'ready'"
    And the cursor should be at the end of the current command line

  @givn.modified
  Scenario: Embedded multiline output remains buffer text without evaluation
    Given an installed Bash shortcut and a fake watn that returns "printf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    When I run the Bash widget with current input "show two lines"
    Then the current command line should be exactly "# show two lines\nprintf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    And the embedded line break should remain in the command line buffer
    And the cursor should be at the end of the current command line
    And the replacement text should not have executed

  @givn.modified @e2e
  Scenario: The generated Bash widget runs through Bash without evaluating its result
    Given an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When I run the generated Bash widget through Bash with current input "find all images"
    Then the Bash process command line should contain "# find all images\nprintf 'hello world'"
    And the Bash process should not execute the replacement text
