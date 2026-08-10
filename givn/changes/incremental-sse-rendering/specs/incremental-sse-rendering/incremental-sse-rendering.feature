# User Interaction Inventory:
# - invoke watn for a question and observe a generated command while the provider is streaming
# - invoke watn with verbose output and observe command and reasoning on their separate channels
# - observe a provider failure after command content has become visible
# - invoke watn with execute enabled from a raw terminal and confirm the generated command
# - invoke watn with execute enabled from piped input and confirm the generated command

@givn.delta @incremental-sse-rendering
Feature: Incremental provider output

  @givn.added @e2e
  Scenario: Command text appears before a delayed stream completes
    Given a streaming provider flushes content "printf first" and delays content " && printf second" while keeping the connection open
    When I start the delayed streaming command `watn "build a two-part command"` in a terminal
    Then the progress indicator is visible before the first streamed content
    And the first streamed content "printf first" is visible before the provider releases the delayed event
    And the terminal shows spinner cleanup after the first streamed content
    When I release the delayed event and wait for watn to exit
    Then the terminal generated command line "printf first && printf second" appears exactly once
    And the exit status should be 0

  @givn.added @e2e
  Scenario: Verbose streaming keeps reasoning on stderr and command text on stdout
    Given a streaming provider emits reasoning "inspect the files" and content "find . -type" before holding a later completion event
    When I start the verbose streaming command `watn -v "list the files"` with captured stdout and stderr
    Then stdout has streamed fragment "find . -type" before the provider releases completion
    And stderr does not yet contain "reasoning: inspect the files"
    When I release completion and wait for watn to exit
    Then stdout generated command line "find . -type f" appears exactly once
    And stdout should not contain "inspect the files"
    And stderr should contain "reasoning:"
    And stderr should contain "inspect the files"
    And the exit status should be 0

  @givn.added
  Scenario: A usage-only final event supplies cost and throughput metadata
    Given the request asks for model "requested-model"
    And a streaming provider emits content "printf usage" and a choices-empty usage event with response model "response-model", 10 prompt tokens, and 20 completion tokens
    And pricing is configured only for "response-model" at 2.50 input and 10.00 output per million tokens
    When I run `watn "show usage"`
    Then stdout should contain "printf usage"
    And the final metadata names exactly "response-model"
    And stderr should not contain final metadata for "requested-model"
    And stderr should contain a non-zero cost for "response-model"
    And stderr should contain a positive throughput value
    And the exit status should be 0

  @givn.added
  Scenario: A DONE event completes a stream successfully
    Given a streaming provider emits content "printf done", sends `[DONE]`, and holds the connection open until released
    When I start the streaming command `watn "finish the command"`
    Then watn exits successfully before the provider connection is released
    And the generated command line "printf done" appears exactly once
    When I release the provider connection

  @givn.added
  Scenario: Partial network reads are reassembled into complete events
    Given a streaming provider sends the first content event one byte at a time with content "printf partial" and holds the next event
    When I start the streaming command `watn "handle partial reads"`
    Then the streamed fragment "printf partial" is visible before the provider releases the next event
    When I release the next event and wait for watn to exit
    Then the generated command line "printf partial" appears exactly once
    And the exit status should be 0

  @givn.added
  Scenario: Malformed nonessential events do not discard valid content
    Given a streaming provider sends a malformed event, flushes valid content "printf valid", and holds `[DONE]`
    When I start the streaming command `watn "ignore an invalid event"`
    Then the valid streamed fragment "printf valid" is visible before the provider releases `[DONE]`
    When I release `[DONE]` and wait for watn to exit
    Then the generated command line "printf valid" appears exactly once
    And the exit status should be 0

  @givn.added @e2e
  Scenario: A mid-stream failure preserves visible content and exits unsuccessfully
    Given a streaming provider flushes content "printf partial" and then resets the connection before `[DONE]`
    When I start the failing streaming command `watn "survive a provider failure"` in a terminal
    Then the terminal output contains "printf partial"
    And the terminal output contains "network error"
    And the terminal output shows spinner clear-line evidence after "printf partial"
    And the terminal output does not contain successful model metadata
    And the terminal output does not contain "Execute now? [Y/n]"
    And the exit status should be 3

  @givn.added
  Scenario: EOF without DONE is a truncated stream
    Given a streaming provider flushes valid content "printf truncated" and closes cleanly without sending `[DONE]`
    When I run `watn "detect a truncated stream"`
    Then stdout should contain "printf truncated"
    And stderr should contain "network error"
    And stderr should not contain successful model metadata
    And stderr should not contain "Execute now? [Y/n]"
    And the exit status should be 3

  @givn.added
  Scenario: Output failure preserves the visible prefix and skips completion actions
    Given the streaming output sink flushes prefix "printf prefix" and fails on the next write
    When I render the streaming response through the controlled output sink
    Then the visible command prefix is preserved as "printf prefix"
    And the existing I/O error is reported
    And final success metadata is omitted
    And execution is not prompted
    And the exit status should be 1

  @givn.added @e2e @wip
  Scenario: Raw terminal confirmation happens after the complete command arrives
    Given a streaming provider emits content "printf raw-confirmed"
    When I start the executable streaming command `watn -x "run the command"` in a terminal
    Then the generated command line "printf raw-confirmed" is visible before confirmation
    And the terminal output does not contain the execution output line "raw-confirmed" before confirmation
    When I confirm execution with the raw terminal Enter key
    Then the generated command line "printf raw-confirmed" appears exactly once
    And the execution output line "raw-confirmed" appears exactly once
    And the exit status should be 0

  @givn.added @e2e @wip
  Scenario: Piped confirmation remains available after streamed output
    Given a streaming provider emits content "printf piped-confirmed"
    When I run `watn -x "run the command"` with piped confirmation "y"
    Then the generated command line "printf piped-confirmed" appears exactly once on stdout
    And the execution output line "piped-confirmed" appears exactly once on stdout
    And the exit status should be 0
