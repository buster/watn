@givn.delta @interactive-shell-shortcut
Feature: Emphasized review shortcut keys

  @givn.added
  Scenario: The review card emphasizes the decision shortcut keys
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    Then the decision keys should be shown colored and bold
    When I press the edit shortcut
    Then the editor keys should be shown colored and bold
    When I press Escape in the command editor
    And I press the reject shortcut
    Then the chooser keys should be shown colored and bold

  @givn.removed
  Scenario: The review card exposes the direct decision shortcuts
    Given this scenario was removed by emphasize-review-shortcut-keys

  @givn.added
  Scenario: The review card exposes the decision shortcuts
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    Then the review surface should show "accept"
    And the review surface should show "edit"
    And the review surface should show "reject"
    And the review surface should show "cancel"
    And the review surface should show "disable"
    And accept should be shown as the default decision
