# User Interaction Inventory:
# - run `watn --version` and inspect the reported package version

@givn.delta @release-truth
Feature: Release truth and repository cleanup

  @givn.added @e2e @wip
  Scenario: Version flag reports the package version
    Given the package version is "0.1.2"
    When I run the release binary with `--version`
    Then the exit status should be 0
    And the output should contain "watn"
    And the output should contain exactly the package version "0.1.2"

  @givn.added
  Scenario: Release artifact reports target-dependent runtime libraries
    Given a release binary has been built for the current host
    When I inspect the release artifact's file type and runtime libraries
    Then it is identified as a dynamically linked executable for the current host
    And the runtime library inspection succeeds with at least one shared library
    And the deployment documentation states that requirements depend on the target

  @givn.added
  Scenario: Active documentation describes current command streaming
    Given the active README and architecture documentation
    When I inspect the current command-output and configuration claims
    Then the documentation states that command content is streamed incrementally
    And the documentation states that reasoning is buffered and verbose-only
    And the documentation names Ctrl-R as the reasoning focus shortcut
    And the documentation describes configuration in the XDG config directory
    And the documentation does not claim universal static deployment
    And the documentation does not claim an XDG data directory
    And the documentation does not claim release verification is deferred
    And the documentation does not use plain r for reasoning focus
    And the documentation does not name obsolete setup helper components

  @givn.added
  Scenario: Active documentation distinguishes archived historical snapshots
    Given the active architecture documentation and archived architecture snapshots
    When I inspect their status labels
    Then active documentation identifies archived snapshots as historical
    And archived snapshots are not presented as the current architecture
