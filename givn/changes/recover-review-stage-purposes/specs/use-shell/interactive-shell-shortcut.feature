@givn.delta @interactive-shell-shortcut
Feature: Review purpose recovery

  @givn.added
  Scenario: An unknown purpose status keeps matching model-written purposes
    Given an installed Bash shortcut and a provider candidate for "find the biggest committed files"
    And the provider returns a review response with an unknown purpose status and matching stage purposes
    When I invoke Ctrl-W with the current input
    Then the review surface should show the stage purpose "List every commit."
    And the review surface should show the stage purpose "List the files in each commit."
    And the review surface should show the stage purpose "Keep the first five files."
    And the review surface should not claim that purposes are loading

  @givn.added
  Scenario: A command broken across lines is explained on one row per stage
    Given an installed Bash shortcut and a provider candidate for "find the biggest committed files"
    And the provider returns a review response whose command spans several lines with matching stage purposes
    When I invoke Ctrl-W with the current input
    Then the reviewed candidate command should be a single line
    And the review surface should show the stage purpose "List every commit."
    And every rendered review value should stay on one inline row

  @givn.added @wip
  Scenario: Mismatched stage text still shows purpose-unavailable
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns a review response with mismatched stage text
    When I invoke Ctrl-W with the current input
    Then the review surface should show the command "df -h"
    And the review surface should show purpose-unavailable or unsupported flow state
    And final acceptance should still be required
