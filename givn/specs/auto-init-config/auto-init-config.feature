Feature: Auto-init config

  Scenario: First non-TTY run does not create a config file
    Given  no config file exists
    When  I run `watn "hello"`
     Then  no config file should exist
    And  the exit status should be 1
    And  stderr should contain actionable guidance to run "watn provider" in a terminal

  Scenario: Existing config file is not overwritten
    Given  an existing config file with provider "openai"
    When  I run `watn "hello"`
    Then  the config file still contains provider "openai"
