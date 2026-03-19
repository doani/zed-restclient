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
    let mut looking_for_request = true;

    let mut current_method = "GET";
    let mut current_url = "";
    let mut current_headers = Vec::new();
    let mut parsing_body = false;
    let mut body_start_idx = None;
    let mut body_end_idx = None;

    let content_ptr = content.as_ptr() as usize;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("###") {
            // Separator found. Finish the current request if we were building one.
            if !looking_for_request && !current_url.is_empty() {
                let body = if let (Some(s), Some(e)) = (body_start_idx, body_end_idx) {
                    if s <= e && s < content.len() {
                        let b = &content[s..e];
                        if b.trim().is_empty() {
                            None
                        } else {
                            Some(b.trim_end())
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                requests.push(HttpRequest {
                    method: current_method,
                    url: current_url,
                    headers: current_headers.clone(),
                    body,
                });
            }

            // Reset state for the next request
            looking_for_request = true;
            current_method = "GET";
            current_url = "";
            current_headers.clear();
            parsing_body = false;
            body_start_idx = None;
            body_end_idx = None;
            continue;
        }

        if looking_for_request {
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            // We found the request line!
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if !parts.is_empty() {
                if parts.len() >= 2 {
                    current_method = parts[0];
                    current_url = parts[1];
                } else {
                    current_method = "GET";
                    current_url = parts[0];
                }
                looking_for_request = false;
            }
            continue;
        }

        // If we are not looking for a request, we are parsing headers or body
        let line_end_ptr = line.as_ptr() as usize - content_ptr + line.len();

        if parsing_body {
            // Just update the end index of the body
            body_end_idx = Some(line_end_ptr);
        } else {
            if trimmed.is_empty() {
                // Empty line separates headers from body
                parsing_body = true;
                // The body starts after this empty line
                let start_ptr = line.as_ptr() as usize - content_ptr + line.len();
                // Check if there is a newline character to skip
                let actual_start = if start_ptr < content.len()
                    && content[start_ptr..].starts_with('\n')
                {
                    start_ptr + 1
                } else if start_ptr + 1 < content.len() && content[start_ptr..].starts_with("\r\n")
                {
                    start_ptr + 2
                } else {
                    start_ptr
                };

                body_start_idx = Some(actual_start);
                body_end_idx = Some(actual_start);
            } else if trimmed.starts_with('#') || trimmed.starts_with("//") {
                // Ignore comments in headers
                continue;
            } else if let Some((key, value)) = trimmed.split_once(':') {
                current_headers.push((key.trim(), value.trim()));
            }
        }
    }

    // Push the final request if there is one
    if !looking_for_request && !current_url.is_empty() {
        let body = if let (Some(s), Some(e)) = (body_start_idx, body_end_idx) {
            if s <= e && s < content.len() {
                let b = &content[s..e];
                if b.trim().is_empty() {
                    None
                } else {
                    Some(b.trim_end())
                }
            } else {
                None
            }
        } else {
            None
        };

        requests.push(HttpRequest {
            method: current_method,
            url: current_url,
            headers: current_headers,
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
