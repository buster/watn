@givn.delta @interactive-shell-shortcut
Feature: Polished review detail block

  @givn.removed
  Scenario: The accept shortcut accepts the current candidate
    Given this scenario was removed by polish-review-detail-block

  @givn.modified @wip
  Scenario: The review card exposes the decision shortcuts
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I switch to the detailed view
    Then the review surface should show "accept"
    And the review surface should show "edit"
    And the review surface should show "reject"
    And the review surface should not show "cancel"
    And the review surface should show "disable review"
    And accept should be shown with the enter key

  @givn.modified @wip
  Scenario: The view toggle switches between the simple and detailed reviews
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the review surface should be in the simple view
    When I switch to the detailed view
    Then the detailed review should show the intent
    And the detailed review should show the command without a label
    And the detailed review should keep a blank row between command and purpose
    And the detailed review should show the stage navigation hint
    And the detailed review should not show the flow or stage labels
    When I move to the next stage
    Then the command stack should mark the selected stage "head -5" with an arrow
    When I switch back to the simple view
    Then the review surface should be in the simple view
