# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | OpenAI API schema drifts from the implemented subset | Low | Medium | Pin tested provider versions in CI; add schema-version field to config |
| R-002 | Users configure providers with non-standard `/v1/chat/completions` paths | Medium | Low | Document the requirement |
| R-003 | Config file with secrets (API keys) committed to version control | Medium | High | Support env-var-only API key config; warn if config file is world-readable |
| R-004 | LiteLLM `/models` endpoint schema drifts | Low | Low | Parse response leniently; error gracefully on unexpected format |
| R-005 | User confirms execution and the command is destructive | Low | High | Tool prints the command before prompting; user sees what they are confirming |
| R-006 | Execution flow UX — command printed to stdout before prompt, user may interpret as already-executed | Medium | Medium | Command printed to stderr with prompt, execution output to stdout |
| R-007 | Model returns empty reasoning tokens on thinking tier | Medium | Low | Trim reasoning content before printing; skip print if content is empty/whitespace after trimming |
| R-008 | Template config generated from code may include irrelevant fields as Config grows | Low | Low | Template is a starting point — users delete what they do not need; template is meant to be edited |
| R-009 | PTY-based E2E tests are flaky across platforms/terminal emulators | Medium | Medium | Run PTY tests in CI with a known terminal type (`TERM=dumb` or `xterm-256color`); add generous read timeouts; document that local test failures may require `script`/`unbuffer` wrappers |
| R-010 | Arrow/page escape sequences differ across terminal emulators in the ratatui dialog | Low | Medium | Standard sequences (`\x1b[A/B`, `\x1b[5~/6~`); PTY E2E tests pin `TERM=xterm-256color`; ratatui/crossterm parse both classic and application-cursor modes |
| R-011 | A user chooses literal credential storage and the API key remains on disk | Medium | High | Prefer environment-backed references, mask input, apply mode `0600`, and warn on world-readable config files |
| R-012 | Automatic onboarding requires an interactive terminal and a reachable model catalog | Medium | Medium | Detect non-TTY use, print actionable setup guidance, keep explicit `watn provider` and `watn models` commands available, and allow rerunning setup after a catalog failure |
| R-013 | Provider setup succeeds but model selection is cancelled or fails, leaving a provider without tiers | Medium | Medium | Save provider first, return a typed model result, preserve the provider on failure, report the model failure clearly, map Escape/Ctrl-C to 1/130, and make model setup repeatable |
| R-014 | Explicit `--provider` or `WATN_PROVIDER` selection does not receive automatic onboarding | Medium | Medium | Preserve the existing error contract deliberately, print the existing actionable error, and document `watn provider` as the explicit setup path |
| R-015 | Successful automatic setup does not resume the original request | Medium | Low | Exit clearly after model selection, assert no chat request was sent, and document that the user reruns the original question |
| R-016 | Fixed `openrouter` and `custom` names can collide with manually maintained entries | Medium | High | Replace only the selected fixed entry, preserve unrelated providers and config, and document the intentional collision before implementation |
| R-017 | Direct config writes are not atomic and may be interrupted | Low | High | Keep the existing direct-write mechanism as the explicit constraint, enforce mode `0600` after every save, and do not promise temp-file/rename semantics |
| R-018 | The ephemeral E2E transport override could leak into persistence/readiness or cover only one request path | Medium | High | Apply it only at HTTP construction, never consult it during readiness, assert the exact persisted OpenRouter endpoint, and cover both `/models` and `/chat/completions` |
| R-019 | Structured columns and multiple regions can become cramped or unreadable on small terminals | Medium | Medium | Use Ratatui layout constraints, truncation-safe cells, wrapped paragraphs, and PTY coverage at the supported test size; keep keyboard flow independent of visual width |
| R-020 | Debounced worker results can race with user input or outlive the dialog | Medium | Medium | Increment a generation for every query change, check it before and after the debounce, apply results only through the event loop, ignore Enter while pending, and reap the dialog before exit |
| R-021 | A shared wizard can make a partial save ambiguous when the user leaves before model selection is complete | Medium | Medium | Keep provider and completed model choices separate in the runtime result, validate before Save, and write only completed sections while Discard performs no write |
| R-022 | Migrating Tab and Escape changes can surprise users of the existing model dialog | Medium | Medium | Keep Shift-Tab as the explicit back-page key, make Ctrl-R reasoning focus visible, migrate permanent scenarios, and retain command-specific entry points |
| R-023 | `watn models` and `watn provider` can start the shared wizard with stale or incomplete page state | Medium | Medium | Seed endpoint, credential storage, current tier selections, and model-specific reasoning from the loaded config; define and test each entry point's initial/final page range |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Non-OpenAI-compatible provider adapters | Medium | Provider trait is designed for extension; new adapters per provider |
| TD-002 | No input validation of shell commands before execution | Medium | User sees full command before confirming |
| TD-003 | Crossterm terminal event behavior varies across terminal emulators | Low | Key bindings use standard sequences (arrows, backspace, enter, escape, ctrl-c); non-standard terminals may require `TERM` detection fallbacks |
| TD-004 | Reasoning config parsing edge cases (unknown strength values read from an edited config) | Low | Parse leniently; only `off`/`low`/`medium`/`high` map to `reasoning_effort`, unknown values fall back to no reasoning |
| TD-005 | E2E tests need a non-persisted endpoint override to exercise the default OpenRouter path without live network access | Medium | Keep the override test-only and ephemeral; assert the real persisted default URL before routing requests to the loopback digital twin |

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
  verified on both required HTTP paths without changing readiness or persisted
  endpoint values.
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
