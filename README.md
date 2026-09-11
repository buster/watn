# Watn? ¯\\\_(ツ)\_/¯

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 92%](https://img.shields.io/badge/line%20coverage-92%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

```text
__      __ __ _ | |_  _ __   ___
\ \ /\ / // _` || __|| '_ \ |__ \
 \ V  V /| (_| || |_ | | | |  / /
  \_/\_/  \__,_| \__||_| |_| |_|
                              (_)
```
[![Crates.io Version](https://img.shields.io/crates/v/watn)](https://crates.io/crates/watn)

watn sends a plain-language question to a configured OpenAI-compatible endpoint
and prints the answer: a shell command for command requests, text otherwise. In
an interactive terminal, generated commands open in the review surface before
anything reaches stdout; nothing is executed without confirmation.

## Demo

*Ctrl-W with the review surface disabled: the current prompt is sent to watn, and
the generated command replaces the shell buffer without executing it.*

![Ctrl-W without the review surface](https://raw.githubusercontent.com/buster/watn/main/ctrlw.gif)

*`watn "query"` with the review surface: the command flow and model-written stage
purposes; accepting prints the command.*

![Review surface for a direct question](https://raw.githubusercontent.com/buster/watn/main/preview.gif)

## Installation

```sh
cargo install watn
```

Requires a Rust toolchain and an OpenAI-compatible endpoint with a usable API
key. Releases are source-only: the crate is published to crates.io and a GitHub
release is created with the changelog; no prebuilt executables are attached.

## Quick setup

On first use without a configuration file, watn starts quick setup. Run it
explicitly with:

```sh
watn quicksetup
```

It asks for the completion endpoint, the credential, one model for each of the
small, normal, and thinking tiers, and which shell integrations to install. An
empty answer accepts the suggestion shown. Everything is validated locally; no
network request is made. Ctrl-C leaves an existing configuration unchanged;
completing an explicit quick setup overwrites it.

## Setup

```sh
watn setup
```

The full wizard covers provider choice, endpoint, credential source (literal
value or `${ENV_VAR}` reference), the provider's model catalog, the model and
reasoning value for each tier, and shell integration, then shows a final review.
Configuration is written only at final confirmation.

The focused commands `watn provider`, `watn models`, and `watn shell` configure
one area each; `watn completions <SHELL>` prints a completion script for `bash`,
`elvish`, `fish`, `powershell`, or `zsh`.

## Usage

```sh
watn "find all files modified in the last day"
printf "find all rust source files" | watn
watn -x "remove all .bak files"
```

```text
$ watn --help
Ask in plain language. Get one command.

Usage: watn [OPTIONS] [QUESTION]... [COMMAND]

Commands:
  setup        Configure provider, models, reasoning, and shell integrations interactively
  models       Configure model tiers and reasoning settings interactively
  provider     Configure a provider endpoint and credential
  shell        Configure shell completion and Ctrl-W integrations
  quicksetup   Configure provider, models, and shell integrations with a minimal question flow
  completions  Generate a shell completion script on stdout for the caller to install or source
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [QUESTION]...  Natural-language question to turn into a command

Options:
  -1, --small                 Use the small/fast model tier
  -2, --normal                Use the balanced model tier
  -3, --thinking              Use the thinking/reasoning model tier
      --model <MODEL>         Use an explicit model instead of a tier
  -x, --execute               Prompt before executing the generated command
      --review-panel          Force the explanatory review surface on for this invocation
      --no-review-panel       Disable the explanatory review surface for this invocation
  -v, --verbose               Print provider reasoning to stderr when available
      --provider <PROVIDER>   Select a configured provider
      --set-small <MODEL>     Set the small-tier model non-interactively
      --set-normal <MODEL>    Set the normal-tier model non-interactively
      --set-thinking <MODEL>  Set the thinking-tier model non-interactively
  -h, --help                  Print help
  -V, --version               Print version
```

Response content goes to stdout; model, tokens/second, elapsed time, and cost
(when pricing is configured) go to stderr. `-x` executes with `sh -c` after
confirmation; when the review surface is active, final acceptance is that
confirmation.

### Review surface

The review surface opens on the controlling terminal after a complete candidate
in a simple default view: the model name, the command flow as a stage stack with
every part on its own row and its separator shown at the end of the preceding
row, and the selected stage's short description. Arrow keys move the selected
stage; `Enter` accepts, `d` or `?` switches to the detailed view, and `Escape`
cancels. The detailed view adds the intent, the flow position, the command
editor (`e`), the model chooser (`r`), and the full decision hints. `D` disables
the surface permanently: watn ends, prints the current command, and names
`watn --review-panel` on stderr. `--review-panel` and `--no-review-panel`
persist the choice they set; without a question they only persist and exit.
Direct edits commit with `Enter`, discard with `Escape`, and still require final
acceptance.

### Shell shortcut

Setup can install an optional Ctrl-W widget for Bash, Zsh, and Fish. It reads
the current shell buffer, calls `watn -- "$question"`, records the question in
shell history as a `#` comment, and replaces the buffer with the accepted
command without evaluating it.

## Configuration

`$XDG_CONFIG_HOME/watn/config.toml`, normally `~/.config/watn/config.toml`.
Per setting, the leftmost source wins: CLI flag, environment variable, config
value, built-in default. Credentials can be stored literally or as
`${ENV_NAME}`; the file is created as a commented template and saved with mode
`0600` on Unix.

## Development

```sh
./run-tests.sh          # not @e2e
./run-tests.sh --e2e    # @e2e
```

The pinned Rust toolchain plus Bash, Fish, and Zsh are required for all checks.
The runner uses mocks and loopback services, so no provider or API key is
needed. Coverage additionally requires `cargo-llvm-cov` and Python 3.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 92% (18121/19664)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
