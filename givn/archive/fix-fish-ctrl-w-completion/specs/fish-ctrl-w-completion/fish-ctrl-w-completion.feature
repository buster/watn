# User Interaction Inventory:
# - press Ctrl-W in an installed Fish shortcut and observe the generated command in the editable command line

@givn.delta @interactive-shell-shortcut
Feature: Fish Ctrl-W buffer insertion

  @givn.added @e2e
  Scenario: Fish inserts a real line break after Ctrl-W
    Given an installed Fish shortcut and a fake watn that returns "df -h"
    When I press Ctrl-W in the Fish shortcut with current input "show available diskspace"
    Then the Fish command line should be exactly "# show available diskspace\ndf -h"
