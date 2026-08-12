use cucumber::{given, then, when};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::WatnWorld;

#[given(regex = r##"^an installed Fish shortcut and a fake watn that returns \"([^\"]*)\"$"##)]
fn installed_fish_shortcut(world: &mut WatnWorld, output: String) {
    let temp = tempfile::tempdir().expect("create Fish shortcut temp dir");
    let home = temp.path().join("home");
    let fish_config = home.join(".config/fish");
    std::fs::create_dir_all(&fish_config).expect("create Fish config directory");

    let environment = watn::shell_shortcut::ShellEnvironment {
        home: home.clone(),
        xdg_config_home: Some(home.join(".config")),
        shell: Some("/bin/fish".to_string()),
    };
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Fish],
        &environment,
    );
    assert!(report.is_success(), "Fish fixture report: {report:?}");

    let bin = temp.path().join("bin");
    std::fs::create_dir_all(&bin).expect("create fake watn bin");
    let fake = bin.join("watn");
    std::fs::write(&fake, "#!/bin/sh\nprintf '%s' \"$WATN_FAKE_OUTPUT\"\n")
        .expect("write fake watn");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755))
            .expect("make fake watn executable");
    }

    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([("fish".to_string(), fish_config.join("config.fish"))]);
    world
        .pending_config
        .insert("fish_fake_bin".to_string(), bin.display().to_string());
    world
        .pending_config
        .insert("fish_fake_output".to_string(), output.replace("\\n", "\n"));
}

#[when(regex = r##"^I press Ctrl-W in the Fish shortcut with current input \"([^\"]*)\"$"##)]
fn press_ctrl_w_in_fish(world: &mut WatnWorld, input: String) {
    let temp = world.temp_dir.as_ref().expect("Fish shortcut temp dir");
    let target = world
        .shortcut_targets
        .get("fish")
        .expect("Fish shortcut target");
    let capture = temp.path().join("fish-command-line");
    let fake_bin = world
        .pending_config
        .get("fish_fake_bin")
        .expect("fake watn bin");
    let fake_output = world
        .pending_config
        .get("fish_fake_output")
        .expect("fake watn output");
    let current_path = std::env::var("PATH").unwrap_or_default();
    let path = format!("{fake_bin}:{current_path}");

    let pty_system = portable_pty::native_pty_system();
    let pair = pty_system
        .openpty(portable_pty::PtySize {
            rows: 40,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("open Fish PTY");

    let init_command = r#"
source "$WATN_SHORTCUT_FILE"
function _watn_test_capture
    commandline > "$WATN_CAPTURE_FILE"
    commandline -f repaint
end
bind \cw _watn_widget
bind -M insert \cw _watn_widget
bind \cx _watn_test_capture
bind -M insert \cx _watn_test_capture
echo FISH_READY >&2
"#;
    let mut command = portable_pty::CommandBuilder::new("fish");
    command.args(["--no-config", "--private", "--interactive"]);
    command.arg("--init-command");
    command.arg(init_command);
    command.env("PATH", &path);
    command.env("TERM", "xterm-256color");
    command.env("WATN_SHORTCUT_FILE", target.display().to_string());
    command.env("WATN_CAPTURE_FILE", capture.display().to_string());
    command.env("WATN_FAKE_OUTPUT", fake_output);

    let mut child = pair.slave.spawn_command(command).expect("spawn Fish");
    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .expect("clone Fish PTY reader");
    let output_buffer = Arc::new(Mutex::new(Vec::new()));
    let reader_buffer = Arc::clone(&output_buffer);
    let reader_handle = thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => reader_buffer
                    .lock()
                    .unwrap()
                    .extend_from_slice(&buffer[..size]),
                Err(_) => break,
            }
        }
    });
    if !wait_for_marker(&output_buffer, "FISH_READY") {
        let transcript = String::from_utf8_lossy(&output_buffer.lock().unwrap()).to_string();
        drop(pair.master);
        let _ = child.kill();
        let _ = child.wait();
        let _ = reader_handle.join();
        panic!("Fish did not finish initialization; output: {transcript:?}");
    }
    let mut writer = pair.master.take_writer().expect("take Fish PTY writer");
    writer
        .write_all(input.as_bytes())
        .expect("write Fish request");
    writer
        .write_all(b"\x17\x18")
        .expect("write Fish Ctrl-W capture keys");
    writer.flush().expect("flush Fish request");

    let deadline = Instant::now() + Duration::from_secs(3);
    let captured = loop {
        if capture.is_file() {
            if let Ok(value) = std::fs::read_to_string(&capture) {
                if !value.is_empty() {
                    break value;
                }
            }
        }
        if Instant::now() >= deadline {
            let transcript = String::from_utf8_lossy(&output_buffer.lock().unwrap()).to_string();
            drop(writer);
            drop(pair.master);
            let _ = child.kill();
            let _ = child.wait();
            let _ = reader_handle.join();
            panic!("Fish did not capture its command line; output: {transcript:?}");
        }
        thread::sleep(Duration::from_millis(25));
    };

    drop(writer);
    drop(pair.master);
    let _ = child.kill();
    let _ = child.wait();
    let _ = reader_handle.join();

    world.shortcut_output = Some(captured.strip_suffix('\n').unwrap_or(&captured).to_string());
    world.shortcut_status = Some(0);
}

fn wait_for_marker(output_buffer: &Arc<Mutex<Vec<u8>>>, marker: &str) -> bool {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if String::from_utf8_lossy(&output_buffer.lock().unwrap()).contains(marker) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(25));
    }
}

#[then(regex = r##"^the Fish command line should be exactly \"([^\"]*)\"$"##)]
fn fish_command_line(world: &mut WatnWorld, expected: String) {
    let expected = expected.replace("\\n", "\n");
    let actual = world
        .shortcut_output
        .as_deref()
        .expect("captured Fish command line");
    assert_eq!(actual, expected);
    assert_eq!(actual.matches('\n').count(), 1);
    assert!(!actual.contains("\\n"));
}
