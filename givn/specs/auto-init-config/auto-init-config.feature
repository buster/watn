Feature: Auto-init config

  Scenario: First run does not write a config before explicit setup
    Given  no config file exists
    When  I run `watn "hello"`
    Then  no config file exists at the standard XDG path
    And  the exit status should be 1
    And  stderr should contain actionable guidance to run "watn setup" in a terminal

  Scenario: Existing config file is not overwritten
    Given  an existing config file with provider "openai"
    When  I run `watn "hello"`
    Then  the config file still contains provider "openai"
