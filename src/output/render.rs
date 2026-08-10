use std::io::{self, Write};

pub struct StreamRenderer<W: Write> {
    writer: W,
    has_content: bool,
    completed: bool,
}

impl<W: Write> StreamRenderer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            has_content: false,
            completed: false,
        }
    }

    pub fn write_content(&mut self, content: &str) -> io::Result<()> {
        if content.is_empty() {
            return Ok(());
        }
        self.writer.write_all(content.as_bytes())?;
        self.has_content = true;
        self.writer.flush()
    }

    pub fn finish_partial(&mut self) -> io::Result<()> {
        finish_streamed_command_to(&mut self.writer, self.has_content)
    }

    pub fn complete(&mut self) -> io::Result<()> {
        self.finish_partial()?;
        self.completed = true;
        Ok(())
    }

    pub fn has_content(&self) -> bool {
        self.has_content
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

fn finish_streamed_command_to<W: Write>(writer: &mut W, has_content: bool) -> io::Result<()> {
    if has_content {
        writer.write_all(b"\n\n")?;
        writer.flush()?;
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::StreamRenderer;
    use std::io::{self, Write};

    struct FlushFailureWriter {
        output: Vec<u8>,
    }

    impl Write for FlushFailureWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "flush failed"))
        }
    }

    #[test]
    fn flush_failure_preserves_visible_content_state() {
        let mut renderer = StreamRenderer::new(FlushFailureWriter { output: Vec::new() });
        let error = renderer.write_content("visible prefix").unwrap_err();

        assert_eq!(error.to_string(), "flush failed");
        assert!(renderer.has_content());
        assert!(!renderer.completed());
        assert_eq!(renderer.into_writer().output, b"visible prefix");
    }
}
