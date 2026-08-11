use cucumber::{given, then, when};
use std::collections::HashMap;

use crate::WatnWorld;

fn shortcut_environment(world: &WatnWorld) -> watn::shell_shortcut::ShellEnvironment {
    let temp = world.temp_dir.as_ref().expect("shortcut temp dir");
    watn::shell_shortcut::ShellEnvironment {
        home: temp.path().join("home"),
        xdg_config_home: Some(temp.path().join("home").join(".config")),
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
