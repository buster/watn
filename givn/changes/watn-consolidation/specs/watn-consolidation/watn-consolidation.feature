# User Interaction Inventory:
# - Run `givn check review --change <fixture-change>` to verify repository-wide dispositions
# - Run `givn archive --change <fixture-change>` to publish consolidated permanent specs

@givn.delta @watn-consolidation
Feature: Watn consolidation

  @givn.added @e2e @wip
  Scenario: Repository-wide review accepts the consolidation dispositions
    Given an isolated watn consolidation fixture with dispositions for every overlap finding
    When the maintainer invokes the fixture review command
    Then the fixture command exits 0
    And fixture stdout contains "overlap dispositions passed"
    And fixture stdout contains "net delta: 1 added, 0 modified, 1 removed"

  @givn.added @e2e @wip
  Scenario: Archive publishes the consolidated permanent specifications
    Given an isolated watn consolidation fixture with dispositions for every overlap finding
    When the maintainer invokes the fixture archive command
    Then the fixture command exits 0
    And fixture stdout contains "Archived 'fixture-consolidation'"
    And the fixture permanent specification tree contains no duplicate scenario titles
    And the fixture permanent specification tree contains "Canonical retained behavior"
    And the fixture permanent specification tree does not contain "Obsolete behavior"

  @givn.added @wip
  Scenario: Failed archive preserves the fixture permanent specification tree
    Given an isolated watn consolidation fixture with a failing archive hook
    When the maintainer runs `givn archive --change fixture-consolidation`
    Then the fixture command fails
    And the fixture permanent specification tree remains unchanged
