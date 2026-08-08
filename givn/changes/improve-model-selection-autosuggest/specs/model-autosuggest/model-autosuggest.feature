# User Interaction Inventory:
# - Run `watn models`, type a model search into the active tier picker, and choose a suggestion.

@givn.delta @model-autosuggest
Feature: Model auto-suggest

  @givn.added @e2e @wip
  Scenario: Find a model outside the initial page while assigning tiers
    Given a provider with a paginated model catalog
    And the initial suggestions include "gpt-4o-mini" and "gpt-4o"
    And a later catalog page includes "o3-pro"
    When I run `watn models`, type "o3" into the small tier picker, and choose "o3-pro"
    And choose "o3-pro" for the normal tier
    And choose "o3-pro" for the thinking tier
    Then the picker displays "o3-pro" as a matching suggestion
    And the completed setup reports small="o3-pro", normal="o3-pro", thinking="o3-pro"

  @givn.added @wip
  Scenario: Suggestions update as the search text changes
    Given a provider with models "gpt-4o-mini", "gpt-4o", "o3-mini", and "o3-pro"
    When I type "gpt" into the active tier picker
    Then the suggestions include "gpt-4o-mini" and "gpt-4o"
    And the suggestions do not include "o3-mini" or "o3-pro"
    When I replace the search text with "o3"
    Then the suggestions include "o3-mini" and "o3-pro"
    And the suggestions do not include "gpt-4o-mini" or "gpt-4o"

  @givn.added @wip
  Scenario: No matching model produces a clear empty state
    Given a provider with models "gpt-4o-mini" and "gpt-4o"
    When I type "does-not-exist" into the active tier picker
    Then the picker says that no models were found
    And the picker remains available for another search

  @givn.added @wip
  Scenario: Clearing the search restores available suggestions
    Given a provider with models "gpt-4o-mini", "gpt-4o", and "o3-mini"
    When I type "o3" into the active tier picker
    And I clear the search text
    Then the initial available suggestions are shown again

  @givn.added @wip
  Scenario: The newest search result stays visible when an older result arrives later
    Given a provider returns the results for "gpt" more slowly than the results for "o3"
    When I type "gpt" into the active tier picker
    And I replace the search text with "o3"
    Then the suggestions for "o3" are displayed
    And a later result for "gpt" does not replace them

  @givn.added @wip
  Scenario: An endpoint without search support reports a usable error
    Given a provider that does not support searching its model catalog
    When I type "o3" into the active tier picker
    Then the picker reports that model search is unavailable
    And the current tier selection remains available

  @givn.added @wip
  Scenario: Selecting a suggestion advances to the next tier
    Given a provider with models "gpt-4o-mini" and "gpt-4o"
    When I type "gpt-4o" into the small tier picker
    And I choose "gpt-4o"
    Then the small tier is assigned to "gpt-4o"
    And the picker presents the normal tier
