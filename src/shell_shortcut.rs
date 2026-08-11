use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::Error;

pub const OPEN_MARKER: &str = "# >>> watn shell shortcut >>>";
pub const CLOSE_MARKER: &str = "# <<< watn shell shortcut <<<";

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub const ALL: [Self; 3] = [Self::Bash, Self::Zsh, Self::Fish];

    pub fn name(self) -> &'static str {
        match self {
            Self::Bash => "Bash",
            Self::Zsh => "Zsh",
            Self::Fish => "Fish",
        }
    }

    pub fn lowercase_name(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
        }
    }

    pub fn reload_instruction(self, path: &Path, home: &Path) -> String {
        format!("Run: source {}", display_home_path(path, home))
    }

    pub fn generated_block(self) -> &'static str {
        match self {
            Self::Bash => BASH_BLOCK,
            Self::Zsh => ZSH_BLOCK,
            Self::Fish => FISH_BLOCK,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellEnvironment {
    pub home: PathBuf,
    pub xdg_config_home: Option<PathBuf>,
    pub shell: Option<String>,
}

impl ShellEnvironment {
    pub fn from_process() -> Self {
        Self {
            home: std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_default(),
            xdg_config_home: std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            shell: std::env::var("SHELL").ok(),
        }
    }

    pub fn target_path(&self, shell: Shell) -> Result<PathBuf, Error> {
        if self.home.as_os_str().is_empty() || !self.home.is_absolute() {
            return Err(Error::ConfigError(format!(
                "cannot resolve {} shell shortcut target: HOME must be an absolute path",
                shell.name()
            )));
        }
        let path = match shell {
            Shell::Bash => self.home.join(".bashrc"),
            Shell::Zsh => self.home.join(".zshrc"),
            Shell::Fish => {
                let config_home = self
                    .xdg_config_home
                    .clone()
                    .filter(|path| !path.as_os_str().is_empty())
                    .unwrap_or_else(|| self.home.join(".config"));
                if !config_home.is_absolute() {
                    return Err(Error::ConfigError(
                        "cannot resolve Fish shell shortcut target: XDG_CONFIG_HOME must be an absolute path"
                            .to_string(),
                    ));
                }
                config_home.join("fish").join("config.fish")
            }
        };
        Ok(path)
    }

    pub fn detected_shells(&self) -> Vec<Shell> {
        let Some(value) = self.shell.as_deref() else {
            return Vec::new();
        };
        let Some(name) = Path::new(value).file_name().and_then(|name| name.to_str()) else {
            return Vec::new();
        };
        Shell::ALL
            .into_iter()
            .filter(|shell| shell.lowercase_name() == name)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetResult {
    pub shell: Shell,
    pub path: Option<PathBuf>,
    pub success: bool,
    pub message: String,
    pub reload: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallReport {
    pub results: Vec<TargetResult>,
}

impl InstallReport {
    pub fn failures(&self) -> impl Iterator<Item = &TargetResult> {
        self.results.iter().filter(|result| !result.success)
    }

    pub fn successes(&self) -> impl Iterator<Item = &TargetResult> {
        self.results.iter().filter(|result| result.success)
    }

    pub fn is_success(&self) -> bool {
        self.results.iter().all(|result| result.success)
    }

    pub fn aggregate_error(&self) -> Option<Error> {
        let failures = self
            .failures()
            .map(|failure| failure.message.as_str())
            .collect::<Vec<_>>();
        (!failures.is_empty()).then(|| {
            Error::ConfigError(format!(
                "shell shortcut installation failed: {}",
                failures.join("; ")
            ))
        })
    }
}

pub fn install(shells: &[Shell]) -> InstallReport {
    install_with_environment(shells, &ShellEnvironment::from_process())
}

pub fn install_with_environment(shells: &[Shell], environment: &ShellEnvironment) -> InstallReport {
    let mut report = InstallReport::default();
    let mut selected = shells.to_vec();
    selected.sort_unstable();
    selected.dedup();
    for shell in selected {
        report.results.push(install_one(shell, environment));
    }
    report
}

fn install_one(shell: Shell, environment: &ShellEnvironment) -> TargetResult {
    let path = match environment.target_path(shell) {
        Ok(path) => path,
        Err(error) => {
            return TargetResult {
                shell,
                path: None,
                success: false,
                message: error.to_string(),
                reload: None,
            }
        }
    };

    match replace_target(shell, &path) {
        Ok(()) => TargetResult {
            shell,
            reload: Some(shell.reload_instruction(&path, &environment.home)),
            path: Some(path),
            success: true,
            message: format!("Configured {}", shell.name()),
        },
        Err(error) => TargetResult {
            shell,
            path: Some(path),
            success: false,
            message: error.to_string(),
            reload: None,
        },
    }
}

fn replace_target(shell: Shell, path: &Path) -> Result<(), Error> {
    let existing = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(target_error(
                shell,
                path,
                "refusing to modify a symbolic link",
            ));
        }
        Ok(metadata) if metadata.is_dir() => {
            return Err(target_error(shell, path, "target is a directory"));
        }
        Ok(_) => Some(fs::read(path).map_err(|error| target_io_error(shell, path, "read", error))?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(target_io_error(shell, path, "inspect", error)),
    };

    let original_mode = existing
        .as_ref()
        .and_then(|_| fs::metadata(path).ok())
        .map(|metadata| metadata.permissions());
    let content = build_content(shell, existing.as_deref().unwrap_or_default())?;
    let parent = path
        .parent()
        .ok_or_else(|| target_error(shell, path, "target has no parent directory"))?;
    fs::create_dir_all(parent)
        .map_err(|error| target_io_error(shell, path, "create parent", error))?;

    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("config");
    let temporary = parent.join(format!(
        ".{file_name}.watn-{counter}-{}.tmp",
        std::process::id()
    ));
    let write_result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| target_io_error(shell, path, "create temporary target", error))?;
        if let Some(mode) = original_mode {
            fs::set_permissions(&temporary, mode)
                .map_err(|error| target_io_error(shell, path, "set target permissions", error))?;
        }
        file.write_all(&content)
            .map_err(|error| target_io_error(shell, path, "write", error))?;
        file.sync_all()
            .map_err(|error| target_io_error(shell, path, "sync", error))?;
        fs::rename(&temporary, path)
            .map_err(|error| target_io_error(shell, path, "replace", error))?;
        // The rename is the committed target change; directory syncing only
        // improves durability and must not report a failure after replacement.
        let _ = sync_directory(parent);
        Ok::<(), Error>(())
    })();
    if write_result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    write_result
}

fn build_content(shell: Shell, existing: &[u8]) -> Result<Vec<u8>, Error> {
    let open_count = count_marker(existing, OPEN_MARKER.as_bytes());
    let close_count = count_marker(existing, CLOSE_MARKER.as_bytes());
    if open_count == 0 && close_count == 0 {
        let mut result = existing.to_vec();
        if !result.is_empty() && !result.ends_with(b"\n") {
            result.push(b'\n');
        }
        result.extend_from_slice(shell.generated_block().as_bytes());
        return Ok(result);
    }
    if open_count != 1 || close_count != 1 {
        return Err(Error::ConfigError(format!(
            "malformed watn shell shortcut markers: expected zero markers or one ordered pair, found {} opening and {} closing markers",
            open_count, close_count
        )));
    }
    let open = find_marker(existing, OPEN_MARKER.as_bytes()).expect("marker count checked");
    let close = find_marker(existing, CLOSE_MARKER.as_bytes()).expect("marker count checked");
    if close < open {
        return Err(Error::ConfigError(
            "malformed watn shell shortcut markers: closing marker precedes opening marker"
                .to_string(),
        ));
    }
    let close_end = close + CLOSE_MARKER.len();
    let mut result = Vec::with_capacity(existing.len() + shell.generated_block().len());
    result.extend_from_slice(&existing[..open]);
    result.extend_from_slice(shell.generated_block().as_bytes());
    result.extend_from_slice(&existing[close_end..]);
    Ok(result)
}

fn count_marker(content: &[u8], marker: &[u8]) -> usize {
    content
        .windows(marker.len())
        .filter(|candidate| *candidate == marker)
        .count()
}

fn find_marker(content: &[u8], marker: &[u8]) -> Option<usize> {
    content
        .windows(marker.len())
        .position(|candidate| candidate == marker)
}

fn target_error(shell: Shell, path: &Path, reason: &str) -> Error {
    Error::ConfigError(format!(
        "{} shell shortcut target {}: {}",
        shell.name(),
        path.display(),
        reason
    ))
}

fn target_io_error(shell: Shell, path: &Path, operation: &str, error: std::io::Error) -> Error {
    target_error(shell, path, &format!("cannot {operation} target: {error}"))
}

fn sync_directory(path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        File::open(path)?.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(())
    }
}

fn display_home_path(path: &Path, home: &Path) -> String {
    if let Ok(relative) = path.strip_prefix(home) {
        return format!("~/{}", relative.display());
    }
    path.display().to_string()
}

const BASH_BLOCK: &str = r#"# >>> watn shell shortcut >>>
_watn_widget() {
    local question="$READLINE_LINE"
    local result

    if [[ -z $question ]]; then
        return
    fi

    result=$(command watn -- "$question")
    local status=$?
    if [[ $status -eq 0 ]]; then
        while [[ $result == *$'\r' || $result == *$'\n' ]]; do
            result=${result:0:${#result}-1}
        done
        if [[ -n $result ]]; then
            READLINE_LINE=$result
            READLINE_POINT=${#READLINE_LINE}
        fi
    fi
}

bind -x '"\C-w":_watn_widget'
# <<< watn shell shortcut <<<
"#;

const ZSH_BLOCK: &str = r#"# >>> watn shell shortcut >>>
_watn_widget() {
    local question="$BUFFER"
    local result

    if [[ -z $question ]]; then
        zle redisplay
        return
    fi

    if result=$(command watn -- "$question"); then
        while [[ $result == *$'\r' || $result == *$'\n' ]]; do
            result=${result%$'\r'}
            result=${result%$'\n'}
        done
        if [[ -n $result ]]; then
            BUFFER=$result
            CURSOR=${#BUFFER}
        fi
    fi

    zle redisplay
}

zle -N _watn_widget
bindkey '^W' _watn_widget
bindkey -M viins '^W' _watn_widget
# <<< watn shell shortcut <<<
"#;

const FISH_BLOCK: &str = r#"# >>> watn shell shortcut >>>
function _watn_widget
    set -l question (commandline)
    if test -z "$question"
        commandline -f repaint
        return
    end

    set -l result (command watn -- "$question" | string collect)
    set -l status_code $pipestatus[1]
    if test $status_code -eq 0
        set result (string replace -r '[\r\n]+$' '' -- "$result")
        if test -n "$result"
            commandline -r -- "$result"
        end
    end
    commandline -f repaint
end

bind \cw _watn_widget
bind -M insert \cw _watn_widget
# <<< watn shell shortcut <<<
"#;
