@givn.delta @interactive-shell-shortcut
Feature: Provider stage split

  @givn.added
  Scenario: A provider stage split that covers the command is shown with its purposes
    Given an installed Bash shortcut and a provider candidate for "find the biggest committed files"
    And the provider returns a review response whose stage split covers the command
    When I invoke Ctrl-W with the current input
    Then the review surface should show the stage "git rev-list --all"
    And the review surface should show the stage "while read commit; do git ls-tree -r $commit | awk '{print $4, $3}'; done"
    And the review surface should show the stage "sort | uniq"
    And the review surface should show the stage purpose "List all commit hashes in the git repository"
    And the review surface should show the stage purpose "For each commit, recursively list all files with their object hashes and extract filename and object hash"
    And the review surface should show the stage purpose "Sort the file entries and remove duplicates"

  @givn.added
  Scenario: Provider stages that do not cover the command are not trusted
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns a review response whose stage text is not part of the command
    When I invoke Ctrl-W with the current input
    Then the review surface should show the command "df -h"
    And the review surface should show purpose-unavailable or unsupported flow state
    And final acceptance should still be required
