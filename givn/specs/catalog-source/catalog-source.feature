Feature: Catalog source resolution

  Scenario: Catalog pagination and search use the configured catalog source
    Given  a provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog has api key "${LITELLM_API_KEY}"
    And  environment variable LITELLM_API_KEY is set to "sk-litellm-key"
    When  the catalog requests page 2 with limit 50 and searches for "o3"
    Then  the catalog page request should be GET "/models?page=2&limit=50" on the LiteLLM endpoint
    And  the catalog search request should be GET "/models?search=o3" on the LiteLLM endpoint
    And  both catalog requests should include Authorization exactly "Bearer sk-litellm-key"
