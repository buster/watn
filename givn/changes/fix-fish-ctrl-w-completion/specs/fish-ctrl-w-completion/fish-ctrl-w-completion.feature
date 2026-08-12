# User Interaction Inventory:
# - press Ctrl-W in an installed Fish shortcut and observe the generated command in the editable command line

@givn.delta @interactive-shell-shortcut
Feature: Fish Ctrl-W command insertion

  @givn.added @e2e
  Scenario: Fish keeps the generated command executable after Ctrl-W
    Given an installed Fish shortcut and a fake watn that returns "df -h"
    When I press Ctrl-W in the Fish shortcut with current input "show available diskspace"
    Then the Fish command line should be exactly "# show available diskspace\ndf -h"
