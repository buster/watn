Feature: Isolated test transport

  @e2e
  Scenario: Normal debug requests ignore test routing settings
    Given  a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And  the configured provider has api key "sk-configured" and default model "test-model"
    And  a separate reachable local competing provider twin returns "wrong-endpoint" for POST "/v1/chat/completions"
    When  I run the default-feature debug binary with the override set to the competing twin
    Then  the binary should exit successfully with output containing "configured-response"
    And  the binary should request exactly the configured twin base URL plus "/v1/chat/completions"
    And  the configured-twin request should be POST path "/v1/chat/completions" exactly once
    And  the configured-twin request should have Authorization exactly "Bearer sk-configured"
    And  the competing twin should receive exactly 0 requests for path "/v1/chat/completions"
    And  the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"

  @e2e
  Scenario: Test-support requests use isolated routing without changing saved configuration
    Given  a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And  the configured provider has api key "sk-configured" and default model "test-model"
    And  a separate reachable local isolated provider twin returns "isolated-response" for POST "/v1/chat/completions"
    When  I run the test-support debug binary with the override set to the isolated twin
    Then  the response should contain "isolated-response"
    And  the isolated twin base URL plus "/v1" should be the exact request endpoint, with path "/chat/completions"
    And  the isolated-twin request should be POST path "/v1/chat/completions" exactly once
    And  the isolated-twin request should have Authorization exactly "Bearer sk-configured"
    And  the configured twin should receive exactly 0 requests for path "/v1/chat/completions"
    And  the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"
    And  the persisted TOML should not contain the isolated twin URL

  @e2e
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

  Scenario: Provider readiness ignores the test routing setting
    Given  a reachable local configured provider record has endpoint path "/v1", api key "sk-configured", and default model "test-model"
    And  a separate reachable local competing endpoint is selected by the test routing setting
    When  I evaluate provider readiness with the test routing setting present without starting an HTTP request
    Then  provider readiness should be ready
    And  the configured endpoint in the provider record should remain exactly the configured local endpoint
    And  both local endpoints should have received exactly 0 requests
