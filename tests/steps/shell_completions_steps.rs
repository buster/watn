use cucumber::{given, then, when};
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::{MockServerWrap, WatnWorld};

#[when(
    regex = r##"^I run `watn completions (bash|elvish|fish|powershell|zsh)` as a regular subprocess$"##
)]
fn regular_completion(world: &mut WatnWorld, shell: String) {
    run_completion(world, &shell);
}

#[when("I run `watn completions nushell` as a regular subprocess")]
fn unsupported_completion(world: &mut WatnWorld) {
    run_completion(world, "nushell");
}

#[when("I run `watn -- completions find files` as a regular subprocess")]
fn reserved_completion_token(world: &mut WatnWorld) {
    super::ensure_test_env(world);
    run_args(world, &["--", "completions", "find", "files"]);
}

#[when("I run `watn \"completions find files\"` as a regular subprocess")]
fn quoted_completion_token(world: &mut WatnWorld) {
    run_args(world, &["completions find files"]);
}

#[given("no provider configuration exists in an isolated XDG config directory")]
fn no_provider_config(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create isolated XDG directory");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        temp.path().display().to_string(),
    );
    world.temp_dir = Some(temp);
}

#[given("the no-config snapshot records that the isolated XDG config file is absent")]
fn no_config_snapshot(world: &mut WatnWorld) {
    assert!(!config_path(world).exists());
}

#[given("an isolated provider-request sentinel is installed")]
fn provider_sentinel(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let mock = server
        .mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/chat/completions");
            then.status(200).body("unexpected provider request");
        })
        .id;
    let endpoint = server.url("");
    world.mock_server = MockServerWrap(Some(server), Some(mock));
    world
        .env_vars
        .insert("WATN_PROVIDER".to_string(), "openai".to_string());
    world.env_vars.insert(
        "WATN_OPENAI_API_KEY".to_string(),
        "sentinel-key".to_string(),
    );
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), endpoint);
}

#[given("the provider-request sentinel snapshot records zero requests")]
fn sentinel_snapshot(world: &mut WatnWorld) {
    assert_eq!(sentinel_hits(world), 0);
}

#[then("stdout should contain Bash completion syntax")]
fn bash_syntax(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("complete -F"));
}

#[then("stdout should not contain Bash completion syntax")]
fn no_bash_syntax(world: &mut WatnWorld) {
    assert!(!world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("complete -F"));
}

#[then("stdout should contain all current root options and subcommands")]
fn all_root_values(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("completion stdout");
    for value in [
        "--small",
        "--normal",
        "--thinking",
        "--model",
        "--execute",
        "--verbose",
        "--provider",
        "--set-small",
        "--set-normal",
        "--set-thinking",
        "--help",
        "--version",
        "setup",
        "models",
        "provider",
        "completions",
    ] {
        assert!(
            output.contains(value),
            "missing root completion value {value}"
        );
    }
}

#[then("stdout should contain Zsh completion syntax")]
fn zsh_syntax(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("#compdef"));
}

#[then("stdout should contain Fish completion syntax")]
fn fish_syntax(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("complete -c watn"));
}

#[then("stdout should contain Elvish completion syntax")]
fn elvish_syntax(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("edit:completion:arg-completer"));
}

#[then("stdout should contain PowerShell completion syntax")]
fn powershell_syntax(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("Register-ArgumentCompleter"));
}

#[then("stdout should contain only the completion script")]
fn stdout_script_only(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(!output.contains("Setup complete"));
    assert!(!output.contains("warning:"));
    assert!(!output.trim().is_empty());
}

#[then("stderr should be empty")]
fn stderr_empty(world: &mut WatnWorld) {
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .is_empty());
}

#[then(regex = r##"^stdout should contain the authoritative root options:$"##)]
fn root_options(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let output = world.output.as_deref().expect("completion stdout");
    for row in &step.table().expect("root option table").rows {
        let option = &row[0];
        let present = output.contains(option)
            || option
                .strip_prefix("--")
                .is_some_and(|long| output.contains(&format!("-l {long}")))
            || match option.as_str() {
                "-1" => output.contains("1/small"),
                "-2" => output.contains("2/normal"),
                "-3" => output.contains("3/thinking"),
                "-x" => output.contains("-s x"),
                "-v" => output.contains("-s v"),
                _ => false,
            };
        assert!(present, "missing root option {}", option);
    }
}

#[then("stdout should contain the authoritative root positional arguments:")]
fn root_positionals(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let output = world.output.as_deref().expect("completion stdout");
    for row in &step.table().expect("root positional table").rows {
        assert!(
            output.contains(&row[0]),
            "missing root positional {}",
            row[0]
        );
    }
}

#[then("stdout should contain the authoritative root subcommands:")]
fn root_subcommands(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let output = world.output.as_deref().expect("completion stdout");
    for row in &step.table().expect("root subcommand table").rows {
        assert!(
            output.contains(&row[0]),
            "missing root subcommand {}",
            row[0]
        );
    }
}

#[then("stdout should contain the closed shell-selector value suggestions:")]
fn selector_values(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let output = world.output.as_deref().expect("completion stdout");
    for row in &step.table().expect("selector table").rows {
        assert!(output.contains(&row[0]), "missing shell value {}", row[0]);
    }
}

#[then("a second bash generation should be byte-for-byte identical")]
fn bash_deterministic(world: &mut WatnWorld) {
    deterministic_again(world, "bash");
}

#[then("a second zsh generation should be byte-for-byte identical")]
fn zsh_deterministic(world: &mut WatnWorld) {
    deterministic_again(world, "zsh");
}

#[then("a second fish generation should be byte-for-byte identical")]
fn fish_deterministic(world: &mut WatnWorld) {
    deterministic_again(world, "fish");
}

#[then("a second elvish generation should be byte-for-byte identical")]
fn elvish_deterministic(world: &mut WatnWorld) {
    deterministic_again(world, "elvish");
}

#[then("a second powershell generation should be byte-for-byte identical")]
fn powershell_deterministic(world: &mut WatnWorld) {
    deterministic_again(world, "powershell");
}

#[then(
    regex = r##"^the generated script should be accepted by (Bash|Elvish|Fish|PowerShell|Zsh)$"##
)]
fn shell_parser(world: &mut WatnWorld, shell: String) {
    let output = world.output.as_deref().expect("completion stdout");
    let commands: &[&str] = match shell.as_str() {
        "PowerShell" => &["pwsh", "powershell"],
        _ => &[&shell.to_ascii_lowercase()],
    };
    let command = commands
        .iter()
        .find(|candidate| Command::new(candidate).arg("--version").output().is_ok());
    let Some(command) = command else {
        eprintln!(
            "environment limitation: {shell} executable is unavailable; generated script was not syntax-checked"
        );
        assert!(
            !output.trim().is_empty(),
            "{shell} is unavailable and output is empty"
        );
        return;
    };
    let mut parser = Command::new(command);
    if shell == "PowerShell" {
        parser.args(["-NoLogo", "-NoProfile", "-Command", "-"]);
    } else {
        parser.arg("-n");
    }
    let mut child = parser
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start shell parser");
    child
        .stdin
        .take()
        .expect("shell parser stdin")
        .write_all(output.as_bytes())
        .expect("write shell script");
    let result = child.wait_with_output().expect("wait for shell parser");
    assert!(
        result.status.success(),
        "{shell} rejected script: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[then("the isolated XDG config file should remain absent after the command")]
fn config_remains_absent(world: &mut WatnWorld) {
    assert!(!config_path(world).exists());
}

#[then("the provider-request sentinel should remain at zero requests after the command")]
fn sentinel_remains_zero(world: &mut WatnWorld) {
    assert_eq!(sentinel_hits(world), 0);
}

#[then("no file should be written in the isolated XDG config directory")]
fn no_config_files(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("isolated XDG directory");
    assert!(
        std::fs::read_dir(directory.path())
            .expect("read XDG directory")
            .next()
            .is_none(),
        "completion generation wrote a file in the isolated XDG directory"
    );
}

#[then("successful completion stdout should contain only the generated script")]
fn successful_script_only(world: &mut WatnWorld) {
    stdout_script_only(world);
}

#[when("I run `watn completions --help`")]
fn completion_help(world: &mut WatnWorld) {
    run_args(world, &["completions", "--help"]);
}

#[then("stdout should mention bash, elvish, fish, powershell, and zsh")]
fn help_shells(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("help stdout");
    for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
        assert!(
            output.contains(shell),
            "missing shell in help output: {shell}"
        );
    }
}

#[then(expr = "completion help stdout should contain {string}")]
fn help_usage(world: &mut WatnWorld, text: String) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(
        output.contains(&text),
        "unexpected completion help: {output:?}"
    );
}

#[then("stdout should explain that the generated script is written to stdout for the caller to install or source")]
fn help_purpose(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("help stdout");
    assert!(output.contains("stdout") && output.contains("source"));
}

#[then("stdout should document that only bash, elvish, fish, powershell, and zsh are supported shell values")]
fn help_selector_contract(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("help stdout");
    for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
        assert!(
            output.contains(shell),
            "missing shell in help output: {shell}"
        );
    }
}

#[then("stderr should contain the exact unsupported-shell contract:")]
fn unsupported_contract(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let stderr = world.stderr_output.as_deref().expect("completion stderr");
    for row in &step.table().expect("unsupported shell table").rows {
        assert!(
            stderr.contains(&row[0]),
            "missing unsupported-shell contract"
        );
    }
}

#[then(expr = "stderr should identify {string} as the rejected value")]
fn rejected_shell(world: &mut WatnWorld, value: String) {
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains(&value));
}

fn deterministic_again(world: &mut WatnWorld, shell: &str) {
    let first = world.output.clone().expect("first completion output");
    run_completion(world, shell);
    assert_eq!(world.output.as_deref(), Some(first.as_str()));
}

fn run_completion(world: &mut WatnWorld, shell: &str) {
    let args = ["completions", shell];
    run_args(world, &args);
}

fn run_args(world: &mut WatnWorld, args: &[&str]) {
    let binary = super::binary_from_env("WATN_TEST_SUPPORT_DEBUG_BIN");
    let mut command = Command::new(binary);
    command.args(args);
    command.env_clear();
    command.env("PATH", std::env::var("PATH").expect("PATH"));
    super::apply_env(world, &mut command);
    let output = command.output().expect("run completion command");
    world.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr_output = Some(String::from_utf8_lossy(&output.stderr).to_string());
    world.exit_status = output.status.code();
}

fn config_path(world: &WatnWorld) -> std::path::PathBuf {
    std::path::PathBuf::from(
        world
            .env_vars
            .get("XDG_CONFIG_HOME")
            .expect("XDG_CONFIG_HOME"),
    )
    .join("watn/config.toml")
}

fn sentinel_hits(world: &WatnWorld) -> usize {
    let id = world.mock_server.1.expect("sentinel mock");
    httpmock::Mock::new(id, world.mock_server.0.as_ref().expect("sentinel server")).hits()
}

fn completion_environment(world: &WatnWorld) -> watn::shell_shortcut::ShellEnvironment {
    let temp = world.temp_dir.as_ref().expect("completion temp dir");
    watn::shell_shortcut::ShellEnvironment {
        home: temp.path().join("home"),
        xdg_config_home: Some(temp.path().join("home/.config")),
        shell: Some("/bin/bash".to_string()),
    }
}

#[given("isolated Bash, Zsh, and Fish completion targets")]
fn isolated_completion_targets(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create completion temp dir");
    let home = temp.path().join("home");
    let fish_dir = home.join(".config/fish");
    std::fs::create_dir_all(&fish_dir).expect("create completion Fish directory");
    let targets = HashMap::from([
        ("bash".to_string(), home.join(".bashrc")),
        ("zsh".to_string(), home.join(".zshrc")),
        ("fish".to_string(), fish_dir.join("config.fish")),
    ]);
    for path in targets.values() {
        std::fs::write(path, b"# existing shell content\n").expect("write completion target");
    }
    world.temp_dir = Some(temp);
    world.completion_targets = targets;
}

#[when("I install shell completion for Bash, Zsh, and Fish")]
fn install_completion_for_all_shells(world: &mut WatnWorld) {
    let environment = completion_environment(world);
    let report = watn::shell_completion::install_with_environment(
        &watn::shell_shortcut::Shell::ALL,
        &environment,
    );
    world.completion_error = report.aggregate_error().map(|error| error.to_string());
    world.completion_output = Some(
        report
            .results
            .iter()
            .map(|result| {
                format!(
                    "{} {} {}",
                    result.shell.lowercase_name(),
                    result
                        .path
                        .as_deref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_default(),
                    result.reload.as_deref().unwrap_or(&result.message)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

fn completion_target_content(world: &WatnWorld, shell: &str) -> String {
    std::fs::read_to_string(world.completion_targets.get(shell).unwrap())
        .expect("read completion target")
}

#[then("the Bash configuration should contain the Bash completion loader")]
fn bash_completion_loader(world: &mut WatnWorld) {
    let content = completion_target_content(world, "bash");
    assert!(content.contains("watn shell completion"));
    assert!(content.contains("watn completions bash"));
}

#[then("the Zsh configuration should contain the Zsh completion loader")]
fn zsh_completion_loader(world: &mut WatnWorld) {
    let content = completion_target_content(world, "zsh");
    assert!(content.contains("watn shell completion"));
    assert!(content.contains("watn completions zsh"));
    assert!(content.contains("compinit"));
}

#[then("the Fish configuration should contain the Fish completion loader")]
fn fish_completion_loader(world: &mut WatnWorld) {
    let content = completion_target_content(world, "fish");
    assert!(content.contains("watn shell completion"));
    assert!(content.contains("watn completions fish"));
    assert!(content.contains("| source"));
}

#[then("completion installation should report a reload instruction for every shell")]
fn completion_reload_instructions(world: &mut WatnWorld) {
    assert!(world.completion_error.is_none());
    let output = world.completion_output.as_deref().unwrap_or_default();
    for shell in ["bash", "zsh", "fish"] {
        assert!(output.contains(shell), "missing {shell} completion report");
    }
    assert_eq!(output.matches("Run: source").count(), 3);
}
