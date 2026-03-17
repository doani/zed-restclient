#[derive(Debug, PartialEq)]
pub struct RequestMarker {
    pub display_line: usize, // The line to show the code lens / action
    pub block_index: usize,  // The index to pass to the execute command
}

pub fn find_request_starts(content: &str) -> Vec<RequestMarker> {
    let mut markers = Vec::new();
    let mut valid_block_count = 0;

    // We split by ###, but we must also keep track of global line numbers.
    let mut current_line = 0;
    let blocks = content.split("###");

    for (i, block) in blocks.enumerate() {
        // split("###") consumes the separator, so we need to account for it in the line count,
        // EXCEPT for the very first block (which didn't have a ### before it).
        let has_separator = i > 0;

        let trimmed = block.trim();
        if trimmed.is_empty() {
            if has_separator {
                // The separator itself might be a line (e.g. "###\n")
                // split doesn't include the separator in the string.
                current_line += block.split('\n').count() - 1; // Count newlines in this empty block
                current_line += 1; // Account for the `###` line
            } else {
                current_line += block.split('\n').count() - 1;
            }
            continue;
        }

        // It's a valid block! We need to find the display line.
        // It should either be the line with `###` (if there was one),
        // or the first non-comment line (e.g. `GET ...`).
        let display_line = if has_separator {
            // The `###` was on the line right before the start of this block string.
            current_line // The `###` itself is at current_line
        } else {
            // Find the actual GET line
            let mut offset = 0;
            for line in block.lines() {
                let t = line.trim();
                if !t.is_empty() && !t.starts_with('#') && !t.starts_with("//") {
                    break;
                }
                offset += 1;
            }
            current_line + offset
        };

        markers.push(RequestMarker {
            display_line,
            block_index: valid_block_count,
        });

        valid_block_count += 1;

        // Advance current_line by the newlines in this block, plus the separator line
        current_line += block.split('\n').count() - 1;
        if has_separator {
            current_line += 1;
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
        assert_eq!(lines[1].display_line, 3); // ### is on line 3
        assert_eq!(lines[1].block_index, 1);
    }

    #[test]
    fn test_starts_with_separator() {
        let content = "###\nGET https://api.github.com\n";
        let lines = find_request_starts(content);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].display_line, 0); // ### is on line 0
        assert_eq!(lines[0].block_index, 0);
    }
}
