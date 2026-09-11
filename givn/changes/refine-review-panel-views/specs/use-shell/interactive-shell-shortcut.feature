@givn.delta @interactive-shell-shortcut
Feature: Refined review panel views

  @givn.removed
  Scenario: The card shows one stage at a time and moves stages with the arrow keys
    Given this scenario was removed by refine-review-panel-views

  @givn.added
  Scenario: The simple review view names only the model without provider or tier
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the configured model is "~anthropic/claude-haiku-latest:nitro"
    When I invoke Ctrl-W with the current input
    Then the review frame should name the model "claude-haiku-latest"
    And the review frame should not name the provider "loopback" or a tier

  @givn.added
  Scenario: The simple view stacks the command flow with its separators
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the command stack should show the stage "git log --oneline"
    And the command stack should show the stage "head -5"
    And the stage "git log --oneline" should end with the "|" separator
    And the stage "head -5" should not end with a separator

  @givn.added
  Scenario: A long stage continues on the next stack rows and keeps its separator
    Given an installed Bash shortcut and a provider candidate "git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' --buffer --unordered | head -5"
    When I invoke Ctrl-W with the current input
    Then the stage "git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' --buffer --unordered" should continue on a following stack row
    And the last stack row of that stage should end with the "|" separator

  @givn.added
  Scenario: The selected stage is marked and its purpose follows the arrow keys
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the command stack should mark the selected stage "git log --oneline" with an arrow
    And the stage purpose "List recent commits." should appear below the stage stack
    When I move to the next stage
    Then the command stack should mark the selected stage "head -5" with an arrow
    And the stage purpose "Keep the first five." should appear below the stage stack

  @givn.added
  Scenario: The purpose below the stack is marked and readable
    Given an installed Bash shortcut and a provider candidate "git log --oneline | head -5"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep the first five."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the purpose below the stack should be marked and readable

  @givn.added
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
    And the detailed review should show the command stack
    And the detailed review should show the stage navigation hint
    When I move to the next stage
    Then the command stack should mark the selected stage "head -5" with an arrow
    When I switch back to the simple view
    Then the review surface should be in the simple view

  @givn.added
  Scenario: A stage longer than the stack window is truncated with a marker
    Given an installed Bash shortcut and a provider candidate with one stage longer than the terminal
    When I invoke Ctrl-W with current input "inspect recent log changes"
    Then the review surface should remain a bounded inline panel
    And the truncated stage should end with a truncation marker

  @givn.modified
  Scenario: Rephrasing starts a new candidate cycle
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I rephrase the intent as "show disk usage for mounted filesystems"
    And I switch to the detailed view
    Then a new candidate should be generated for the current intent
    And the visible intent should be "show disk usage for mounted filesystems"
    And the prior intent should remain only in current-review history
    And the prior candidate should not be accepted by the new cycle

  @givn.modified
  Scenario: Higher-tier review generates a candidate at the next configured tier
    Given an installed Bash shortcut and a candidate generated at the small tier
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I request a higher tier
    And I switch to the detailed view
    Then a new candidate should be generated at the next configured tier
    And its tier and provider/model context should be visible
    And the current intent should remain unchanged

  @givn.modified
  Scenario: The review card emphasizes the decision shortcut keys
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I switch to the detailed view
    Then the decision keys should be shown colored and bold
    When I press the edit shortcut
    Then the editor keys should be shown colored and bold
    When I press Escape in the command editor
    And I press the reject shortcut
    Then the chooser keys should be shown colored and bold

  @givn.modified
  Scenario: The review card exposes the decision shortcuts
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    And I switch to the detailed view
    Then the review surface should show "accept"
    And the review surface should show "edit"
    And the review surface should show "reject"
    And the review surface should show "cancel"
    And the review surface should show "disable"
    And accept should be shown as the default decision

  @givn.modified
  Scenario: A narrow terminal keeps the inline review bounded and readable
    Given an installed Bash shortcut and a provider candidate with more stages than fit in the terminal
    When I invoke Ctrl-W with current input "inspect recent log changes"
    Then the review surface should remain a bounded inline panel
    And the command stack should mark hidden stages with a hidden-window marker
    And the command stack should mark the selected stage "a" with an arrow
    And arrow navigation should reach every command-flow stage

  @e2e @givn.added
  Scenario: Developer switches to the detailed review view during Ctrl-W review
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And the candidate has a visible command flow with model-written stage purposes
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I switch to the detailed view in the review surface
    Then the review surface should show the detailed view
    And the review surface should show the stage navigation hint
    When I accept the selected candidate in the review surface
    Then the Bash command line should contain the accepted candidate

  @e2e @givn.modified @wip
  Scenario: The panel can permanently disable the review
    Given a configured provider candidate "git log --oneline | head -5"
    When I ask interactively for "inspect recent commits"
    And I switch to the detailed view in the review surface
    And I press the disable-review decision in the review surface
    Then the review should be disabled in the configuration
    And normal command output should contain only "git log --oneline | head -5"
    And the terminal should show "watn --review-panel"
    And watn should exit successfully

  @e2e @givn.added @wip
  Scenario: The review surface switches configure without a request
    Given a configured provider with candidate "df -h"
    And the persisted review surface is enabled
    When I run watn without a request with --no-review-panel
    Then the exit status should be 0
    And the command-output channel should remain empty
    And the review surface should be disabled in the configuration
    When I run watn without a request with --review-panel
    Then the exit status should be 0
    And the command-output channel should remain empty
    And the review surface should be enabled in the configuration
