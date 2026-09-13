Feature: Explain an existing command

  @e2e
  Scenario: Developer explains an existing command in the review card
    Given  a configured provider whose explanation covers the command:
      """
      printf 'explained' > /tmp/watn-explain-should-not-run && echo done
      """
    When  I run `watn explain` with that command as one argument in a terminal
    Then  the explanation card should show the stage:
      """
      printf 'explained' > /tmp/watn-explain-should-not-run
      """
    And  the explanation card should show the stage:
      """
      echo done
      """
    And  the explanation card should show each model-written stage purpose
    And  the explanation card should show the close hint "esc close"
    And  the explanation card should name "watn"
    And  the explanation card should not show internal identifiers
    When  I close the explanation card
    Then  watn should exit successfully
    And  the command-output channel should contain no command
    And  the file "/tmp/watn-explain-should-not-run" should not exist

  Scenario: Enter closes the explanation card without releasing the command
    Given  a configured provider whose explanation returns no stage purposes
    When  I run `watn explain` with this single argument in a terminal, then press Enter:
      """
      ls > /tmp/watn-explain-enter-should-not-run
      """
    Then  watn should exit successfully
    And  the command-output channel should contain no command
    And  the file "/tmp/watn-explain-enter-should-not-run" should not exist

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

  Scenario: A positional command takes precedence over standard input
    Given  a configured provider whose explanation returns no stage purposes
    When  I run watn explain with the argument "echo positional" in a terminal with the command "echo standard input" on standard input
    Then  the explanation card should show the stage:
      """
      echo positional
      """
    And  the explanation card should not show the text "echo standard input"

  Scenario: Empty input is rejected without opening a card
    Given  no config file exists
    When  I run `watn explain` with an empty argument
    Then  watn should report a usage error
    And  no explanation card should open
    When  I run `watn explain` with empty standard input
    Then  watn should report a usage error
    And  no explanation card should open

  Scenario: A second positional argument is refused
    Given  no config file exists
    When  I run watn explain with the arguments "echo one" and "echo two"
    Then  watn should report a usage error
    And  no explanation card should open

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

  Scenario: Explain never executes the command
    Given  no config file exists
    When  I run `watn -x explain` with this single argument:
      """
      printf 'ran' > /tmp/watn-explain-should-not-run
      """
    Then  watn should report that explain never executes
    And  the file "/tmp/watn-explain-should-not-run" should not exist
    When  I run `watn explain -x` with this single argument:
      """
      printf 'ran' > /tmp/watn-explain-should-not-run
      """
    Then  watn should report a usage error
    And  the file "/tmp/watn-explain-should-not-run" should not exist

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

  Scenario: A terminal is required for the explanation card
    Given  no config file exists
    When  I run `watn explain 'echo test'` without a controlling terminal
    Then  watn should report that the explanation requires a terminal
    And  no explanation card should open

  Scenario: A malformed configuration file is reported
    Given  the configuration file is malformed
    When  I run `watn explain` with this single argument:
      """
      ls
      """
    Then  watn should report a configuration error
    And  no explanation card should open
