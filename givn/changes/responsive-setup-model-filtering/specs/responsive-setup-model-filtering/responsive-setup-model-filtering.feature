# User Interaction Inventory:
# - type a model filter in the setup wizard and observe the query and matching results update while the catalog search is delayed
#
@givn.delta @responsive-setup-model-filtering
Feature: Responsive setup model filtering

  @givn.added
  Scenario: A complete catalog is filtered locally
    Given a provider with a complete model catalog containing "gpt-4o-mini", "gpt-4o", and "o3-mini"
    And the catalog can be loaded in one response
    When I type "gpt" into the active model filter
    Then the model filter should show "gpt"
    And the suggestions should contain "gpt-4o-mini" and "gpt-4o"
    And the suggestions should not contain "o3-mini"
    And the provider should not receive a search request

  @givn.added
  Scenario: A catalog requiring more data uses provider-backed filtering
    Given a provider with a catalog larger than one response
    And the provider search returns "o3-pro" for the query "o3"
    When I type "o3" into the active model filter
    Then the model filter should show "o3"
    And the suggestions should contain "o3-pro"
    And the provider should receive a search request for "o3"

  @givn.added @wip
  Scenario: A newer model query remains authoritative
    Given a provider returns the result for "gpt" after the result for "o3"
    When I type "gpt" and then replace it with "o3" before either result is applied
    Then the suggestions should show only the results for "o3"
    And a later result for "gpt" should not replace them

  @givn.added @e2e @wip
  Scenario: The terminal model filter stays responsive during a delayed search
    Given a configured provider with an incomplete model catalog containing "gpt-4o-mini", "gpt-4o", and "o3-pro"
    And the provider delays a model search response
    When I start the setup wizard in a terminal
    And I type "gpt" into the active model filter
    And I replace the filter with "o3" before the delayed response arrives
    Then the terminal should keep showing the current filter "o3"
    And the terminal should show the matching "o3-pro" suggestion
    When I replace the filter with "gpt"
    Then the terminal should show the current filter "gpt"
