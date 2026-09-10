@givn.delta @interactive-shell-shortcut
Feature: Persisted review panel setting

  @givn.added
  Scenario: Enabling the review panel from the command line persists it
    Given a configured provider with candidate "df -h"
    And the persisted review surface is disabled
    When I run watn with --review-panel for "show disk usage"
    Then the exit status should be 0
    And the output should contain "df -h"
    And the review surface should be enabled in the configuration

  @givn.added
  Scenario: Disabling the review panel from the command line persists it
    Given a configured provider with candidate "df -h"
    And the persisted review surface is enabled
    When I run watn with --no-review-panel for "show disk usage"
    Then the exit status should be 0
    And the output should contain "df -h"
    And the review surface should be disabled in the configuration
