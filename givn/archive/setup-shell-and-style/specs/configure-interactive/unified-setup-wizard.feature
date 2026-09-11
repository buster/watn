@givn.delta @unified-setup-wizard

Feature: Unified setup wizard

  @givn.added
  Scenario: Setup surfaces use the review visual language
    Given  a configured provider with catalog models "alpha", "beta", and "gamma"
    When  I start `watn setup` in a terminal
    Then  the setup frame should show the watn setup label
    And  the active page marker should be "◆"
    And  the footer should show bold key hints with dim labels
    And  the setup palette should use cyan labels and dim borders

  @givn.added
  Scenario: Setup warnings use amber attention markup
    Given  the provider catalog contains an empty model identifier and a duplicate model identifier
    When  I start `watn models` in a terminal
    Then  the setup warning should be marked with "⚠" in amber

  @givn.added
  Scenario: Setup renders readable text without color support
    Given  a configured provider with catalog models "alpha", "beta", and "gamma"
    And  the terminal color capability is disabled
    When  I start `watn setup` in a terminal
    Then  the setup output should not contain indexed color sequences
    And  the active page marker should be "◆"
