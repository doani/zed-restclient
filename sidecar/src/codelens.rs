#[derive(Debug, PartialEq)]
pub struct RequestMarker {
    pub display_line: usize, // The line to show the code lens / action
    pub block_index: usize,  // The index to pass to the execute command
}

pub fn find_request_starts(content: &str) -> Vec<RequestMarker> {
    let mut markers = Vec::new();
    let mut block_index = 0;
    
    // By default, the start of the file acts as an implicit separator
    let mut looking_for_request = true;

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // A separator resets our search for the next request
        if trimmed.starts_with("###") {
            // Wait, what if a line is just "###"?
            // If we are already looking for a request, we just continue looking.
            // If we found one, we start looking for the NEXT one.
            looking_for_request = true;
            
            // If the separator has text right after it, like "### GET http...", 
            // the user might have put the request ON the separator line.
            // Standard .http format says "###" must be on its own line (or followed by comments).
            // We ignore whatever is on the "###" line itself as it's a separator/comment.
            continue;
        }

        if looking_for_request {
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            // This is the first valid line after a separator (or start of file)
            // It must be the request line (Method URL)
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if !parts.is_empty() {
                markers.push(RequestMarker {
                    display_line: line_idx,
                    block_index,
                });
                block_index += 1;
                looking_for_request = false;
            }
        }
    }

    markers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_request_starts() {
        let content = "
# Das ist ein Kommentar
GET https://api.example.com
###
POST https://api.example.com
Content-Type: application/json

{ \"test\": 1 }
";
        let lines = find_request_starts(content);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].display_line, 2); // GET is on line 2
        assert_eq!(lines[0].block_index, 0);
        assert_eq!(lines[1].display_line, 4); // POST is on line 4
        assert_eq!(lines[1].block_index, 1);
    }

    #[test]
    fn test_users_file() {
        let content = "###\n\nGET https://api.github.com HTTP/1.1\nAuthorization: Bearer alskjfjaslkfjlaksfj\nContent-Type: application/json\n\n###\n\nGET http://localhost:3000\n\n### GET\n";
        let lines = find_request_starts(content);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].display_line, 2); // The first GET is on line 2
        assert_eq!(lines[1].display_line, 8); // The second GET is on line 8
    }
}
