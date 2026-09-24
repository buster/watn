@givn.delta @visible-request-amount

Feature: Billed amount of a request

  @givn.added
  Scenario: A provider's unknown model falls back to the requested model's recorded price
    Given an installed Bash shortcut and a provider candidate "find . -type f"
    And the configured model is "~deepseek/deepseek-flash-latest" reported by the provider as "deepseek/deepseek-v4.1-flash"
    And a recorded price of 0.04 input and 1.00 output per million tokens for the model "~deepseek/deepseek-flash-latest"
    And the provider response reports 1200 prompt and 90 completion tokens
    When I invoke Ctrl-W with current input "find all files"
    Then the review surface should show a billed amount of "0.01" cents with the model name

  @givn.modified @e2e
  Scenario: The review surface shows the billed amount of the request that produced it
    Given  a configured model "gpt-4o-mini" with a recorded price of 150.00 input and 600.00 output per million tokens
    And  a configured provider candidate "find . -type f"
    When  I ask interactively for "find all files"
    Then  the review surface should show a billed amount of "1" cents with the model name

  @givn.modified
  Scenario: A rejected candidate's replacement carries its own billed amount
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
    And  a recorded price of 2.50 input and 10.00 output per million tokens for the model "review-model-2"
    And  the provider response reports 1200 prompt and 90 completion tokens
    And  a regeneration for the model "review-model-2" reports 1500 prompt and 200 completion tokens
    When  I invoke Ctrl-W with current input "find all files"
    And  I reject the candidate and choose the normal tier
    Then  the review surface should show a billed amount of "0.6" cents with the model name

  @givn.modified
  Scenario: The billed model's amount is shown under the requested model name
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  the configured model is "openai/gpt-4o" reported by the provider as "gpt-4o-2024-08-06"
    And  a recorded price of 2.50 input and 10.00 output per million tokens for the model "gpt-4o-2024-08-06"
    And  the provider response reports 1000 prompt and 100 completion tokens
    When  I invoke Ctrl-W with current input "find all files"
    Then  the review frame should name the model "gpt-4o"
    And  the review surface should show a billed amount of "0.4" cents with the model name

  @givn.modified
  Scenario: A narrow terminal keeps the billed amount at the model name
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  the configured model is "~deepseek/deepseek-v4-flash-latest"
    And  a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
    And  the provider response reports 1200 prompt and 90 completion tokens
    And  the review terminal is 40 columns wide
    When  I invoke Ctrl-W with current input "find all files"
    Then  the review surface should shorten the model name to fit
    And  the review surface should show a billed amount of "0.02" cents with the model name

  @givn.modified
  Scenario: The detailed view keeps the billed amount at the model name
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
    And  the provider response reports 1200 prompt and 90 completion tokens
    When  I invoke Ctrl-W with current input "find all files"
    And  I switch to the detailed view
    Then  the review surface should show a billed amount of "0.02" cents with the model name

  @givn.modified
  Scenario: A failed regeneration keeps the preserved candidate's billed amount
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
    And  the provider response reports 1200 prompt and 90 completion tokens
    And  regeneration fails
    When  I invoke Ctrl-W with current input "find all files"
    And  I reject the candidate and choose the normal tier
    Then  the preserved candidate should still show its billed amount of "0.02" cents
    And  the review surface should report the generation failure

  @givn.modified
  Scenario: An unusable response still shows the amount it was billed
    Given  an installed Bash shortcut and a provider candidate "find . -type f"
    And  a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
    And  the provider returns a structured review response whose values contain unescaped quotation marks
    And  the provider response reports 1200 prompt and 90 completion tokens
    When  I invoke Ctrl-W with current input "find all files"
    Then  the review surface should show purpose-unavailable
    And  the review surface should show a billed amount of "0.02" cents with the model name

  @givn.modified @e2e
  Scenario: The explanation card shows the billed amount of its explanation request
    Given  a configured provider whose explanation covers the command:
      """
      git log --oneline | head -5
      """
    And  a configured model "gpt-4o-mini" with a recorded price of 150.00 input and 600.00 output per million tokens
    When  I run `watn explain` with that command as one argument in a terminal
    Then  the explanation card should show each model-written stage purpose
    And  the explanation card should show a billed amount of "1" cents with the model name
    When  I close the explanation card
    Then  the command-output channel should contain no command
