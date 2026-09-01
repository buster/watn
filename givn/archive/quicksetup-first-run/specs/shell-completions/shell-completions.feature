# User Interaction Inventory:
# - run `watn completions <shell>` for a supported shell and receive its script
#   (this delta only extends the asserted command tree with the `quicksetup`
#   subcommand; no new interaction is introduced)

@givn.delta @shell-completions

Feature: Shell completion generation

  @givn.modified
  Scenario Outline: Shell completion exposes the authoritative command tree
    When  I run `watn completions <shell>` as a regular subprocess
    Then  the exit status should be 0
    And  stdout should contain <syntax> completion syntax
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
    And  stdout should contain only the completion script
    And  stderr should be empty
    And  a second <shell> generation should be byte-for-byte identical
    And  the generated script should be accepted by <syntax>
    Examples:
      | shell      | syntax      |
      | bash       | Bash        |
      | zsh        | Zsh         |
      | fish       | Fish        |
      | elvish     | Elvish      |
      | powershell | PowerShell  |
