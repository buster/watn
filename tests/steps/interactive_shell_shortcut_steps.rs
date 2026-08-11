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
