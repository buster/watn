Feature: Release truth and repository cleanup

  @e2e
  Scenario: Version flag reports the package version
    Given  the package version is current
    When  I run the release binary with `--version`
    Then  the exit status should be 0
    And  the output should contain "watn"
    And  the output should contain exactly the package version

  Scenario: Release artifact reports target-dependent runtime libraries
    Given  a release binary has been built for the current host
    When  I inspect the release artifact's file type and runtime libraries
    Then  it is identified as a dynamically linked executable for the current host
    And  the runtime library inspection succeeds with at least one shared library
    And  the deployment documentation states that requirements depend on the target

  Scenario: Active documentation describes current command streaming
    Given  the active README and architecture documentation
    When  I inspect the current command-output and configuration claims
    Then  the documentation states that command content is streamed incrementally
    And  the documentation states that reasoning is buffered and verbose-only
    And  the documentation names Ctrl-R as the reasoning focus shortcut
    And  the documentation describes configuration in the XDG config directory
    And  the documentation does not claim universal static deployment
    And  the documentation does not claim an XDG data directory
    And  the documentation does not claim release verification is deferred
    And  the documentation does not use plain r for reasoning focus
    And  the documentation does not name obsolete setup helper components

  Scenario: Active documentation distinguishes archived historical snapshots
    Given  the active architecture documentation and archived architecture snapshots
    When  I inspect their status labels
    Then  active documentation identifies archived snapshots as historical
    And  archived snapshots are not presented as the current architecture
  @e2e
  Scenario: Repeated revisions of one change yield one changelog entry
    Given  a release range with three revisions of one change that share the release note "Explain an existing shell command without executing it"
    And  a documentation revision in the same range
    When  I generate the changelog for the release range
    Then  the changelog lists "Explain an existing shell command without executing it" exactly once
    And  the changelog lists no documentation entry

  Scenario: A changelog section keeps only user-visible groups
    Given  a release range with a feature, a bug fix, a performance change, a revert, and a breaking change
    And  a documentation, a refactoring, a test, a chore, and an uncategorised revision in the same range
    When  I generate the changelog for the release range
    Then  the changelog section lists the feature, the bug fix, the performance change, the revert, and the breaking change
    And  the changelog section contains no "Documentation", "Refactoring", or "Other Changes" group
