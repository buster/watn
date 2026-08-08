Feature: Model auto-suggest

  @e2e
  Scenario: Find a model outside the initial page while assigning tiers
    Given  a provider with a paginated model catalog
    And  the initial suggestions include "gpt-4o-mini" and "gpt-4o"
    And  a later catalog page includes "o3-pro"
    When  I run `watn models`, type "o3" into the small tier picker, and choose "o3-pro"
    And  choose "o3-pro" for the normal tier
    And  choose "o3-pro" for the thinking tier
    Then  the picker displays "o3-pro" as a matching suggestion
    And  the completed setup reports small="o3-pro", normal="o3-pro", thinking="o3-pro"

  Scenario: Suggestions update as the search text changes
    Given  a provider with models "gpt-4o-mini", "gpt-4o", "o3-mini", and "o3-pro"
    When  I type "gpt" into the active tier picker
    Then  the suggestions include "gpt-4o-mini" and "gpt-4o"
    And  the suggestions do not include "o3-mini" or "o3-pro"
    When  I replace the search text with "o3"
    Then  the suggestions include "o3-mini" and "o3-pro"
    And  the suggestions do not include "gpt-4o-mini" or "gpt-4o"

  Scenario: No matching model produces a clear empty state
    Given  a provider with models "gpt-4o-mini" and "gpt-4o"
    When  I type "does-not-exist" into the active tier picker
    Then  the picker says that no models were found

  Scenario: The newest search result stays visible when an older result arrives later
    Given  a provider returns the results for "gpt" more slowly than the results for "o3"
    When  I type "gpt" into the active tier picker
    And  I replace the search text with "o3"
    Then  the suggestions for "o3" are displayed
    And  a later result for "gpt" does not replace them

  Scenario: An endpoint without search support reports a usable error
    Given  a provider that does not support searching its model catalog
    When  I type "o3" into the active tier picker
    Then  the picker reports that model search is unavailable
