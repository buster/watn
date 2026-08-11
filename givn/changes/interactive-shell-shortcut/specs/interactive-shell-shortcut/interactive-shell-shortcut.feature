# User Interaction Inventory:
# - complete implicit first-use setup and choose shell shortcut options
# - press Ctrl-W in an installed Bash shell command line

@givn.delta @interactive-shell-shortcut
Feature: Interactive shell shortcut for watn

  @givn.added @e2e @wip
  Scenario: Implicit first-use setup installs the shortcut from the optional question
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["model-small", "model-middle", "model-large"] for "/models"
    And isolated Bash, Zsh, and Fish configuration files
    When I start interactive `watn "hello"` in a terminal
    And I complete provider setup and select the three model tiers
    And I confirm the Large Model selection with `y` for shortcut configuration
    Then the shell shortcut multi-select should be shown
    When I select Bash for the shell shortcut
    And I confirm the shell shortcut selection
    Then setup should report the modified Bash file and its reload command
    And the Bash configuration should contain one watn shell shortcut block

  @givn.added @e2e @wip
  Scenario: Pressing Ctrl-W replaces the current Bash command line without executing it
    Given an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When I start an interactive Bash command line containing "find all images"
    And I press Ctrl-W
    Then the Bash command line should contain "printf 'hello world'"
    And the Bash prompt should be redisplayed
    And the terminal cursor should be at the end of the replacement
    And the fake watn should receive exactly one question "find all images"
    And the replacement command should not have executed

  @givn.added
  Scenario: Enter accepts the default decline for shortcut setup
    Given Bash, Zsh, and Fish configuration files with existing user content
    And a snapshot of every shell configuration file
    When I press Enter to accept the default decline on the optional shortcut question
    Then every shell configuration file should match its snapshot byte-for-byte

  @givn.added
  Scenario: Selecting no shells leaves shell configuration unchanged
    Given Bash, Zsh, and Fish configuration files with existing user content
    And a snapshot of every shell configuration file
    When I answer `y` to the optional shortcut question
    And I select no shells in the shortcut multi-select
    Then every shell configuration file should match its snapshot byte-for-byte

  @givn.added
  Scenario: The shell basename alone controls shortcut preselection
    Given `SHELL` is "/usr/local/bin/bash"
    And Zsh and Fish target files already exist
    When the shell shortcut choices are shown
    Then Bash should be preselected
    And Zsh and Fish should remain available and unselected
    When I select Zsh and Fish as well
    Then Bash, Zsh, and Fish should all be selected

  @givn.added
  Scenario: Multiple selected shells are installed independently
    Given Bash, Zsh, and Fish configuration paths in an isolated home
    When I install the shell shortcut for Bash, Zsh, and Fish
    Then the Bash configuration should contain the Bash widget and Ctrl-W binding
    And the Zsh configuration should contain the ZLE widget and Ctrl-W binding
    And the Fish configuration should contain the Fish widget and Ctrl-W binding
    And setup should report a success for every selected shell
    And each selected shell should have its own reload instruction

  @givn.added
  Scenario: A partial multi-shell failure reports every result without rollback
    Given writable Bash and Fish targets and a Zsh target that cannot be written
    And the Bash and Fish targets have existing user content
    When I install the shell shortcut for Bash, Zsh, and Fish
    Then the Bash configuration should contain one watn shell shortcut block
    And the Fish configuration should contain one watn shell shortcut block
    And the Bash and Fish user content should remain unchanged
    And the Zsh configuration should remain unchanged
    And setup should report success for Bash and Fish
    And setup should report the Zsh target path and write failure reason
    And setup should report an aggregate shell installation failure

  @givn.added
  Scenario: Missing parent directories are created only for selected shells
    Given missing Bash and Fish configuration parent directories
    When I install the shell shortcut for Fish
    Then the Fish configuration parent directory should exist
    And the Bash configuration parent directory should remain absent

  @givn.added
  Scenario: Installing again replaces the generated block without disturbing user content
    Given a Bash configuration containing unrelated user content and one watn shell shortcut block
    When I install the Bash shell shortcut again
    Then the Bash configuration should contain exactly one watn shell shortcut block
    And the unrelated user content should remain unchanged

  @givn.added
  Scenario: A shell configuration failure reports the exact target and reason
    Given a Bash shortcut target that is a directory and cannot be written
    And a snapshot of the Bash target failure state
    When I install the Bash shell shortcut
    Then setup should report that the Bash target could not be written
    And the error should identify the write failure reason
    And the Bash target should remain a directory

  @givn.added
  Scenario: Invalid marker layouts fail before any target write
    Given isolated Bash targets with these malformed marker layouts:
      | layout                                             |
      | two complete watn shell shortcut blocks            |
      | two opening markers and one closing marker         |
      | one opening marker and two closing markers         |
      | an opening marker without a closing marker         |
      | a closing marker without an opening marker         |
      | a closing marker before an opening marker          |
    When I install the Bash shell shortcut for every malformed layout
    Then setup should report malformed watn shell shortcut markers
    And every malformed Bash target should match its snapshot byte-for-byte

  @givn.added
  Scenario: Generated shell blocks use the installed watn command and preserve shell syntax
    Given isolated Bash, Zsh, and Fish shortcut targets
    When I install the shell shortcut for Bash, Zsh, and Fish
    Then no generated block should contain a repository-local watn path
    And every generated widget should invoke `command watn -- "$question"`
    And the Bash block should use the current Readline line and cursor
    And the Zsh block should use the current buffer and cursor
    And the Fish block should replace and repaint the current command line
    And every generated block should bind Ctrl-W

  @givn.added
  Scenario: A successful widget inserts one normalized command and moves the cursor to its end
    Given an installed Bash shortcut and a fake watn that returns "printf 'ready'\n\n"
    When I run the Bash widget with current input "show status"
    Then the current command line should be exactly "printf 'ready'"
    And the cursor should be at the end of the current command line

  @givn.added @wip
  Scenario: Embedded multiline output remains buffer text without evaluation
    Given an installed Bash shortcut and a fake watn that returns:
      """
      printf 'first line'
      touch /tmp/watn-shortcut-should-not-run

      """
    When I run the Bash widget with current input "show two lines"
    Then the current command line should be exactly:
      """
      printf 'first line'
      touch /tmp/watn-shortcut-should-not-run
      """
    And the embedded line break should remain in the command line buffer
    And the cursor should be at the end of the current command line
    And the replacement text should not have executed

  @givn.added @wip
  Scenario: Empty input does not invoke watn or change the command line
    Given an installed Bash shortcut and a fake watn that records invocations
    When I run the Bash widget with empty input
    Then the fake watn should not have been invoked
    And the current command line should remain empty

  @givn.added @wip
  Scenario: Failed or empty output preserves the original command line
    Given an installed Bash shortcut and a fake watn that fails
    When I run the Bash widget with current input "list files"
    Then the current command line should remain "list files"
    When the fake watn returns empty output
    And I run the Bash widget with current input "show files"
    Then the current command line should remain "show files"

  @givn.added @wip
  Scenario: Non-zero watn status discards partial stdout
    Given an installed Bash shortcut and a fake watn that writes "partial" to stdout and exits non-zero
    When I run the Bash widget with current input "show partial result"
    Then the current command line should remain "show partial result"
    And the partial stdout should not be inserted

  @givn.added @wip
  Scenario: The complete command line is passed as one quoted question
    Given an installed Bash shortcut and a fake watn that records its question
    When I run the Bash widget with current input "find files; echo unsafe *"
    Then the fake watn should receive exactly one question "find files; echo unsafe *"
    And the wildcard should not be expanded before watn receives the question

  @givn.added @wip
  Scenario: Leading-option and reserved-token questions remain one argument
    Given an installed Bash shortcut and a fake watn that records each question
    When I run the Bash widget with current input "--help"
    Then the fake watn should receive exactly one question "--help"
    When I run the Bash widget with current input "completions find files"
    Then the fake watn should have received exactly two questions "--help" and "completions find files"

  @givn.added @wip
  Scenario: Setup reports the exact reload instruction for every modified shell
    Given isolated Bash, Zsh, and Fish shortcut targets
    When I install the shell shortcut for Bash, Zsh, and Fish
    Then setup should report "source ~/.bashrc" for Bash
    And setup should report "source ~/.zshrc" for Zsh
    And setup should report "source ~/.config/fish/config.fish" for Fish
