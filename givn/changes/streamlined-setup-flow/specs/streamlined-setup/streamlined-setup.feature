# User Interaction Inventory:
# - invoke `watn setup` and complete the coordinated configuration flow
# - invoke `watn provider` and configure a provider independently
# - invoke `watn models` and configure the three model roles independently
# - invoke `watn shell` and configure shell integrations independently
# - invoke an interactive `watn "question"` request when setup is incomplete

@givn.delta @streamlined-setup

Feature: Streamlined setup flow

  @givn.added @e2e
  Scenario: Coordinated setup completes provider models reasoning and shell choices
    Given no config file exists
    And no supported provider environment variable is set
    And the ephemeral E2E transport returns models ["small-model", "normal-model", "thinking-model"] for "/models"
    When I start `watn setup` in a terminal
    Then the setup coordinator should show the provider question first
    When I choose provider "OpenRouter"
    And accept the default completion endpoint
    And choose to paste an API key
    And enter API key "sk-coordinated-key"
    And accept the derived catalog endpoint
    And choose "small-model" for the small role
    And choose reasoning "low" for the small role
    And choose "normal-model" for the normal role
    And choose reasoning "medium" for the normal role
    And choose "thinking-model" for the thinking role
    And choose reasoning "high" for the thinking role
    And choose no shell completion integrations
    And choose no Ctrl-W shortcut integrations
    And confirm the setup review
    Then setup should exit successfully
    And the config file should contain provider "openrouter"
    And the config file should contain small model "small-model" with reasoning "low"
    And the config file should contain normal model "normal-model" with reasoning "medium"
    And the config file should contain thinking model "thinking-model" with reasoning "high"

  @givn.added
  Scenario: Coordinated setup displays one separate reasoning question after each model
    Given a configured provider with catalog models "alpha", "beta", and "gamma"
    When I start `watn setup` in a terminal
    And advance to the small model question
    Then the small model question should not contain the reasoning choices
    When I choose model "alpha" for the small role
    Then the small reasoning question should identify model "alpha"
    When I choose reasoning "low" for the small role
    Then the normal model question should be active

  @givn.added
  Scenario: Rerunning coordinated setup prefills current values and preserves a masked literal credential
    Given a config file contains provider "custom" with endpoint "https://llm.example/v1"
    And the provider credential is the literal "sk-existing-key"
    And the config file contains models "small-old", "normal-old", and "thinking-old"
    When I start `watn setup` in a terminal
    Then the provider question should show "custom" selected
    And the completion endpoint input should show "https://llm.example/v1"
    And the credential input should remain masked
    And the small model input should show "small-old"
    And the normal model input should show "normal-old"
    And the thinking model input should show "thinking-old"

  @givn.added
  Scenario: Cancelling coordinated setup leaves an existing configuration unchanged
    Given an existing config contains provider "legacy" with credential "sk-old-key"
    And the existing config content is recorded
    When I start `watn setup` in a terminal
    And cancel setup before final confirmation
    Then the config file should be byte-for-byte unchanged

  @givn.added @e2e
  Scenario: Provider setup configures an OpenAI provider with an environment credential
    Given no provider is configured
    And environment variable OPENAI_API_KEY is set to "sk-openai-test"
    When I start `watn provider` in a terminal
    Then provider setup should show provider choices "OpenRouter", "OpenAI", and "Custom"
    When I choose provider "OpenAI"
    And accept the default completion endpoint
    And choose environment variable "OPENAI_API_KEY"
    Then provider setup should exit successfully
    And the config file should contain provider "openai"
    And the config file should contain endpoint exactly "https://api.openai.com/v1"
    And the config file should contain credential reference "${OPENAI_API_KEY}"
    And the config file should not contain "sk-openai-test"

  @givn.added
  Scenario: Provider setup requires a custom endpoint
    Given no provider is configured
    When I start `watn provider` in a terminal
    And choose provider "Custom"
    Then provider setup should not allow the empty endpoint
    When I enter endpoint "https://llm.example/v1"
    Then provider setup should allow the credential question

  @givn.added
  Scenario: Provider setup refuses an unresolved environment credential
    Given no provider is configured
    And environment variable MISSING_API_KEY is not set
    When provider setup chooses environment variable "MISSING_API_KEY"
    Then provider setup should show that "MISSING_API_KEY" must contain a non-empty value
    And the config file should not contain a provider entry for the attempted setup

  @givn.added
  Scenario: Provider setup preserves unrelated settings
    Given a config file contains provider "legacy" with endpoint "https://legacy.example/v1"
    And the config file contains pricing and LiteLLM settings
    When provider setup saves provider "custom" with endpoint "https://new.example/v1" and credential "sk-new-key"
    Then provider "custom" should contain endpoint "https://new.example/v1"
    And the pricing and LiteLLM settings should remain unchanged
    And provider "legacy" should not exist

  @givn.added
  Scenario: Provider setup does not probe the catalog
    Given no provider is configured
    When provider setup saves provider "custom" with endpoint "https://llm.example/v1" and credential "sk-key"
    Then no model catalog request should be sent

  @givn.added @e2e
  Scenario: Models setup configures all three roles from an available catalog
    Given a configured provider with catalog models "small-model", "normal-model", and "thinking-model"
    When I start `watn models` in a terminal
    Then the model setup should begin with the small role
    When I choose "small-model" for the small role
    And choose reasoning "low" for the small role
    And choose "normal-model" for the normal role
    And choose reasoning "medium" for the normal role
    And choose "thinking-model" for the thinking role
    And choose reasoning "high" for the thinking role
    Then models setup should exit successfully
    And the config file should contain the three selected model roles

  @givn.added
  Scenario: Models setup gives guidance when no provider is configured
    Given no provider is configured
    When I run `watn models` without a terminal
    Then the output should instruct me to run `watn provider`
    And no provider question should be shown

  @givn.added
  Scenario: Available catalog restricts model choices
    Given a configured provider with catalog models "catalog-one" and "catalog-two"
    And the config file contains model "not-in-catalog" for the small role
    When I start `watn models` in a terminal
    Then the small role should require a replacement model
    And the model choices should include only "catalog-one" and "catalog-two"

  @givn.added
  Scenario: Unavailable catalog allows manual model identifiers
    Given a configured provider with an unreachable catalog endpoint
    When I start `watn models` in a terminal
    Then model setup should warn that catalog discovery is unavailable
    And model setup should allow a manually entered model identifier

  @givn.added
  Scenario: Catalog metadata selects supported reasoning efforts for the chosen model
    Given a configured provider catalog model "reasoning-model" supports efforts "low", "medium", and "high"
    And the catalog default effort for "reasoning-model" is "medium"
    When I start `watn models` in a terminal
    And choose "reasoning-model" for the small role
    Then the small reasoning question should show only "low", "medium", and "high"
    And "medium" should be selected by default

  @givn.added
  Scenario: Missing reasoning metadata provides generic efforts and free-form input
    Given a configured provider catalog model "plain-model" has no reasoning metadata
    When I start `watn models` in a terminal
    And choose "plain-model" for the small role
    Then the small reasoning question should warn that supported efforts are unavailable
    And the generic reasoning choices should include "off", "low", "minimal", "medium", and "high"
    And the generic reasoning choices should include a custom effort entry
    When I enter custom reasoning effort "x-high"
    Then the small role should use reasoning "x-high"

  @givn.added
  Scenario: Off reasoning omits the reasoning setting from a request
    Given a configured provider with model "plain-model" for the small role
    And the small role reasoning is "off"
    When I send a small-role request through the configured provider
    Then the API request should omit the reasoning effort

  @givn.added @e2e
  Scenario: Shell setup independently configures completion and Ctrl-W integrations
    Given no Watn-managed shell integrations are installed
    When I start `watn shell` in a terminal
    Then shell setup should show independent completion and Ctrl-W questions
    And the shell choices should include only Bash, Fish, and Zsh
    When I choose Bash for completion
    And choose Zsh for the Ctrl-W shortcut
    Then shell setup should exit successfully
    And Bash should contain a Watn-managed completion block
    And Zsh should contain a Watn-managed Ctrl-W block
    And Fish should remain unchanged

  @givn.added
  Scenario: Shell setup prefills installed integrations and removes only managed blocks when deselected
    Given Bash contains a valid Watn-managed completion block
    And Bash contains user-owned shell content
    When I start `watn shell` in a terminal
    Then Bash completion should be selected
    When I deselect Bash completion
    Then the Watn-managed completion block should be removed from Bash
    And the user-owned shell content should remain

  @givn.added
  Scenario: Shell setup refuses malformed managed markers
    Given Bash contains duplicated Watn completion markers
    When I deselect Bash completion in shell setup
    Then shell setup should report a malformed managed block
    And the Bash file should remain unchanged

  @givn.added
  Scenario: Shell failure does not discard successful shell changes or configuration
    Given Bash can accept a Watn completion block
    And Zsh cannot be modified
    When coordinated setup is confirmed with Bash completion and Zsh Ctrl-W selected
    Then the Bash completion change should remain
    And the provider and model configuration should be saved
    And setup should report a nonzero result

  @givn.added @e2e
  Scenario: Incomplete interactive request opens setup and does not send the original request
    Given a usable provider credential is configured
    And the normal model role is missing
    And the request transport would return a successful answer
    When I start interactive `watn "hello"` in a terminal
    Then the setup coordinator should open
    And the existing provider values should be prefilled
    When I cancel setup before final confirmation
    Then no original chat completion request should be sent

  @givn.added
  Scenario: Non-interactive incomplete request prints setup guidance without probing
    Given no config file exists
    And a catalog request sentinel is installed
    When I run a non-TTY request for "hello"
    Then the exit status should be nonzero
    And stderr should instruct me to run `watn setup` or `watn provider` in a terminal
    And no model catalog request should be sent
    And no original chat completion request should be sent

  @givn.added
  Scenario: Malformed configuration is reported without modification
    Given the config file contains malformed TOML
    And the malformed config content is recorded
    When I run `watn setup`
    Then setup should exit with a configuration error
    And the malformed config file should be byte-for-byte unchanged

  @givn.added @wip
  Scenario: Cancelling after provider and credential validation does not create a config file
    Given no config file exists
    And a catalog request sentinel is installed
    When I accept a valid provider endpoint and credential in coordinated setup
    And cancel before the catalog question
    Then no config file should exist
    And no provider entry should be persisted
    And no model catalog request should be sent

  @givn.added
  Scenario: Cancelling after a successful catalog probe leaves the baseline unchanged
    Given an existing config is recorded byte-for-byte
    And the provider catalog returns valid models
    When I accept the provider, credential, and catalog probe in coordinated setup
    And cancel setup before final confirmation
    Then the config file should be byte-for-byte unchanged
    And no selected shell target should change

  @givn.added
  Scenario: Catalog failure does not persist an unconfirmed provider
    Given no config file exists
    And the provider-derived catalog request fails
    When I accept a valid provider endpoint and credential in coordinated setup
    And the catalog probe fails
    And cancel setup after catalog failure
    Then no config file should exist
    And no catalog endpoint should be persisted

  @givn.added
  Scenario: A successful edited catalog endpoint is promoted only at final confirmation
    Given a configured provider has catalog endpoint "https://old.example/v1"
    And the edited catalog endpoint "https://new.example/v1" returns valid models
    When I enter the edited catalog endpoint and probe it successfully
    Then the config file should still contain catalog endpoint "https://old.example/v1"
    When I confirm the final setup review
    Then the config file should contain catalog endpoint "https://new.example/v1"

  @givn.added
  Scenario: A failed edited catalog endpoint preserves the previous endpoint
    Given a configured provider has reachable catalog endpoint "https://old.example/v1"
    And the edited catalog endpoint "https://new.example/v1" is unreachable
    When I probe the edited catalog endpoint
    Then setup should keep catalog endpoint "https://old.example/v1"
    And setup should allow manual model entry
    And the config file should remain unchanged before confirmation

  @givn.added
  Scenario: A failed new catalog endpoint remains unset
    Given a configured provider has no saved catalog endpoint
    And the derived catalog endpoint is unreachable
    When I probe the derived catalog endpoint
    Then setup should show catalog status "Unset"
    And no catalog endpoint should be persisted before confirmation
    And setup should allow manual model entry after unset

  @givn.added
  Scenario: Invalid catalog data switches to manual model selection
    Given the provider catalog returns an empty model list
    When I run `watn models` in a terminal
    Then setup should report that catalog discovery is unusable
    And setup should not invent model identifiers
    And setup should allow a manually entered model identifier

  @givn.added
  Scenario: Catalog entries without unique non-empty identifiers are rejected
    Given the provider catalog contains an empty model identifier and a duplicate model identifier
    When I run `watn models` in a terminal
    Then setup should report that catalog discovery is unusable
    And setup should not deduplicate or select those entries
    And setup should allow manual model selection

  @givn.added
  Scenario: Provider catalog takes precedence over a conflicting legacy LiteLLM source
    Given a configured provider endpoint "https://provider.example/v1" with credential "sk-provider"
    And a legacy LiteLLM source points to "https://litellm.example/v1"
    And the provider-local catalog returns models ["provider-small", "provider-normal", "provider-thinking"]
    When I run `watn models` and select the provider-local models
    Then every provider-local catalog request should receive the provider credential
    And every provider-local catalog request should receive the provider models
    And the legacy LiteLLM source should receive zero requests
    And the legacy LiteLLM configuration should remain unchanged

  @givn.added
  Scenario: Provider catalog pagination and search use the provider source
    Given a configured provider endpoint "https://provider.example/v1" with credential "sk-provider"
    And the provider catalog supports pagination and search
    And a legacy LiteLLM source records every request
    When model setup requests page 2 with limit 50
    And model setup searches the catalog for "o3"
    Then the requests should use "https://provider.example/v1/models?page=2&limit=50" and "https://provider.example/v1/models?search=o3"
    And the legacy LiteLLM source should receive zero requests

  @givn.added
  Scenario: Manual model identifiers are persisted exactly after catalog failure
    Given a configured provider-derived catalog endpoint is unreachable
    When I enter manual models "small/manual", "normal/manual", and "thinking/manual"
    And confirm the models setup
    Then the three model identifiers should be persisted exactly as entered
    And the failed catalog endpoint should not become available

  @givn.added
  Scenario: Changing provider invalidates catalog-backed model choices
    Given a configured provider catalog contains model "old-model"
    When I change provider during coordinated setup
    Then the catalog status should become pending for the new provider
    And the old catalog-backed model should require revalidation or replacement

  @givn.added
  Scenario: The final review shows all draft domains without exposing a secret
    Given a complete coordinated setup draft with provider, catalog, models, reasoning, and shell choices
    When I open the final setup review
    Then the review should show the provider and completion endpoint
    And the review should show catalog endpoint status
    And the review should show all three model and reasoning pairs
    And the review should show completion and Ctrl-W shell choices
    And the review should show credential source and masked status
    And the review should not show the resolved credential

  @givn.added
  Scenario: Final confirmation is blocked while a required draft value is invalid
    Given a coordinated setup draft has a missing model role
    When I open the final setup review
    Then confirmation should be blocked
    And the review should identify the missing model role
    And no configuration or shell target should be changed

  @givn.added
  Scenario: Back navigation preserves draft values across model and reasoning questions
    Given coordinated setup has selected model "alpha" and reasoning "low" for the small role
    When I navigate back from the normal model question to the small reasoning question
    Then model "alpha" and reasoning "low" should remain selected
    When I navigate forward again
    Then the normal model question should be active

  @givn.added
  Scenario: Selected provider migration moves an arbitrary provider to custom
    Given the selected provider key is "legacy"
    And provider "legacy" has endpoint "https://legacy.example/v1" and default model "legacy-model"
    When I confirm provider setup without replacing its credential
    Then the default provider should be "custom"
    And provider "custom" should contain the legacy endpoint and default model "legacy-model"
    And provider "legacy" should not exist

  @givn.added
  Scenario: Provider migration preserves the destination default model on collision
    Given the selected provider key is "legacy"
    And provider "legacy" has default model "source-model"
    And provider "custom" has default model "destination-model"
    When I confirm provider setup with endpoint "https://new.example/v1"
    Then provider "legacy" should not exist
    And provider "custom" should contain endpoint "https://new.example/v1"
    And provider "custom" should contain default model "destination-model"

  @givn.added
  Scenario: Provider migration is idempotent after the first conversion
    Given the selected provider is already "custom"
    When I rerun provider setup without changing its values
    And confirm provider setup
    Then there should be exactly one "custom" provider entry
    And no arbitrary provider key should be created

  @givn.added
  Scenario: Free-form reasoning survives persistence and request construction
    Given a configured provider with model "plain-model" for the small role
    When I configure reasoning as "  provider-specific-mode  "
    And confirm the setup
    And send a request through the small role
    Then the saved reasoning should be exactly "  provider-specific-mode  "
    And the request should contain reasoning_effort exactly "  provider-specific-mode  "

  @givn.added
  Scenario: Existing unknown reasoning remains active after rerunning setup
    Given a configured provider has small reasoning "unknown-provider-mode"
    When I rerun setup without changing the small reasoning value
    And confirm the setup
    Then the saved small reasoning should remain exactly "unknown-provider-mode"
    And a small-role request should contain reasoning_effort exactly "unknown-provider-mode"

  @givn.added
  Scenario: Whitespace-only custom reasoning is rejected
    Given an existing config is recorded byte-for-byte
    When I enter custom reasoning "   "
    Then setup should report that the reasoning value is invalid
    And final confirmation should be blocked
    And the config file should be byte-for-byte unchanged

  @givn.added
  Scenario: Catalog reasoning choices still permit a custom non-empty value
    Given a catalog model supports efforts "low", "medium", and "high"
    When I select that model and open its reasoning question
    Then the supported efforts should be shown
    And a custom reasoning entry should be available
    When I enter custom reasoning "x-high"
    Then the selected reasoning should be exactly "x-high"

  @givn.added
  Scenario: Declining shell setup performs no target inspection or write
    Given no shell integration choice has been accepted
    And shell target files do not exist
    When I decline both shell integration questions
    Then no shell target file should be inspected or created
    And no configuration field should change

  @givn.added
  Scenario: Shell removal preserves bytes outside the managed block
    Given Bash contains a valid Watn completion block surrounded by user content
    And the original Bash bytes are recorded
    When I deselect Bash completion
    Then only the Watn completion block should be removed
    And all user-owned bytes should remain in their original order

  @givn.added
  Scenario: Missing model roles trigger implicit setup even with a usable provider
    Given a usable provider credential is configured
    And one required model role is missing
    When I start an interactive request
    Then the setup coordinator should open
    And the original request should not be sent before setup completes

  @givn.added
  Scenario: Focused model setup preserves provider-owned and unrelated fields
    Given a configured provider has endpoint, credential, catalog endpoint, default model, pricing, LiteLLM settings, and an unrelated provider
    When I confirm new model roles and reasoning through `watn models`
    Then provider identity, endpoint, credential, catalog endpoint, default model, pricing, LiteLLM settings, and the unrelated provider should remain unchanged
    And only model roles and reasoning should change

  @givn.added
  Scenario: A failed final config write prevents shell operations
    Given a coordinated setup draft is complete
    And the final configuration write cannot complete
    When I confirm the setup review with shell integrations selected
    Then setup should report a configuration error
    And no shell operation should begin
    And the previous configuration should remain unchanged
