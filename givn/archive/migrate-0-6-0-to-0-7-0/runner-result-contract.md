# Runner Result Contract: migrate-0-6-0-to-0-7-0

## Scope inventory

| Scope | Command | Script/file that runs scenarios | Where counts come from today |
|---|---|---|---|
| regular | `./run-tests.sh` | `tests/features_runner.rs` via `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` | cucumber `Summarize` writer: `writer.scenarios_stats()` after the run |
| e2e | `./run-tests.sh --e2e` | the same runner with `--tags '@e2e and not @wip'` | same |

Both scopes are configured in `givn/commands.yaml`. The script exports
`GIVN_RESULT_SCOPE` (`regular`/`e2e`); the runner writes one JSON object to
`GIVN_RESULT_FILE` when the variable is present, before exiting with the
run's failure code.

## Before

Commit `0aec03b` added the result write but replaced cucumber's
`run_and_exit()` with a manual `stats.failed`/`stats.skipped` check, which
omitted parsing and hook errors. A malformed feature could silently drop its
scenarios while the run exited 0 and the result still reconciled:

```
$ GIVN_RESULT_FILE=/tmp/givn-red.json ./run-tests.sh --name 'Custom OpenAI-compatible provider from config'
[Summary]
1 feature
1 scenario (1 passed)
3 steps (3 passed)
1 parsing error
EXIT: 0
$ cat /tmp/givn-red.json
{"failed":0,"passed":1,"scope":"regular","skipped":0,"total":1}
```

## After

The exit condition now includes the harness failure signal
(`StatsWriter::execution_has_failed()`: failed steps, parsing errors, hook
errors) plus skipped scenarios; the result is still written before the exit.
The same command after commit `6c52d08`:

```
[Summary]
1 feature
1 scenario (1 passed)
3 steps (3 passed)
1 parsing error
EXIT: 1
{"failed":0,"passed":1,"scope":"regular","skipped":0,"total":1}
```

## Sample results

```
$ result=$(mktemp); GIVN_RESULT_FILE="$result" ./run-tests.sh
222 scenarios (222 passed)
{"failed":0,"passed":222,"scope":"regular","skipped":0,"total":222}

$ result=$(mktemp); GIVN_RESULT_FILE="$result" ./run-tests.sh --e2e
88 scenarios (88 passed)
{"failed":0,"passed":88,"scope":"e2e","skipped":0,"total":88}
```

Both JSON objects parse and reconcile (`passed + failed + skipped == total`),
the scopes are exact, and `total > 0`.

## Fail-closed gate

Givn rejects a missing file, malformed JSON, a wrong scope, `total == 0`,
`failed != 0`, `skipped != 0`, and non-reconciling sums. A non-zero command
exit still blocks the archive even when the result reconciles.
