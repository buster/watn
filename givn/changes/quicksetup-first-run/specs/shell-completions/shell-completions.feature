# User Interaction Inventory:
# - run `watn completions <shell>` for a supported shell and receive its script
#   (this delta only extends the asserted command tree with the `quicksetup`
#   subcommand; no new interaction is introduced)

@givn.delta @shell-completions

Feature: Shell completion generation

  @givn.modified
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
      | quicksetup |
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

  @givn.modified
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
      | quicksetup |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second zsh generation should be byte-for-byte identical
    And  the generated script should be accepted by Zsh

  @givn.modified
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
      | quicksetup |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second fish generation should be byte-for-byte identical
    And  the generated script should be accepted by Fish

  @givn.modified
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
      | quicksetup |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second elvish generation should be byte-for-byte identical
    And  the generated script should be accepted by Elvish

  @givn.modified
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
      | quicksetup |
      | completions |
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second powershell generation should be byte-for-byte identical
    And  the generated script should be accepted by PowerShell
