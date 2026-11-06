# watn reference

Exhaustive flags, keybindings, configuration keys, and exit codes for `watn`.
First-run use lives in [README.md](../README.md).

## Commands

| Command | Behavior |
|---|---|
| `watn [OPTIONS] [QUESTION]...` | Sends the question and prints the command or answer |
| `watn setup` | Full interactive wizard: provider through shell integration and review |
| `watn models` | Tier models and reasoning only |
| `watn provider` | Provider endpoint and credential only |
| `watn shell` | Completion and Ctrl-W integration only |
| `watn quicksetup [OPTIONS]` | Minimal first-run flow: endpoint, credential, three models, shells |
| `watn completions <SHELL>` | Prints a completion script on stdout |
| `watn explain [OPTIONS] [COMMAND]` | Opens the explanation card for an existing command |

`setup`, `provider`, `shell`, and `quicksetup` require stdin to be a terminal.
Without one they print setup guidance and exit 1. `models` runs the wizard on a
terminal; without one, and with a configured provider, it selects small, normal,
and thinking models from the provider catalog and saves them.

## Options

| Option | Effect |
|---|---|
| `-1`, `--small` | Small/fast tier |
| `-2`, `--normal` | Balanced tier |
| `-3`, `--thinking` | Thinking/reasoning tier |
| `--model <MODEL>` | Explicit model; conflicts with the three tier flags |
| `-x`, `--execute` | Prompt before executing the generated command |
| `--review-panel` | Force the review surface on and persist the choice |
| `--no-review-panel` | Disable the review surface and persist the choice; conflicts with `--review-panel` |
| `-v`, `--verbose` | Print provider reasoning to stderr; in review and explain, print the raw response after the surface closes |
| `--provider <PROVIDER>` | Select a configured provider |
| `--set-small <MODEL>` | Set the small tier non-interactively; must precede `models` |
| `--set-normal <MODEL>` | Set the normal tier non-interactively; must precede `models` |
| `--set-thinking <MODEL>` | Set the thinking tier non-interactively; must precede `models` |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

A question with no tier or model flag uses the small tier. `--set-small`,
`--set-normal`, and `--set-thinking` save only when all three are present:

```sh
watn --set-small small-model --set-normal normal-model --set-thinking thinking-model models
```

### quicksetup options

| Option | Prefills |
|---|---|
| `--url <URL>` | Completion endpoint |
| `--key <KEY>` | Credential; a literal value or `${ENV_VAR}` |
| `--model <MODEL>` | The small, normal, and thinking model questions |
| `--model-small <MODEL>` | Small model |
| `--model-normal <MODEL>` | Normal model |
| `--model-thinking <MODEL>` | Thinking model |

Every question still runs; a flag only supplies the suggestion that Enter
accepts.

### explain options

| Option | Effect |
|---|---|
| `--review-panel`, `--no-review-panel` | Accepted for compatibility; the card always opens |
| `-v`, `--verbose` | Print the raw provider response to stderr after the card closes |

A top-level `-x` with `explain` is refused with exit 2. The command reaches watn
verbatim as one argument, through `-`, or from piped stdin; stdin loses at most
one trailing `\r\n` or `\n`.

When no provider or model is usable, `watn explain` starts quick setup or the
setup wizard, then exits 0 and asks for the command to be rerun. Without a
terminal it prints setup guidance and exits 1.

### completions shells

`bash`, `elvish`, `fish`, `powershell`, `zsh`. Any other value exits 2.

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Success; review cancelled; surface disabled with `D`; successful explain card closed |
| 1 | Configuration, I/O, or unknown-provider error; missing question; setup command or explain without a terminal |
| 2 | Authentication or API error; argument conflict; explain usage error; `-x` with explain |
| 3 | Network error |
| 130 | Interrupted with Ctrl-C |
| child status | `-x` runs `sh -c <command>` and exits with the child's code; a signal-terminated child reports 0 |

A failed explanation request still opens the card; the command exits with the
mapped code after the card closes. An unusable explanation response is not a
failure: the card shows `purpose-unavailable` and exits 0. Setup cancellation
via Escape exits 1; Ctrl-C exits 130.

## Output

| Stream | Content |
|---|---|
| stdout | The answer, or the accepted command |
| stderr | Metadata, reasoning under `-v`, warnings, and error messages |

The metadata line is written after a direct (non-review) response:

```text
<model> · <N> tok/s · $<cost> · <N.N>s · <fixed signature>
```

`tok/s` is completion tokens divided by elapsed seconds. The cost field is
omitted when no recorded price applies; when a price applies it uses four
decimal places, including `$0.0000` for a model with no usage report.

With `-x`, execution uses `sh -c`. When the review surface accepted the command,
that acceptance is the confirmation. Otherwise the prompt is
`Execute now? [Y/n/?]`: Enter, `y`, or `yes` executes, `?` opens the explanation
card when a controlling terminal is usable, and any other answer cancels. Ctrl-C
exits 130.

## Review surface

The surface opens when all of these hold:

- stdin is a terminal;
- stderr is a terminal;
- `/dev/tty` opens for read and write;
- `TERM` is not `dumb`;
- `[review] panel` is `true` (the default), or `--review-panel` was passed.

`--no-review-panel` disables it. If the surface is not eligible, the command
streams directly to stdout. A surface that is enabled and eligible opens on
`/dev/tty` even when stdout is piped. `--review-panel` and `--no-review-panel`
persist the choice; without a question they only persist and exit 0.

### Review keys

| Key | Action |
|---|---|
| `↑`, `↓`, `←`, `→` | Move the selected stage |
| `Enter` | Accept; the command goes to stdout |
| `d`, `?` | Toggle simple and detailed views |
| `e` | Open the command editor |
| `r` | Reject the candidate and open the model chooser; the catalog loads in the background |
| `c`, `Escape` | Cancel; nothing goes to stdout, exit 0 |
| `D` | Persist `[review] panel = false`, print the current command, exit 0 |

`D` also names `watn --review-panel` in a hint on stderr.

### Editor

| Key | Action |
|---|---|
| `Enter` | Commit the edited command; final acceptance is still required |
| `Escape` | Discard the edit |
| `←`, `→`, `Home`, `End` | Move the cursor |
| `Backspace`, `Delete` | Edit the buffer |
| Any other character | Insert it |

### Model chooser

| Key | Action |
|---|---|
| `1`, `2`, `3` | Regenerate with the small, normal, or thinking tier |
| Characters | Filter the catalog by model id |
| `↑`, `↓` | Move the highlight |
| `Enter` | Regenerate with the highlighted model |
| `Escape` | Return to the review without regenerating |

### Explanation card keys

| Key | Action |
|---|---|
| `↑`, `↓`, `←`, `→` | Move through the stages |
| `Enter`, `Escape` | Close the card |

The card requires stderr and `/dev/tty`; stdin may be a pipe. Explanation never
executes the command.

### Amount

The header shows `<amount> ¢` when the provider reported usage and a recorded
price applies to the request. Rules:

- one significant digit, never more than four decimal places: `0.1`, `0.02`,
  `0.0009`;
- whole cents from one cent upward;
- the price of the model the provider reported wins; the price of the requested
  model is the fallback;
- a regenerated candidate replaces the amount;
- with no recorded price or no usage report, the model name stands alone;
- when the header is too narrow for both, the model name is shortened first.

The post-request stderr line is independent: when a price applies it uses four
decimal places, including `$0.0000`.

### Unusable responses

A review response that is not valid JSON, is cut off, or has mismatched stages
is never shown as the command. watn recovers the provider-written command when
it can and shows `purpose-unavailable` with the reason in the card.

If no complete candidate can be recovered, watn prints
`review unavailable: no complete command candidate`, saves the raw response,
and exits 1.

The raw response is written to
`$XDG_STATE_HOME/watn/last-unusable-response.txt`, normally
`~/.local/state/watn/last-unusable-response.txt`, and the path is named on
stderr after the surface closes. `-v` prints the raw response to stderr after
the surface closes.

## Setup

### Quick setup

Questions, in order: completion endpoint, API key, small model, normal model,
thinking model, shells to integrate. An empty answer accepts the suggestion
shown. Shell selection lists Bash, Zsh, and Fish; typing a shell name toggles
it, and Enter confirms.

The configuration is validated locally and written only at the end. Ctrl-C
before the write leaves an existing configuration unchanged. Completing an
explicit quick setup writes the provider and tier settings to the configuration
file.

### Full wizard

Pages: Provider (`openrouter`, `openai`, `custom`), URL, API key, catalog
endpoint, small model, small reasoning, normal model, normal reasoning, thinking
model, thinking reasoning, shell completion, shell shortcut, final review.
Configuration is written at final confirmation. `watn provider`, `watn models`,
and `watn shell` open the matching page ranges.

Credential storage is either a literal value or an environment reference. The
environment form is stored as `${ENV_NAME}` and expanded at request time;
The setup wizard suggests `OPENROUTER_API_KEY`, `OPENAI_API_KEY`, or
`WATN_API_KEY` by endpoint.

Model selection searches the provider catalog from `catalog_endpoint` when set,
otherwise from `providers.<name>.endpoint`. When the catalog publishes prices,
the chosen model's input and output prices are recorded per million tokens.

Reasoning values are `off`, `low`, `minimal`, `medium`, `high`, or a
provider-specific non-empty value. An unset thinking tier uses `high`; an unset
small or normal tier sends no reasoning effort. `off` sends none.

## Shell integration

`watn shell` manages fenced blocks in the shell startup file:

| Shell | File |
|---|---|
| Bash | `~/.bashrc` |
| Zsh | `~/.zshrc` |
| Fish | `$XDG_CONFIG_HOME/fish/config.fish`, normally `~/.config/fish/config.fish` |

Two blocks are managed independently: `# >>> watn shell completion >>>` and
`# >>> watn shell shortcut >>>`. Reinstalling replaces the matching block.
Shells that are on `PATH` or already contain a managed block are preselected.
When a feature is enabled, deselected shells have its block removed.

The Ctrl-W widget reads the current buffer, strips one leading `# `, and calls
`watn -- "$question"`. On success it records the question in shell history as
`# <question>` and replaces the buffer with the accepted command without
evaluating it. An empty buffer or a non-zero watn exit leaves the buffer
unchanged.

`watn completions <SHELL>` prints a script to stdout; the completion block
sources it.

## Configuration

Default path: `$XDG_CONFIG_HOME/watn/config.toml`, normally
`~/.config/watn/config.toml`. A missing file uses built-in defaults. The first
interactive question with no usable provider or model starts quick setup when no
config file exists, otherwise the full setup wizard.

Provider selection: `--provider` → `WATN_PROVIDER` → `defaults.provider` →
`openrouter`. Model selection: `--model` → the selected tier's model →
`defaults.model` (which `WATN_MODEL` overrides) → the provider default. Other
settings come from the config file only.

```toml
[defaults]
provider = "openrouter"
model = "~deepseek/deepseek-v4-flash-latest"

[providers.openrouter]
endpoint = "https://openrouter.ai/api/v1"
api_key = "${OPENROUTER_API_KEY}"
default_model = "~deepseek/deepseek-v4-flash-latest"
catalog_endpoint = "https://openrouter.ai/api/v1"

[tiers]
small = "~anthropic/claude-haiku-latest:nitro"
normal = "deepseek/deepseek-v4-pro"
thinking = "z-ai/glm-5.2"

[tiers.reasoning]
small = "off"
normal = "low"
thinking = "high"

[pricing."z-ai/glm-5.2"]
input = 1.10
output = 4.40

[review]
panel = true
```

| Key | Meaning |
|---|---|
| `defaults.provider` | Default provider name |
| `defaults.model` | Fallback model for every tier |
| `providers.<name>.endpoint` | OpenAI-compatible base URL |
| `providers.<name>.api_key` | Literal value or `${ENV_NAME}` reference |
| `providers.<name>.default_model` | Fallback model for a custom provider |
| `providers.<name>.catalog_endpoint` | Model catalog base URL; defaults to the provider endpoint |
| `tiers.small`, `tiers.normal`, `tiers.thinking` | Tier models |
| `tiers.reasoning.small`, `.normal`, `.thinking` | Reasoning effort per tier |
| `pricing.<model>.input`, `.output` | USD per million tokens |
| `review.panel` | Review surface default; `true` when absent |

### API key resolution

1. `providers.<name>.api_key` when set, after `${ENV_NAME}` expansion.
2. `OPENROUTER_API_KEY` for `openrouter`, or `WATN_<NAME>_API_KEY` for other
   providers.
3. `WATN_API_KEY`.

A missing or empty expanded value exits 2 with an authentication error. The
config file is saved with mode `0600` on Unix; loading a world-readable file
warns.

### Environment variables

| Variable | Effect |
|---|---|
| `WATN_PROVIDER` | Overrides `defaults.provider` |
| `WATN_MODEL` | Overrides `defaults.model` |
| `WATN_API_KEY` | Fallback API key |
| `WATN_<PROVIDER>_API_KEY` | Provider-specific fallback key |
| `OPENROUTER_API_KEY` | OpenRouter fallback key |
| `XDG_CONFIG_HOME` | Config directory base |
| `XDG_STATE_HOME` | State directory base for the unusable-response file |
| `HOME` | Fallback base for config, state, and shell files |
| `SHELL` | Shell named to the provider prompts |
| `NO_COLOR` | Disables card color |
| `TERM=dumb` | Disables the review and explanation surfaces |

## Model resolution

1. `--model <MODEL>`.
2. `tiers.<selected tier>`, where the selected tier is small by default.
3. `defaults.model`.
4. Provider default: `gpt-4o-mini` for `openai`,
   `~deepseek/deepseek-v4-flash-latest` for `openrouter`, otherwise
   `providers.<name>.default_model`.

When no tier model or `defaults.model` applies, a custom provider without
`default_model` exits 1 with a configuration error.

## Files

| Path | Content |
|---|---|
| `$XDG_CONFIG_HOME/watn/config.toml` | Configuration |
| `$XDG_STATE_HOME/watn/last-unusable-response.txt` | Most recent unusable provider response |
| `~/.bashrc`, `~/.zshrc`, `~/.config/fish/config.fish` | Managed shell blocks |
