use std::io::Write;

pub fn prompt_and_execute(command: &str) {
    eprint!("Execute now? [Y/n] ");
    std::io::stderr().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let input = input.trim().to_lowercase();
    if input.is_empty() || input == "y" || input == "yes" {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("failed to execute command");
        std::process::exit(status.code().unwrap_or(0));
    }
}
