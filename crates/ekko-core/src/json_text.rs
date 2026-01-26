use serde_json::Value;
use std::collections::BTreeMap;

pub fn upsert_string_property(content: &str, key: &str, value: &str) -> Result<String, String> {
    let mut obj = if content.trim().is_empty() {
        Value::Object(Default::default())
    } else {
        serde_json::from_str::<Value>(content).map_err(|e| format!("JSON 解析失败: {e}"))?
    };

    let map = obj
        .as_object_mut()
        .ok_or_else(|| "JSON 顶层不是对象".to_string())?;
    map.insert(key.to_string(), Value::String(value.to_string()));
    Ok(to_stable_pretty_json(&obj))
}

pub fn upsert_string_map_entry(
    content: &str,
    key: &str,
    entry_key: &str,
    entry_value: &str,
) -> Result<String, String> {
    let mut obj = if content.trim().is_empty() {
        Value::Object(Default::default())
    } else {
        serde_json::from_str::<Value>(content).map_err(|e| format!("JSON 解析失败: {e}"))?
    };

    let map = obj
        .as_object_mut()
        .ok_or_else(|| "JSON 顶层不是对象".to_string())?;

    let entry = map
        .entry(key.to_string())
        .or_insert_with(|| Value::Object(Default::default()));

    let entry_map = entry
        .as_object_mut()
        .ok_or_else(|| format!("JSON 字段 {key} 不是对象"))?;
    entry_map.insert(
        entry_key.to_string(),
        Value::String(entry_value.to_string()),
    );

    Ok(to_stable_pretty_json(&obj))
}

fn to_stable_pretty_json(v: &Value) -> String {
    // serde_json preserves insertion order, but we want deterministic output for tests.
    // Convert objects to BTreeMap recursively.
    fn normalize(v: &Value) -> Value {
        match v {
            Value::Object(map) => {
                let mut out = BTreeMap::new();
                for (k, vv) in map.iter() {
                    out.insert(k.clone(), normalize(vv));
                }
                Value::Object(out.into_iter().collect())
            }
            Value::Array(arr) => Value::Array(arr.iter().map(normalize).collect()),
            _ => v.clone(),
        }
    }

    let n = normalize(v);
    // Pretty JSON with trailing newline.
    let mut s = serde_json::to_string_pretty(&n).expect("json");
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_string_property_creates_object() {
        let out = upsert_string_property("", "A", "B").expect("ok");
        assert!(out.contains("\"A\": \"B\""));
    }

    #[test]
    fn upsert_string_map_entry_creates_nested_object() {
        let out = upsert_string_map_entry("", "auth", "KEY", "VAL").expect("ok");
        assert!(out.contains("\"auth\""));
        assert!(out.contains("\"KEY\": \"VAL\""));
    }
}
