use cucumber::{given, then, when};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::WatnWorld;

fn shortcut_environment(world: &WatnWorld) -> watn::shell_shortcut::ShellEnvironment {
    let temp = world.temp_dir.as_ref().expect("shortcut temp dir");
    watn::shell_shortcut::ShellEnvironment {
        home: temp.path().join("home"),
        xdg_config_home: Some(
            world
                .pending_config
                .get("shortcut_xdg")
                .map(PathBuf::from)
                .unwrap_or_else(|| temp.path().join("home").join(".config")),
        ),
        shell: Some("/bin/bash".to_string()),
    }
}

#[when("I press Enter to accept the default decline on the optional shortcut question")]
fn default_decline(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(&[], &environment);
    assert!(
        report.results.is_empty(),
        "default decline installed a target"
    );
}

#[given("Bash, Zsh, and Fish configuration files with existing user content")]
fn shortcut_files(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    let fish_config = home.join(".config/fish");
    std::fs::create_dir_all(&fish_config).expect("create Fish config directory");
    let files = HashMap::from([
        ("bash".to_string(), home.join(".bashrc")),
        ("zsh".to_string(), home.join(".zshrc")),
        ("fish".to_string(), fish_config.join("config.fish")),
    ]);
    for path in files.values() {
        std::fs::write(path, b"# existing user content\n").expect("write shell fixture");
    }
    world.temp_dir = Some(temp);
    world.shortcut_targets = files;
}

#[given("a snapshot of every shell configuration file")]
fn shortcut_snapshot(world: &mut WatnWorld) {
    world.shortcut_snapshots = world
        .shortcut_targets
        .iter()
        .map(|(shell, path)| {
            (
                shell.clone(),
                std::fs::read(path).expect("read shell fixture snapshot"),
            )
        })
        .collect::<HashMap<String, Vec<u8>>>();
}

#[then("every shell configuration file should match its snapshot byte-for-byte")]
fn shortcut_unchanged(world: &mut WatnWorld) {
    for (shell, path) in &world.shortcut_targets {
        let current = std::fs::read(path).expect("read shell fixture");
        assert_eq!(
            world.shortcut_snapshots.get(shell),
            Some(&current),
            "{shell} configuration changed"
        );
    }
}

#[when("I answer `y` to the optional shortcut question")]
fn enable_shortcut(world: &mut WatnWorld) {
    world.shortcut_shells.clear();
}

#[when("I select no shells in the shortcut multi-select")]
fn select_no_shells(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(&[], &environment);
    assert!(
        report.results.is_empty(),
        "empty selection installed a target"
    );
}

#[given("`SHELL` is \"/usr/local/bin/bash\"")]
fn shell_environment(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    std::fs::create_dir_all(temp.path().join("home")).expect("create shortcut home");
    world.temp_dir = Some(temp);
    world.pending_config.insert(
        "shortcut_shell".to_string(),
        "/usr/local/bin/bash".to_string(),
    );
}

#[given("Zsh and Fish target files already exist")]
fn existing_other_shell_targets(world: &mut WatnWorld) {
    let temp = world.temp_dir.as_ref().expect("shortcut temp dir");
    let home = temp.path().join("home");
    let fish_dir = home.join(".config/fish");
    std::fs::create_dir_all(&fish_dir).expect("create Fish config directory");
    let targets = HashMap::from([
        ("zsh".to_string(), home.join(".zshrc")),
        ("fish".to_string(), fish_dir.join("config.fish")),
    ]);
    for path in targets.values() {
        std::fs::write(path, b"# existing target\n").expect("write shell target");
    }
    world.shortcut_targets = targets;
}

#[when("the shell shortcut choices are shown")]
fn show_shortcut_choices(world: &mut WatnWorld) {
    let temp = world.temp_dir.as_ref().expect("shortcut temp dir");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home: temp.path().join("home"),
        xdg_config_home: Some(temp.path().join("home/.config")),
        shell: world.pending_config.get("shortcut_shell").cloned(),
    };
    world.shortcut_shells = environment
        .detected_shells()
        .into_iter()
        .map(|shell| shell.lowercase_name().to_string())
        .collect();
}

#[then("Bash should be preselected")]
fn bash_preselected(world: &mut WatnWorld) {
    assert_eq!(world.shortcut_shells, vec!["bash"]);
}

#[then("Zsh and Fish should remain available and unselected")]
fn other_shells_unselected(world: &mut WatnWorld) {
    assert!(!world.shortcut_shells.contains(&"zsh".to_string()));
    assert!(!world.shortcut_shells.contains(&"fish".to_string()));
    assert_eq!(world.shortcut_targets.len(), 2);
}

#[when("I select Zsh and Fish as well")]
fn select_other_shells(world: &mut WatnWorld) {
    world
        .shortcut_shells
        .extend(["zsh".to_string(), "fish".to_string()]);
}

#[then("Bash, Zsh, and Fish should all be selected")]
fn all_shells_selected(world: &mut WatnWorld) {
    assert_eq!(
        world.shortcut_shells,
        vec!["bash", "zsh", "fish"],
        "all supported shells should be selectable"
    );
}

#[given("Bash, Zsh, and Fish configuration paths in an isolated home")]
fn isolated_shell_paths(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    let fish_dir = home.join(".config/fish");
    std::fs::create_dir_all(&fish_dir).expect("create Fish config directory");
    world.shortcut_targets = HashMap::from([
        ("bash".to_string(), home.join(".bashrc")),
        ("zsh".to_string(), home.join(".zshrc")),
        ("fish".to_string(), fish_dir.join("config.fish")),
    ]);
    world.temp_dir = Some(temp);
}

#[when("I install the shell shortcut for Bash, Zsh, and Fish")]
fn install_all_shells(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(
        &watn::shell_shortcut::Shell::ALL,
        &environment,
    );
    world.shortcut_error = report.aggregate_error().map(|error| error.to_string());
    world.shortcut_shells = report
        .successes()
        .map(|result| result.shell.lowercase_name().to_string())
        .collect();
    world.shortcut_output = Some(
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

#[then("the Bash configuration should contain the Bash widget and Ctrl-W binding")]
fn bash_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    assert!(content.contains("READLINE_LINE"));
    assert!(content.contains("READLINE_POINT"));
    assert!(content.contains("bind -x"));
    assert!(content.contains("\\C-w"));
}

#[then("the Zsh configuration should contain the ZLE widget and Ctrl-W binding")]
fn zsh_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("zsh").unwrap())
        .expect("read Zsh target");
    assert!(content.contains("$BUFFER"));
    assert!(content.contains("CURSOR"));
    assert!(content.contains("zle -N"));
    assert!(content.contains("bindkey '^W'"));
}

#[then("the Fish configuration should contain the Fish widget and Ctrl-W binding")]
fn fish_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("fish").unwrap())
        .expect("read Fish target");
    assert!(content.contains("commandline"));
    assert!(content.contains("commandline -r --"));
    assert!(content.contains("commandline -f repaint"));
    assert!(content.contains("bind \\cw"));
}

#[then("setup should report a success for every selected shell")]
fn all_shells_success(world: &mut WatnWorld) {
    assert_eq!(world.shortcut_shells, vec!["bash", "zsh", "fish"]);
}

#[then("each selected shell should have its own reload instruction")]
fn reload_instructions(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    for shell in ["bash", "zsh", "fish"] {
        assert!(output.contains(shell), "missing {shell} report");
        assert!(output.contains("Run: source"), "missing reload instruction");
    }
}

#[given("writable Bash and Fish targets and a Zsh target that cannot be written")]
fn partial_failure_targets(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    let fish_dir = home.join(".config/fish");
    std::fs::create_dir_all(&fish_dir).expect("create Fish config directory");
    let zsh_path = home.join(".zshrc");
    std::fs::create_dir_all(&zsh_path).expect("create unwritable Zsh target directory");
    let targets = HashMap::from([
        ("bash".to_string(), home.join(".bashrc")),
        ("zsh".to_string(), zsh_path),
        ("fish".to_string(), fish_dir.join("config.fish")),
    ]);
    std::fs::write(targets.get("bash").unwrap(), b"# Bash user content\n")
        .expect("write Bash target");
    std::fs::write(targets.get("fish").unwrap(), b"# Fish user content\n")
        .expect("write Fish target");
    world.temp_dir = Some(temp);
    world.shortcut_targets = targets;
}

#[given("the Bash and Fish targets have existing user content")]
fn partial_failure_content(world: &mut WatnWorld) {
    world.shortcut_snapshots = world
        .shortcut_targets
        .iter()
        .filter_map(|(shell, path)| {
            std::fs::read(path)
                .ok()
                .map(|content| (shell.clone(), content))
        })
        .collect();
}

#[then("the Bash configuration should contain one watn shell shortcut block")]
fn bash_one_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    assert_eq!(
        content.matches(watn::shell_shortcut::OPEN_MARKER).count(),
        1
    );
    assert_eq!(
        content.matches(watn::shell_shortcut::CLOSE_MARKER).count(),
        1
    );
}

#[then("the Fish configuration should contain one watn shell shortcut block")]
fn fish_one_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("fish").unwrap())
        .expect("read Fish target");
    assert_eq!(
        content.matches(watn::shell_shortcut::OPEN_MARKER).count(),
        1
    );
    assert_eq!(
        content.matches(watn::shell_shortcut::CLOSE_MARKER).count(),
        1
    );
}

#[then("the Bash and Fish user content should remain unchanged")]
fn partial_content_unchanged(world: &mut WatnWorld) {
    for shell in ["bash", "fish"] {
        let current = std::fs::read(world.shortcut_targets.get(shell).unwrap())
            .expect("read selected target");
        let original = world.shortcut_snapshots.get(shell).unwrap();
        assert!(
            current.starts_with(original),
            "{shell} unrelated user content was not preserved"
        );
    }
}

#[then("the Zsh configuration should remain unchanged")]
fn zsh_unchanged(world: &mut WatnWorld) {
    assert!(world.shortcut_targets.get("zsh").unwrap().is_dir());
}

#[then("setup should report success for Bash and Fish")]
fn partial_success_report(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    assert!(output.contains("bash"));
    assert!(output.contains("fish"));
    assert!(!world.shortcut_shells.is_empty());
}

#[then("setup should report the Zsh target path and write failure reason")]
fn zsh_failure_report(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let path = world.shortcut_targets.get("zsh").unwrap();
    assert!(output.contains(&path.display().to_string()));
    assert!(output.contains("target is a directory"));
}

#[then("setup should report an aggregate shell installation failure")]
fn aggregate_failure(world: &mut WatnWorld) {
    assert!(world
        .shortcut_error
        .as_deref()
        .unwrap_or_default()
        .contains("shell shortcut installation failed"));
}

#[given("missing Bash and Fish configuration parent directories")]
fn missing_parent_dirs(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    let xdg = temp.path().join("xdg");
    world
        .pending_config
        .insert("shortcut_xdg".to_string(), xdg.display().to_string());
    world.shortcut_targets = HashMap::from([
        ("bash".to_string(), home.join(".bashrc")),
        ("fish".to_string(), xdg.join("fish/config.fish")),
    ]);
    world.temp_dir = Some(temp);
}

#[when("I install the shell shortcut for Fish")]
fn install_fish(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Fish],
        &environment,
    );
    assert!(report.is_success(), "installation report: {report:?}");
}

#[then("the Fish configuration parent directory should exist")]
fn fish_parent_exists(world: &mut WatnWorld) {
    assert!(world
        .shortcut_targets
        .get("fish")
        .unwrap()
        .parent()
        .unwrap()
        .is_dir());
}

#[then("the Bash configuration parent directory should remain absent")]
fn bash_parent_absent(world: &mut WatnWorld) {
    assert!(!world
        .shortcut_targets
        .get("bash")
        .unwrap()
        .parent()
        .unwrap()
        .exists());
}

#[given("a Bash configuration containing unrelated user content and one watn shell shortcut block")]
fn existing_generated_bash(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).expect("create shortcut home");
    let path = home.join(".bashrc");
    let content = format!(
        "# before user content\n{}# after user content\n",
        watn::shell_shortcut::Shell::Bash.generated_block()
    );
    std::fs::write(&path, content).expect("write generated Bash fixture");
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("bash".to_string(), path)]);
}

#[when("I install the Bash shell shortcut again")]
fn reinstall_bash(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    assert!(report.is_success(), "installation report: {report:?}");
}

#[then("the Bash configuration should contain exactly one watn shell shortcut block")]
fn exactly_one_bash_block(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    assert_eq!(
        content.matches(watn::shell_shortcut::OPEN_MARKER).count(),
        1
    );
    assert_eq!(
        content.matches(watn::shell_shortcut::CLOSE_MARKER).count(),
        1
    );
}

#[then("the unrelated user content should remain unchanged")]
fn unrelated_content(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    assert!(content.contains("# before user content"));
    assert!(content.contains("# after user content"));
}

#[given("a Bash shortcut target that is a directory and cannot be written")]
fn unwritable_bash_target(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    let path = home.join(".bashrc");
    std::fs::create_dir_all(&path).expect("create directory target");
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("bash".to_string(), path)]);
}

#[given("a snapshot of the Bash target failure state")]
fn unwritable_bash_snapshot(world: &mut WatnWorld) {
    assert!(world.shortcut_targets.get("bash").unwrap().is_dir());
}

#[when("I install the Bash shell shortcut")]
fn install_bash(world: &mut WatnWorld) {
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    world.shortcut_error = report.aggregate_error().map(|error| error.to_string());
    world.shortcut_output = Some(
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
                    result.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

#[then("setup should report that the Bash target could not be written")]
fn bash_write_failure(world: &mut WatnWorld) {
    assert!(world
        .shortcut_error
        .as_deref()
        .unwrap_or_default()
        .contains("shell shortcut installation failed"));
}

#[then("the error should identify the write failure reason")]
fn write_failure_reason(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let path = world.shortcut_targets.get("bash").unwrap();
    assert!(output.contains(&path.display().to_string()));
    assert!(output.contains("target is a directory"));
}

#[then("the Bash target should remain a directory")]
fn bash_target_directory(world: &mut WatnWorld) {
    assert!(world.shortcut_targets.get("bash").unwrap().is_dir());
}

#[given("a Bash shortcut target that is a symbolic link to a regular file")]
fn symlinked_bash_target(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create symlink target temp dir");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).expect("create symlink target home");
    let real_target = home.join(".bashrc.real");
    let link = home.join(".bashrc");
    std::fs::write(&real_target, b"# existing Bash content\n").expect("write symlink target");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_target, &link).expect("create Bash symlink");
    world.pending_config.insert(
        "shortcut_real_target".to_string(),
        real_target.display().to_string(),
    );
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("bash".to_string(), link)]);
}

#[then("the Bash shortcut symlink should remain intact")]
fn bash_symlink_remains(world: &mut WatnWorld) {
    let link = world.shortcut_targets.get("bash").unwrap();
    assert!(
        std::fs::symlink_metadata(link)
            .expect("read Bash symlink metadata")
            .file_type()
            .is_symlink(),
        "Bash target was replaced instead of its resolved file"
    );
}

#[then("the resolved Bash shortcut target should contain the Bash widget")]
fn resolved_bash_target_contains_widget(world: &mut WatnWorld) {
    let target = PathBuf::from(
        world
            .pending_config
            .get("shortcut_real_target")
            .expect("resolved Bash target path"),
    );
    let content = std::fs::read_to_string(target).expect("read resolved Bash target");
    assert!(content.contains("READLINE_LINE"));
    assert!(content.contains("bind -x"));
}

#[given("isolated Bash targets with these malformed marker layouts:")]
fn malformed_bash_targets(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let table = &step.table().expect("malformed layout table").rows;
    let open = watn::shell_shortcut::OPEN_MARKER;
    let close = watn::shell_shortcut::CLOSE_MARKER;
    let block = watn::shell_shortcut::Shell::Bash.generated_block();
    for (index, row) in table.iter().enumerate() {
        let layout = row.first().expect("malformed layout value");
        if layout == "layout" {
            continue;
        }
        let content = match layout.as_str() {
            "two complete watn shell shortcut blocks" => format!("{block}{block}"),
            "two opening markers and one closing marker" => {
                format!("{open}\n{open}\n{close}\n")
            }
            "one opening marker and two closing markers" => {
                format!("{open}\n{close}\n{close}\n")
            }
            "an opening marker without a closing marker" => format!("{open}\n"),
            "a closing marker without an opening marker" => format!("{close}\n"),
            "a closing marker before an opening marker" => format!("{close}\n{open}\n"),
            other => panic!("unknown malformed layout: {other}"),
        };
        let target_home = temp.path().join(format!("layout-{index}"));
        std::fs::create_dir_all(&target_home).expect("create malformed target home");
        let path = target_home.join(".bashrc");
        std::fs::write(&path, content).expect("write malformed Bash target");
        world
            .shortcut_targets
            .insert(format!("bash-{index}"), path.clone());
        world.shortcut_snapshots.insert(
            format!("bash-{index}"),
            std::fs::read(path).expect("snapshot target"),
        );
    }
    world.temp_dir = Some(temp);
}

#[when("I install the Bash shell shortcut for every malformed layout")]
fn install_malformed_bash_targets(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("malformed target temp dir");
    let mut messages = Vec::new();
    for path in world.shortcut_targets.values() {
        let environment = watn::shell_shortcut::ShellEnvironment {
            home: root.path().to_path_buf(),
            xdg_config_home: None,
            shell: Some("/bin/bash".to_string()),
        };
        let target_home = path.parent().unwrap().to_path_buf();
        let environment = watn::shell_shortcut::ShellEnvironment {
            home: target_home,
            ..environment
        };
        let report = watn::shell_shortcut::install_with_environment(
            &[watn::shell_shortcut::Shell::Bash],
            &environment,
        );
        messages.extend(report.results.into_iter().map(|result| result.message));
    }
    world.shortcut_error = Some(messages.join("; "));
}

#[then("setup should report malformed watn shell shortcut markers")]
fn malformed_report(world: &mut WatnWorld) {
    assert!(world
        .shortcut_error
        .as_deref()
        .unwrap_or_default()
        .contains("malformed watn shell shortcut markers"));
}

#[then("every malformed Bash target should match its snapshot byte-for-byte")]
fn malformed_unchanged(world: &mut WatnWorld) {
    for (key, path) in &world.shortcut_targets {
        assert_eq!(
            std::fs::read(path).expect("read malformed Bash target"),
            *world.shortcut_snapshots.get(key).unwrap(),
            "malformed target {key} changed"
        );
    }
}

#[given(regex = r##"^an installed Bash shortcut and a fake watn that returns \"([^\"]*)\"$"##)]
fn widget_success_fixture(world: &mut WatnWorld, output: String) {
    let temp = tempfile::tempdir().expect("create widget temp dir");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).expect("create widget home");
    let target = home.join(".bashrc");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home: home.clone(),
        xdg_config_home: None,
        shell: Some("/bin/bash".to_string()),
    };
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    assert!(report.is_success(), "widget fixture report: {report:?}");
    let fake_log = temp
        .path()
        .join("watn-invocations.log")
        .display()
        .to_string();
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("bash".to_string(), target)]);
    world
        .pending_config
        .insert("fake_output".to_string(), output.replace("\\n", "\n"));
    world
        .pending_config
        .insert("fake_status".to_string(), "0".to_string());
    world
        .pending_config
        .insert("fake_log".to_string(), fake_log);
    let _ = std::fs::remove_file("/tmp/watn-shortcut-should-not-run");
}

#[when(regex = r##"^I run the Bash widget with current input \"([^\"]*)\"$"##)]
pub(crate) fn run_bash_widget(world: &mut WatnWorld, input: String) {
    let temp = world.temp_dir.as_ref().expect("widget temp dir");
    let bin = temp.path().join("bin");
    std::fs::create_dir_all(&bin).expect("create fake watn bin");
    let fake = bin.join("watn");
    std::fs::write(
        &fake,
        "#!/bin/sh\nif test -n \"$WATN_FAKE_LOG\"; then printf '%s\\n' \"$2\" >> \"$WATN_FAKE_LOG\"; fi\nprintf '%s' \"$WATN_FAKE_OUTPUT\"\nexit \"${WATN_FAKE_STATUS:-0}\"\n",
    )
    .expect("write fake watn");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755))
            .expect("make fake watn executable");
    }
    let target = world.shortcut_targets.get("bash").expect("Bash target");
    let shell_script = r#"
source "$WATN_SHORTCUT_FILE"
READLINE_LINE="$WATN_INPUT"
READLINE_POINT=0
_watn_widget
printf 'LINE<<%s>>\n' "$READLINE_LINE"
printf 'POINT<<%s>>\n' "$READLINE_POINT"
"#;
    let current_path = std::env::var("PATH").unwrap_or_default();
    let path = format!("{}:{current_path}", bin.display());
    let result = Command::new("bash")
        .args(["--noprofile", "--norc", "-c", shell_script])
        .env("PATH", path)
        .env("WATN_SHORTCUT_FILE", target)
        .env("WATN_INPUT", input)
        .env(
            "WATN_FAKE_OUTPUT",
            world
                .pending_config
                .get("fake_output")
                .cloned()
                .unwrap_or_default(),
        )
        .env(
            "WATN_FAKE_STATUS",
            world
                .pending_config
                .get("fake_status")
                .cloned()
                .unwrap_or_else(|| "0".to_string()),
        )
        .env(
            "WATN_FAKE_LOG",
            world
                .pending_config
                .get("fake_log")
                .cloned()
                .unwrap_or_default(),
        )
        .output()
        .expect("run Bash widget");
    world.shortcut_output = Some(String::from_utf8_lossy(&result.stdout).to_string());
    world.stderr_output = Some(String::from_utf8_lossy(&result.stderr).to_string());
    world.shortcut_status = result.status.code();
}

#[then(regex = r##"^the current command line should be exactly \"([^\"]*)\"$"##)]
fn current_line(world: &mut WatnWorld, line: String) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let actual = output
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("widget line output");
    assert_eq!(actual, line.replace("\\n", "\n"));
}

#[then("the cursor should be at the end of the current command line")]
fn cursor_end(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let point = output
        .split("POINT<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("widget cursor output")
        .parse::<usize>()
        .expect("numeric cursor position");
    let line = output
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("widget line output");
    assert_eq!(point, line.chars().count());
}

#[given("an installed Bash shortcut and a fake watn that records invocations")]
fn widget_recording_fixture(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create recording widget temp dir");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).expect("create recording widget home");
    let target = home.join(".bashrc");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home: home.clone(),
        xdg_config_home: None,
        shell: Some("/bin/bash".to_string()),
    };
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    assert!(report.is_success(), "recording fixture report: {report:?}");
    let log = temp.path().join("watn-invocations.log");
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("bash".to_string(), target)]);
    world
        .pending_config
        .insert("fake_output".to_string(), String::new());
    world
        .pending_config
        .insert("fake_status".to_string(), "0".to_string());
    world
        .pending_config
        .insert("fake_log".to_string(), log.display().to_string());
}

#[then("the fake watn should not have been invoked")]
fn fake_not_invoked(world: &mut WatnWorld) {
    let temp = world.temp_dir.as_ref().expect("recording widget temp dir");
    let log = temp.path().join("watn-invocations.log");
    assert!(!log.exists() || std::fs::read_to_string(log).unwrap().is_empty());
}

#[then("the current command line should remain empty")]
fn empty_line(world: &mut WatnWorld) {
    current_line(world, String::new());
}

#[when("I run the Bash widget with empty input")]
fn run_empty_bash_widget(world: &mut WatnWorld) {
    run_bash_widget(world, String::new());
}

#[given("an installed Bash shortcut and a fake watn that fails")]
fn widget_failure_fixture(world: &mut WatnWorld) {
    widget_recording_fixture(world);
    world
        .pending_config
        .insert("fake_output".to_string(), "partial".to_string());
    world
        .pending_config
        .insert("fake_status".to_string(), "1".to_string());
}

#[then(regex = r##"^the current command line should remain \"([^\"]*)\"$"##)]
fn current_line_remains(world: &mut WatnWorld, line: String) {
    current_line(world, line);
}

#[when("the fake watn returns empty output")]
fn empty_output(world: &mut WatnWorld) {
    world
        .pending_config
        .insert("fake_output".to_string(), String::new());
    world
        .pending_config
        .insert("fake_status".to_string(), "0".to_string());
}

#[given("an installed Bash shortcut and a fake watn that writes \"partial\" to stdout and exits non-zero")]
fn partial_stdout_fixture(world: &mut WatnWorld) {
    widget_recording_fixture(world);
    world
        .pending_config
        .insert("fake_output".to_string(), "partial".to_string());
    world
        .pending_config
        .insert("fake_status".to_string(), "1".to_string());
}

#[then("the partial stdout should not be inserted")]
fn partial_stdout_not_inserted(world: &mut WatnWorld) {
    current_line(world, "show partial result".to_string());
}

#[given("an installed Bash shortcut and a fake watn that records its question")]
fn question_recording_fixture(world: &mut WatnWorld) {
    widget_recording_fixture(world);
}

#[then(regex = r##"^the fake watn should receive exactly one question \"([^\"]*)\"$"##)]
fn exact_question(world: &mut WatnWorld, question: String) {
    let temp = world.temp_dir.as_ref().expect("question fixture temp dir");
    let log = temp.path().join("watn-invocations.log");
    let calls = std::fs::read_to_string(log).expect("read fake watn log");
    assert_eq!(calls, format!("{question}\n"));
}

#[then("the wildcard should not be expanded before watn receives the question")]
fn wildcard_not_expanded(world: &mut WatnWorld) {
    let temp = world.temp_dir.as_ref().expect("question fixture temp dir");
    let log = std::fs::read_to_string(temp.path().join("watn-invocations.log"))
        .expect("read fake watn log");
    assert!(log.contains("*"));
}

#[then(
    regex = r##"^the fake watn should have received exactly two questions \"([^\"]*)\" and \"([^\"]*)\"$"##
)]
fn two_questions(world: &mut WatnWorld, first: String, second: String) {
    let temp = world.temp_dir.as_ref().expect("question fixture temp dir");
    let log = std::fs::read_to_string(temp.path().join("watn-invocations.log"))
        .expect("read fake watn log");
    assert_eq!(log, format!("{first}\n{second}\n"));
}

#[given("an installed Bash shortcut and a fake watn that records each question")]
fn each_question_fixture(world: &mut WatnWorld) {
    widget_recording_fixture(world);
}

#[then("setup should report \"source ~/.bashrc\" for Bash")]
fn bash_reload(world: &mut WatnWorld) {
    assert!(world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .contains("Run: source ~/.bashrc"));
}

#[then("setup should report \"source ~/.zshrc\" for Zsh")]
fn zsh_reload(world: &mut WatnWorld) {
    assert!(world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .contains("Run: source ~/.zshrc"));
}

#[then("setup should report \"source ~/.config/fish/config.fish\" for Fish")]
fn fish_reload(world: &mut WatnWorld) {
    assert!(world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .contains("Run: source ~/.config/fish/config.fish"));
}

#[given("a shortcut selection with Bash enabled and Zsh and Fish disabled")]
fn selected_shells_fixture(world: &mut WatnWorld) {
    world.shortcut_shells = vec!["bash".to_string()];
}

#[when("the setup result confirms the shortcut selection")]
fn confirm_selected_shells(world: &mut WatnWorld) {
    world.shortcut_shells = watn::setup::selected_shortcut_shells(true, [true, false, false])
        .into_iter()
        .map(|shell| shell.lowercase_name().to_string())
        .collect();
}

#[then("the selected shortcut shells should contain only Bash")]
fn selected_shortcut_shells(world: &mut WatnWorld) {
    assert_eq!(world.shortcut_shells, vec!["bash"]);
}

#[then("the embedded line break should remain in the command line buffer")]
fn embedded_break(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let line = output
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("widget line output");
    assert!(line.contains('\n'));
}

#[then("the replacement text should not have executed")]
fn no_evaluation(_world: &mut WatnWorld) {
    assert!(!std::path::Path::new("/tmp/watn-shortcut-should-not-run").exists());
}

#[given("isolated Bash, Zsh, and Fish shortcut targets")]
fn isolated_target_contract(world: &mut WatnWorld) {
    isolated_shell_paths(world);
}

#[then("no generated block should contain a repository-local watn path")]
fn no_local_watn_path(world: &mut WatnWorld) {
    for path in world.shortcut_targets.values() {
        let content = std::fs::read_to_string(path).expect("read generated target");
        assert!(!content.contains("target/debug"));
        assert!(!content.contains("/home/buster/projects/watn"));
    }
}

#[then("every generated widget should invoke `command watn -- \"$question\"`")]
fn widget_invocation(world: &mut WatnWorld) {
    for path in world.shortcut_targets.values() {
        let content = std::fs::read_to_string(path).expect("read generated target");
        assert!(content.contains(r#"command watn -- "$question""#));
    }
}

#[then("the Bash block should use the current Readline line and cursor")]
fn bash_line_contract(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    assert!(content.contains("READLINE_LINE"));
    assert!(content.contains("READLINE_POINT"));
}

#[then("the Zsh block should use the current buffer and cursor")]
fn zsh_line_contract(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("zsh").unwrap())
        .expect("read Zsh target");
    assert!(content.contains("$BUFFER"));
    assert!(content.contains("CURSOR"));
}

#[then("the Fish block should replace and repaint the current command line")]
fn fish_line_contract(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("fish").unwrap())
        .expect("read Fish target");
    assert!(content.contains("commandline -r --"));
    assert!(content.contains("commandline -f repaint"));
}

#[then("every generated block should bind Ctrl-W")]
fn all_bindings(world: &mut WatnWorld) {
    let bash = std::fs::read_to_string(world.shortcut_targets.get("bash").unwrap())
        .expect("read Bash target");
    let zsh = std::fs::read_to_string(world.shortcut_targets.get("zsh").unwrap())
        .expect("read Zsh target");
    let fish = std::fs::read_to_string(world.shortcut_targets.get("fish").unwrap())
        .expect("read Fish target");
    assert!(bash.contains("\\C-w"));
    assert!(zsh.contains("bindkey '^W'"));
    assert!(fish.contains("bind \\cw"));
}
