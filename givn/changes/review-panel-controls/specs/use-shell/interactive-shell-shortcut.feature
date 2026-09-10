@givn.delta @interactive-shell-shortcut
Feature: Review panel controls

  @givn.added
  Scenario: The review card is the only review panel
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with the current input
    Then the review surface should show a framed card
    And the review surface should not show the plain panel

  @givn.added @wip
  Scenario: A failing review card releases no candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the review card cannot open
    When I invoke Ctrl-W with the current input
    Then the original Bash command line should remain unchanged
    And no candidate should be released to the shell

  @givn.added @wip
  Scenario: A color-incapable terminal shows the review card without color
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the terminal does not support color
    When I invoke Ctrl-W with the current input
    Then the review surface should show a framed card
    And the review surface should not use color

  @givn.added @wip
  Scenario: Disabling the review panel preserves the original command handling
    Given an installed Bash shortcut with the explanatory review surface disabled
    And a provider candidate "df -h"
    When I invoke Ctrl-W with current input "show available diskspace"
    Then the Bash command line should contain "df -h"
    And no command-flow review should open

  @givn.added @wip
  Scenario: The panel can permanently disable the review
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with the current input
    And I choose to disable the review permanently
    Then the review should be disabled in the configuration
    And the review surface should close and preserve the original input

  @givn.added @wip
  Scenario: The -x confirmation offers to explain the command
    Given the explanatory review surface is disabled
    And a configured provider candidate "printf 'reviewed'"
    When I run `watn -x "print reviewed"` in an eligible terminal
    And I ask to explain the command
    Then the review surface should show a framed card for the command "printf 'reviewed'"
    When I close the explanation
    Then the existing "Execute now?" confirmation should be shown
    And execution should require the existing confirmation response

  @givn.removed
  Scenario: Enhanced renderer failure falls back to the inline review surface
    Given this scenario was removed by review-panel-controls

  @givn.removed
  Scenario: Portable review-surface failure releases no candidate
    Given this scenario was removed by review-panel-controls

  @givn.removed
  Scenario: A color-incapable terminal falls back to the plain review surface
    Given this scenario was removed by review-panel-controls

  @givn.removed
  Scenario: Disabling the enhanced card preserves the plain review surface
    Given this scenario was removed by review-panel-controls
