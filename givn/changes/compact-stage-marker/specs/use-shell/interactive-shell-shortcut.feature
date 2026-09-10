@givn.delta @interactive-shell-shortcut
Feature: Compact stage marker

  @givn.removed
  Scenario: The card marks nested shell syntax
    Given this scenario was removed by compact-stage-marker

  @givn.added
  Scenario: The card marks a stage it cannot decompose
    Given an installed Bash shortcut and a provider candidate containing unsupported shell syntax
    When I invoke Ctrl-W with the current input
    Then the raw candidate should remain visible
    And the card should mark the stage as not decomposed
