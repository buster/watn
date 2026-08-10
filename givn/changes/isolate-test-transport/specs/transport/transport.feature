# User Interaction Inventory:
# - run a normal release-profile watn request while a non-empty test routing setting is present
# - run a test-support debug watn request through an isolated local provider twin
# - run a test-support debug watn request with a missing or whitespace override and fall back to the configured local provider

@givn.delta @transport
Feature: Isolated test transport

  @givn.added @e2e
  Scenario: Normal release requests ignore test routing settings
    Given a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And the configured provider has api key "sk-configured" and default model "test-model"
    And a separate reachable local competing provider twin returns "wrong-endpoint" for POST "/v1/chat/completions"
    When I run the default-feature release binary and the test-support release binary with the override set to the competing twin
    Then each binary should exit successfully with output containing "configured-response"
    And each binary should request exactly the configured twin base URL plus "/v1/chat/completions"
    And each configured-twin request should be POST path "/v1/chat/completions" exactly once
    And each configured-twin request should have Authorization exactly "Bearer sk-configured"
    And the competing twin should receive exactly 0 requests for path "/v1/chat/completions"
    And the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"

  @givn.added @e2e
  Scenario: Test-support requests use isolated routing without changing saved configuration
    Given a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And the configured provider has api key "sk-configured" and default model "test-model"
    And a separate reachable local isolated provider twin returns "isolated-response" for POST "/v1/chat/completions"
    When I run the test-support debug binary with the override set to the isolated twin
    Then the response should contain "isolated-response"
    And the isolated twin base URL plus "/v1" should be the exact request endpoint, with path "/chat/completions"
    And the isolated-twin request should be POST path "/v1/chat/completions" exactly once
    And the isolated-twin request should have Authorization exactly "Bearer sk-configured"
    And the configured twin should receive exactly 0 requests for path "/v1/chat/completions"
    And the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"
    And the persisted TOML should not contain the isolated twin URL

  @givn.added @e2e @wip
  Scenario Outline: Missing or whitespace test overrides fall back to the configured provider
    Given a reachable local configured provider twin returns "configured-response" for POST "/v1/chat/completions"
    And the configured provider has api key "sk-configured" and default model "test-model"
    And a separate reachable local competing provider twin returns "wrong-endpoint" for POST "/v1/chat/completions"
    When I run the test-support debug binary with the override state "<override-state>"
    Then the response should contain "configured-response"
    And the configured twin base URL plus "/v1" should be the exact request endpoint, with path "/chat/completions"
    And the configured-twin request should be POST path "/v1/chat/completions" exactly once
    And the configured-twin request should have Authorization exactly "Bearer sk-configured"
    And the competing twin should receive exactly 0 requests for path "/v1/chat/completions"
    And the persisted configured endpoint should remain exactly the configured twin base URL plus "/v1"

    Examples:
      | override-state |
      | missing        |
      | whitespace     |

  @givn.added
  Scenario: Provider readiness ignores the test routing setting
    Given a reachable local configured provider record has endpoint path "/v1", api key "sk-configured", and default model "test-model"
    And a separate reachable local competing endpoint is selected by the test routing setting
    When I evaluate provider readiness with the test routing setting present without starting an HTTP request
    Then provider readiness should be ready
    And the configured endpoint in the provider record should remain exactly the configured local endpoint
    And both local endpoints should have received exactly 0 requests
