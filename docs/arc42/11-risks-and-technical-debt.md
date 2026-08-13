# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | OpenAI API schema drifts from the implemented subset | Low | Medium | Pin tested provider versions in CI; add schema-version field to config |
| R-002 | Users configure providers with non-standard `/v1/chat/completions` paths | Medium | Low | Document the requirement |
| R-003 | Config file with secrets (API keys) committed to version control | Medium | High | Support env-var-only API key config; warn if config file is world-readable |
| R-004 | Provider-local `/models` endpoint schema drifts | Low | Low | Parse response leniently, reject empty/duplicate identifiers, switch to manual mode, and report unexpected format |
| R-005 | User confirms execution and the command is destructive | Low | High | Tool prints the command before prompting; user sees what they are confirming |
| R-006 | Execution flow UX — command content is streamed to stdout before the prompt and may be mistaken for already-executed work | Medium | Medium | Keep command content on stdout for piping, put the confirmation prompt and metadata on stderr, and require confirmation before execution |
| R-007 | Model returns empty reasoning tokens on thinking tier | Medium | Low | Trim reasoning content before printing; skip print if content is empty/whitespace after trimming |
| R-008 | Template config generated from code may include irrelevant fields as Config grows | Low | Low | Template is a starting point — users delete what they do not need; template is meant to be edited |
| R-009 | PTY-based E2E tests are flaky across platforms/terminal emulators | Medium | Medium | Run PTY tests in CI with a known terminal type (`TERM=dumb` or `xterm-256color`); add generous read timeouts; document that local test failures may require `script`/`unbuffer` wrappers |
| R-010 | Arrow/page escape sequences differ across terminal emulators in the SetupWizard | Low | Medium | Standard sequences (`\x1b[A/B`, `\x1b[5~/6~`); PTY E2E tests pin `TERM=xterm-256color`; ratatui/crossterm parse both classic and application-cursor modes |
| R-011 | A user chooses literal credential storage and the API key remains on disk | Medium | High | Prefer environment-backed references, mask input, apply mode `0600`, and warn on world-readable config files |
| R-012 | Automatic onboarding requires an interactive terminal and a reachable model catalog | Medium | Medium | Detect non-TTY use, print actionable setup guidance, keep explicit `watn provider` and `watn models` commands available, and allow rerunning setup after a catalog failure |
| R-013 | Coordinated setup is cancelled or model selection fails, leaving an unconfirmed draft | Medium | High | Keep the complete draft in memory, write only after final review, preserve the baseline on failure, and make focused provider/model commands repeatable |
| R-014 | Explicit `--provider` or `WATN_PROVIDER` selection does not receive automatic onboarding | Medium | Medium | Preserve the existing error contract deliberately, print the existing actionable error, and document `watn provider` as the explicit setup path |
| R-015 | Successful automatic setup does not resume the original request | Medium | Low | Exit clearly after model selection, assert no chat request was sent, and document that the user reruns the original question |
| R-016 | Canonical provider migration can collide with an existing `custom` entry | Medium | High | Define source-key removal, destination default-model precedence, saved-credential authority, unrelated-provider preservation, and idempotent reruns |
| R-017 | A confirmed config snapshot may fail during serialization or replacement | Low | High | Write a same-directory temporary file, flush and permission it, rename only after preparation succeeds, and prevent shell operations on failure |
| R-018 | The ephemeral E2E transport override could leak into persistence/readiness or cover only one request path | Medium | High | Apply it only at HTTP construction under the debug-plus-feature guard, never consult it during readiness, assert the exact persisted endpoint, and cover every touched `/models` and `/chat/completions` path |
| R-019 | Structured columns and multiple regions can become cramped or unreadable on small terminals | Medium | Medium | Use Ratatui layout constraints, truncation-safe cells, wrapped paragraphs, and PTY coverage at the supported test size; keep keyboard flow independent of visual width |
| R-020 | Debounced worker results can race with user input or outlive the SetupWizard | Medium | Medium | Keep the query visible, choose local filtering for complete catalogs, increment a generation for every remote query, check it before and after the debounce and before apply, ignore Enter while pending, and join every retained worker before exit |
| R-021 | A shared coordinator can make a complete draft and focused save boundary ambiguous | Medium | Medium | Use one final snapshot for coordinated setup, domain-owned writes for focused commands, and explicit review/cancellation scenarios |
| R-022 | Migrating Tab and Escape changes can surprise users of the existing model pages | Medium | Medium | Keep Shift-Tab as the explicit back-page key, make Ctrl-R reasoning focus visible, migrate permanent scenarios, and retain command-specific entry points |
| R-023 | `watn models` and `watn provider` can start the shared wizard with stale or incomplete page state | Medium | Medium | Seed endpoint, credential storage, current tier selections, and model-specific reasoning from the loaded config; define and test each entry point's initial/final page range |
| R-024 | A release build with `test-support` could accidentally retain the endpoint override | Medium | High | Guard the lookup with `cfg(all(feature = "test-support", debug_assertions))`; inspect the release-profile artifact and keep the configured-endpoint source invariant explicit |
| R-025 | A stale or overwritten debug executable could make transport tests execute the wrong binary | Medium | High | Build the two debug variants sequentially through Cargo's shared target cache, copy each to a unique temporary path, pass only those absolute paths to the harness, and fail before scenarios when a path is missing |
| R-026 | Broad mocks could report a successful request from the wrong endpoint or credential | Medium | High | Use separate local twin servers and mocks matching exact method/path/Authorization; assert expected counts, competing zero hits, response source, and persisted endpoint |
| R-027 | A legacy catalog source may receive requests after provider-local routing is selected | Medium | High | Resolve list/page/search only from the selected provider, carry legacy `[litellm]` unchanged, and assert exact provider requests plus zero legacy hits |
| R-028 | A malformed reasoning default or provider-specific value could silently change request behavior | Medium | Medium | Preserve every non-empty value verbatim, reject whitespace-only setup input, enforce mandatory non-off selection, and cover request bodies plus persisted TOML |
| R-029 | Saving at the wrong wizard transition could write unconfirmed input or lose focused-domain ownership | Medium | High | Keep coordinated values until final review, save provider/model only at focused confirmation, and drive failure/cancellation through the real wizard |
| R-030 | A corrected concurrent-search test could still pass without proving worker overlap or cleanup | Medium | Medium | Coordinate slow and fast workers with channels/barriers, apply through the generation guard, assert exact final IDs, and join every worker before scenario exit |
| R-031 | A provider closes a valid SSE response without `[DONE]`, causing a user to mistake a visible prefix for a complete command | Medium | High | Require `[DONE]`, preserve the prefix for diagnosis, map truncation to network status 3, omit success metadata and execution, and cover clean EOF separately from connection reset |
| R-032 | A synchronous content callback couples provider read progress to terminal write speed and can fail after visible output | Medium | Medium | Propagate write/flush errors as the existing I/O status, finish the spinner on every path, preserve the prefix, omit metadata and execution, and test with a controlled writer |
| R-033 | Buffered verbose reasoning may be mistaken for missing reasoning while a stream is active | Medium | Low | Keep reasoning in the final aggregate, print it only after successful `[DONE]` and only under `-v`, and assert its absence before the release gate |
| R-034 | A provider sends usage or model metadata in a choices-empty event and final accounting falls back to the requested model | Medium | Medium | Extract top-level model and usage independently of choices, configure pricing only for the response model in the fixture, and assert exact model, cost, and throughput |
| R-035 | Terminal spinner cleanup can race first content or a stream error and leave control sequences visible | Medium | Medium | Keep one CLI-owned spinner lifecycle, finish it on first content and every return path, and assert PTY clear-line evidence for both delayed success and mid-stream failure |
| R-036 | A release artifact's target-dependent shared-library requirements are hidden by a universal static-deployment claim | Medium | High | Build the exact release target, inspect it with `file` and `ldd` on Linux or `otool -L` on macOS, and document only the verified target requirements |
| R-037 | The CLI version can drift from the Cargo package version used to build the release artifact | Medium | Medium | Derive the CLI version from package metadata and assert the exact package version through the real release-binary scenario |
| R-038 | Generated completion output can omit or misrepresent a new option, positional argument, subcommand, or value if its metadata source diverges from the CLI parser | Medium | High | Render only from `Cli::command()`, enumerate the complete root tree and selector values in feature scenarios, and avoid a separately maintained command list |
| R-039 | Exposing the completion library's shell enum directly could change the stable error contract or version-dependent value surface | Medium | High | Keep a local closed `CompletionShell` selector aligned explicitly to the pinned native set `bash`, `elvish`, `fish`, `powershell`, and `zsh`, and assert the literal `unsupported shell '<value>'; choose bash, elvish, fish, powershell, or zsh` contract |
| R-040 | A generated script may be syntactically invalid for the selected shell or the shell executable may be unavailable in the verification environment | Medium | Medium | Run the corresponding Bash, Elvish, Fish, PowerShell, and Zsh parser checks; report a missing executable as an explicit environment limitation rather than a false syntax pass |
| R-041 | Completion generation could accidentally load config, auto-create the XDG file, contact a provider, write shell configuration, or contaminate stdout/stderr | Low | High | Dispatch before configuration and provider setup; require stdout-only success, empty stderr, an absent-config before/after snapshot, no isolated-directory writes, and a zero-hit provider sentinel |
| R-042 | Completion renderer or command traversal order could make repeated output differ byte-for-byte | Medium | Medium | Generate twice from the same binary and selector, compare raw stdout bytes, and keep the command definition and renderer mapping deterministic |
| R-043 | Reserving `completions` can surprise users whose unquoted question begins with that token | Medium | Medium | Document the intentional reservation and require a quoted question or `--` separator; keep the consequence in the proposal, help contract, feature, and Arc42 docs |
| R-044 | A shortcut installer could corrupt or duplicate a user shell startup file | Medium | High | Validate exact marker counts before any write, preserve bytes outside one block, use same-directory atomic replacement, reject malformed layouts, and test byte-for-byte failure preservation |
| R-045 | A selected shell target may fail after another target has been changed | Medium | Medium | Attempt every selected target independently, report every success and OS failure, retain successful changes deliberately, and return an aggregate non-zero result |
| R-046 | Shell-native widget syntax or key maps differ across Bash, Zsh, and Fish environments | Medium | Medium | Keep one native generated block per shell, run installed Bash/Fish parser checks, use real Bash and Fish PTY/subprocess checks for buffer behavior, and retain the remaining Zsh interactive-runtime limitation |
| R-047 | A widget could pass a leading option, reserved token, or generated output into an unintended shell path | Medium | High | Use `command watn -- "$question"`, capture stdout without evaluation, preserve stderr, and assert leading-option/reserved-token, multiline, failure, and no-execution scenarios |
| R-048 | Automatic first-use onboarding could surprise a user by mutating shell files | Medium | High | Make the shortcut question explicit and opt-in; Enter/no and empty selection perform no shell I/O; report every selected target before returning |
| R-049 | A shell path, symlink, non-UTF-8 file, or permission failure could make installation platform-dependent | Medium | Medium | Resolve absolute HOME/XDG targets, preserve bytes outside ASCII markers, reject unsafe symlinks and directories, use temporary files in the target directory, and include exact path/reason diagnostics |
| R-050 | A terminal color palette or environment color policy may make the green active border hard to distinguish or suppress ANSI styling | Low | Low | Keep the visible cursor and focus text unchanged, remove inherited `NO_COLOR` in the PTY child, set `TERM=xterm-256color`, parse green SGR foreground parameters semantically, and retain the cursor/focus text as redundant cues |
| R-055 | Flattened comment construction or interactive-buffer redraw could misrepresent the request or not reflect wrapped input | Low | Low | Replace only CR, LF, and TAB with spaces so the request stays one comment line; verify the actual Fish buffer newline while retaining existing Bash commit-time execution and no-evaluation coverage; interactive wrapped-line redraw remains outside the measured contract |
| R-056 | The 500 ms grace hard-exit can cut off final buffered bytes in a stalled or connecting stream | Medium | Low | Keep the parse-loop flag check for the common streaming case, preserve visible content, and exit 130 without an error; a partial tail is an accepted cost of hard cancellation |
| R-057 | The detached worker thread can outlive the main thread by microseconds after the grace expires, racing process teardown | Low | Low | Stdout writes are internally locked; the process exits immediately after the flag-driven cleanup so no shared mutable state is exposed |
| R-058 | A provider-local catalog endpoint may be stale after provider or endpoint change | Medium | High | Invalidate the catalog state on provider/endpoint/credential change, re-probe, preserve the prior saved base on failed edits, and use manual mode when no base is available |
| R-059 | Migration from an arbitrary provider key may silently replace a user-selected credential or default model | Medium | High | Treat saved credential representation as authoritative, define deterministic destination-default precedence, and assert source removal and unrelated-entry preservation |
| R-060 | A custom reasoning value may contain whitespace or provider-specific syntax that is normalized accidentally | Medium | Medium | Reject only blank/whitespace-only input, preserve non-empty bytes, and assert round-trip request construction |
| R-061 | The final configuration write may succeed while a shell target fails | Medium | Medium | Keep config and shell operations independent, retain successful target changes, report every target, and return non-zero for any failed target |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Non-OpenAI-compatible provider adapters | Medium | Provider trait is designed for extension; new adapters per provider |
| TD-002 | No input validation of shell commands before execution | Medium | User sees full command before confirming |
| TD-003 | Crossterm terminal event behavior varies across terminal emulators | Low | Key bindings use standard sequences (arrows, backspace, enter, escape, ctrl-c); non-standard terminals may require `TERM` detection fallbacks |
| TD-004 | Reasoning config parsing edge cases (provider-specific values read from an edited config) | Low | Parse non-empty strings without normalization; only `off` omits `reasoning_effort`, while provider-specific values remain request-visible |
| TD-005 | E2E tests need a non-persisted endpoint override to exercise configured-provider paths without live network access | Medium | Keep the override behind the debug-plus-feature guard; use reachable loopback twins and explicit binary paths; assert the exact persisted configured URL before and after routing |
| TD-009 | Cancellation uses a fixed 500 ms grace heuristic because the blocking reqwest client cannot split connect and read timeouts | Low | Migrate to an async client with `tokio::select!` if a future change makes the grace heuristic or partial-bytes truncation unacceptable |

## ADR-0011 bad-consequence coverage

The following consequences are accepted and mitigated explicitly:

- TTY and model-catalog dependence: non-TTY implicit first use prints guidance
  and exits 1; explicit setup commands remain available; catalog failures are
  visible and repeatable.
- Explicit-provider first-use errors: `--provider` and `WATN_PROVIDER` retain
  unknown-provider and missing-key behavior rather than silently entering a
  renderer.
- No automatic request resume: successful setup exits after model selection;
  the original request is deliberately not sent and must be rerun.
 - Partial onboarding: coordinated setup keeps the provider in the draft until
   final confirmation, so cancellation/failure leaves the baseline unchanged;
   focused provider setup remains available when a user wants only that domain.
- Literal secrets on disk: input is masked and every save enforces Unix mode
  `0600`; loading may warn about a pre-existing world-readable file.
- Fixed-name collisions: `openrouter`/`custom` replacement is limited to one
  entry and unrelated providers/configuration are preserved.
 - Atomic config replacement: a failed snapshot write leaves the previous file
   untouched, while shell target changes remain independent and may be partial.
- Test transport seam: the override is ephemeral, construction-time only, and
  verified on all touched HTTP paths without changing readiness or persisted
  endpoint values. The compile-time guard keeps it out of every release
  profile, including release with `test-support` enabled; the release artifact
  check records the target-specific runtime result.
- Debug build selection: the two debug variants reuse Cargo's dependency cache
  but are copied to distinct absolute paths before Cucumber starts, so a stale
  or overwritten `target/debug/watn` cannot produce a false-green result.
- Exact transport evidence: separate local twins, exact method/path and
  Authorization matchers, expected/competing request counts, response-source
  checks, and raw TOML endpoint checks prevent a broad mock from hiding a route
  or credential regression.
- Widget layout width: the native widget composition improves scanning on the
  supported terminal size but cannot guarantee every column remains spacious on
  a very narrow terminal; constraints and wrapped guidance limit the damage.
- Search worker lifecycle: asynchronous search adds timing and channel state;
  the generation guard and event-loop ownership of applied results prevent stale
  rows from replacing current input, while cleanup prevents detached test
  processes from leaking.
- Partial wizard saves: a user can leave after credentials but before all
  model pages; the caller persists the valid provider and only completed tier
  assignments, leaving uncompleted tiers unchanged.

 - Provider-local catalog routing: the selected provider owns discovery and
   authorization; legacy LiteLLM data is retained but inert for setup/model
   discovery. Separate twins and exact request matchers prevent source crossover.
- Credential-source preservation: literal and exact environment references are
  retained through setup and expanded only when used. A missing saved reference
  fails before a request rather than falling back to another variable.
 - Reasoning policy: catalog suggestions coexist with verbatim non-empty values;
   `off` omits the field and whitespace-only input is rejected.
 - Provider confirmation boundary: coordinated final review is the only config
   write boundary, while focused commands save only their owned domains.
- Concurrent search evidence: deterministic overlap and worker joins ensure the
  newest-result guarantee is tested rather than inferred from serialized sleeps.
- Synchronous stream callback: no channel or background stderr writer exists, so
  callback write speed is part of provider progress; the direct controlled-writer
  test verifies I/O status 1, prefix preservation, spinner cleanup, and skipped
  completion actions.
- Mandatory completion marker: `[DONE]` is a deliberate compatibility boundary.
  Clean EOF is a network failure with status 3 even after valid content, which
  prevents partial commands from being treated as complete or executed.
- Buffered reasoning: verbose reasoning is intentionally delayed until successful
  completion; the E2E release gate proves command stdout is progressive while
  reasoning remains absent until the stream is complete.
- First-event timing and connection close: elapsed time starts before decoding at
  the first non-DONE event and ends at `[DONE]`; the held-connection scenario
  proves the client does not wait for a server close.
- Exact-once output: content chunks are written once and the final aggregate is
  never reprinted; raw-terminal and piped scenarios count generated and execution
  lines separately.

## ADR-0017 consequence coverage

The completion-generation decision has these durable consequences:

- Authoritative metadata: generated output follows `Cli::command()`, while the
  complete root tree is asserted for every supported shell and selector values
  are asserted where the renderer emits positional suggestions; R-038 covers
  accidental drift.
- Closed selector: only lowercase `bash`, `elvish`, `fish`, `powershell`, and `zsh` are product values;
  R-039 covers leakage of the library's broader enum and the literal parser
  contract.
- Renderer dependency: adding a shell requires selector, mapping, help, and
  feature changes; renderer bytes remain a dependency-sensitive surface and are
  checked by R-040 and R-042.
- Shell validation: Bash, Elvish, Fish, PowerShell, and Zsh parser checks expose syntax or local
  executable availability instead of treating generated text as inherently
  valid.
- Side-effect boundary: early dispatch and the no-config/sentinel snapshots
  prevent config creation, provider access, network traffic, shell-file writes,
  and stderr contamination; R-041 covers the failure mode.
- Reserved token: the new subcommand changes an unquoted question's parse path;
  the quote/`--` guidance and R-043 make that compatibility consequence visible.

## ADR-0018 consequence coverage

The shell-shortcut decision has these durable consequences:

- Startup-file mutation: the feature is opt-in but changes user-owned files;
  R-044 and R-048 require exact marker ownership, a default decline, isolated
  tests, and visible target reports.
- Atomic replacement: same-directory temporary files and rename protect an
  existing target from pre-rename failures, while R-049 records platform/path
  limitations and R-044 covers malformed markers.
- Independent targets: Bash, Zsh, and Fish can be partially installed because
  rollback is not promised; R-045 requires every result and aggregate failure.
- Shell/version dependence: native line-editor APIs and key maps vary, so
  R-046 combines installed Bash/Fish syntax checks with one real Bash
  subprocess rather than claiming identical runtime evidence for all shells.
- PATH and reserved arguments: `command watn -- "$question"` keeps the
  installed command resolution explicit and prevents leading options or the
  reserved `completions` token from changing the question parse; R-047 covers
  this and the no-evaluation boundary.
- Multiline output: embedded line breaks remain text in the buffer while only
  trailing CR/LF is normalized; R-047 and the quality scenarios verify that the
  text is never evaluated.
- Request preservation: the buffer is replaced with a flattened `#`-comment of
  the request followed by the generated command; the shell executes only the
  command on Enter. R-055 covers comment flattening and interactive-buffer
  testability limits, while QS-055 and the real-Bash E2E verify the contract.

## ADR-0016 consequence coverage

The release-truth decision has these durable consequences:

- Package metadata is the single version source, so a changed package version
  requires a rebuilt binary before verification.
- Release deployment requires compatible runtime libraries for the selected
  target. Linux uses `file` and `ldd`; macOS uses `otool -L`.
- The shared-library set varies by target, so the documentation records the
  verified target rather than promising one universal list.
- No static artifact is introduced. Static portability remains a separate
  release-engineering decision.

## ADR-0019 consequence coverage

The interruptible-completion decision has these durable consequences:

- Blocking provider: no async runtime was introduced; the SSE parser checks the
  shared interrupt flag at each loop iteration, so a flowing stream cancels on
  the next SSE line.
- Worker watchdog: the streaming call runs on a worker thread and the main
  thread bounds the unreachable phases with a 500 ms grace before detaching the
  worker and exiting 130; R-056 and R-057 track the fixed-heuristic and
  short-lived-detached-worker costs, and TD-009 records the async migration if
  the heuristic becomes unacceptable.
- Interrupted status: a new error variant exits 130 with no error text. On the
  join path the spinner and partial output still finish and the already-streamed
  prefix remains visible; the grace path exits 130 directly without cleanup.

## Cleanup boundary

Repository cleanup is deliberately conservative:

- Remove only the unused `_config` parameter from `build_registry()` after
  implementation.
- Retain public `ProviderRegistry` because it remains the provider lookup
  boundary and current CLI code uses it.
- Retain public `ProviderSetupResult`, `cancellation_result`, and
  `configured_result` because current provider setup feature steps consume them
  and external consumers of the public library modules are unknown.
- Remove only `WatnWorld` fields proven write-only by repository-wide search
  after scenario migration. Any field used by a permanent feature step remains.
