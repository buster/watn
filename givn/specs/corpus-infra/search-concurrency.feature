Feature: Newest model search wins

  Scenario: The terminal model picker displays the newest overlapping search result
    Given  a configured provider "test" with a searchable models endpoint
    And  the endpoint returns "gpt" results before "o3" results
    When  I type "gpt" and then "o3" before either search result is applied in the terminal picker
    Then  the terminal suggestions should contain only the "o3" results
    And  the picker should join the search workers before exit
