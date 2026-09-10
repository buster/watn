Feature: Interactive shell shortcut for watn

  @e2e
  Scenario: Generated Bash, Zsh, and Fish configurations pass shell syntax checks
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the generated Bash configuration should pass a Bash syntax check
    And  the generated Zsh configuration should pass a Zsh syntax check
    And  the generated Fish configuration should pass a Fish syntax check

  Scenario: Enter accepts the default decline for shortcut setup
    Given  Bash, Zsh, and Fish configuration files with existing user content
    And  a snapshot of every shell configuration file
    When  I press Enter to accept the default decline on the optional shortcut question
    Then  every shell configuration file should match its snapshot byte-for-byte

  Scenario: Selecting no shells leaves shell configuration unchanged
    Given  Bash, Zsh, and Fish configuration files with existing user content
    And  a snapshot of every shell configuration file
    When  I answer `y` to the optional shortcut question
    And  I select no shells in the shortcut multi-select
    Then  every shell configuration file should match its snapshot byte-for-byte

  Scenario: The shell basename alone controls shortcut preselection
    Given  `SHELL` is "/usr/local/bin/bash"
    And  Zsh and Fish target files already exist
    When  the shell shortcut choices are shown
    Then  Bash should be preselected
    And  Zsh and Fish should remain available and unselected
    When  I select Zsh and Fish as well
    Then  Bash, Zsh, and Fish should all be selected

  Scenario: Multiple selected shells are installed independently
    Given  Bash, Zsh, and Fish configuration paths in an isolated home
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the Bash configuration should contain the Bash widget and Ctrl-W binding
    And  the Zsh configuration should contain the ZLE widget and Ctrl-W binding
    And  the Fish configuration should contain the Fish widget and Ctrl-W binding
    And  setup should report a success for every selected shell
    And  each selected shell should have its own reload instruction

  Scenario: A partial multi-shell failure reports every result without rollback
    Given  writable Bash and Fish targets and a Zsh target that cannot be written
    And  the Bash and Fish targets have existing user content
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  the Bash configuration should contain one watn shell shortcut block
    And  the Fish configuration should contain one watn shell shortcut block
    And  the Bash and Fish user content should remain unchanged
    And  the Zsh configuration should remain unchanged
    And  setup should report success for Bash and Fish
    And  setup should report the Zsh target path and write failure reason
    And  setup should report an aggregate shell installation failure

  Scenario: Missing parent directories are created only for selected shells
    Given  missing Bash and Fish configuration parent directories
    When  I install the shell shortcut for Fish
    Then  the Fish configuration parent directory should exist
    And  the Bash configuration parent directory should remain absent

  Scenario: Installing again replaces the generated block without disturbing user content
    Given  a Bash configuration containing unrelated user content and one watn shell shortcut block
    When  I install the Bash shell shortcut again
    Then  the Bash configuration should contain exactly one watn shell shortcut block
    And  the unrelated user content should remain unchanged

  Scenario: A shell configuration failure reports the exact target and reason
    Given  a Bash shortcut target that is a directory and cannot be written
    And  a snapshot of the Bash target failure state
    When  I install the Bash shell shortcut
    Then  setup should report that the Bash target could not be written
    And  the error should identify the write failure reason
    And  the Bash target should remain a directory

  Scenario: A symlinked shell target is updated without replacing the link
    Given  a Bash shortcut target that is a symbolic link to a regular file
    When  I install the Bash shell shortcut
    Then  the Bash shortcut symlink should remain intact
    And  the resolved Bash shortcut target should contain the Bash widget

  Scenario: Invalid marker layouts fail before any target write
    Given  isolated Bash targets with these malformed marker layouts:
      | layout |
      | two complete watn shell shortcut blocks |
      | two opening markers and one closing marker |
      | one opening marker and two closing markers |
      | an opening marker without a closing marker |
      | a closing marker without an opening marker |
      | a closing marker before an opening marker |
    When  I install the Bash shell shortcut for every malformed layout
    Then  setup should report malformed watn shell shortcut markers
    And  every malformed Bash target should match its snapshot byte-for-byte

  Scenario: Generated shell blocks use the installed watn command and preserve shell syntax
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  no generated block should contain a repository-local watn path
    And  every generated widget should invoke `command watn -- "$question"`
    And  the Bash block should use the current Readline line and cursor
    And  the Zsh block should use the current buffer and cursor
    And  the Fish block should replace and repaint the current command line
    And  every generated block should bind Ctrl-W

  Scenario: A successful widget inserts one normalized command and moves the cursor to its end
    Given  an installed Bash shortcut and a fake watn that returns "printf 'ready'\n\n"
    When  I run the Bash widget with current input "show status"
    Then  the current command line should be exactly "printf 'ready'"
    And  the cursor should be at the end of the current command line

  Scenario: Embedded multiline output remains buffer text without evaluation
    Given  an installed Bash shortcut and a fake watn that returns "printf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    When  I run the Bash widget with current input "show two lines"
    Then  the current command line should be exactly "printf 'first line'\ntouch /tmp/watn-shortcut-should-not-run"
    And  the embedded line break should remain in the command line buffer
    And  the cursor should be at the end of the current command line
    And  the replacement text should not have executed

  Scenario: Empty input does not invoke watn or change the command line
    Given  an installed Bash shortcut and a fake watn that records invocations
    When  I run the Bash widget with empty input
    Then  the fake watn should not have been invoked
    And  the current command line should remain empty

  Scenario: Failed or empty output preserves the original command line
    Given  an installed Bash shortcut and a fake watn that fails
    When  I run the Bash widget with current input "list files"
    Then  the current command line should remain "list files"
    When  the fake watn returns empty output
    And  I run the Bash widget with current input "show files"
    Then  the current command line should remain "show files"

  Scenario: Non-zero watn status discards partial stdout
    Given  an installed Bash shortcut and a fake watn that writes "partial" to stdout and exits non-zero
    When  I run the Bash widget with current input "show partial result"
    Then  the current command line should remain "show partial result"
    And  the partial stdout should not be inserted

  Scenario: The complete command line is passed as one quoted question
    Given  an installed Bash shortcut and a fake watn that records its question
    When  I run the Bash widget with current input "find files; echo unsafe *"
    Then  the fake watn should receive exactly one question "find files; echo unsafe *"
    And  the wildcard should not be expanded before watn receives the question

  Scenario: Leading-option and reserved-token questions remain one argument
    Given  an installed Bash shortcut and a fake watn that records each question
    When  I run the Bash widget with current input "--help"
    Then  the fake watn should receive exactly one question "--help"
    When  I run the Bash widget with current input "completions find files"
    Then  the fake watn should have received exactly two questions "--help" and "completions find files"

  Scenario: Setup reports the exact reload instruction for every modified shell
    Given  isolated Bash, Zsh, and Fish shortcut targets
    When  I install the shell shortcut for Bash, Zsh, and Fish
    Then  setup should report "source ~/.bashrc" for Bash
    And  setup should report "source ~/.zshrc" for Zsh
    And  setup should report "source ~/.config/fish/config.fish" for Fish

  Scenario: The optional setup result includes only explicitly selected shells
    Given  a shortcut selection with Bash enabled and Zsh and Fish disabled
    When  the setup result confirms the shortcut selection
    Then  the selected shortcut shells should contain only Bash
  Scenario: A successful generation records the request as a history comment
    Given  an installed Bash shortcut and a fake watn that returns "printf 'ready'"
    When  I run the Bash widget with current input "show status"
    Then  the current command line should be exactly "printf 'ready'"
    And  the shell history should contain the recorded request comment "# show status"
    And  the cursor should be at the end of the current command line

  Scenario: Only the generated command executes when the buffer is committed
    Given  an installed Bash shortcut and a fake watn that returns "touch /tmp/watn-shortcut-executed"
    When  I run the Bash widget with current input "run the task; touch /tmp/watn-shortcut-comment-should-not-run"
    And  I execute the resulting Bash buffer
    Then  the file "/tmp/watn-shortcut-executed" should exist
    And  the file "/tmp/watn-shortcut-comment-should-not-run" should not exist

  Scenario: Requests with metacharacters and embedded newlines remain one comment line
    Given  an installed Bash shortcut and a fake watn that returns "ls"
    When  I run the Bash widget with current input containing "show files; echo unsafe *\nsecond line"
    Then  the current command line should be exactly "ls"
    And  the recorded request history entry should be exactly "# show files; echo unsafe * second line"

  Scenario: Zsh and Fish widgets record the request in the shell history
    Given  an installed Zsh and Fish shortcut
    Then  the Zsh configuration should record the request in the shell history
    And  the Fish configuration should record the request in the shell history
    And  the generated Zsh configuration should pass a Zsh syntax check
    And  the generated Fish configuration should pass a Fish syntax check

  @e2e
  Scenario: The generated Bash widget keeps the request visible and does not evaluate the command
    Given  an installed Bash shortcut and a fake watn that returns "printf 'hello world'"
    When  I run the generated Bash widget through Bash with current input "find all images"
    Then  the Bash process should record the request "# find all images" in the shell history
    And  the Bash process should not execute the replacement text
  @e2e
  Scenario: Fish replaces the buffer with the generated command after Ctrl-W
    Given  an installed Fish shortcut and a fake watn that returns "df -h"
    When  I press Ctrl-W in the Fish shortcut with current input "show available diskspace"
    Then  the Fish command line should be exactly "df -h"
  @e2e
  Scenario: Developer accepts an explained candidate from Ctrl-W
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And  the candidate has a visible command flow with model-written stage purposes
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I accept the selected candidate in the review surface
    Then  the Bash command line should contain the accepted candidate
    And  the Bash history should contain the original request comment "# inspect recent log changes"
    And  the accepted candidate should not have executed

  @e2e
  Scenario: Developer cancels a review without changing the shell buffer
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the current Bash command line is "show disk usage"
    When  I invoke Ctrl-W with the current input
    And  I cancel the review surface
    Then  the Bash command line should remain "show disk usage"
    And  the Bash history should not contain a new request comment for "show disk usage"
    And  no candidate should be released to the shell

  Scenario: The review surface explains a complex command flow
    Given  an installed Bash shortcut and a provider candidate "git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'"
    And  the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'","stages":[{"stage_text":"git log --format='%H' --since='7 days ago'","purpose":"Collect commit identifiers from the recent history."},{"stage_text":"xargs -n1","purpose":"Pass each collected identifier to the per-commit command."},{"stage_text":"git show --stat --oneline","purpose":"Inspect each collected commit with a compact change summary."},{"stage_text":"printf 'done'","purpose":"Report that the success branch completed."}],"purpose_status":"ready"}
      """
    When  I invoke Ctrl-W with current input "inspect recent commits"
    Then  the review surface should show the git log stage
    And  the review surface should show the xargs stage
    And  the review surface should show the git show stage
    And  the review surface should show the success branch
    And  the review surface should show each model-written stage purpose

  Scenario: A complete candidate is buffered before review
    Given  an installed Bash shortcut and a provider that streams a candidate in multiple events
    And  the provider has not sent [DONE]
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    Then  the review surface should not open before [DONE]
    And  the existing progress line should remain the first feedback
    And  no candidate text should be released before final acceptance
    When  the provider sends [DONE]
    Then  the review surface should open with the complete candidate

  Scenario: Direct command editing preserves the original intent
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I open the separate command editor
    And  I edit the selected candidate without changing the original intent
    And  I press Enter in the command editor
    Then  the review surface should refresh the command flow for the edited candidate
    And  the original intent should remain visible in the review context
    And  final acceptance should still be required

  Scenario: Escape discards a direct command edit
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I open the separate command editor
    And  I change the selected candidate
    And  I press Escape in the command editor
    Then  the unedited candidate should remain selected
    And  the review surface should remain open
    And  final acceptance should still be required

  Scenario: Edited candidate purpose refresh failure remains reviewable
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I edit the selected candidate and purpose refresh fails
    Then  the edited candidate should remain visible
    And  the review surface should show purpose-unavailable or unsupported flow state
    And  the original intent should remain visible
    And  final acceptance should still be required

  Scenario: The compact review surface cycles three focus regions
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I open the review surface
    Then  the initial focus should be Actions with final acceptance selected
    When  I press Tab
    Then  focus should move to Flow
    When  I press Tab
    Then  focus should move to Candidates
    When  I press Tab
    Then  focus should move to Actions
    When  I press an arrow key within the Flow region
    Then  the selected command-flow stage should change
    When  I press Shift-Tab within the Actions region
    Then  focus should move to Candidates

  Scenario: Review actions use Enter and Escape
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I open the review surface
    When  I press Enter on the selected action
    Then  the selected action should activate
    When  I press Escape in the review surface
    Then  the review should be cancelled

  Scenario: Stage purposes can load after a structured candidate appears
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the structured review response supports delayed stage purposes
    And  stage purposes are delayed
    When  I invoke Ctrl-W with the current input
    Then  the review surface should appear with stage purposes loading
    When  stage purposes become available
    Then  the review surface should update with the model-written stage purposes

  Scenario: A command-only response shows purpose-unavailable
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns command text without a structured review response
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show purpose-unavailable immediately
    And  the review surface should not claim that purposes are loading
    And  the candidate should remain reviewable

  Scenario: Unsupported command flow remains reviewable
    Given  an installed Bash shortcut and a provider candidate containing unsupported shell syntax
    When  I invoke Ctrl-W with the current input
    Then  the raw candidate should remain visible
    And  unsupported command-flow portions should be marked
    And  the review surface should still offer final acceptance and cancellation

  Scenario: Enhanced renderer failure falls back to the inline review surface
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the selected enhanced presentation adapter cannot open
    When  I invoke Ctrl-W with the current input
    Then  the portable inline review surface should open
    And  the current candidate should remain available

  Scenario: Portable review-surface failure releases no candidate
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the portable inline review surface cannot open
    When  I invoke Ctrl-W with the current input
    Then  the original Bash command line should remain unchanged
    And  no candidate should be released to the shell
    And  no new request comment should be recorded in Bash history

  Scenario: Provider failure preserves a selected candidate during review
    Given  an installed Bash shortcut and a selected candidate for "show disk usage"
    When  a purpose refresh or replacement generation fails
    Then  the selected candidate should remain available
    And  the review surface should show purpose-unavailable or generation failure
    And  final acceptance should still be required

  Scenario: Disabled review preserves direct Ctrl-W replacement
    Given  an installed Bash shortcut with the explanatory review surface disabled
    And  a provider candidate "df -h"
    When  I invoke Ctrl-W with current input "show available diskspace"
    Then  the Bash command line should contain "df -h"
    And  no command-flow review should open
    And  the existing Ctrl-W history and no-evaluation behavior should be unchanged

  Scenario: Disabled review preserves direct positional output
    Given  the explanatory review surface is disabled
    And  a configured provider candidate "find . -type f"
    When  I ask positionally for "find all files"
    Then  the existing command-output channel should contain only "find . -type f"
    When  I submit "find all files" through interactive stdin
    Then  the existing command-output channel should contain only "find . -type f"
    And  no review surface should open

  Scenario: Disabled review preserves -x confirmation
    Given  the explanatory review surface is disabled
    And  a configured provider candidate "printf 'reviewed'"
    When  I run `watn -x "print reviewed"` in an eligible terminal
    Then  the existing "Execute now?" confirmation should be shown
    And  execution should require the existing confirmation response

  Scenario: Non-review -x preserves confirmation
    Given  an `-x` request is redirected or otherwise not review-eligible
    And  a configured provider candidate "printf 'reviewed'"
    When  I run the request
    Then  the existing "Execute now?" confirmation should be shown
    And  review acceptance should not authorize execution

  @e2e
  Scenario: Developer accepts a candidate from an interactive terminal request
    Given  a configured provider candidate "find . -type f"
    When  I ask interactively for "find all files"
    And  I accept the candidate in the review surface
    Then  normal command output should contain only "find . -type f"
    And  the review surface should not appear in normal command output

  @e2e
  Scenario: Developer accepts an eligible -x candidate and it executes once
    Given  a configured provider candidate "printf 'reviewed'"
    When  I run `watn -x "print reviewed"` in an eligible terminal
    And  I accept the candidate in the review surface
    Then  "reviewed" should be printed once
    And  no second execution confirmation should be shown

  Scenario: Rephrasing starts a new candidate cycle
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I invoke Ctrl-W with current input "show disk usage"
    And  I rephrase the intent as "show disk usage for mounted filesystems"
    Then  a new candidate should be generated for the current intent
    And  the visible intent should be "show disk usage for mounted filesystems"
    And  the prior intent should remain only in current-review history
    And  the prior candidate should not be accepted by the new cycle

  Scenario: Regeneration replaces the current candidate by default
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I invoke Ctrl-W with current input "show disk usage"
    And  I regenerate the candidate
    Then  a replacement candidate should be shown for the same intent
    And  the prior candidate should not be retained unless comparison was requested
    And  final acceptance should still be required

  Scenario: Higher-tier review generates a candidate at the next configured tier
    Given  an installed Bash shortcut and a candidate generated at the small tier
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I request a higher tier
    Then  a new candidate should be generated at the next configured tier
    And  its tier and provider/model context should be visible
    And  the current intent should remain unchanged

  Scenario: Highest-tier review opens explicit provider catalog model selection
    Given  an installed Bash shortcut and a candidate generated at the highest configured tier
    And  the provider catalog contains "model-a" and "model-b"
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    And  I request a higher tier
    Then  the provider catalog model selection should open
    When  I select "model-b"
    Then  the next candidate should use "model-b"
    And  the selected model should apply only to the next candidate

  Scenario: Rejected candidate returns to the current intent
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I invoke Ctrl-W with current input "show disk usage"
    And  I reject the selected candidate
    Then  no candidate should be released
    And  the current intent should remain "show disk usage"
    And  the review should offer regeneration or rephrasing

  Scenario: Retained candidates can be compared and one selected
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I invoke Ctrl-W with current input "show disk usage"
    And  I retain the current candidate for comparison
    And  I regenerate the candidate
    Then  both candidates should be available in the current review only
    And  each candidate should show its tier and provider/model context
    When  I select the retained candidate
    Then  it should become the selected candidate for final acceptance
    And  only the selected candidate should be eligible for acceptance

  Scenario: Interrupting an in-progress review operation preserves the selected candidate
    Given  an installed Bash shortcut and a selected candidate for "show disk usage"
    When  I start a regeneration, purpose refresh, or provider catalog model selection
    And  I interrupt that in-progress operation
    Then  only that operation should be cancelled
    And  the selected candidate should remain available
    And  the review state should remain open

  Scenario: Shell repaint remains owned by the line editor after review
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    When  I invoke Ctrl-W with current input "show disk usage"
    And  I accept the selected candidate
    Then  the review surface should disappear immediately
    And  Watn should leave cursor and inline rows restored
    And  the Bash line editor should repaint the prompt with the accepted candidate

  Scenario: A narrow terminal keeps the inline review bounded and readable
    Given  an installed Bash shortcut and a provider candidate with more stages than fit in the terminal
    When  I invoke Ctrl-W with current input "inspect recent log changes"
    Then  the review surface should remain a bounded inline panel
    And  it should show a compact command-flow overview and one readable selected stage
    And  arrow navigation should reach every command-flow stage
  Scenario: A markdown-fenced structured response is still explained
    Given  an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And  the provider returns a markdown-fenced structured review response:
      """
      Here is the review result:
      ```json
      {"review_version":1,"command":"git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'","stages":[{"stage_text":"git log --format='%H' --since='7 days ago'","purpose":"Collect commit identifiers from the recent history."},{"stage_text":"xargs -n1","purpose":"Pass each collected identifier to the per-commit command."},{"stage_text":"git show --stat --oneline","purpose":"Inspect each collected commit with a compact change summary."},{"stage_text":"printf 'done'","purpose":"Report that the success branch completed."}],"purpose_status":"ready"}
      ```
      """
    When  I invoke Ctrl-W with current input "inspect recent commits"
    Then  the review surface should show the git log stage
    And  the review surface should show the xargs stage
    And  the review surface should show the git show stage
    And  the review surface should show the success branch
    And  the review surface should show each model-written stage purpose

  Scenario: An invalid structured response with a command stays reviewable
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns an invalid structured review response with the command "df -h"
    When  I invoke Ctrl-W with the current input
    Then  the review surface should show the command "df -h"
    And  the review surface should show purpose-unavailable or unsupported flow state
    And  final acceptance should still be required

  Scenario: A provider payload without a usable command releases nothing
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a structured review payload without a complete command
    When  I invoke Ctrl-W with the current input
    Then  the original Bash command line should remain unchanged
    And  no candidate should be released to the shell

  Scenario: A multiline provider payload keeps every rendered value on one row
    Given  an installed Bash shortcut and a provider candidate for "show disk usage"
    And  the provider returns a markdown-fenced structured review response with line breaks
    When  I invoke Ctrl-W with the current input
    Then  every rendered review value should stay on one inline row
    And  the review surface should remain a bounded inline panel
