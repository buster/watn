use std::io::{self, Write};

pub fn finish_streamed_command(has_content: bool) -> io::Result<()> {
    if !has_content {
        return Ok(());
    }

    let mut stdout = io::stdout();
    stdout.write_all(b"\n\n")?;
    stdout.flush()
}

pub fn print_reasoning(reasoning: &str) -> io::Result<()> {
    let mut stderr = io::stderr();
    writeln!(stderr, "reasoning: {}", reasoning.trim())?;
    stderr.flush()
}

pub fn print_metadata(
    model: &str,
    tok_s: f64,
    cost: Option<f64>,
    elapsed_secs: f64,
) -> io::Result<()> {
    let mut meta = format!("{} · {:.0} tok/s", model, tok_s);
    if let Some(c) = cost {
        meta.push_str(&format!(" · ${:.4}", c));
    }
    meta.push_str(&format!(" · {:.1}s · ¯\\_(ツ)_/¯", elapsed_secs));

    let mut stderr = io::stderr();
    writeln!(stderr, "{}", meta)?;
    stderr.flush()
}
