@givn.delta @ask

Feature: Ask for a command

  @givn.added
  Scenario: The metadata line prices a request from the requested model when the reported model has none
    Given the request asks for model "requested-model"
    And a streaming provider emits content "printf priced" and a choices-empty usage event with response model "response-model", 10 prompt tokens, and 20 completion tokens
    And pricing is configured only for "requested-model" at 2.50 input and 10.00 output per million tokens
    When I run `watn "show usage"`
    Then stdout should contain "printf priced"
    And the final metadata names exactly "response-model"
    And stderr should contain a non-zero cost for "response-model"
    And the exit status should be 0
