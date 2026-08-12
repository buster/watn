# Watn? ¯\\\_(ツ)\_/¯

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 91%](https://img.shields.io/badge/line%20coverage-91%25-brightgreen)](https://github.com/buster/watn/blob/main/coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](https://github.com/buster/watn/blob/main/coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

![Crates.io Version](https://img.shields.io/crates/v/watn)

You know what you want. watn knows how to type it.
Ask in plain language. Get a command for command requests, or an answer for
questions. Response content is streamed to stdout; use `-x` to confirm execution,
or pipe the output anywhere.

## Demo

```text
┌──────────────────────────────────────────────┐
│ $ watn "files changed in the last day"       │
│                                              │
│   find . -type f -mtime -1                   │
│                                              │
│   deepseek · 100 tok/s · 5.3s · ¯\_(ツ)_/¯   │
└──────────────────────────────────────────────┘
```

The demo output is illustrative. The model, throughput, elapsed time, and cost
depend on the configured provider, response, and local runtime.

## Quick start

- Rust 1.97.1 (the repository pins this in `rust-toolchain.toml`)
- An OpenAI-compatible API endpoint and a usable API key

```sh
cargo install watn
export OPENROUTER_API_KEY=your-key
watn setup
watn "find all files modified in the last day"
```

Use `watn provider` for only the endpoint and credential; use `watn setup` for
the full wizard. Setup also assigns model tiers and can install shell
completion or the optional Ctrl-W shortcut. Ctrl-W turns the current shell
prompt into a command.

## Usage

Pass a question as a positional argument or via stdin. Response content is
streamed incrementally to stdout. Completion metadata (model, tokens/s, elapsed
time, and cost when configured) is written to stderr. With `-v`, nonblank
provider reasoning is printed to stderr after successful completion.

```text
$ watn find all files modified in the last day
find . -type f -mtime -1
$ printf "find all rust source files" | watn
find . -type f -name "*.rs"
```

### Model tiers

```text
$ watn -2 "list pods with the most memory usage"
$ watn -3 "debug this strace"
$ watn --model <provider>/<model> "convert flac to mp3 recursively"
$ watn --set-small <small> --set-normal <normal> --set-thinking <thinking> models
```

Use `watn models` to choose identifiers from the provider catalog. The
`--set-*` options must appear before `models`.

### Execute mode

```text
$ watn -x "remove all .bak files"
  rm *.bak
  Execute now? [Y/n]
```

Press Enter, `y`, or `yes` to execute. Any other text cancels; Ctrl-C
interrupts.
Execution uses `sh -c` with the generated command.

### Options and commands

| Option | Description |
|---|---|
| `-1`, `--small` | Small/fast model tier (default) |
| `-2`, `--normal` | Balanced model tier |
| `-3`, `--thinking` | Thinking/reasoning model tier |
| `--model <NAME>` | Explicit model override |
| `-x`, `--execute` | Prompt for confirmation before executing the command |
| `-v`, `--verbose` | Print provider reasoning to stderr after successful completion when available |
| `--provider <NAME>` | Select a configured provider |
| `--set-small <NAME>` | Set the small-tier model non-interactively |
| `--set-normal <NAME>` | Set the normal-tier model non-interactively |
| `--set-thinking <NAME>` | Set the thinking-tier model non-interactively |
| `-V`, `--version` | Print version and exit |
| `-h`, `--help` | Print help and exit |

Subcommands are `setup`, `provider`, `models`, `completions <SHELL>`, and `help`.
Completion scripts are supported for `bash`, `elvish`, `fish`, `powershell`,
and `zsh`.

For Bash, generate and load a completion script with:

```sh
watn completions bash > watn.bash
source watn.bash
```

## Media

*Illustrative `watn setup` wizard screenshot. The current wizard has seven
pages; this screenshot predates that flow.*
![watn setup page](https://raw.githubusercontent.com/buster/watn/main/watnsetup.png)

*Ctrl-W shell shortcut to ask watn directly from the terminal prompt.*
![shell shortcut](https://raw.githubusercontent.com/buster/watn/main/ctrlw.gif)

*Illustrative command-generation flow.*
![command generation](https://raw.githubusercontent.com/buster/watn/main/watncmd.gif)

## Configuration

Config is TOML at `$XDG_CONFIG_HOME/watn/config.toml` (normally
`~/.config/watn/config.toml`). Resolution is per setting:
leftmost value wins.

| Setting | Resolution order |
|---|---|
| Provider | `--provider` > `WATN_PROVIDER` > `[defaults].provider` > `openrouter` |
| Model | `--model` > selected tier > `WATN_MODEL`/`[defaults].model` > provider default |
| Endpoint | Saved provider entry > built-in `openrouter`/`openai` endpoint; custom providers require an entry |
| API key | Saved literal/reference > provider-specific env var > `WATN_API_KEY` |

The default request uses the small tier. `WATN_MODEL` changes only the default
model, not a configured tier. Provider-specific key variables are
`OPENROUTER_API_KEY` or `WATN_<PROVIDER>_API_KEY`. A missing file is created as
a commented template; Unix saves use mode `0600`. The wizard can save
`${ENV_NAME}` instead of a literal credential.

`watn setup` visits `URL`, `API key`, `Small Model`, `Middle Model`, `Large
Model`, `Shell Completion`, and `Shell Shortcut`. `watn provider` covers the
first two; `watn models` covers the model pages. `Ctrl-R` switches model-page
focus between the table and reasoning strength.

## Release artifacts

`cargo build --locked --release --bin watn` produces
`target/release/watn` for the selected target. For version tags, the release
workflow publishes the source crate to crates.io and creates a GitHub release;
it does not publish the executable to crates.io. Runtime library requirements
depend on the target, and the project does not provide a universal static
binary.

## Development

From a repository checkout, the acceptance runner uses separate default and
`test-support` binaries, mocks, and loopback services, so no live provider or
API key is needed. The
pinned Rust toolchain plus Bash, Fish, and Zsh are required for all checks.

```sh
./run-tests.sh
```

The command above runs non-E2E scenarios (`not @wip and not @e2e`). Run E2E
scenarios (`@e2e and not @wip`) with:

```sh
./run-tests.sh --e2e
```

Coverage additionally requires `cargo-llvm-cov` and Python 3.

## License

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue)](LICENSE)

GPL-3.0-or-later

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](https://github.com/buster/watn/blob/main/coverage/cobertura-coverage.xml)

Line coverage: 91% (9586/10578)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->

Branch coverage is reported as `n/a (0/0)` because the stable
`cargo llvm-cov` Cobertura export contains no branch counters. The current
`cargo llvm-cov` branch-coverage mode is unstable, so the source reports are
merged without inventing a branch percentage.
