mod cli;
mod explain;
mod question;
mod review_flow;
mod runtime;
mod setup_commands;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    if cli::dispatch_command(&cli) {
        return;
    }
    question::run_question(&cli);
}
