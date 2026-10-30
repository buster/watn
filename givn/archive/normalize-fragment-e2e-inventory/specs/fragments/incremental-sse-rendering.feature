@givn.delta @incremental-sse-rendering

Feature: Incremental provider output

  @givn.modified
  Scenario: Verbose streaming keeps reasoning on stderr and command text on stdout
    Given  a streaming provider emits reasoning "inspect the files" and content "find . -type" before holding a later completion event
    When  I start the verbose streaming command `watn -v "list the files"` with captured stdout and stderr
    Then  stdout has streamed fragment "find . -type" before the provider releases completion
    And  stderr does not yet contain "reasoning: inspect the files"
    When  I release completion and wait for watn to exit
    Then  stdout generated command line "find . -type f" appears exactly once
    And  stdout should not contain "inspect the files"
    And  stderr should contain "reasoning:"
    And  stderr should contain "inspect the files"
    And  the exit status should be 0

  @givn.modified
  Scenario: A mid-stream failure preserves visible content and exits unsuccessfully
    Given  a streaming provider flushes content "printf partial" and then resets the connection before `[DONE]`
    When  I start the failing streaming command `watn "survive a provider failure"` in a terminal
    Then  the terminal output contains "printf partial"
    And  the terminal output contains "network error"
    And  the terminal output shows spinner clear-line evidence after "printf partial"
    And  the terminal output does not contain successful model metadata
    And  the terminal output does not contain "Execute now? [Y/n]"
    And  the exit status should be 3

  @givn.modified
  Scenario: Piped confirmation remains available after streamed output
    Given  a streaming provider emits content "printf piped-confirmed"
    When  I run `watn -x "run the command"` with piped confirmation "y"
    Then  the generated command line "printf piped-confirmed" appears exactly once on stdout
    And  the execution output line "piped-confirmed" appears exactly once on stdout
    And  the exit status should be 0
