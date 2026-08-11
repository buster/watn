use cucumber::{given, then, when};
use std::collections::HashMap;
use std::path::PathBuf;

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
