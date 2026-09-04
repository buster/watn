Feature: Catalog source resolution

  @e2e
  Scenario: Configured LiteLLM is used for model catalog requests
    Given  a provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog requires api key "sk-litellm-key"
    And  the LiteLLM catalog returns models ["model-small", "model-normal", "model-thinking"]
    When  I run `watn models` and select "model-small" for small, "model-normal" for normal, and "model-thinking" for thinking
    Then  the output should contain "Tiers configured"
    And  the model catalog request should use the LiteLLM endpoint
    And  the model catalog request should use GET path "/models"
    And  the model catalog request should include Authorization exactly "Bearer sk-litellm-key"
    And  the config file should contain the selected tier assignments

  Scenario: LiteLLM discovery without a key sends no authorization header
    Given  a provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog has no api key
    And  the LiteLLM catalog returns models ["model-a", "model-b", "model-c"]
    When  I run `watn models` and select "model-a" for small, "model-b" for normal, and "model-c" for thinking
    Then  the model catalog request should use the LiteLLM endpoint
    And  the model catalog request should not include an Authorization header

  Scenario: Provider discovery is used when LiteLLM is absent
    Given  a provider "custom" with a provider catalog endpoint and api key "sk-provider-key"
    And  the provider catalog returns models ["model-a", "model-b", "model-c"]
    When  I run `watn models` and select "model-a" for small, "model-b" for normal, and "model-c" for thinking
    Then  the model catalog request should use the provider endpoint
    And  the model catalog request should include Authorization exactly "Bearer sk-provider-key"

  @e2e
  Scenario: LiteLLM discovery does not replace the active chat provider
    Given  a provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog has no api key
    And  the LiteLLM catalog returns models ["custom-model"]
    And  the provider chat endpoint returns "provider-response"
    When  I run `watn models` and select "custom-model" for the small tier
    And  I run `watn "hello"`
    Then  the output should contain "provider-response"
    And  the chat request should use the provider endpoint
    And  the chat request should not use the LiteLLM endpoint

  Scenario: Catalog pagination and search use the configured catalog source
    Given  a provider "custom" with a separate LiteLLM catalog endpoint
    And  the LiteLLM catalog has api key "${LITELLM_API_KEY}"
    And  environment variable LITELLM_API_KEY is set to "sk-litellm-key"
    When  the catalog requests page 2 with limit 50 and searches for "o3"
    Then  the catalog page request should be GET "/models?page=2&limit=50" on the LiteLLM endpoint
    And  the catalog search request should be GET "/models?search=o3" on the LiteLLM endpoint
    And  both catalog requests should include Authorization exactly "Bearer sk-litellm-key"
