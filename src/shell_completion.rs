use crate::shell_shortcut::{
    self, BlockIntent, BlockState, InstallReport, Shell, ShellEnvironment,
};

pub const OPEN_MARKER: &str = "# >>> watn shell completion >>>";
pub const CLOSE_MARKER: &str = "# <<< watn shell completion <<<";

pub fn install(shells: &[Shell]) -> InstallReport {
    install_with_environment(shells, &ShellEnvironment::from_process())
}

pub fn install_with_environment(shells: &[Shell], environment: &ShellEnvironment) -> InstallReport {
    shell_shortcut::install_blocks_with_environment(
        shells,
        environment,
        "shell completion",
        OPEN_MARKER,
        CLOSE_MARKER,
        completion_block,
    )
}

pub fn marker_state(
    shell: Shell,
    environment: &ShellEnvironment,
) -> Result<BlockState, crate::error::Error> {
    shell_shortcut::inspect_block_with_environment(shell, environment, OPEN_MARKER, CLOSE_MARKER)
}

pub fn reconcile_with_environment(
    intents: &[(Shell, BlockIntent)],
    environment: &ShellEnvironment,
) -> InstallReport {
    shell_shortcut::reconcile_blocks_with_environment(
        intents,
        environment,
        "shell completion",
        OPEN_MARKER,
        CLOSE_MARKER,
        completion_block,
    )
}

fn completion_block(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => BASH_BLOCK,
        Shell::Zsh => ZSH_BLOCK,
        Shell::Fish => FISH_BLOCK,
    }
}

const BASH_BLOCK: &str = r#"# >>> watn shell completion >>>
if command -v watn >/dev/null 2>&1; then
    source <(command watn completions bash)
fi
# <<< watn shell completion <<<
"#;

const ZSH_BLOCK: &str = r#"# >>> watn shell completion >>>
if (( $+commands[watn] )); then
    autoload -Uz compinit
    compinit
    source <(command watn completions zsh)
fi
# <<< watn shell completion <<<
"#;

const FISH_BLOCK: &str = r#"# >>> watn shell completion >>>
if type -q watn
    command watn completions fish | source
end
# <<< watn shell completion <<<
"#;
