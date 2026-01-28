/// Upsert a managed block delimited by `start_marker` and `end_marker`.
///
/// Content outside the markers is preserved. If the block is missing, it is appended.
pub fn upsert_managed_block(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    block: &str,
) -> String {
    let start_idx = content.find(start_marker);
    let end_idx = content.find(end_marker);

    match (start_idx, end_idx) {
        (Some(s), Some(e)) if e >= s => {
            let mut out = String::with_capacity(content.len() + block.len() + 64);
            out.push_str(&content[..s]);
            if !out.ends_with('\n') && !out.is_empty() {
                out.push('\n');
            }
            out.push_str(start_marker);
            out.push('\n');
            out.push_str(block.trim_end_matches('\n'));
            out.push('\n');
            out.push_str(end_marker);
            out.push_str(&content[e + end_marker.len()..]);
            if !out.ends_with('\n') {
                out.push('\n');
            }
            out
        }
        (Some(s), None) => {
            // Start marker exists but end marker is missing; replace from start marker to EOF.
            let mut out = String::with_capacity(content.len() + block.len() + 64);
            out.push_str(&content[..s]);
            if !out.ends_with('\n') && !out.is_empty() {
                out.push('\n');
            }
            out.push_str(start_marker);
            out.push('\n');
            out.push_str(block.trim_end_matches('\n'));
            out.push('\n');
            out.push_str(end_marker);
            out.push('\n');
            out
        }
        _ => {
            // No block found; append to end.
            let mut out = String::with_capacity(content.len() + block.len() + 64);
            out.push_str(content);
            if !out.ends_with('\n') && !out.is_empty() {
                out.push('\n');
            }
            out.push_str(start_marker);
            out.push('\n');
            out.push_str(block.trim_end_matches('\n'));
            out.push('\n');
            out.push_str(end_marker);
            out.push('\n');
            out
        }
    }
}

/// Extract the content between `start_marker` and `end_marker` (without surrounding newlines).
pub fn extract_managed_block(
    content: &str,
    start_marker: &str,
    end_marker: &str,
) -> Option<String> {
    let s = content.find(start_marker)?;
    let e = content.find(end_marker)?;
    if e < s {
        return None;
    }

    let start_end = s + start_marker.len();
    let block_raw = &content[start_end..e];
    let block = block_raw.trim_matches('\n');
    Some(block.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_appends_when_missing() {
        let out = upsert_managed_block("a\n", "# prismctl:start", "# prismctl:end", "x=1\n");
        assert!(out.contains("# prismctl:start\nx=1\n# prismctl:end\n"));
    }

    #[test]
    fn upsert_replaces_when_present() {
        let input = "a\n# prismctl:start\nx=1\n# prismctl:end\nb\n";
        let out = upsert_managed_block(input, "# prismctl:start", "# prismctl:end", "x=2\n");
        assert!(out.contains("# prismctl:start\nx=2\n# prismctl:end\n"));
        assert!(!out.contains("x=1"));
    }

    #[test]
    fn extract_reads_block() {
        let input = "a\n# prismctl:start\nx=1\n# prismctl:end\n";
        let block =
            extract_managed_block(input, "# prismctl:start", "# prismctl:end").expect("block");
        assert_eq!(block, "x=1");
    }
}
