use anyhow::{Context, Result};
use std::io::{self, BufRead, Read, Write};

fn main() -> Result<()> {
    // Log message to stderr so it doesn't interfere with the stdout JSON-RPC communication
    eprintln!("Zed REST Client Sidecar started.");

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let mut header_buf = String::new();
        // Read headers one by one. The first one is usually Content-Length.
        if stdin.read_line(&mut header_buf)? == 0 {
            // EOF reached, meaning the parent process (Zed) closed the connection
            break;
        }

        let header = header_buf.trim();
        if header.is_empty() {
            // Empty line between headers and body, skip it
            continue;
        }

        if let Some(len_str) = header.strip_prefix("Content-Length: ") {
            let len: usize = len_str.parse().context("Invalid Content-Length")?;

            // Read the mandatory empty line (\r\n) before the body
            let mut empty_line = String::new();
            stdin.read_line(&mut empty_line)?;

            // Read the actual JSON-RPC body based on the Content-Length
            let mut body = vec![0; len];
            stdin.read_exact(&mut body)?;

            let msg = String::from_utf8_lossy(&body);
            eprintln!("Sidecar received RPC message: {}", msg);

            // Send a Mock response back to the Zed Extension
            // In future Sprints, we will parse this message and execute a real HTTP request
            let response = r#"{"jsonrpc":"2.0","result":"mock_response","id":1}"#;

            // Format according to LSP standard: Content-Length header, empty line, JSON body
            write!(
                stdout,
                "Content-Length: {}\r\n\r\n{}",
                response.len(),
                response
            )?;

            // Ensure the data is actually sent out immediately
            stdout.flush()?;
        }
    }

    eprintln!("Zed REST Client Sidecar shutting down.");
    Ok(())
}
