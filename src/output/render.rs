pub fn print_response(command: &str, model: &str, tok_s: f64, cost: Option<f64>) {
    println!("{}", command);
    eprintln!("model: {}", model);
    eprintln!("tokens/s: {:.1}", tok_s);
    if let Some(c) = cost {
        eprintln!("cost: ${:.4}", c);
    }
}
