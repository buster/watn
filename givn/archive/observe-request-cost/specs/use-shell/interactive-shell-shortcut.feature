@givn.delta @interactive-shell-shortcut

Feature: Interactive shell shortcut

  @givn.modified @e2e
  Scenario: Developer accepts an explained candidate from Ctrl-W
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And  the candidate has a visible command flow with model-written stage purposes
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I accept the selected candidate in the review surface
    Then  the Bash command line should be exactly the accepted candidate
    And  the Bash command line should not contain a billed amount
    And  the Bash history should contain the original request comment "# inspect recent log changes"
    And  the accepted candidate should not have executed
