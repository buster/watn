@givn.delta @interactive-shell-shortcut
Feature: Clearer review card guidance

  @givn.added
  Scenario: The model chooser explains how to choose
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the reject shortcut
    Then the model chooser should explain its keys and input
    And the current model should be marked

  @givn.added
  Scenario: The first suggestion is ready to choose
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider catalog contains "model-a" and "model-b"
    When I open the review surface
    And I press the reject shortcut
    And the catalog suggestions arrive
    Then the first suggestion should be highlighted
    When I choose the highlighted suggestion
    Then a new candidate should use "model-a"

  @givn.removed
  Scenario: The card marks unsupported stage syntax
    Given this scenario was removed by clarify-review-card-guidance

  @givn.added @wip
  Scenario: The card marks nested shell syntax
    Given an installed Bash shortcut and a provider candidate containing unsupported shell syntax
    When I invoke Ctrl-W with the current input
    Then the raw candidate should remain visible
    And the card should mark the nested syntax
