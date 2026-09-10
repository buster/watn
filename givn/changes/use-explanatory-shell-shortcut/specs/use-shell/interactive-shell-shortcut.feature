@givn.delta @interactive-shell-shortcut
Feature: Explanatory interactive shell shortcut

  @givn.added @e2e @wip
  Scenario: Developer accepts an explained candidate from Ctrl-W
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    And the candidate has a visible command flow with model-written stage purposes
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I accept the selected candidate in the review surface
    Then the Bash command line should contain the accepted candidate
    And the Bash history should contain the original request comment "# inspect recent log changes"
    And the accepted candidate should not have executed

  @givn.added @e2e @wip
  Scenario: Developer cancels a review without changing the shell buffer
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the current Bash command line is "show disk usage"
    When I invoke Ctrl-W with the current input
    And I cancel the review surface
    Then the Bash command line should remain "show disk usage"
    And the Bash history should not contain a new request comment for "show disk usage"
    And no candidate should be released to the shell

  @givn.added
  Scenario: The review surface explains a complex command flow
    Given an installed Bash shortcut and a provider candidate "git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'"
    And the provider returns this structured review response:
      """
      {"review_version":1,"command":"git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'","stages":[{"stage_text":"git log --format='%H' --since='7 days ago'","purpose":"Collect commit identifiers from the recent history."},{"stage_text":"xargs -n1","purpose":"Pass each collected identifier to the per-commit command."},{"stage_text":"git show --stat --oneline","purpose":"Inspect each collected commit with a compact change summary."},{"stage_text":"printf 'done'","purpose":"Report that the success branch completed."}],"purpose_status":"ready"}
      """
    When I invoke Ctrl-W with current input "inspect recent commits"
    Then the review surface should show the git log stage
    And the review surface should show the xargs stage
    And the review surface should show the git show stage
    And the review surface should show the success branch
    And the review surface should show each model-written stage purpose

  @givn.added
  Scenario: A complete candidate is buffered before review
    Given an installed Bash shortcut and a provider that streams a candidate in multiple events
    And the provider has not sent [DONE]
    When I invoke Ctrl-W with current input "inspect recent log changes"
    Then the review surface should not open before [DONE]
    And the existing progress line should remain the first feedback
    And no candidate text should be released before final acceptance
    When the provider sends [DONE]
    Then the review surface should open with the complete candidate

  @givn.added
  Scenario: Direct command editing preserves the original intent
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I open the separate command editor
    And I edit the selected candidate without changing the original intent
    And I press Enter in the command editor
    Then the review surface should refresh the command flow for the edited candidate
    And the original intent should remain visible in the review context
    And final acceptance should still be required

  @givn.added
  Scenario: Escape discards a direct command edit
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I open the separate command editor
    And I change the selected candidate
    And I press Escape in the command editor
    Then the unedited candidate should remain selected
    And the review surface should remain open
    And final acceptance should still be required

  @givn.added
  Scenario: Edited candidate purpose refresh failure remains reviewable
    Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I edit the selected candidate and purpose refresh fails
    Then the edited candidate should remain visible
    And the review surface should show purpose-unavailable or unsupported flow state
    And the original intent should remain visible
    And final acceptance should still be required

  @givn.added
  Scenario: The compact review surface cycles three focus regions
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    Then the initial focus should be Actions with final acceptance selected
    When I press Tab
    Then focus should move to Flow
    When I press Tab
    Then focus should move to Candidates
    When I press Tab
    Then focus should move to Actions
    When I press an arrow key within the Flow region
    Then the selected command-flow stage should change
    When I press Shift-Tab within the Actions region
    Then focus should move to Candidates

  @givn.added
  Scenario: Review actions use Enter and Escape
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I open the review surface
    When I press Enter on the selected action
    Then the selected action should activate
    When I press Escape in the review surface
    Then the review should be cancelled

  @givn.added
  Scenario: Stage purposes can load after a structured candidate appears
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the structured review response supports delayed stage purposes
    And stage purposes are delayed
    When I invoke Ctrl-W with the current input
    Then the review surface should appear with stage purposes loading
    When stage purposes become available
    Then the review surface should update with the model-written stage purposes

  @givn.added
  Scenario: A command-only response shows purpose-unavailable
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the provider returns command text without a structured review response
    When I invoke Ctrl-W with the current input
    Then the review surface should show purpose-unavailable immediately
    And the review surface should not claim that purposes are loading
    And the candidate should remain reviewable

  @givn.added
  Scenario: Unsupported command flow remains reviewable
    Given an installed Bash shortcut and a provider candidate containing unsupported shell syntax
    When I invoke Ctrl-W with the current input
    Then the raw candidate should remain visible
    And unsupported command-flow portions should be marked
    And the review surface should still offer final acceptance and cancellation

  @givn.added
  Scenario: Enhanced renderer failure falls back to the inline review surface
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the selected enhanced presentation adapter cannot open
    When I invoke Ctrl-W with the current input
    Then the portable inline review surface should open
    And the current candidate should remain available

  @givn.added
  Scenario: Portable review-surface failure releases no candidate
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    And the portable inline review surface cannot open
    When I invoke Ctrl-W with the current input
    Then the original Bash command line should remain unchanged
    And no candidate should be released to the shell
    And no new request comment should be recorded in Bash history

  @givn.added
  Scenario: Provider failure preserves a selected candidate during review
    Given an installed Bash shortcut and a selected candidate for "show disk usage"
    When a purpose refresh or replacement generation fails
    Then the selected candidate should remain available
    And the review surface should show purpose-unavailable or generation failure
    And final acceptance should still be required

  @givn.added
  Scenario: Disabled review preserves direct Ctrl-W replacement
    Given an installed Bash shortcut with the explanatory review surface disabled
    And a provider candidate "df -h"
    When I invoke Ctrl-W with current input "show available diskspace"
    Then the Bash command line should contain "df -h"
    And no command-flow review should open
    And the existing Ctrl-W history and no-evaluation behavior should be unchanged

  @givn.added
  Scenario: Disabled review preserves direct positional output
    Given the explanatory review surface is disabled
    And a configured provider candidate "find . -type f"
    When I ask positionally for "find all files"
    Then the existing command-output channel should contain only "find . -type f"
    When I submit "find all files" through interactive stdin
    Then the existing command-output channel should contain only "find . -type f"
    And no review surface should open

  @givn.added
  Scenario: Disabled review preserves -x confirmation
    Given the explanatory review surface is disabled
    And a configured provider candidate "printf 'reviewed'"
    When I run `watn -x "print reviewed"` in an eligible terminal
    Then the existing "Execute now?" confirmation should be shown
    And execution should require the existing confirmation response

  @givn.added
  Scenario: Non-review -x preserves confirmation
    Given an `-x` request is redirected or otherwise not review-eligible
    And a configured provider candidate "printf 'reviewed'"
    When I run the request
    Then the existing "Execute now?" confirmation should be shown
    And review acceptance should not authorize execution

  @givn.added @e2e @wip
  Scenario: Developer accepts a candidate from an interactive terminal request
    Given a configured provider candidate "find . -type f"
    When I ask interactively for "find all files"
    And I accept the candidate in the review surface
    Then normal command output should contain only "find . -type f"
    And the review surface should not appear in normal command output

  @givn.added @e2e @wip
  Scenario: Developer accepts an eligible -x candidate and it executes once
    Given a configured provider candidate "printf 'reviewed'"
    When I run `watn -x "print reviewed"` in an eligible terminal
    And I accept the candidate in the review surface
    Then "reviewed" should be printed once
    And no second execution confirmation should be shown

  @givn.added
  Scenario: Rephrasing starts a new candidate cycle
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I rephrase the intent as "show disk usage for mounted filesystems"
    Then a new candidate should be generated for the current intent
    And the visible intent should be "show disk usage for mounted filesystems"
    And the prior intent should remain only in current-review history
    And the prior candidate should not be accepted by the new cycle

  @givn.added
  Scenario: Regeneration replaces the current candidate by default
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I regenerate the candidate
    Then a replacement candidate should be shown for the same intent
    And the prior candidate should not be retained unless comparison was requested
    And final acceptance should still be required

  @givn.added
  Scenario: Higher-tier review generates a candidate at the next configured tier
    Given an installed Bash shortcut and a candidate generated at the small tier
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I request a higher tier
    Then a new candidate should be generated at the next configured tier
    And its tier and provider/model context should be visible
    And the current intent should remain unchanged

  @givn.added
  Scenario: Highest-tier review opens explicit provider catalog model selection
    Given an installed Bash shortcut and a candidate generated at the highest configured tier
    And the provider catalog contains "model-a" and "model-b"
    When I invoke Ctrl-W with current input "inspect recent log changes"
    And I request a higher tier
    Then the provider catalog model selection should open
    When I select "model-b"
    Then the next candidate should use "model-b"
    And the selected model should apply only to the next candidate

  @givn.added
  Scenario: Rejected candidate returns to the current intent
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I reject the selected candidate
    Then no candidate should be released
    And the current intent should remain "show disk usage"
    And the review should offer regeneration or rephrasing

  @givn.added
  Scenario: Retained candidates can be compared and one selected
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I retain the current candidate for comparison
    And I regenerate the candidate
    Then both candidates should be available in the current review only
    And each candidate should show its tier and provider/model context
    When I select the retained candidate
    Then it should become the selected candidate for final acceptance
    And only the selected candidate should be eligible for acceptance

  @givn.added
  Scenario: Interrupting an in-progress review operation preserves the selected candidate
    Given an installed Bash shortcut and a selected candidate for "show disk usage"
    When I start a regeneration, purpose refresh, or provider catalog model selection
    And I interrupt that in-progress operation
    Then only that operation should be cancelled
    And the selected candidate should remain available
    And the review state should remain open

  @givn.added
  Scenario: Shell repaint remains owned by the line editor after review
    Given an installed Bash shortcut and a provider candidate for "show disk usage"
    When I invoke Ctrl-W with current input "show disk usage"
    And I accept the selected candidate
    Then the review surface should disappear immediately
    And Watn should leave cursor and inline rows restored
    And the Bash line editor should repaint the prompt with the accepted candidate

  @givn.added
  Scenario: A narrow terminal keeps the inline review bounded and readable
    Given an installed Bash shortcut and a provider candidate with more stages than fit in the terminal
    When I invoke Ctrl-W with current input "inspect recent log changes"
    Then the review surface should remain a bounded inline panel
    And it should show a compact command-flow overview and one readable selected stage
    And arrow navigation should reach every command-flow stage
