Feature: Interactive shell shortcut for watn

  @e2e

  Scenario: Generated Bash, Zsh, and Fish configurations pass shell syntax checks
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the generated Bash configuration should pass a Bash syntax check
    And  the generated Zsh configuration should pass a Zsh syntax check
    And  the generated Fish configuration should pass a Fish syntax check

  @e2e
  Scenario: The generated Bash widget runs through Bash without evaluating its result
    Given  an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When  I run the generated Bash widget through Bash with current input "find all images"
    Then  the Bash process command line should contain "# find all images\nprintf 'hello world'"
    And  the Bash process should not execute the replacement text

  Scenario: Enter accepts the default decline for shortcut setup
    Given  Bash, Zsh, and Fish configuration files with existing user content
    And  a snapshot of every shell configuration file
    When  I press Enter to accept the default decline on the optional shortcut question
    Then  every shell configuration file should match its snapshot byte-for-byte

  Scenario: Selecting no shells leaves shell configuration unchanged
    Given  Bash, Zsh, and Fish configuration files with existing user content
    And  a snapshot of every shell configuration file
    When  I answer `y` to the optional shortcut question
    And  I select no shells in the shortcut multi-select
    Then  every shell configuration file should match its snapshot byte-for-byte

  Scenario: The shell basename alone controls shortcut preselection
    Given  `SHELL` is "/usr/local/bin/bash"
    And  Zsh and Fish target files already exist
    When  the shell shortcut choices are shown
    Then  Bash should be preselected
    And  Zsh and Fish should remain available and unselected
    When  I select Zsh and Fish as well
    Then  Bash, Zsh, and Fish should all be selected

  Scenario: Multiple selected shells are installed independently
    Given  Bash, Zsh, and Fish configuration paths in an isolated home
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the Bash configuration should contain the Bash widget and Ctrl-W binding
    And  the Zsh configuration should contain the ZLE widget and Ctrl-W binding
    And  the Fish configuration should contain the Fish widget and Ctrl-W binding
    And  setup should report a success for every selected shell
    And  each selected shell should have its own reload instruction

  Scenario: A partial multi-shell failure reports every result without rollback
    Given  writable Bash and Fish targets and a Zsh target that cannot be written
    And  the Bash and Fish targets have existing user content
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the Bash configuration should contain one watn shell shortcut block
    And  the Fish configuration should contain one watn shell shortcut block
    And  the Bash and Fish user content should remain unchanged
    And  the Zsh configuration should remain unchanged
    And  setup should report success for Bash and Fish
    And  setup should report the Zsh target path and write failure reason
    And  setup should report an aggregate shell installation failure

  Scenario: Missing parent directories are created only for selected shells
    Given  missing Bash and Fish configuration parent directories
    When  I install the shell shortcut for Fish
    Then  the Fish configuration parent directory should exist
    And  the Bash configuration parent directory should remain absent

  Scenario: Installing again replaces the generated block without disturbing user content
    Given  a Bash configuration containing unrelated user content and one watn shell shortcut block
    When  I install the Bash shell shortcut again
    Then  the Bash configuration should contain exactly one watn shell shortcut block
    And  the unrelated user content should remain unchanged

  Scenario: A shell configuration failure reports the exact target and reason
    Given  a Bash shortcut target that is a directory and cannot be written
    And  a snapshot of the Bash target failure state
    When  I install the Bash shell shortcut
    Then  setup should report that the Bash target could not be written
    And  the error should identify the write failure reason
    And  the Bash target should remain a directory

  Scenario: A symlinked shell target is updated without replacing the link
    Given  a Bash shortcut target that is a symbolic link to a regular file
    When  I install the Bash shell shortcut
    Then  the Bash shortcut symlink should remain intact
    And  the resolved Bash shortcut target should contain the Bash widget

  Scenario: Invalid marker layouts fail before any target write
    Given  isolated Bash targets with these malformed marker layouts:
      | layout |
      | two complete watn shell shortcut blocks |
      | two opening markers and one closing marker |
      | one opening marker and two closing markers |
      | an opening marker without a closing marker |
      | a closing marker without an opening marker |
      | a closing marker before an opening marker |
    When  I install the Bash shell shortcut for every malformed layout
    Then  setup should report malformed watn shell shortcut markers
    And  every malformed Bash target should match its snapshot byte-for-byte

  Scenario: Generated shell blocks use the installed watn command and preserve shell syntax
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  no generated block should contain a repository-local watn path
    And  every generated widget should invoke `command watn -- "$question"`
    And  the Bash block should use the current Readline line and cursor
    And  the Zsh block should use the current buffer and cursor
    And  the Fish block should replace and repaint the current command line
    And  every generated block should bind Ctrl-W

  Scenario: A successful widget inserts one normalized command and moves the cursor to its end
    Given  an installed Bash shortcut and a fake watn that returns "printf 'ready'\n\n"
    When  I run the Bash widget with current input "show status"
    Then  the current command line should be exactly "# show status\nprintf 'ready'"
    And  the cursor should be at the end of the current command line

  Scenario: Embedded multiline output remains buffer text without evaluation
    Given  an installed Bash shortcut and a fake watn that returns "printf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    When  I run the Bash widget with current input "show two lines"
    Then  the current command line should be exactly "# show two lines\nprintf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    And  the embedded line break should remain in the command line buffer
    And  the cursor should be at the end of the current command line
    And  the replacement text should not have executed

  Scenario: Empty input does not invoke watn or change the command line
    Given  an installed Bash shortcut and a fake watn that records invocations
    When  I run the Bash widget with empty input
    Then  the fake watn should not have been invoked
    And  the current command line should remain empty

  Scenario: Failed or empty output preserves the original command line
    Given  an installed Bash shortcut and a fake watn that fails
    When  I run the Bash widget with current input "list files"
    Then  the current command line should remain "list files"
    When  the fake watn returns empty output
    And  I run the Bash widget with current input "show files"
    Then  the current command line should remain "show files"

  Scenario: Non-zero watn status discards partial stdout
    Given  an installed Bash shortcut and a fake watn that writes "partial" to stdout and exits non-zero
    When  I run the Bash widget with current input "show partial result"
    Then  the current command line should remain "show partial result"
    And  the partial stdout should not be inserted

  Scenario: The complete command line is passed as one quoted question
    Given  an installed Bash shortcut and a fake watn that records its question
    When  I run the Bash widget with current input "find files; echo unsafe *"
    Then  the fake watn should receive exactly one question "find files; echo unsafe *"
    And  the wildcard should not be expanded before watn receives the question

  Scenario: Leading-option and reserved-token questions remain one argument
    Given  an installed Bash shortcut and a fake watn that records each question
    When  I run the Bash widget with current input "--help"
    Then  the fake watn should receive exactly one question "--help"
    When  I run the Bash widget with current input "completions find files"
    Then  the fake watn should have received exactly two questions "--help" and "completions find files"

  Scenario: Setup reports the exact reload instruction for every modified shell
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  setup should report "source ~/.bashrc" for Bash
    And  setup should report "source ~/.zshrc" for Zsh
    And  setup should report "source ~/.config/fish/config.fish" for Fish

  Scenario: The optional setup result includes only explicitly selected shells
    Given  a shortcut selection with Bash enabled and Zsh and Fish disabled
    When  the setup result confirms the shortcut selection
    Then  the selected shortcut shells should contain only Bash
  Scenario: A successful generation keeps the original request visible as a comment
    Given  an installed Bash shortcut and a fake watn that returns "printf 'ready'"
    When  I run the Bash widget with current input "show status"
    Then  the current command line should be exactly "# show status\nprintf 'ready'"
    And  the cursor should be at the end of the current command line

  Scenario: Only the generated command executes when the buffer is committed
    Given  an installed Bash shortcut and a fake watn that returns "touch /tmp/watn-shortcut-executed"
    When  I run the Bash widget with current input "run the task; touch /tmp/watn-shortcut-comment-should-not-run"
    And  I execute the resulting Bash buffer
    Then  the file "/tmp/watn-shortcut-executed" should exist
    And  the file "/tmp/watn-shortcut-comment-should-not-run" should not exist

  Scenario: Requests with metacharacters and embedded newlines remain one comment line
    Given  an installed Bash shortcut and a fake watn that returns "ls"
    When  I run the Bash widget with current input containing "show files; echo unsafe *\nsecond line"
    Then  the current command line should be exactly "# show files; echo unsafe * second line\nls"
    And  the preserved request comment should be a single line

  Scenario: Failed or empty generation preserves the original buffer
    Given  an installed Bash shortcut and a fake watn that fails
    When  I run the Bash widget with current input "list files"
    Then  the current command line should be exactly "list files"
    When  the fake watn returns empty output
    And  I run the Bash widget with current input "show files"
    Then  the current command line should be exactly "show files"

  Scenario: Zsh and Fish widgets preserve the request as a comment
    Given  an installed Zsh and Fish shortcut
    Then  the Zsh configuration should keep the request above the generated command
    And  the Fish configuration should keep the request above the generated command
    And  the generated Zsh configuration should pass a Zsh syntax check
    And  the generated Fish configuration should pass a Fish syntax check

  @e2e
  Scenario: The generated Bash widget keeps the request visible and does not evaluate the command
    Given  an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When  I run the generated Bash widget through Bash with current input "find all images"
    Then  the Bash process command line should contain "# find all images\nprintf 'hello world'"
    And  the Bash process should preserve the request as a comment
    And  the Bash process should not execute the replacement text
  @e2e
  Scenario: Fish inserts a real line break after Ctrl-W
    Given  an installed Fish shortcut and a fake watn that returns "df -h"
    When  I press Ctrl-W in the Fish shortcut with current input "show available diskspace"
    Then  the Fish command line should be exactly "# show available diskspace\ndf -h"
