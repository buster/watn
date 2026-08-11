Feature: Shell completion generation

  Scenario: Bash completion exposes the authoritative command tree
    When  I run `watn completions bash` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain Bash completion syntax
    And  stdout should contain the authoritative root options:
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
    And  stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And  stdout should contain the closed shell-selector value suggestions:
      | bash |
      | elvish |
      | fish |
      | powershell |
      | zsh |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second bash generation should be byte-for-byte identical
    And  the generated script should be accepted by Bash

  Scenario: Zsh completion exposes the authoritative command tree
    When  I run `watn completions zsh` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain Zsh completion syntax
    And  stdout should contain the authoritative root options:
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
      | --help |
      | --version |
    And  stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second zsh generation should be byte-for-byte identical
    And  the generated script should be accepted by Zsh

  Scenario: Fish completion exposes the authoritative command tree
    When  I run `watn completions fish` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain Fish completion syntax
    And  stdout should contain the authoritative root options:
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
      | --help |
      | --version |
    And  stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second fish generation should be byte-for-byte identical
    And  the generated script should be accepted by Fish

  Scenario: Elvish completion exposes the authoritative command tree
    When  I run `watn completions elvish` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain Elvish completion syntax
    And  stdout should contain the authoritative root options:
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
      | --help |
      | --version |
    And  stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second elvish generation should be byte-for-byte identical
    And  the generated script should be accepted by Elvish

  Scenario: PowerShell completion exposes the authoritative command tree
    When  I run `watn completions powershell` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain PowerShell completion syntax
    And  stdout should contain the authoritative root options:
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
      | --help |
      | --version |
    And  stdout should contain the authoritative root subcommands:
      | setup |
      | models |
      | provider |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second powershell generation should be byte-for-byte identical
    And  the generated script should be accepted by PowerShell

  @e2e
  Scenario: Built Bash completion generation emits the current command tree
    When  I run the built `watn completions bash` command
    Then  the exit status should be 0
    And  stdout should contain Bash completion syntax
    And  stdout should contain the authoritative root options and subcommands
    And  stdout should contain bash, elvish, fish, powershell, and zsh value suggestions
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second built Bash generation should be byte-for-byte identical
    And  the generated script should be accepted by Bash

  Scenario: Unsupported shell returns actionable guidance
    When  I run `watn completions nushell` as a regular subprocess
    Then  the exit status should be non-zero
    And  stderr should contain the exact unsupported-shell contract:
      | unsupported shell 'nushell'; choose bash, elvish, fish, powershell, or zsh |
    And  stderr should identify "nushell" as the rejected value

  Scenario: Completion generation does not load configuration or contact a provider
    Given  no provider configuration exists in an isolated XDG config directory
    And  the no-config snapshot records that the isolated XDG config file is absent
    And  an isolated provider-request sentinel is installed
    And  the provider-request sentinel snapshot records zero requests
    When  I run `watn completions bash` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain Bash completion syntax
    And  stderr should be empty
    And  the isolated XDG config file should remain absent after the command
    And  the provider-request sentinel should remain at zero requests after the command
    And  no file should be written in the isolated XDG config directory
    And  successful completion stdout should contain only the generated script

  Scenario: Completion help documents the supported selector and output contract
    When  I run `watn completions --help`
    Then  the exit status should be 0
    And  completion help stdout should contain "Usage:"
    And  completion help stdout should contain "completions <SHELL>"
    And  stdout should mention bash, elvish, fish, powershell, and zsh
    And  stdout should explain that the generated script is written to stdout for the caller to install or source
    And  stdout should document that only bash, elvish, fish, powershell, and zsh are supported shell values
    And  stderr should be empty

  Scenario: The reserved completion token can remain question text after `--`
    Given  a configured default provider "openai"
    When  I run `watn -- completions find files` as a regular subprocess
    Then  the exit status should be 0
    And  the output should contain "find"
    And  stdout should not contain Bash completion syntax
    When  I run `watn "completions find files"` as a regular subprocess
    Then  the exit status should be 0
    And  the output should contain "find"
    And  stdout should not contain Bash completion syntax

  Scenario: Setup installs shell completion loaders in selected shell files
    Given  isolated Bash, Zsh, and Fish completion targets
    When  I install shell completion for Bash, Zsh, and Fish
    Then  the Bash configuration should contain the Bash completion loader
    And  the Zsh configuration should contain the Zsh completion loader
    And  the Fish configuration should contain the Fish completion loader
    And  completion installation should report a reload instruction for every shell
