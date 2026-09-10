@givn.delta @interactive-shell-shortcut
Feature: Simplified review card controls

  @givn.removed
  Scenario: The compact review surface cycles three focus regions
    Given this scenario was removed by simplify-review-card-controls

  @givn.removed
  Scenario: Review actions use Enter and Escape
    Given this scenario was removed by simplify-review-card-controls

  @givn.removed
  Scenario: The enhanced review card frames the command, stage, and actions
    Given this scenario was removed by simplify-review-card-controls

  @givn.removed
  Scenario: Regeneration replaces the current candidate by default
    Given this scenario was removed by simplify-review-card-controls

  @givn.removed
  Scenario: Rejected candidate returns to the current intent
    Given this scenario was removed by simplify-review-card-controls

  @givn.removed
  Scenario: Retained candidates can be compared and one selected
    Given this scenario was removed by simplify-review-card-controls

  @givn.added
  Scenario: The review card opens on the first command-flow stage
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I open the review surface
    Then the first command-flow stage should be selected
    And its stage purpose should be visible
    And no focus region should be shown

  @givn.added
  Scenario: The review card exposes the direct decision shortcuts
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    Then the review surface should show "a accept"
    And the review surface should show "e edit"
    And the review surface should show "r reject"
    And the review surface should show "c cancel"
    And accept should be shown as the default decision

  @givn.added
  Scenario: Enter accepts the current candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press Enter in the review surface
    Then the current candidate should be accepted
    And no alternative candidate should be generated

  @givn.added
  Scenario: The accept shortcut accepts the current candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the accept shortcut
    Then the current candidate should be accepted
    And no alternative candidate should be generated

  @givn.added
  Scenario: The cancel shortcut cancels the review
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the cancel shortcut
    Then the review should be cancelled
    And no candidate should be released

  @givn.added
  Scenario: Rejecting a candidate opens the model chooser
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the reject shortcut
    Then the model chooser should open
    And it should offer the configured small, normal, and thinking tiers with number shortcuts
    And it should offer a field for another model name
    And no candidate should be released
    And the current intent should remain "show disk usage"

  @givn.added
  Scenario: A configured tier can be chosen with its number
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I reject the candidate and choose the normal tier
    Then a new candidate should be generated at the normal tier
    And its provider and model should be visible
    And final acceptance should still be required

  @givn.added
  Scenario: A model name is suggested from the provider catalog while typing
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider catalog contains "model-a" and "model-b"
    When I open the review surface
    And I reject the candidate and type "model-b"
    Then the model chooser should suggest "model-b"
    When I choose the suggested model
    Then a new candidate should use "model-b"
    And final acceptance should still be required

  @givn.added
  Scenario: A typed model name works when the catalog is unavailable
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider catalog is unavailable
    When I open the review surface
    And I reject the candidate and type "custom/model-9"
    And I choose the typed model
    Then a new candidate should use "custom/model-9"
    And final acceptance should still be required

  @givn.added
  Scenario: The model chooser ignores incomplete choices
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the reject shortcut without a thinking tier
    Then the model chooser should remain open
    When I press Enter without a model choice
    Then the model chooser should remain open
    And the previous candidate should remain visible

  @givn.added
  Scenario: A failed regeneration preserves the previous candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And regeneration fails
    When I open the review surface
    And I reject the candidate and choose the normal tier
    Then the previous candidate should remain visible
    And the review surface should report the generation failure
    And final acceptance should still be required

  @givn.added
  Scenario: Leaving the model chooser preserves the candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the reject shortcut
    And I leave the model chooser
    Then the previous candidate should remain visible
    And the review should remain open

  @givn.added
  Scenario: The command editor moves the insertion point with arrows and Home and End
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the edit shortcut
    Then the command editor should be open
    When I move the insertion point to the start
    Then the insertion point should be at the start
    And typing should insert at the insertion point
    When I move the insertion point to the end
    Then the insertion point should be at the end
    When I move the insertion point one character left with the arrow key
    Then the insertion point should be before the last character

  @givn.added
  Scenario: Backspace and Delete remove text at the insertion point
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I press the edit shortcut
    When I move the insertion point one character left with the arrow key
    And I press Backspace
    Then the text before the insertion point should be removed
    When I press Delete
    Then the text at the insertion point should be removed

  @givn.added
  Scenario: The explanation card ignores review decisions
    Given the explanatory review surface is disabled
    And a configured provider candidate "printf 'reviewed'"
    When I run `watn -x "print reviewed"` in an eligible terminal
    And I ask to explain the command
    And I press the accept shortcut on the explanation card
    Then the explanation card should remain open
    When I close the explanation
    Then the existing "Execute now?" confirmation should be shown
    And execution should require the existing confirmation response

  @givn.added @e2e @wip
  Scenario: Developer rejects a candidate and regenerates with another model
    Given a configured provider candidate "df -h"
    And the configured provider can return a replacement candidate "du -sh ."
    When I ask interactively for "show disk usage"
    And I reject the candidate and choose the normal tier in the review surface
    Then the replacement candidate should be visible
    When I accept the candidate in the review surface
    Then normal command output should contain only "du -sh ."
