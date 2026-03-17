#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct HttpRequest<'a> {
    pub method: &'a str,
    pub url: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: Option<&'a str>,
}

#[allow(dead_code)]
pub fn parse_http_file(content: &str) -> Vec<HttpRequest<'_>> {
    let mut requests = Vec::new();

    // Split the file by the standard `###` delimiter used in .http files
    let blocks = content.split("###");

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let mut lines = block.lines();
        let mut method = "GET"; // Default method
        let mut url = "";

        // Find the first non-empty line (ignoring comments starting with # or //)
        for line in lines.by_ref() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            // The first valid line is the Request Line (Method URL HTTP/Version)
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                method = parts[0];
                url = parts[1];
            } else if parts.len() == 1 {
                url = parts[0];
            }
            break;
        }

        if url.is_empty() {
            continue; // Invalid request block
        }

        let mut headers = Vec::new();
        let mut body_lines = Vec::new();
        let mut parsing_body = false;

        for line in lines {
            if parsing_body {
                body_lines.push(line);
            } else {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    // An empty line separates headers from the body
                    parsing_body = true;
                } else if trimmed.starts_with('#') || trimmed.starts_with("//") {
                    // Skip comments in header section
                    continue;
                } else if let Some((key, value)) = trimmed.split_once(':') {
                    headers.push((key.trim(), value.trim()));
                }
            }
        }

        let body = if body_lines.is_empty() {
            None
        } else {
            // Reconstruct body and trim trailing/leading whitespace from the whole block,
            // but preserve internal formatting
            Some(block[block.find(body_lines[0]).unwrap()..].trim_end())
        };

        requests.push(HttpRequest {
            method,
            url,
            headers,
            body,
        });
    }

    requests
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_get() {
        let content = "GET https://api.example.com/users";
        let reqs = parse_http_file(content);
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[0].url, "https://api.example.com/users");
        assert!(reqs[0].headers.is_empty());
        assert_eq!(reqs[0].body, None);
    }

    #[test]
    fn test_parse_post_with_headers_and_body() {
        let content = r#"
POST https://api.example.com/users
Content-Type: application/json
Authorization: Bearer token123

{
    "name": "John Doe",
    "email": "john@example.com"
}
"#;
        let reqs = parse_http_file(content);
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, "POST");
        assert_eq!(reqs[0].url, "https://api.example.com/users");
        assert_eq!(reqs[0].headers.len(), 2);
        assert_eq!(reqs[0].headers[0], ("Content-Type", "application/json"));
        assert_eq!(reqs[0].headers[1], ("Authorization", "Bearer token123"));
        assert_eq!(
            reqs[0].body,
            Some("{\n    \"name\": \"John Doe\",\n    \"email\": \"john@example.com\"\n}")
        );
    }

    #[test]
    fn test_parse_multiple_requests() {
        let content = r#"
GET http://localhost:8080/1
###
PUT http://localhost:8080/2
"#;
        let reqs = parse_http_file(content);
        assert_eq!(reqs.len(), 2);
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[1].method, "PUT");
    }
}
