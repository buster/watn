@givn.delta @transport

Feature: Isolated test transport

  @givn.modified
  Scenario: Missing or whitespace test overrides fall back to the configured provider
    Given  a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And  the configured provider has api key "sk-configured" and default model "test-model"
    And  a separate reachable local competing provider twin returns "wrong-endpoint" for POST "/v1/chat/completions"
    When  I run the test-support debug binary once with no override and once with a whitespace override
    Then  each fallback response should contain "configured-response"
    And  each fallback request should use exactly the configured twin base URL plus "/v1/chat/completions"
    And  each fallback request should have Authorization exactly "Bearer sk-configured"
    And  the configured twin should receive exactly 2 requests for path "/v1/chat/completions"
    And  the competing twin should receive exactly 0 requests for path "/v1/chat/completions"
    And  the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"
