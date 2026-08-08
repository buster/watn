Feature: Auto-init config

  Scenario: First run writes a template config file
    Given  no config file exists
    When  I run `watn "hello"`
    Then  a config file exists at the standard XDG path
    And  the config file contains a commented-out "defaults" section
    And  the command succeeds as if the file already existed

  Scenario: Existing config file is not overwritten
    Given  an existing config file with provider "openai"
    When  I run `watn "hello"`
    Then  the config file still contains provider "openai"
