@givn.delta @explain-command

Feature: Explain an existing command

  @givn.modified
  Scenario: Enter closes the explanation card without releasing the command
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain` with this single argument in a terminal, then press Enter:
      """
      ls > /tmp/watn-explain-enter-should-not-run
      """
    Then  watn should exit successfully
    And  the command-output channel should contain no command
    And  the file "/tmp/watn-explain-enter-should-not-run" should not exist

  @givn.modified
  Scenario: Shell metacharacters and embedded quoting reach the card unchanged
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain` with this single argument:
      """
      git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done %s' "$(date)" || echo `hostname`; grep -E '[0-9]+ err(or)?' < input.log > output.log &
      """
    Then  the explanation card should show the stage:
      """
      git log --format='%H' --since='7 days ago'
      """
    And  the explanation card should show the stage:
      """
      printf 'done %s' "$(date)"
      """
    And  the explanation card should show the stage:
      """
      echo `hostname`
      """
    And  the explanation card should show the stage:
      """
      grep -E '[0-9]+ err(or)?' < input.log > output.log &
      """
    And  the explanation card should show purpose-unavailable

  @givn.modified
  Scenario: A command beginning with a dash passes after the option terminator
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain --` with this single argument:
      """
      --version
      """
    Then  the explanation card should show the stage:
      """
      --version
      """

  @givn.modified
  Scenario: A command read from standard input keeps its quoting and line breaks
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain -` in a terminal with this command on standard input:
      """
      awk '{print $1}' access.log
      grep -F 'error' access.log
      """
    Then  the explanation card should show the stage:
      """
      awk '{print $1}' access.log
      """
    And  the explanation card should show the stage:
      """
      grep -F 'error' access.log
      """
    And  the explanation card should show the stages "awk '{print $1}' access.log" and "grep -F 'error' access.log" as separate stages

  @givn.modified
  Scenario: A piped command without the marker is explained
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain` in a terminal with the command "ls | wc -l" piped on standard input
    Then  the explanation card should show the stage:
      """
      ls
      """
    And  the explanation card should show the stage:
      """
      wc -l
      """
    And  the explanation card should show the stages "ls" and "wc -l" as separate stages

  @givn.modified
  Scenario: A positional command takes precedence over standard input
    Given  a configured provider whose explanation returns no stage purposes
    When  I run watn explain with the argument "echo positional" in a terminal with the command "echo standard input" on standard input
    Then  the explanation card should show the stage:
      """
      echo positional
      """
    And  the explanation card should not show the text "echo standard input"

  @givn.modified
  Scenario: The review-panel switches are inert for explain
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain --no-review-panel` with this single argument:
      """
      ls
      """
    Then  the explanation card should show the stage:
      """
      ls
      """

  @givn.modified
  Scenario: The explanation card opens even when the review surface is disabled
    Given  a configured provider whose explanation returns no stage purposes
    And  the persisted review surface is disabled
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the review surface should be disabled in the configuration

  @givn.modified
  Scenario: A configured provider without a usable credential is not contacted
    Given  a configured provider without a usable credential
    When  I run `watn explain` with this single argument and let the setup flow start:
      """
      git log --oneline | head -5
      """
    Then  the setup flow should start
    And  no explanation card should open
    And  no provider request should have been made
    When  I abandon the setup flow
    Then  the exit status should be 1

  @givn.modified @wip
  Scenario: A failed explanation keeps the command reviewable
    Given  a configured provider whose explanation request fails
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the explanation card should show purpose-unavailable
    And  watn should report that the explanation request failed
    And  the exit status should be 2

  @givn.added @wip
  Scenario: A network failure reports the mapped exit status
    Given  a configured provider whose endpoint refuses connections
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the explanation card should show purpose-unavailable
    And  watn should report that the explanation request failed
    And  the exit status should be 3

  @givn.modified @wip
  Scenario: An explanation that does not cover the command is not trusted
    Given  a configured provider whose explanation does not cover the command
    When  I run `watn explain` with this single argument:
      """
      git log --oneline | head -5
      """
    Then  the explanation card should show the stage:
      """
      git log --oneline
      """
    And  the explanation card should show purpose-unavailable
    And  the explanation card should not show the text "unrelated stage"
    And  watn should report that the explanation response was not usable
    And  watn should exit successfully

  @givn.added @wip
  Scenario: An unconfigured machine starts quick setup instead of explaining
    Given  no config file exists
    When  I run `watn explain` with this single argument and let quick setup start:
      """
      git log --oneline | head -5
      """
    Then  the quick setup should announce that no configuration was found
    When  I accept the suggested endpoint
    And  I answer the credential with "sk-quick-test"
    And  I accept the suggested small model
    And  I accept the pre-filled normal model
    And  I accept the pre-filled thinking model
    And  I keep the pre-selected shell integrations and confirm
    Then  watn should exit successfully
    And  watn should report that setup is complete and the command must be rerun
    And  no provider request should have been made
    And  no explanation card should open

  @givn.added @wip
  Scenario: An existing configuration without a usable model completes the setup wizard
    Given  a configured provider with catalog models "model-small", "model-middle", and "model-large"
    When  I run `watn explain` with this single argument and let the setup flow start:
      """
      git log --oneline | head -5
      """
    Then  the setup flow should start
    When  I configure the provider and models through the wizard
    And  I complete the optional shell pages without integrations
    Then  watn should exit successfully
    And  watn should report that setup is complete and the command must be rerun
    And  no explanation card should open
    And  no provider request should have been made

  @givn.added @wip
  Scenario: Interrupting the explanation request opens no card
    Given  a provider accepts a connection and never sends a response
    When  I run `watn explain` with this single argument and interrupt the request:
      """
      git log --oneline | head -5
      """
    Then  no explanation card should open
    And  the exit status should be 130

  @givn.added @wip
  Scenario: A command supplied through standard input without a usable model reports setup guidance
    Given  no config file exists
    When  I run `watn explain` in a terminal with this command on standard input and let it report setup guidance:
      """
      ls | wc -l
      """
    Then  watn should report that setup is required
    And  no explanation card should open
    And  no provider request should have been made
    And  the exit status should be 1

  @givn.added @wip
  Scenario: An explicit provider selection with a broken configuration reports its error
    Given  no config file exists
    When  I run `watn --provider missing explain` with this single argument in a terminal:
      """
      ls
      """
    Then  watn should report an unknown provider error
    And  no explanation card should open
    And  the exit status should be 1
    When  I run `watn --provider openrouter explain` with this single argument in a terminal:
      """
      ls
      """
    Then  watn should report a missing credential error
    And  no explanation card should open
    And  the exit status should be 2

  @givn.added @wip
  Scenario: An explicit provider without a resolvable model reports its error
    Given  a configured provider with no default model
    When  I run `watn --provider custom explain` with this single argument in a terminal:
      """
      ls
      """
    Then  watn should report a configuration error
    And  no explanation card should open
    And  the exit status should be 1
