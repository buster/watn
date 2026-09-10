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
printf 'HIST<<%s>>\n' "$(history | sed 's/^ *[0-9]*  *//')"
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
    assert_eq!(current_buffer(world), line.replace("\\n", "\n"));
}

fn current_buffer(world: &WatnWorld) -> &str {
    world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("widget line output")
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

pub(crate) fn current_history(world: &WatnWorld) -> &str {
    world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .split("HIST<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("generated Bash history output")
}

#[then(regex = r##"^the shell history should contain the recorded request comment "([^"]*)"$"##)]
fn history_contains_request_comment(world: &mut WatnWorld, comment: String) {
    let history = current_history(world);
    assert!(
        history.contains(&comment),
        "the shell history should contain the request comment {comment:?}, got: {history:?}"
    );
}

#[then(regex = r##"^the recorded request history entry should be exactly "([^"]*)"$"##)]
fn history_entry_exact(world: &mut WatnWorld, comment: String) {
    let history = current_history(world);
    assert_eq!(
        history,
        comment.replace("\\n", "\n"),
        "the recorded request history entry should match exactly"
    );
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

#[derive(Debug, Clone, Default)]
pub enum ReviewDriver {
    #[default]
    Structured,
    Streaming {
        chunks: Vec<String>,
        done: bool,
    },
}

#[derive(Debug, Default)]
pub struct ReviewState {
    pub driver: ReviewDriver,
    pub candidate_command: String,
    pub structured_response: Option<String>,
    pub intent: String,
    pub context: Option<watn::review::ReviewContext>,
    pub candidate: Option<watn::review::ReviewCandidate>,
    pub panel: Option<watn::review::ReviewPanelState>,
    pub buffer: watn::review::ReviewBuffer,
    pub rendered: Vec<String>,
    pub surface_open: bool,
    pub progress_line: Option<String>,
    pub command_output: String,
    pub released: Option<String>,
    pub panel_outcome: Option<watn::review::PanelOutcome>,
    pub delayed_purposes: bool,
    pub enhanced_adapter: Option<bool>,
    pub portable_adapter_fails: bool,
    pub adapter: Option<watn::review::PresentationSelection>,
    pub bash_command_line: String,
    pub bash_history: Vec<String>,
    pub review_disabled: bool,
    pub review_ineligible: bool,
    pub confirmation_shown: bool,
    pub executed: bool,
    pub tier: String,
    pub highest_tier: bool,
    pub catalog_models: Vec<String>,
    pub cleanup: String,
    pub narrow: bool,
    pub e2e: bool,
    pub stdout_path: Option<std::path::PathBuf>,
}

fn review_context(intent: &str, tier: &str) -> watn::review::ReviewContext {
    let tier = if tier.is_empty() { "1" } else { tier };
    watn::review::ReviewContext {
        intent: intent.to_string(),
        tier: tier.to_string(),
        provider: "loopback".to_string(),
        model: "review-model".to_string(),
    }
}

fn review_layout(world: &WatnWorld) -> watn::review::InlineLayout {
    if world.review.narrow {
        watn::review::InlineLayout::for_dimensions(40, 8)
    } else {
        watn::review::InlineLayout::for_dimensions(100, 40)
    }
}

fn render_panel_state(
    rendered: &mut Vec<String>,
    state: &watn::review::ReviewPanelState,
    layout: watn::review::InlineLayout,
) {
    let mut terminal = watn::review::ControllingTerminal::new(Vec::new(), layout);
    terminal
        .render(state)
        .expect("render the review surface through the controlling terminal");
    let bytes = terminal.into_writer();
    rendered.push(String::from_utf8(bytes).expect("review surface bytes are UTF-8"));
}

fn render_surface(world: &mut WatnWorld) {
    let state = world
        .review
        .panel
        .as_ref()
        .expect("review panel state")
        .clone();
    let layout = review_layout(world);
    let stage_count = state.candidate().flow.stages.len().max(1);
    let mut rendered = Vec::new();
    for stage_index in 0..stage_count {
        let mut stage_state = state.clone();
        stage_state.flow_stage = stage_index.min(stage_count - 1);
        render_panel_state(&mut rendered, &stage_state, layout);
    }
    world.review.candidate = Some(state.candidate().clone());
    world.review.rendered = rendered;
    world.review.surface_open = true;
}

fn panel_mut(world: &mut WatnWorld) -> &mut watn::review::ReviewPanelState {
    world.review.panel.as_mut().expect("review panel state")
}

fn key(code: crossterm::event::KeyCode) -> crossterm::event::KeyEvent {
    crossterm::event::KeyEvent::new(code, crossterm::event::KeyModifiers::NONE)
}

fn replace_editor_text(panel: &mut watn::review::ReviewPanelState, text: &str) {
    let original = panel
        .editor_buffer()
        .expect("command editor buffer")
        .to_string();
    for _ in 0..original.chars().count() {
        panel.handle_key(key(crossterm::event::KeyCode::Backspace));
    }
    for character in text.chars() {
        panel.handle_key(key(crossterm::event::KeyCode::Char(character)));
    }
}

fn install_bash_shortcut(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create shortcut temp dir");
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).expect("create isolated home");
    world.shortcut_targets = HashMap::from([("bash".to_string(), home.join(".bashrc"))]);
    world.temp_dir = Some(temp);
    let environment = shortcut_environment(world);
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    assert!(
        report.aggregate_error().is_none(),
        "Bash shortcut installation failed: {:?}",
        report.aggregate_error()
    );
}

fn review_rendered_text(world: &WatnWorld) -> String {
    world.review.rendered.join("\n")
}

fn assert_review_rendered_contains(world: &WatnWorld, needle: &str) {
    let rendered = review_rendered_text(world);
    assert!(
        rendered.contains(needle),
        "review surface should show {needle:?}, got:\n{rendered}"
    );
}

const REVIEW_COMMAND_STAGES: [&str; 4] = [
    "git log --format='%H' --since='7 days ago'",
    "xargs -n1",
    "git show --stat --oneline",
    "printf 'done'",
];

const REVIEW_STAGE_PURPOSES: [&str; 4] = [
    "Collect commit identifiers from the recent history.",
    "Pass each collected identifier to the per-commit command.",
    "Inspect each collected commit with a compact change summary.",
    "Report that the success branch completed.",
];

#[given(expr = "an installed Bash shortcut and a provider candidate {string}")]
fn review_candidate_given(world: &mut WatnWorld, command: String) {
    install_bash_shortcut(world);
    world.review = ReviewState::default();
    world.review.candidate_command = command;
}

#[given("the provider returns this structured review response:")]
fn review_structured_response(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let response = step
        .docstring
        .as_deref()
        .expect("structured review response docstring")
        .trim()
        .to_string();
    world.review.structured_response = Some(response);
}

#[when(expr = "I invoke Ctrl-W with current input {string}")]
fn review_invoke_ctrl_w(world: &mut WatnWorld, input: String) {
    world.review.intent = input.clone();
    world.review.progress_line = Some(format!("Generating command for {input}"));
    if world.review.e2e {
        crate::steps::interactive_shell_shortcut_e2e_steps::invoke_review_widget_pty(world);
        return;
    }
    match world.review.driver.clone() {
        ReviewDriver::Structured if world.review.review_disabled => {
            review_invoke_disabled_ctrl_w(world, &input);
        }
        ReviewDriver::Structured => review_invoke_structured(world, input),
        ReviewDriver::Streaming { chunks, done } => {
            review_invoke_streaming(world, &chunks, done);
        }
    }
}

fn review_invoke_disabled_ctrl_w(world: &mut WatnWorld, input: &str) {
    assert_eq!(
        watn::review::request_route(false, false),
        watn::review::RequestRoute::DirectCommandOutput,
        "disabled Ctrl-W must route to the existing direct replacement path"
    );
    world.review.bash_command_line = world.review.candidate_command.clone();
    world.review.command_output = world.review.candidate_command.clone();
    world.review.bash_history.push(format!("# {input}"));
    world.review.surface_open = false;
    world.review.panel = None;
    world.review.rendered.clear();
}

fn build_review_panel(world: &mut WatnWorld) {
    let selection = watn::review::PresentationSelection::open(
        world.review.enhanced_adapter,
        !world.review.portable_adapter_fails,
    );
    world.review.adapter = Some(selection);
    if world
        .review
        .adapter
        .as_ref()
        .is_some_and(|selection| selection.is_unavailable())
    {
        world.review.surface_open = false;
        return;
    }
    let candidate = match world.review.structured_response.clone() {
        Some(raw) => watn::review::candidate_from_provider_response(&raw),
        None => Some(watn::review::ReviewCandidate::from_command(
            world.review.candidate_command.clone(),
        )),
    };
    let Some(candidate) = candidate else {
        world.review.surface_open = false;
        world.review.panel = None;
        world.review.rendered.clear();
        return;
    };
    let intent = world.review.intent.clone();
    let tier = world.review.tier.clone();
    let context = review_context(&intent, &tier);
    world.review.context = Some(context.clone());
    world.review.panel = Some(watn::review::ReviewPanelState::new(context, candidate));
    render_surface(world);
}

fn review_invoke_structured(world: &mut WatnWorld, _input: String) {
    build_review_panel(world);
    world.review.command_output.clear();
}

fn open_review_surface(world: &mut WatnWorld, command: &str) {
    world.review.candidate = Some(watn::review::ReviewCandidate::from_command(command));
    world.review.surface_open = true;
}

fn review_invoke_streaming(world: &mut WatnWorld, chunks: &[String], done: bool) {
    for chunk in chunks {
        world.review.buffer.receive(chunk);
    }
    if done {
        world.review.buffer.complete();
    }
    if let Some(command) = world.review.buffer.candidate().map(str::to_string) {
        open_review_surface(world, &command);
    }
}

#[then("the review surface should show the git log stage")]
fn review_shows_git_log(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, REVIEW_COMMAND_STAGES[0]);
}

#[then("the review surface should show the xargs stage")]
fn review_shows_xargs(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, REVIEW_COMMAND_STAGES[1]);
}

#[then("the review surface should show the git show stage")]
fn review_shows_git_show(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, REVIEW_COMMAND_STAGES[2]);
}

#[then("the review surface should show the success branch")]
fn review_shows_success_branch(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, REVIEW_COMMAND_STAGES[3]);
}

#[then("the review surface should show each model-written stage purpose")]
fn review_shows_stage_purposes(world: &mut WatnWorld) {
    for purpose in REVIEW_STAGE_PURPOSES {
        assert_review_rendered_contains(world, purpose);
    }
    assert!(
        world.review.command_output.is_empty(),
        "review bytes must not reach the command-output channel"
    );
}

#[given("an installed Bash shortcut and a provider that streams a candidate in multiple events")]
fn review_streaming_provider(world: &mut WatnWorld) {
    install_bash_shortcut(world);
    world.review = ReviewState {
        driver: ReviewDriver::Streaming {
            chunks: vec!["df ".to_string(), "-h".to_string()],
            done: false,
        },
        ..ReviewState::default()
    };
}

#[given("the provider has not sent [DONE]")]
fn review_provider_not_done(world: &mut WatnWorld) {
    match &mut world.review.driver {
        ReviewDriver::Streaming { done, .. } => *done = false,
        _ => panic!("provider driver is not streaming"),
    }
    assert!(!world.review.buffer.is_complete());
}

#[then("the review surface should not open before [DONE]")]
fn review_surface_not_open(world: &mut WatnWorld) {
    assert!(
        !world.review.surface_open,
        "review surface opened before [DONE]"
    );
    assert!(!world.review.buffer.is_complete());
    assert!(world.review.buffer.candidate().is_none());
}

#[then("the existing progress line should remain the first feedback")]
fn review_progress_first(world: &mut WatnWorld) {
    assert!(
        world.review.progress_line.is_some(),
        "existing progress line was not shown first"
    );
    assert!(
        world.review.rendered.is_empty(),
        "review surface rendered before the progress line"
    );
    assert!(
        world.review.command_output.is_empty(),
        "command output appeared before the review surface"
    );
}

#[then("no candidate text should be released before final acceptance")]
fn review_no_release_before_acceptance(world: &mut WatnWorld) {
    assert!(
        world.review.released.is_none(),
        "candidate was released before final acceptance"
    );
    assert!(
        world.review.command_output.is_empty(),
        "command-output channel is not empty before acceptance"
    );
}

#[when("the provider sends [DONE]")]
fn review_provider_sends_done(world: &mut WatnWorld) {
    if let ReviewDriver::Streaming { done, .. } = &mut world.review.driver {
        *done = true;
    }
    world.review.buffer.complete();
    if let Some(command) = world.review.buffer.candidate().map(str::to_string) {
        open_review_surface(world, &command);
    }
}

#[then("the review surface should open with the complete candidate")]
fn review_surface_opens_complete(world: &mut WatnWorld) {
    assert!(
        world.review.surface_open,
        "review surface did not open after [DONE]"
    );
    let candidate = world
        .review
        .candidate
        .as_ref()
        .expect("complete candidate after [DONE]");
    assert_eq!(candidate.command, "df -h");
    assert!(
        world.review.released.is_none(),
        "complete candidate was released without final acceptance"
    );
}

const REVIEW_FIXTURE_COMMAND: &str = "git log --oneline | head -5";

const REVIEW_LOADING_RESPONSE: &str = r#"{"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline"},{"stage_text":"head -5"}],"purpose_status":"loading","purpose_request":"opaque"}"#;

const REVIEW_READY_RESPONSE: &str = r#"{"review_version":1,"command":"git log --oneline | head -5","stages":[{"stage_text":"git log --oneline","purpose":"List recent commits."},{"stage_text":"head -5","purpose":"Keep only the first five."}],"purpose_status":"ready"}"#;

#[given(expr = "an installed Bash shortcut and a provider candidate for {string}")]
fn review_candidate_for_intent(world: &mut WatnWorld, intent: String) {
    install_bash_shortcut(world);
    world.review = ReviewState {
        candidate_command: REVIEW_FIXTURE_COMMAND.to_string(),
        bash_command_line: intent.clone(),
        intent,
        ..ReviewState::default()
    };
}

fn open_command_editor(panel: &mut watn::review::ReviewPanelState) {
    panel.focus = watn::review::FocusRegion::Actions;
    panel.action_cursor = 1;
    assert_eq!(
        panel.selected_action(),
        watn::review::PanelAction::EditCommand
    );
    assert_eq!(
        panel.handle_key(key(crossterm::event::KeyCode::Enter)),
        watn::review::PanelOutcome::Continue
    );
    assert_eq!(
        panel.input_mode,
        watn::review::PanelInputMode::CommandEditor
    );
}

#[when("I open the separate command editor")]
fn review_open_command_editor(world: &mut WatnWorld) {
    open_command_editor(panel_mut(world));
}

#[when("I edit the selected candidate without changing the original intent")]
fn review_edit_candidate(world: &mut WatnWorld) {
    replace_editor_text(panel_mut(world), "git status --short");
    assert_eq!(panel_mut(world).editor_buffer(), Some("git status --short"));
    assert_eq!(
        world
            .review
            .context
            .as_ref()
            .map(|context| context.intent.as_str()),
        Some("inspect recent log changes")
    );
}

#[when("I press Enter in the command editor")]
fn review_enter_command_editor(world: &mut WatnWorld) {
    let outcome = panel_mut(world).handle_key(key(crossterm::event::KeyCode::Enter));
    assert_eq!(
        outcome,
        watn::review::PanelOutcome::EditCommitted("git status --short".to_string())
    );
    render_surface(world);
}

#[then("the review surface should refresh the command flow for the edited candidate")]
fn review_refreshed_flow(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "git status --short");
}

#[then("the original intent should remain visible in the review context")]
fn review_original_intent_visible(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "inspect recent log changes");
}

#[then("final acceptance should still be required")]
fn review_final_acceptance_required(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.input_mode, watn::review::PanelInputMode::Review);
    assert!(world.review.surface_open, "review surface closed early");
    assert!(
        world.review.released.is_none(),
        "candidate was released without final acceptance"
    );
    assert_review_rendered_contains(world, "Accept candidate");
}

#[when("I change the selected candidate")]
fn review_change_candidate(world: &mut WatnWorld) {
    replace_editor_text(panel_mut(world), "git status --short");
}

#[when("I press Escape in the command editor")]
fn review_escape_command_editor(world: &mut WatnWorld) {
    let outcome = panel_mut(world).handle_key(key(crossterm::event::KeyCode::Esc));
    assert_eq!(outcome, watn::review::PanelOutcome::EditDiscarded);
    render_surface(world);
}

#[then("the unedited candidate should remain selected")]
fn review_unedited_candidate_selected(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
    assert_eq!(panel.input_mode, watn::review::PanelInputMode::Review);
    assert_review_rendered_contains(world, REVIEW_FIXTURE_COMMAND);
}

#[then("the review surface should remain open")]
fn review_surface_remains_open(world: &mut WatnWorld) {
    assert!(world.review.surface_open, "review surface closed");
    assert_review_rendered_contains(world, "Accept candidate");
}

#[when("I edit the selected candidate and purpose refresh fails")]
fn review_edit_and_refresh_fails(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    open_command_editor(panel);
    replace_editor_text(panel, "git status --short");
    assert_eq!(
        panel.handle_key(key(crossterm::event::KeyCode::Enter)),
        watn::review::PanelOutcome::EditCommitted("git status --short".to_string())
    );
    let selected = panel.selected_candidate;
    let result = panel.candidates[selected].apply_response("{\"review_version\":1,");
    assert!(
        matches!(
            result,
            watn::review::ReviewParseResult::PurposeUnavailable(_)
        ),
        "malformed refresh response must produce purpose-unavailable"
    );
    render_surface(world);
}

#[then("the edited candidate should remain visible")]
fn review_edited_candidate_visible(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, "git status --short");
    assert_review_rendered_contains(world, "git status --short");
}

#[then("the review surface should show purpose-unavailable or unsupported flow state")]
fn review_purpose_unavailable_shown(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().purpose_status,
        watn::review::PurposeStatus::Unavailable
    );
    assert_review_rendered_contains(world, "purpose-unavailable");
}

#[then("the original intent should remain visible")]
fn review_intent_visible(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "inspect recent log changes");
}

#[when("I open the review surface")]
fn review_open_surface(world: &mut WatnWorld) {
    build_review_panel(world);
}

#[then("the initial focus should be Actions with final acceptance selected")]
fn review_initial_focus(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.focus, watn::review::FocusRegion::Actions);
    assert_eq!(panel.selected_action(), watn::review::PanelAction::Accept);
}

#[when("I press Tab")]
fn review_press_tab(world: &mut WatnWorld) {
    panel_mut(world).handle_key(key(crossterm::event::KeyCode::Tab));
    render_surface(world);
}

fn assert_review_focus(world: &WatnWorld, expected: watn::review::FocusRegion, label: &str) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.focus, expected, "focus should move to {label}");
    assert_review_rendered_contains(world, &format!("Focus: {label}"));
}

#[then("focus should move to Flow")]
fn review_focus_flow(world: &mut WatnWorld) {
    assert_review_focus(world, watn::review::FocusRegion::Flow, "Flow");
}

#[then("focus should move to Candidates")]
fn review_focus_candidates(world: &mut WatnWorld) {
    assert_review_focus(world, watn::review::FocusRegion::Candidates, "Candidates");
}

#[then("focus should move to Actions")]
fn review_focus_actions(world: &mut WatnWorld) {
    assert_review_focus(world, watn::review::FocusRegion::Actions, "Actions");
}

#[when("I press an arrow key within the Flow region")]
fn review_arrow_in_flow(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    panel.focus = watn::review::FocusRegion::Flow;
    panel.handle_key(key(crossterm::event::KeyCode::Down));
    render_surface(world);
}

#[then("the selected command-flow stage should change")]
fn review_flow_stage_changed(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.flow_stage, 1,
        "arrow key within Flow should select the next stage"
    );
    assert_review_rendered_contains(world, "head -5");
}

#[when("I press Shift-Tab within the Actions region")]
fn review_shift_tab_in_actions(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    panel.focus = watn::review::FocusRegion::Actions;
    panel.handle_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Tab,
        crossterm::event::KeyModifiers::SHIFT,
    ));
    render_surface(world);
}

#[when("I press Enter on the selected action")]
fn review_enter_selected_action(world: &mut WatnWorld) {
    let outcome = panel_mut(world).handle_key(key(crossterm::event::KeyCode::Enter));
    world.review.panel_outcome = Some(outcome);
}

#[then("the selected action should activate")]
fn review_selected_action_activates(world: &mut WatnWorld) {
    assert!(
        matches!(
            world.review.panel_outcome,
            Some(watn::review::PanelOutcome::Accepted(_))
        ),
        "Enter should activate the selected Accept action, got {:?}",
        world.review.panel_outcome
    );
}

#[when("I press Escape in the review surface")]
fn review_escape_surface(world: &mut WatnWorld) {
    let outcome = panel_mut(world).handle_key(key(crossterm::event::KeyCode::Esc));
    world.review.panel_outcome = Some(outcome.clone());
    if outcome == watn::review::PanelOutcome::Cancelled {
        world.review.surface_open = false;
    }
}

#[then("the review should be cancelled")]
fn review_cancelled(world: &mut WatnWorld) {
    assert_eq!(
        world.review.panel_outcome,
        Some(watn::review::PanelOutcome::Cancelled)
    );
    assert!(
        !world.review.surface_open,
        "cancellation should close the review surface"
    );
    assert!(
        world.review.released.is_none(),
        "cancellation must release no candidate"
    );
}

#[given("the structured review response supports delayed stage purposes")]
fn review_delayed_response_supported(world: &mut WatnWorld) {
    let mut probe = watn::review::ReviewCandidate::from_command(REVIEW_FIXTURE_COMMAND);
    assert_eq!(
        probe.apply_response(REVIEW_LOADING_RESPONSE),
        watn::review::ReviewParseResult::Loading,
        "loading response must validate with the delayed-purpose request"
    );
    world.review.structured_response = Some(REVIEW_LOADING_RESPONSE.to_string());
}

#[given("stage purposes are delayed")]
fn review_stage_purposes_delayed(world: &mut WatnWorld) {
    world.review.delayed_purposes = true;
    assert!(
        world.review.structured_response.is_some(),
        "a structured loading response is required"
    );
}

#[when("I invoke Ctrl-W with the current input")]
fn review_invoke_current_input(world: &mut WatnWorld) {
    let input = world.review.intent.clone();
    review_invoke_ctrl_w(world, input);
}

#[then("the review surface should appear with stage purposes loading")]
fn review_surface_loading(world: &mut WatnWorld) {
    assert!(world.review.surface_open, "review surface did not open");
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().purpose_status,
        watn::review::PurposeStatus::Loading
    );
    assert_review_rendered_contains(world, "loading");
}

#[when("stage purposes become available")]
fn review_stage_purposes_available(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    let selected = panel.selected_candidate;
    let result = panel.candidates[selected].apply_response(REVIEW_READY_RESPONSE);
    assert_eq!(result, watn::review::ReviewParseResult::Ready);
    render_surface(world);
}

#[then("the review surface should update with the model-written stage purposes")]
fn review_surface_updated_purposes(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().purpose_status,
        watn::review::PurposeStatus::Ready
    );
    assert_review_rendered_contains(world, "List recent commits.");
    assert_review_rendered_contains(world, "Keep only the first five.");
}

#[given("the provider returns command text without a structured review response")]
fn review_command_only_response(world: &mut WatnWorld) {
    world.review.structured_response = None;
    assert_eq!(
        world.review.candidate_command, REVIEW_FIXTURE_COMMAND,
        "command-only response must still provide a candidate command"
    );
}

#[then("the review surface should show purpose-unavailable immediately")]
fn review_purpose_unavailable_immediately(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().purpose_status,
        watn::review::PurposeStatus::Unavailable
    );
    assert_review_rendered_contains(world, "purpose-unavailable");
}

#[then("the review surface should not claim that purposes are loading")]
fn review_no_loading_claim(world: &mut WatnWorld) {
    assert!(
        !review_rendered_text(world).contains("loading"),
        "command-only response must not claim purposes are loading"
    );
}

#[then("the candidate should remain reviewable")]
fn review_candidate_reviewable(world: &mut WatnWorld) {
    assert!(world.review.surface_open, "review surface must remain open");
    assert!(
        world.review.released.is_none(),
        "candidate must not be released without acceptance"
    );
    assert_review_rendered_contains(world, "Accept candidate");
}

const REVIEW_UNSUPPORTED_COMMAND: &str = "for file in *.log; do cat < \"$file\"; done";

#[given("an installed Bash shortcut and a provider candidate containing unsupported shell syntax")]
fn review_unsupported_candidate(world: &mut WatnWorld) {
    install_bash_shortcut(world);
    world.review = ReviewState {
        candidate_command: REVIEW_UNSUPPORTED_COMMAND.to_string(),
        intent: "inspect log files".to_string(),
        ..ReviewState::default()
    };
}

#[then("the raw candidate should remain visible")]
fn review_raw_candidate_visible(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, REVIEW_UNSUPPORTED_COMMAND);
}

#[then("unsupported command-flow portions should be marked")]
fn review_unsupported_marked(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert!(
        panel.candidate().flow.has_unsupported(),
        "unsupported syntax must be marked in the derived flow"
    );
    assert_review_rendered_contains(world, "unsupported");
}

#[then("the review surface should still offer final acceptance and cancellation")]
fn review_acceptance_and_cancellation_offered(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "Accept candidate");
    assert_review_rendered_contains(world, "Cancel review");
    assert!(world.review.released.is_none());
}

#[given("the selected enhanced presentation adapter cannot open")]
fn review_enhanced_adapter_cannot_open(world: &mut WatnWorld) {
    world.review.enhanced_adapter = Some(false);
}

#[then("the portable inline review surface should open")]
fn review_portable_surface_open(world: &mut WatnWorld) {
    let selection = world.review.adapter.as_ref().expect("adapter selection");
    assert_eq!(
        selection.active(),
        Some(watn::review::PresentationAdapter::Portable)
    );
    assert_eq!(
        selection.attempts(),
        &[
            watn::review::PresentationAdapter::Enhanced,
            watn::review::PresentationAdapter::Portable
        ]
    );
    assert!(
        world.review.surface_open,
        "portable review surface did not open"
    );
    assert!(
        !world.review.rendered.is_empty(),
        "portable review surface rendered nothing"
    );
}

#[then("the current candidate should remain available")]
fn review_current_candidate_available(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
    assert_review_rendered_contains(world, REVIEW_FIXTURE_COMMAND);
}

#[given("the portable inline review surface cannot open")]
fn review_portable_cannot_open(world: &mut WatnWorld) {
    world.review.portable_adapter_fails = true;
}

#[then("the original Bash command line should remain unchanged")]
fn review_bash_line_unchanged(world: &mut WatnWorld) {
    assert_eq!(world.review.bash_command_line, "show disk usage");
    assert!(!world.review.surface_open, "review surface must not open");
}

#[then("no candidate should be released to the shell")]
fn review_no_candidate_released(world: &mut WatnWorld) {
    assert!(
        world.review.released.is_none(),
        "unavailable review must release no candidate"
    );
    assert!(
        world.review.command_output.is_empty(),
        "command-output channel must stay empty"
    );
    assert!(world.review.panel.is_none(), "no review panel may exist");
}

#[then("no new request comment should be recorded in Bash history")]
fn review_no_history_comment(world: &mut WatnWorld) {
    assert!(
        !world
            .review
            .bash_history
            .iter()
            .any(|entry| entry.contains("# show disk usage")),
        "failed review must not record a request comment"
    );
}

const REVIEW_FAILED_RESPONSE: &str = "{\"review_version\":1,";

#[given(expr = "an installed Bash shortcut and a selected candidate for {string}")]
fn review_selected_candidate(world: &mut WatnWorld, intent: String) {
    review_candidate_for_intent(world, intent);
    build_review_panel(world);
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.selected_candidate, 0);
}

#[when("a purpose refresh or replacement generation fails")]
fn review_refresh_or_generation_fails(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    let selected = panel.selected_candidate;
    let result = panel.candidates[selected].apply_response(REVIEW_FAILED_RESPONSE);
    assert!(
        matches!(
            result,
            watn::review::ReviewParseResult::PurposeUnavailable(_)
        ),
        "provider failure must be isolated as purpose-unavailable"
    );
    render_surface(world);
}

#[then("the selected candidate should remain available")]
fn review_selected_candidate_available(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
    assert!(
        world.review.surface_open,
        "review surface closed on failure"
    );
    assert!(world.review.released.is_none());
}

#[then("the review surface should show purpose-unavailable or generation failure")]
fn review_purpose_unavailable_or_generation_failure(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().purpose_status,
        watn::review::PurposeStatus::Unavailable
    );
    assert_review_rendered_contains(world, "purpose-unavailable");
}

#[given("an installed Bash shortcut with the explanatory review surface disabled")]
fn review_disabled_shortcut(world: &mut WatnWorld) {
    install_bash_shortcut(world);
    let config = watn::config::types::Config {
        review: watn::config::types::ReviewConfig { panel: false },
        ..watn::config::types::Config::default()
    };
    assert!(
        !watn::review::resolve_review_enabled(
            &config,
            watn::config::types::ReviewPanelOverride::Unset,
            true
        ),
        "persisted panel = false must disable review"
    );
    world.review = ReviewState {
        review_disabled: true,
        ..ReviewState::default()
    };
}

#[given(expr = "a provider candidate {string}")]
fn review_provider_candidate(world: &mut WatnWorld, command: String) {
    world.review.candidate_command = command;
}

#[then(expr = "the Bash command line should contain {string}")]
fn review_bash_line_contains(world: &mut WatnWorld, expected: String) {
    assert!(
        world.review.bash_command_line.contains(&expected),
        "Bash command line {:?} should contain {expected:?}",
        world.review.bash_command_line
    );
}

#[then("no command-flow review should open")]
fn review_no_surface(world: &mut WatnWorld) {
    assert!(!world.review.surface_open, "review surface must not open");
    assert!(world.review.panel.is_none(), "no review panel may exist");
    assert!(
        world.review.rendered.is_empty(),
        "no review surface may be rendered"
    );
}

#[then("the existing Ctrl-W history and no-evaluation behavior should be unchanged")]
fn review_history_no_evaluation_unchanged(world: &mut WatnWorld) {
    assert_eq!(
        world.review.bash_history,
        vec!["# show available diskspace".to_string()],
        "disabled Ctrl-W records exactly the existing history comment"
    );
    assert!(
        !world.review.executed,
        "disabled review must not evaluate the command"
    );
}

#[given("the explanatory review surface is disabled")]
fn review_surface_disabled(world: &mut WatnWorld) {
    let config = watn::config::types::Config {
        review: watn::config::types::ReviewConfig { panel: false },
        ..watn::config::types::Config::default()
    };
    assert!(!watn::review::resolve_review_enabled(
        &config,
        watn::config::types::ReviewPanelOverride::Unset,
        true
    ));
    world.review = ReviewState {
        review_disabled: true,
        ..ReviewState::default()
    };
}

#[given(expr = "a configured provider candidate {string}")]
fn review_configured_candidate(world: &mut WatnWorld, command: String) {
    world.review.candidate_command = command;
}

#[when(expr = "I ask positionally for {string}")]
fn review_ask_positionally(world: &mut WatnWorld, question: String) {
    assert_ne!(
        watn::review::request_route(!world.review.review_disabled, false),
        watn::review::RequestRoute::ReviewSurface,
        "disabled positional request must not route through review"
    );
    world.review.intent = question;
    world.review.command_output = world.review.candidate_command.clone();
    world.review.surface_open = false;
    world.review.panel = None;
}

#[then(expr = "the existing command-output channel should contain only {string}")]
fn review_command_output_only(world: &mut WatnWorld, expected: String) {
    assert_eq!(
        world.review.command_output, expected,
        "disabled review must keep the existing command-output contract"
    );
    assert_eq!(
        watn::review::request_route(false, false),
        watn::review::RequestRoute::DirectCommandOutput
    );
}

#[when(expr = "I submit {string} through interactive stdin")]
fn review_submit_stdin(world: &mut WatnWorld, question: String) {
    world.review.intent = question;
    world.review.command_output = world.review.candidate_command.clone();
    world.review.surface_open = false;
    world.review.panel = None;
}

#[then("no review surface should open")]
fn review_no_surface_opens(world: &mut WatnWorld) {
    assert!(!world.review.surface_open, "review surface must not open");
    assert!(world.review.panel.is_none(), "no review panel may exist");
}

#[when(regex = r##"^I run `watn -x "([^"]*)"` in an eligible terminal$"##)]
fn review_run_x_eligible(world: &mut WatnWorld, question: String) {
    let review_enabled = !world.review.review_disabled && !world.review.review_ineligible;
    if !review_enabled {
        assert_eq!(
            watn::review::request_route(false, true),
            watn::review::RequestRoute::ExecuteWithConfirmation,
            "disabled review must keep the existing -x confirmation"
        );
        world.review.confirmation_shown = true;
        world.review.surface_open = false;
        world.review.panel = None;
        return;
    }
    crate::steps::interactive_shell_shortcut_e2e_steps::run_eligible_x_review(world, question);
}

#[then("the existing \"Execute now?\" confirmation should be shown")]
fn review_existing_confirmation_shown(world: &mut WatnWorld) {
    assert!(
        world.review.confirmation_shown,
        "existing confirmation prompt must be shown"
    );
    assert!(world.review.panel.is_none(), "no review surface may open");
}

#[then("execution should require the existing confirmation response")]
fn review_execution_requires_confirmation(world: &mut WatnWorld) {
    assert!(
        !world.review.executed,
        "no execution may happen before the confirmation response"
    );
    assert!(world.review.confirmation_shown);
}

#[given("an `-x` request is redirected or otherwise not review-eligible")]
fn review_x_not_eligible(world: &mut WatnWorld) {
    world.review.review_ineligible = true;
    let config = watn::config::types::Config::default();
    assert!(
        !watn::review::resolve_review_enabled(
            &config,
            watn::config::types::ReviewPanelOverride::Unset,
            false
        ),
        "an ineligible request must not enter review even with the default panel"
    );
}

#[when("I run the request")]
fn review_run_request(world: &mut WatnWorld) {
    let review_enabled = !world.review.review_disabled && !world.review.review_ineligible;
    assert_eq!(
        watn::review::request_route(review_enabled, true),
        watn::review::RequestRoute::ExecuteWithConfirmation,
        "non-eligible -x must route to the existing confirmation"
    );
    world.review.confirmation_shown = true;
}

#[then("review acceptance should not authorize execution")]
fn review_acceptance_not_authorize(world: &mut WatnWorld) {
    assert!(
        !world.review.executed,
        "only the existing confirmation may authorize execution"
    );
    assert!(world.review.confirmation_shown);
    assert!(
        world.review.released.is_none(),
        "no review candidate may be released for a non-review -x request"
    );
}

const REVIEW_REPLACEMENT_COMMAND: &str = "df -h --local";

#[when(expr = "I rephrase the intent as {string}")]
fn review_rephrase_intent(world: &mut WatnWorld, intent: String) {
    let panel = panel_mut(world);
    panel.rephrase_intent(intent);
    let replacement = watn::review::ReviewCandidate::from_command(REVIEW_REPLACEMENT_COMMAND);
    panel.replace_current(replacement);
    render_surface(world);
}

#[then("a new candidate should be generated for the current intent")]
fn review_new_candidate_for_intent(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, REVIEW_REPLACEMENT_COMMAND);
    assert_eq!(
        panel.selected_candidate, 0,
        "the new cycle selects its own candidate"
    );
    assert_review_rendered_contains(world, REVIEW_REPLACEMENT_COMMAND);
}

#[then(expr = "the visible intent should be {string}")]
fn review_visible_intent(world: &mut WatnWorld, intent: String) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.context.intent, intent);
    assert_review_rendered_contains(world, &format!("intent: {intent}"));
}

#[then("the prior intent should remain only in current-review history")]
fn review_prior_intent_only_history(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.intent_history(), ["show disk usage".to_string()]);
    assert_ne!(
        panel.context.intent, "show disk usage",
        "the prior intent must not remain the active intent"
    );
}

#[then("the prior candidate should not be accepted by the new cycle")]
fn review_prior_candidate_not_accepted(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_ne!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
    match panel
        .clone()
        .handle_key(key(crossterm::event::KeyCode::Enter))
    {
        watn::review::PanelOutcome::Accepted(candidate) => {
            assert_eq!(candidate.command, REVIEW_REPLACEMENT_COMMAND);
        }
        outcome => panic!("the new cycle must accept only its own candidate, got {outcome:?}"),
    }
}

const REVIEW_REGENERATED_COMMAND: &str = "df -h --all";

#[when("I regenerate the candidate")]
fn review_regenerate_candidate(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    let replacement = watn::review::ReviewCandidate::from_command(REVIEW_REGENERATED_COMMAND);
    panel.replace_current(replacement);
    render_surface(world);
}

#[then("a replacement candidate should be shown for the same intent")]
fn review_replacement_for_intent(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, REVIEW_REGENERATED_COMMAND);
    assert_eq!(panel.context.intent, "show disk usage");
    assert_review_rendered_contains(world, REVIEW_REGENERATED_COMMAND);
}

#[then("the prior candidate should not be retained unless comparison was requested")]
fn review_prior_not_retained(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidates.len(),
        1,
        "default regeneration replaces the prior candidate"
    );
    assert_ne!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
}

const REVIEW_ESCALATED_COMMAND: &str = "git log --oneline --graph | head -5";

const REVIEW_SELECTED_MODEL_COMMAND: &str = "git log --oneline --graph --decorate | head -5";

#[given("an installed Bash shortcut and a candidate generated at the small tier")]
fn review_small_tier_candidate(world: &mut WatnWorld) {
    review_candidate_for_intent(world, "inspect recent log changes".to_string());
}

#[when("I request a higher tier")]
fn review_request_higher_tier(world: &mut WatnWorld) {
    if world.review.highest_tier {
        let models = world.review.catalog_models.clone();
        panel_mut(world).open_model_selection(models);
        render_surface(world);
        return;
    }
    let panel = panel_mut(world);
    assert_eq!(panel.context.tier, "1", "small tier is the starting tier");
    let context = watn::review::ReviewContext {
        tier: "2".to_string(),
        model: "review-model-2".to_string(),
        ..panel.context.clone()
    };
    let replacement = watn::review::ReviewCandidate::from_command(REVIEW_ESCALATED_COMMAND);
    panel.escalate(context, replacement);
    render_surface(world);
}

#[then("a new candidate should be generated at the next configured tier")]
fn review_next_tier_candidate(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.context.tier, "2");
    assert_eq!(panel.candidate().command, REVIEW_ESCALATED_COMMAND);
}

#[then("its tier and provider/model context should be visible")]
fn review_tier_context_visible(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "tier 2");
    assert_review_rendered_contains(world, "loopback/review-model-2");
}

#[then("the current intent should remain unchanged")]
fn review_intent_unchanged(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.context.intent, "inspect recent log changes");
    assert_review_rendered_contains(world, "intent: inspect recent log changes");
}

#[given("an installed Bash shortcut and a candidate generated at the highest configured tier")]
fn review_highest_tier_candidate(world: &mut WatnWorld) {
    review_candidate_for_intent(world, "inspect recent log changes".to_string());
    world.review.tier = "3".to_string();
    world.review.highest_tier = true;
}

#[given(expr = "the provider catalog contains {string} and {string}")]
fn review_catalog_contains(world: &mut WatnWorld, first: String, second: String) {
    world.review.catalog_models = vec![first, second];
}

#[then("the provider catalog model selection should open")]
fn review_model_selection_open(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.model_selection(),
        Some(["model-a".to_string(), "model-b".to_string()].as_slice())
    );
    assert_review_rendered_contains(world, "model-a");
    assert_review_rendered_contains(world, "model-b");
}

#[when(expr = "I select {string}")]
fn review_select_model(world: &mut WatnWorld, model: String) {
    let candidate = watn::review::ReviewCandidate::from_command(REVIEW_SELECTED_MODEL_COMMAND);
    panel_mut(world).select_model(model, candidate);
    render_surface(world);
}

#[then(expr = "the next candidate should use {string}")]
fn review_next_candidate_model(world: &mut WatnWorld, model: String) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.context.model, model);
    assert_review_rendered_contains(world, &format!("loopback/{model}"));
}

#[then("the selected model should apply only to the next candidate")]
fn review_model_oneshot(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    panel.complete_model_selection();
    assert_eq!(
        panel.context.model,
        panel.configured_model(),
        "the configured model is restored after the next candidate cycle"
    );
    assert!(panel.model_selection().is_none(), "selection is closed");
}

#[when("I reject the selected candidate")]
fn review_reject_candidate(world: &mut WatnWorld) {
    let panel = panel_mut(world);
    panel.focus = watn::review::FocusRegion::Actions;
    panel.action_cursor = 2;
    assert_eq!(panel.selected_action(), watn::review::PanelAction::Reject);
    let outcome = panel.handle_key(key(crossterm::event::KeyCode::Enter));
    assert_eq!(outcome, watn::review::PanelOutcome::Rejected);
    world.review.panel_outcome = Some(outcome);
    render_surface(world);
}

#[then("no candidate should be released")]
fn review_nothing_released(world: &mut WatnWorld) {
    assert!(
        world.review.released.is_none(),
        "rejection must release no candidate"
    );
    assert!(
        world.review.command_output.is_empty(),
        "command-output channel must stay empty"
    );
}

#[then(expr = "the current intent should remain {string}")]
fn review_current_intent_remains(world: &mut WatnWorld, intent: String) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.context.intent, intent);
    assert_review_rendered_contains(world, &format!("intent: {intent}"));
}

#[then("the review should offer regeneration or rephrasing")]
fn review_offer_regeneration(world: &mut WatnWorld) {
    assert!(world.review.surface_open, "review must stay available");
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.input_mode, watn::review::PanelInputMode::Review);
    assert_eq!(panel.focus, watn::review::FocusRegion::Actions);
    assert_review_rendered_contains(world, "Accept candidate");
    assert_review_rendered_contains(world, "Cancel review");
}

#[when("I retain the current candidate for comparison")]
fn review_retain_candidate(world: &mut WatnWorld) {
    panel_mut(world).retain_current();
    render_surface(world);
}

#[then("both candidates should be available in the current review only")]
fn review_both_candidates_available(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidates.len(),
        2,
        "retention keeps both candidates in the current review"
    );
    assert_eq!(panel.candidate().command, REVIEW_REGENERATED_COMMAND);
    assert_eq!(panel.candidates[1].command, REVIEW_FIXTURE_COMMAND);
}

#[then("each candidate should show its tier and provider/model context")]
fn review_candidates_show_context(world: &mut WatnWorld) {
    assert_review_rendered_contains(
        world,
        &format!("1. {REVIEW_REGENERATED_COMMAND} | tier 1 | loopback/review-model"),
    );
    assert_review_rendered_contains(
        world,
        &format!("2. {REVIEW_FIXTURE_COMMAND} | tier 1 | loopback/review-model"),
    );
}

#[when("I select the retained candidate")]
fn review_select_retained(world: &mut WatnWorld) {
    panel_mut(world).select_candidate(1);
    render_surface(world);
}

#[then("it should become the selected candidate for final acceptance")]
fn review_retained_selected(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.selected_candidate, 1);
    assert_eq!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
}

#[then("only the selected candidate should be eligible for acceptance")]
fn review_only_selected_accept(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    match panel
        .clone()
        .handle_key(key(crossterm::event::KeyCode::Enter))
    {
        watn::review::PanelOutcome::Accepted(candidate) => {
            assert_eq!(candidate.command, REVIEW_FIXTURE_COMMAND);
        }
        outcome => panic!("only the selected candidate may be accepted, got {outcome:?}"),
    }
}

#[when("I start a regeneration, purpose refresh, or provider catalog model selection")]
fn review_start_operation(world: &mut WatnWorld) {
    panel_mut(world).begin_operation(watn::review::ReviewOperation::Regeneration);
}

#[when("I interrupt that in-progress operation")]
fn review_interrupt_operation(world: &mut WatnWorld) {
    let interrupted = panel_mut(world).interrupt_operation();
    assert_eq!(
        interrupted,
        Some(watn::review::ReviewOperation::Regeneration),
        "the in-progress operation must be interruptible"
    );
}

#[then("only that operation should be cancelled")]
fn review_only_operation_cancelled(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.pending_operation(), None);
    assert_eq!(panel.candidate().command, REVIEW_FIXTURE_COMMAND);
    assert!(
        world.review.released.is_none(),
        "interruption must release no candidate"
    );
}

#[then("the review state should remain open")]
fn review_state_remains_open(world: &mut WatnWorld) {
    assert!(world.review.surface_open, "review state must remain open");
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.input_mode, watn::review::PanelInputMode::Review);
}

#[when("I accept the selected candidate")]
fn review_accept_candidate(world: &mut WatnWorld) {
    let state = world.review.panel.clone().expect("review panel state");
    let layout = watn::review::InlineLayout::for_dimensions(100, 40);
    let mut terminal = watn::review::ControllingTerminal::new(Vec::new(), layout);
    terminal.begin().expect("hide the review cursor");
    let mut panel = watn::review::InlineReviewPanel::new(terminal, state);
    panel.render().expect("render review surface");
    let outcome = panel
        .handle_key(key(crossterm::event::KeyCode::Enter))
        .expect("accept the selected candidate");
    panel.finish().expect("restore the controlling terminal");
    let bytes = panel.terminal().writer().clone();
    world.review.cleanup = String::from_utf8(bytes).expect("cleanup bytes are UTF-8");
    match outcome {
        watn::review::PanelOutcome::Accepted(candidate) => {
            world.review.released = Some(candidate.command.clone());
            world.review.bash_command_line = candidate.command.clone();
        }
        outcome => panic!("expected candidate acceptance, got {outcome:?}"),
    }
    world.review.surface_open = false;
}

#[then("the review surface should disappear immediately")]
fn review_surface_disappears(world: &mut WatnWorld) {
    assert!(!world.review.surface_open, "review surface must close");
    assert!(
        world.review.cleanup.contains("\u{1b}[J"),
        "inline rows must be cleared on acceptance: {:?}",
        world.review.cleanup
    );
}

#[then("Watn should leave cursor and inline rows restored")]
fn review_terminal_restored(world: &mut WatnWorld) {
    let cleanup = &world.review.cleanup;
    assert!(
        cleanup.contains("\u{1b}[?25h"),
        "cursor visibility must be restored: {cleanup:?}"
    );
    assert!(
        cleanup.contains("\u{1b}["),
        "inline row movement must be reset: {cleanup:?}"
    );
}

#[then("the Bash line editor should repaint the prompt with the accepted candidate")]
fn review_line_editor_repaint(world: &mut WatnWorld) {
    assert_eq!(world.review.bash_command_line, REVIEW_FIXTURE_COMMAND);
    assert_eq!(
        world.review.released.as_deref(),
        Some(REVIEW_FIXTURE_COMMAND),
        "only the accepted candidate is released"
    );
}

const REVIEW_MANY_STAGE_COMMAND: &str = "a | b | c | d | e | f | g | h";

#[given(
    "an installed Bash shortcut and a provider candidate with more stages than fit in the terminal"
)]
fn review_many_stages_candidate(world: &mut WatnWorld) {
    install_bash_shortcut(world);
    world.review = ReviewState {
        candidate_command: REVIEW_MANY_STAGE_COMMAND.to_string(),
        intent: "inspect recent log changes".to_string(),
        narrow: true,
        ..ReviewState::default()
    };
}

#[then("the review surface should remain a bounded inline panel")]
fn review_bounded_panel(world: &mut WatnWorld) {
    let layout = review_layout(world);
    assert!(layout.is_bounded(), "narrow layout must stay bounded");
    for rendered in &world.review.rendered {
        let rows = rendered.split("\r\n").count();
        assert!(
            rows <= layout.max_rows as usize,
            "rendered panel has {rows} rows, above the {} row bound",
            layout.max_rows
        );
    }
}

#[then("it should show a compact command-flow overview and one readable selected stage")]
fn review_compact_overview(world: &mut WatnWorld) {
    assert_review_rendered_contains(world, "Flow [1/8]");
    assert_review_rendered_contains(world, "> a");
}

#[then("arrow navigation should reach every command-flow stage")]
fn review_arrow_reaches_every_stage(world: &mut WatnWorld) {
    let mut panel = world.review.panel.clone().expect("review panel state");
    panel.focus = watn::review::FocusRegion::Flow;
    let stage_count = panel.candidate().flow.stages.len();
    let mut seen = vec![panel.flow_stage];
    for _ in 0..stage_count.saturating_sub(1) {
        panel.handle_key(key(crossterm::event::KeyCode::Down));
        seen.push(panel.flow_stage);
    }
    for index in 0..stage_count {
        assert!(
            seen.contains(&index),
            "arrow navigation must reach stage {index}, saw {seen:?}"
        );
    }
    assert_eq!(panel.flow_stage, stage_count - 1);
}

#[given("the provider returns a markdown-fenced structured review response:")]
fn review_fenced_response(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let response = step
        .docstring
        .as_deref()
        .expect("fenced response docstring")
        .to_string();
    world.review.structured_response = Some(response);
}

#[given(
    expr = "the provider returns an invalid structured review response with the command {string}"
)]
fn review_invalid_response_with_command(world: &mut WatnWorld, command: String) {
    let response = serde_json::json!({
        "review_version": 1,
        "command": command,
        "stages": [{"stage_text": command}],
        "purpose_status": "incomplete"
    })
    .to_string();
    world.review.structured_response = Some(response);
}

#[then(expr = "the review surface should show the command {string}")]
fn review_shows_command(world: &mut WatnWorld, command: String) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(panel.candidate().command, command);
    assert_review_rendered_contains(world, &command);
}

#[given("the provider returns a structured review payload without a complete command")]
fn review_response_without_command(world: &mut WatnWorld) {
    let response = serde_json::json!({
        "review_version": 1,
        "stages": [],
        "purpose_status": "ready"
    })
    .to_string();
    world.review.structured_response = Some(response);
}

#[given("the provider returns a markdown-fenced structured review response with line breaks")]
fn review_multiline_fenced_response(world: &mut WatnWorld) {
    let response = "```json\n{\n  \"review_version\": 1,\n  \"command\": \"df -h --local\",\n  \"stages\": [\n    {\"stage_text\": \"df -h --local\", \"purpose\": \"Show local disk usage.\"}\n  ],\n  \"purpose_status\": \"ready\"\n}\n```";
    world.review.structured_response = Some(response.to_string());
}

#[then("every rendered review value should stay on one inline row")]
fn review_rendered_rows_are_single_line(world: &mut WatnWorld) {
    let panel = world.review.panel.as_ref().expect("review panel state");
    let layout = review_layout(world);
    let lines = watn::review::render_lines(panel, layout);
    assert!(!lines.is_empty(), "review surface rendered no rows");
    for line in &lines {
        assert!(
            !line.contains('\n') && !line.contains('\r') && !line.contains('\t'),
            "rendered review value contains a row break: {line:?}"
        );
    }
}
