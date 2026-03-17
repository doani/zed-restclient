pub fn find_request_starts(content: &str) -> Vec<usize> {
    let mut request_starts = Vec::new();
    let mut looking_for_request_start = true;

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // A block separator resets our search for the next request
        if trimmed.starts_with("###") {
            looking_for_request_start = true;
            continue;
        }

        if looking_for_request_start {
            // Ignore empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            // The first line that is not empty, not a comment, and not a separator
            // is the start of an HTTP request.
            request_starts.push(i);
            looking_for_request_start = false;
        }
    }

    request_starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_request_starts() {
        let content = r#"
# Das ist ein Kommentar
GET https://api.example.com
###
POST https://api.example.com
Content-Type: application/json

{ "test": 1 }
"#;
        // We expect the function to find the line numbers (0-based).
        // Line 2 (GET) and Line 4 (POST).
        let lines = find_request_starts(content);
        assert_eq!(lines, vec![2, 4]);
    }
}
