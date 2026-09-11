@givn.delta @streamlined-setup

Feature: Streamlined setup flow

  @givn.modified @e2e
  Scenario: Shell setup independently configures completion and Ctrl-W integrations
    Given  the shell binaries on PATH are "bash"
    And  no Watn-managed shell integrations are installed
    When  I start `watn shell` in a terminal
    Then  shell setup should show the completion and Ctrl-W shell lists
    And  the shell choices should include only Bash, Fish, and Zsh
    When  I choose Bash for completion
    And  choose Zsh for the Ctrl-W shortcut
    Then  shell setup should exit successfully
    And  Bash should contain a Watn-managed completion block
    And  Zsh should contain a Watn-managed Ctrl-W block
    And  Fish should remain unchanged

  @givn.modified
  Scenario: Shell setup prefills installed integrations and removes only managed blocks when deselected
    Given  the shell binaries on PATH are "bash"
    And  Bash contains a valid Watn-managed completion block
    And  Bash contains user-owned shell content
    When  I start `watn shell` in a terminal
    Then  Bash completion should be selected
    When  I deselect Bash completion
    Then  the Watn-managed completion block should be removed from Bash
    And  the user-owned shell content should remain

  @givn.modified
  Scenario: Shell setup refuses malformed managed markers
    Given  the shell binaries on PATH are "bash"
    And  Bash contains duplicated Watn completion markers
    When  I deselect Bash completion in shell setup
    Then  shell setup should report a malformed managed block
    And  the Bash file should remain unchanged

  @givn.removed
  Scenario: Declining shell setup performs no target inspection or write
    Given this obsolete scenario is removed

  @givn.added
  Scenario: Declining shell setup writes no shell target
    Given  no shell binaries are on PATH
    And  no shell integration choice has been accepted
    And  shell target files do not exist
    When  I advance through both shell integration pages without a selection
    Then  no shell target file should be inspected or created
    And  no configuration field should change

  @givn.added
  Scenario: Detected shells are preselected on the shell pages
    Given  the shell binaries on PATH are "bash" and "zsh"
    And  no Watn-managed shell integrations are installed
    When  I start `watn shell` in a terminal
    Then  no shell opt-in question should be shown
    And  Bash completion should be selected
    And  Zsh completion should be selected
    And  Fish completion should be unselected
    When  I accept the completion selection
    And  I accept the Ctrl-W selection
    Then  shell setup should exit successfully
    And  Bash should contain a Watn-managed completion block
    And  Zsh should contain a Watn-managed Ctrl-W block
    And  Fish should remain unchanged

  @givn.added
  Scenario: A managed shell without a binary stays selected
    Given  no shell binaries are on PATH
    And  Bash contains a valid Watn-managed completion block
    When  I start `watn shell` in a terminal
    Then  Bash completion should be selected
    When  I accept the completion selection
    And  I accept the Ctrl-W selection
    Then  shell setup should exit successfully
    And  Bash should contain a Watn-managed completion block
