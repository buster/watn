# User Interaction Inventory:
# - type overlapping searches into the model picker and observe the final suggestions

@givn.delta @search-concurrency
Feature: Newest model search wins

  @givn.modified
  Scenario: The newest search result stays visible when an older result arrives later
    Given a provider returns the results for "gpt" more quickly than the results for "o3"
    When I start the "gpt" search and the "o3" search before either result is applied
    Then the suggestions for "o3" are displayed after the newer search completes
    And a later result for "gpt" does not replace them
    And search workers are cleaned up before the scenario ends

  @givn.added @e2e
  Scenario: The terminal model picker displays the newest overlapping search result
    Given a configured provider "test" with a searchable models endpoint
    And the endpoint returns "gpt" results before "o3" results
    When I type "gpt" and then "o3" before either search result is applied in the terminal picker
    Then the terminal suggestions should contain only the "o3" results
    And the picker should join the search workers before exit
