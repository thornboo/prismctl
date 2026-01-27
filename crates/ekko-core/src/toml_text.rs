#[derive(Debug, Clone, PartialEq, Eq)]
/// Scalar values supported by Ekko's lightweight TOML upsert utilities.
pub enum TomlScalar {
    Str(String),
    Bool(bool),
    Int(i64),
}

impl TomlScalar {
    fn render(&self) -> String {
        match self {
            TomlScalar::Str(s) => render_toml_string(s),
            TomlScalar::Bool(b) => b.to_string(),
            TomlScalar::Int(i) => i.to_string(),
        }
    }
}

fn render_toml_string(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn is_table_header(line: &str) -> bool {
    let t = line.trim();
    (t.starts_with('[') && t.ends_with(']')) || (t.starts_with("[[") && t.ends_with("]]"))
}

pub fn upsert_root_key(content: &str, key: &str, value: TomlScalar) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut replaced = false;

    for line in lines.iter_mut() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if is_table_header(trimmed) {
            break;
        }

        let Some((k, _v)) = trimmed.split_once('=') else {
            continue;
        };
        if k.trim() == key {
            *line = format!("{} = {}", key, value.render());
            replaced = true;
            break;
        }
    }

    if !replaced {
        // Insert after initial comment/blank header block to keep files readable.
        let mut insert_at = 0;
        while insert_at < lines.len() {
            let t = lines[insert_at].trim();
            if t.is_empty() || t.starts_with('#') {
                insert_at += 1;
                continue;
            }
            break;
        }
        lines.insert(insert_at, format!("{} = {}", key, value.render()));
    }

    let mut out = lines.join("\n");
    out.push('\n');
    out
}

/// Upsert key/value pairs under a single TOML table header (e.g. `model_providers.ekko`).
pub fn upsert_table_kv(content: &str, header: &str, kv: &[(String, TomlScalar)]) -> String {
    let header_line = format!("[{}]", header);
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let mut header_idx: Option<usize> = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == header_line {
            header_idx = Some(i);
            break;
        }
    }

    if let Some(i) = header_idx {
        let mut end = i + 1;
        while end < lines.len() && !is_table_header(lines[end].trim()) {
            end += 1;
        }

        // Track which keys were found.
        let mut found = vec![false; kv.len()];

        for line in lines[i + 1..end].iter_mut() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let Some((k, _v)) = trimmed.split_once('=') else {
                continue;
            };
            let key = k.trim();

            for (idx, (want_k, want_v)) in kv.iter().enumerate() {
                if want_k == key {
                    *line = format!("{} = {}", want_k, want_v.render());
                    found[idx] = true;
                    break;
                }
            }
        }

        // Append missing keys at end of table.
        let mut insert_at = end;
        for (idx, (k, v)) in kv.iter().enumerate() {
            if found[idx] {
                continue;
            }
            lines.insert(insert_at, format!("{} = {}", k, v.render()));
            insert_at += 1;
        }
    } else {
        // Append new table.
        if !lines.is_empty() && !lines.last().expect("last").trim().is_empty() {
            lines.push(String::new());
        }
        lines.push(header_line);
        for (k, v) in kv {
            lines.push(format!("{} = {}", k, v.render()));
        }
    }

    let mut out = lines.join("\n");
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_root_key_inserts_before_tables() {
        let input = "[a]\nx=1\n";
        let out = upsert_root_key(input, "model_provider", TomlScalar::Str("ekko".into()));
        assert!(out.starts_with("model_provider = \"ekko\"\n[a]\n"));
    }

    #[test]
    fn upsert_table_kv_appends_table_if_missing() {
        let input = "model = \"x\"\n";
        let out = upsert_table_kv(
            input,
            "model_providers.ekko",
            &[("base_url".into(), TomlScalar::Str("https://x".into()))],
        );
        assert!(out.contains("[model_providers.ekko]\nbase_url = \"https://x\"\n"));
    }

    #[test]
    fn upsert_table_kv_updates_existing_key() {
        let input = "[model_providers.ekko]\nbase_url = \"a\"\n";
        let out = upsert_table_kv(
            input,
            "model_providers.ekko",
            &[("base_url".into(), TomlScalar::Str("b".into()))],
        );
        assert!(out.contains("base_url = \"b\""));
        assert!(!out.contains("base_url = \"a\""));
    }
}
