use crate::parser::HttpRequest;
use reqwest::{Client, Method, Request};
use std::collections::HashMap;
use std::str::FromStr;

fn resolve_variables(text: &str, variables: &HashMap<&str, &str>) -> String {
    let mut resolved = text.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        resolved = resolved.replace(&placeholder, value);
    }
    resolved
}

/// Converts our parsed HttpRequest into a native reqwest::Request.
pub fn build_request(
    client: &Client,
    req: &HttpRequest<'_>,
    variables: &HashMap<&str, &str>,
) -> anyhow::Result<Request> {
    let method = Method::from_str(req.method)
        .map_err(|_| anyhow::anyhow!("Invalid HTTP Method: {}", req.method))?;

    let url = resolve_variables(req.url, variables);
    let mut request_builder = client.request(method, &url);

    for (key, value) in &req.headers {
        let resolved_key = resolve_variables(key, variables);
        let resolved_value = resolve_variables(value, variables);
        request_builder = request_builder.header(resolved_key, resolved_value);
    }

    if let Some(body_text) = req.body {
        let resolved_body = resolve_variables(body_text, variables);
        request_builder = request_builder.body(resolved_body);
    }

    let request = request_builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build request: {}", e))?;

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Method;

    #[test]
    fn test_build_request() {
        let client = Client::new();
        let http_req = HttpRequest {
            method: "POST",
            url: "https://httpbin.org/post",
            headers: vec![("Content-Type", "application/json"), ("X-Custom", "Test")],
            body: Some("{\"hello\":\"world\"}"),
        };

        let reqwest_req = build_request(&client, &http_req, &HashMap::new()).expect("Failed to build request");

        // Verify Method
        assert_eq!(reqwest_req.method(), Method::POST);

        // Verify URL
        assert_eq!(reqwest_req.url().as_str(), "https://httpbin.org/post");

        // Verify Headers
        let headers = reqwest_req.headers();
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("X-Custom").unwrap(), "Test");

        // Verify Body
        let body_bytes = reqwest_req.body().unwrap().as_bytes().unwrap();
        assert_eq!(body_bytes, b"{\"hello\":\"world\"}");
    }

    #[test]
    fn test_build_request_with_variables() {
        let client = Client::new();
        let http_req = HttpRequest {
            method: "GET",
            url: "{{baseUrl}}/api/{{userId}}",
            headers: vec![("Authorization", "Bearer {{token}}")],
            body: Some("{\"id\":\"{{userId}}\"}"),
        };

        let mut vars = HashMap::new();
        vars.insert("baseUrl", "https://api.example.com");
        vars.insert("userId", "123");
        vars.insert("token", "secret123");

        let reqwest_req = build_request(&client, &http_req, &vars).expect("Failed to build request");

        assert_eq!(reqwest_req.url().as_str(), "https://api.example.com/api/123");
        assert_eq!(reqwest_req.headers().get("Authorization").unwrap(), "Bearer secret123");
        let body_bytes = reqwest_req.body().unwrap().as_bytes().unwrap();
        assert_eq!(body_bytes, b"{\"id\":\"123\"}");
    }
}
