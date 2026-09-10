@givn.delta @interactive-shell-shortcut
Feature: Enhanced review card

  @givn.added
  Scenario: The enhanced review card frames the command, stage, and actions
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the review surface should show a framed card
    And the review surface should show the intent
    And the review surface should show the command "git log --oneline | head -5"
    And the review surface should show the selected stage "git log --oneline"
    And the review surface should show the stage purpose "List recent commits."
    And the review surface should show the review actions
    And the review surface should show key hints

  @givn.added @wip
  Scenario: The card shows one stage at a time and moves stages with the arrow keys
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the review surface should show only the selected stage "git log --oneline"
    When I move to the next stage
    Then the review surface should show only the selected stage "head -5"
    And the review surface should show the stage purpose "Keep the first five."
    When I move to the previous stage
    Then the review surface should show only the selected stage "git log --oneline"

  @givn.added @wip
  Scenario: The card marks unsupported stage syntax
    Given an installed Bash shortcut and a provider candidate containing unsupported shell syntax
    When I invoke Ctrl-W with the current input
    Then the raw candidate should remain visible
    And unsupported command-flow portions should be marked

  @givn.added @wip
  Scenario: Disabling the enhanced card preserves the plain review surface
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the enhanced review card is disabled
    When I invoke Ctrl-W with the current input
    Then the plain review surface should open
    And the current candidate should remain available

  @givn.added @wip
  Scenario: A color-incapable terminal falls back to the plain review surface
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the terminal does not support color
    When I invoke Ctrl-W with the current input
    Then the plain review surface should open

  @givn.added @wip
  Scenario: The card's edit shortcut opens the command editor
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with the current input
    And I press the edit shortcut
    Then the command editor should be open
