@givn.delta @release-truth

Feature: Release truth and repository cleanup

  @givn.added @e2e
  Scenario: Repeated revisions of one change yield one changelog entry
    Given  a release range with three revisions of one change that share the release note "Explain an existing shell command without executing it"
    And  a documentation revision in the same range
    When  I generate the changelog for the release range
    Then  the changelog lists "Explain an existing shell command without executing it" exactly once
    And  the changelog lists no documentation entry

  @givn.added
  Scenario: A changelog section keeps only user-visible groups
    Given  a release range with a feature, a bug fix, a performance change, a revert, and a breaking change
    And  a documentation, a refactoring, a test, a chore, and an uncategorised revision in the same range
    When  I generate the changelog for the release range
    Then  the changelog section lists the feature, the bug fix, the performance change, the revert, and the breaking change
    And  the changelog section contains no "Documentation", "Refactoring", or "Other Changes" group
