@givn.delta @interactive-shell-shortcut
Feature: Review response recovery and inline rendering safety

  @givn.added
  Scenario: A markdown-fenced structured response is still explained
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And the provider returns a markdown-fenced structured review response:
      """
      Here is the review result:
      ```json
      {"review_version":1,"command":"git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'","stages":[{"stage_text":"git log --format='%H' --since='7 days ago'","purpose":"Collect commit identifiers from the recent history."},{"stage_text":"xargs -n1","purpose":"Pass each collected identifier to the per-commit command."},{"stage_text":"git show --stat --oneline","purpose":"Inspect each collected commit with a compact change summary."},{"stage_text":"printf 'done'","purpose":"Report that the success branch completed."}],"purpose_status":"ready"}
      ```
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the review surface should show the git log stage
    And the review surface should show the xargs stage
    And the review surface should show the git show stage
    And the review surface should show the success branch
    And the review surface should show each model-written stage purpose

  @givn.added @wip
  Scenario: An invalid structured response with a command stays reviewable
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns an invalid structured review response with the command "df -h"
    When I invoke Ctrl-W with the current input
    Then the review surface should show the command "df -h"
    And the review surface should show purpose-unavailable or unsupported flow state
    And final acceptance should still be required

  @givn.added @wip
  Scenario: A provider payload without a usable command releases nothing
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns a structured review payload without a complete command
    When I invoke Ctrl-W with the current input
    Then the original Bash command line should remain unchanged
    And no candidate should be released to the shell

  @givn.added @wip
  Scenario: A multiline provider payload keeps every rendered value on one row
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns a markdown-fenced structured review response with line breaks
    When I invoke Ctrl-W with the current input
    Then every rendered review value should stay on one inline row
    And the review surface should remain a bounded inline panel
