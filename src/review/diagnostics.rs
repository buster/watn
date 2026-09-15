use std::io::Write;
use std::path::PathBuf;

/// Save the raw provider response to the overwritten state file so a bug
/// report can attach what the provider actually sent. The capture is
/// best-effort: the caller warns on failure and never changes the review
/// outcome. The file is created with Unix mode `0600` like the config file.
pub fn capture_unusable_response(raw: &str) -> std::io::Result<PathBuf> {
    let path = crate::config::unusable_response_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&path)?;
    file.write_all(raw.as_bytes())?;
    file.flush()?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::capture_unusable_response;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn capture_writes_the_raw_response_and_creates_the_state_directory() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let root = tempfile::tempdir().expect("state temp dir");
        let state_home = root.path().join("state");
        std::env::set_var("XDG_STATE_HOME", &state_home);
        let path = capture_unusable_response("raw payload").expect("capture");
        std::env::remove_var("XDG_STATE_HOME");

        assert_eq!(path, state_home.join("watn/last-unusable-response.txt"));
        assert_eq!(
            std::fs::read_to_string(&path).expect("captured response"),
            "raw payload"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path)
                .expect("capture metadata")
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600, "capture must be user-only");
        }
    }

    #[test]
    fn capture_reports_a_blocked_state_directory() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let root = tempfile::tempdir().expect("state temp dir");
        let blocker = root.path().join("blocked");
        std::fs::write(&blocker, "not a directory").expect("blocker file");
        std::env::set_var("XDG_STATE_HOME", &blocker);
        let error = capture_unusable_response("raw payload");
        std::env::remove_var("XDG_STATE_HOME");

        assert!(error.is_err(), "a blocked state directory must be reported");
    }
}
