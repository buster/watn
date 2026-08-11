# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | OpenAI API schema drifts from the implemented subset | Low | Medium | Pin tested provider versions in CI; add schema-version field to config |
| R-002 | Users configure providers with non-standard `/v1/chat/completions` paths | Medium | Low | Document the requirement |
| R-003 | Config file with secrets (API keys) committed to version control | Medium | High | Support env-var-only API key config; warn if config file is world-readable |
| R-004 | LiteLLM `/models` endpoint schema drifts | Low | Low | Parse response leniently; error gracefully on unexpected format |
| R-005 | User confirms execution and the command is destructive | Low | High | Tool prints the command before prompting; user sees what they are confirming |
| R-006 | Execution flow UX — command content is streamed to stdout before the prompt and may be mistaken for already-executed work | Medium | Medium | Keep command content on stdout for piping, put the confirmation prompt and metadata on stderr, and require confirmation before execution |
| R-007 | Model returns empty reasoning tokens on thinking tier | Medium | Low | Trim reasoning content before printing; skip print if content is empty/whitespace after trimming |
| R-008 | Template config generated from code may include irrelevant fields as Config grows | Low | Low | Template is a starting point — users delete what they do not need; template is meant to be edited |
| R-009 | PTY-based E2E tests are flaky across platforms/terminal emulators | Medium | Medium | Run PTY tests in CI with a known terminal type (`TERM=dumb` or `xterm-256color`); add generous read timeouts; document that local test failures may require `script`/`unbuffer` wrappers |
| R-010 | Arrow/page escape sequences differ across terminal emulators in the SetupWizard | Low | Medium | Standard sequences (`\x1b[A/B`, `\x1b[5~/6~`); PTY E2E tests pin `TERM=xterm-256color`; ratatui/crossterm parse both classic and application-cursor modes |
| R-011 | A user chooses literal credential storage and the API key remains on disk | Medium | High | Prefer environment-backed references, mask input, apply mode `0600`, and warn on world-readable config files |
| R-012 | Automatic onboarding requires an interactive terminal and a reachable model catalog | Medium | Medium | Detect non-TTY use, print actionable setup guidance, keep explicit `watn provider` and `watn models` commands available, and allow rerunning setup after a catalog failure |
| R-013 | Provider setup succeeds but model selection is cancelled or fails, leaving a provider without tiers | Medium | Medium | Save provider first, return a typed model result, preserve the provider on failure, report the model failure clearly, map Escape/Ctrl-C to 1/130, and make model setup repeatable |
| R-014 | Explicit `--provider` or `WATN_PROVIDER` selection does not receive automatic onboarding | Medium | Medium | Preserve the existing error contract deliberately, print the existing actionable error, and document `watn provider` as the explicit setup path |
| R-015 | Successful automatic setup does not resume the original request | Medium | Low | Exit clearly after model selection, assert no chat request was sent, and document that the user reruns the original question |
| R-016 | Fixed `openrouter` and `custom` names can collide with manually maintained entries | Medium | High | Replace only the selected fixed entry, preserve unrelated providers and config, and document the intentional collision before implementation |
| R-017 | Direct config writes are not atomic and may be interrupted | Low | High | Keep the existing direct-write mechanism as the explicit constraint, enforce mode `0600` after every save, and do not promise temp-file/rename semantics |
| R-018 | The ephemeral E2E transport override could leak into persistence/readiness or cover only one request path | Medium | High | Apply it only at HTTP construction under the debug-plus-feature guard, never consult it during readiness, assert the exact persisted endpoint, and cover every touched `/models` and `/chat/completions` path |
| R-019 | Structured columns and multiple regions can become cramped or unreadable on small terminals | Medium | Medium | Use Ratatui layout constraints, truncation-safe cells, wrapped paragraphs, and PTY coverage at the supported test size; keep keyboard flow independent of visual width |
| R-020 | Debounced worker results can race with user input or outlive the SetupWizard | Medium | Medium | Increment a generation for every query change, check it before and after the debounce, apply results only through the event loop, ignore Enter while pending, and reap the SetupWizard before exit |
| R-021 | A shared wizard can make a partial save ambiguous when the user leaves before model selection is complete | Medium | Medium | Keep provider and completed model choices separate in the runtime result, validate before Save, and write only completed sections while Discard performs no write |
| R-022 | Migrating Tab and Escape changes can surprise users of the existing model pages | Medium | Medium | Keep Shift-Tab as the explicit back-page key, make Ctrl-R reasoning focus visible, migrate permanent scenarios, and retain command-specific entry points |
| R-023 | `watn models` and `watn provider` can start the shared wizard with stale or incomplete page state | Medium | Medium | Seed endpoint, credential storage, current tier selections, and model-specific reasoning from the loaded config; define and test each entry point's initial/final page range |
| R-024 | A release build with `test-support` could accidentally retain the endpoint override | Medium | High | Guard the lookup with `cfg(all(feature = "test-support", debug_assertions))`; inspect the release-profile artifact and keep the configured-endpoint source invariant explicit |
| R-025 | A stale or overwritten debug executable could make transport tests execute the wrong binary | Medium | High | Build the two debug variants sequentially through Cargo's shared target cache, copy each to a unique temporary path, pass only those absolute paths to the harness, and fail before scenarios when a path is missing |
| R-026 | Broad mocks could report a successful request from the wrong endpoint or credential | Medium | High | Use separate local twin servers and mocks matching exact method/path/Authorization; assert expected counts, competing zero hits, response source, and persisted endpoint |
| R-027 | A catalog source may be configured with an endpoint or credential policy that differs from the active chat provider | Medium | High | Resolve catalog and chat sources separately, pass the selected source explicitly to list/page/search calls, and assert exact source, query, and Authorization behavior |
| R-028 | A malformed reasoning default or supported-effort list could silently change request behavior | Medium | Medium | Use one closed-set policy, ignore unknown efforts, enforce mandatory non-off selection, preserve valid existing choices, and cover request bodies plus persisted TOML |
| R-029 | Saving a provider at the wrong wizard transition could either lose a confirmed source or write unconfirmed input | Medium | High | Validate and resolve before the first catalog request, persist only at credential confirmation, keep tier writes separate, and drive failure/cancellation through the real wizard |
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

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Non-OpenAI-compatible provider adapters | Medium | Provider trait is designed for extension; new adapters per provider |
| TD-002 | No input validation of shell commands before execution | Medium | User sees full command before confirming |
| TD-003 | Crossterm terminal event behavior varies across terminal emulators | Low | Key bindings use standard sequences (arrows, backspace, enter, escape, ctrl-c); non-standard terminals may require `TERM` detection fallbacks |
| TD-004 | Reasoning config parsing edge cases (unknown strength values read from an edited config) | Low | Parse leniently; only `off`/`low`/`minimal`/`medium`/`high` map to `reasoning_effort`, unknown values fall back to no reasoning |
| TD-005 | E2E tests need a non-persisted endpoint override to exercise configured-provider paths without live network access | Medium | Keep the override behind the debug-plus-feature guard; use reachable loopback twins and explicit binary paths; assert the exact persisted configured URL before and after routing |

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
- Partial onboarding: the provider is saved before model setup, so model
  cancellation/failure leaves a usable provider and no accidental request.
- Literal secrets on disk: input is masked and every save enforces Unix mode
  `0600`; loading may warn about a pre-existing world-readable file.
- Fixed-name collisions: `openrouter`/`custom` replacement is limited to one
  entry and unrelated providers/configuration are preserved.
- Direct-write interruption: no atomic rename guarantee is claimed; the risk is
  documented rather than hidden behind a false atomicity promise.
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

- Catalog-source separation: LiteLLM is discovery-only and optional-auth; the
  active provider remains the chat destination. Separate twins and exact
  request matchers prevent source crossover.
- Credential-source preservation: literal and exact environment references are
  retained through setup and expanded only when used. A missing saved reference
  fails before a request rather than falling back to another variable.
- Reasoning policy: the closed strength set and shared resolver prevent TTY and
  non-TTY model selection from writing empty or unsupported values.
- Provider confirmation boundary: saving after validation but before catalog
  access leaves a usable provider after discovery failure while preserving old
  tiers and avoiding an unintended original request.
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
