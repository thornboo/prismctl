use serde_json::Value;
use std::collections::BTreeMap;

pub fn set_gemini_model_name_in_settings_json(
    existing: &str,
    model_name: &str,
) -> Result<String, String> {
    let mut root = if existing.trim().is_empty() {
        Value::Object(Default::default())
    } else {
        serde_json::from_str::<Value>(existing).map_err(|e| format!("JSON 解析失败: {e}"))?
    };

    let root_obj = root
        .as_object_mut()
        .ok_or_else(|| "settings.json 顶层不是对象".to_string())?;

    let model_obj = root_obj
        .entry("model".to_string())
        .or_insert_with(|| Value::Object(Default::default()));

    let model_map = model_obj
        .as_object_mut()
        .ok_or_else(|| "settings.json 的 model 字段不是对象".to_string())?;

    model_map.insert(
        "name".to_string(),
        Value::String(model_name.trim().to_string()),
    );

    Ok(to_stable_pretty_json(&root))
}

fn to_stable_pretty_json(v: &Value) -> String {
    // Keep deterministic output for tests.
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
    let mut s = serde_json::to_string_pretty(&n).expect("json");
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_model_name_in_empty_settings() {
        let out = set_gemini_model_name_in_settings_json("", "gemini-2.5-pro").expect("ok");
        assert!(out.contains("\"model\""));
        assert!(out.contains("\"name\": \"gemini-2.5-pro\""));
    }
}
