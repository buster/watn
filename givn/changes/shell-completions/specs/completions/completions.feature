# User Interaction Inventory:
# - run `watn completions <shell>` for a supported shell and receive its script

@givn.delta @shell-completions
Feature: Shell completion generation

  @givn.added
  Scenario: Bash completion exposes the authoritative command tree
    When I run `watn completions bash` as a regular subprocess
    Then the exit status should be 0
    And stdout should contain Bash completion syntax
    And stdout should contain the authoritative root options:
      | -1 |
      | --small |
      | -2 |
      | --normal |
      | -3 |
      | --thinking |
      | --model |
      | -x |
      | --execute |
      | -v |
      | --verbose |
      | --provider |
      | --set-small |
      | --set-normal |
      | --set-thinking |
      | --help |
      | --version |
    And stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And stdout should contain only the completion script
    And stderr should be empty
    And a second bash generation should be byte-for-byte identical
    And the generated script should be accepted by Bash

  @givn.added
  Scenario: Zsh completion exposes the authoritative command tree
    When I run `watn completions zsh` as a regular subprocess
    Then the exit status should be 0
    And stdout should contain Zsh completion syntax
    And stdout should contain the authoritative root options:
      | --small |
      | --normal |
      | --thinking |
      | --model |
      | --execute |
      | --verbose |
      | --provider |
      | --help |
      | --version |
    And stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And stdout should contain only the completion script
    And stderr should be empty
    And a second zsh generation should be byte-for-byte identical
    And the generated script should be accepted by Zsh

  @givn.added
  Scenario: Fish completion exposes the authoritative command tree
    When I run `watn completions fish` as a regular subprocess
    Then the exit status should be 0
    And stdout should contain Fish completion syntax
    And stdout should contain the authoritative root options:
      | --small |
      | --normal |
      | --thinking |
      | --model |
      | --execute |
      | --verbose |
      | --provider |
      | --help |
      | --version |
    And stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And stdout should contain only the completion script
    And stderr should be empty
    And a second fish generation should be byte-for-byte identical
    And the generated script should be accepted by Fish

  @givn.added @e2e
  Scenario: Built Bash completion generation emits the current command tree
    When I run the built `watn completions bash` command
    Then the exit status should be 0
    And stdout should contain Bash completion syntax
    And stdout should contain the authoritative root options and subcommands
    And stdout should contain bash, zsh, and fish value suggestions
    And stdout should contain only the completion script
    And stderr should be empty
    And a second built Bash generation should be byte-for-byte identical
    And the generated script should be accepted by Bash

  @givn.added
  Scenario: Unsupported shell returns actionable guidance
    When I run `watn completions powershell` as a regular subprocess
    Then the exit status should be non-zero
    And stderr should contain the exact unsupported-shell contract:
      | unsupported shell 'powershell'; choose bash, zsh, or fish |
    And stderr should identify "powershell" as the rejected value

  @givn.added
  Scenario: Completion generation does not load configuration or contact a provider
    Given no provider configuration exists in an isolated XDG config directory
    And the no-config snapshot records that the isolated XDG config file is absent
    And an isolated provider-request sentinel is installed
    And the provider-request sentinel snapshot records zero requests
    When I run `watn completions bash` as a regular subprocess
    Then the exit status should be 0
    And stdout should contain Bash completion syntax
    And stderr should be empty
    And the isolated XDG config file should remain absent after the command
    And the provider-request sentinel should remain at zero requests after the command
    And no file should be written in the isolated XDG config directory
    And successful completion stdout should contain only the generated script

  @givn.added
  Scenario: Completion help documents the supported selector and output contract
    When I run `watn completions --help`
    Then the exit status should be 0
    And completion help stdout should contain "Usage:"
    And completion help stdout should contain "completions <SHELL>"
    And stdout should mention bash, zsh, and fish
    And stdout should explain that the generated script is written to stdout for the caller to install or source
    And stdout should document that only bash, zsh, and fish are supported shell values
    And stderr should be empty
