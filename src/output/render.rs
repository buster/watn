pub fn print_response(
    command: &str,
    model: &str,
    tok_s: f64,
    cost: Option<f64>,
    elapsed_secs: f64,
) {
    let mut meta = format!("{} · {:.0} tok/s", model, tok_s);
    if let Some(c) = cost {
        meta.push_str(&format!(" · ${:.4}", c));
    }
    meta.push_str(&format!(" · {:.1}s · ¯\\_(ツ)_/¯", elapsed_secs));
    println!("{}", command);
    println!();
    eprintln!("{}", meta);
}
