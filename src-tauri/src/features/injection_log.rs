use serde::Deserialize;
use tauri::{AppHandle, Listener};

pub const EVENT_LOG: &str = "injection-log";

#[derive(Deserialize)]
struct LogEntry {
    level: String,
    target: String,
    message: String,
    #[serde(default)]
    fields: serde_json::Value,
}

fn format_summary(message: &str, fields: &serde_json::Value) -> String {
    let empty_object = matches!(fields, serde_json::Value::Object(m) if m.is_empty());
    if fields.is_null() || empty_object {
        message.to_string()
    } else {
        format!("{message} {fields}")
    }
}

pub fn setup_listener(app: &AppHandle) {
    app.listen(EVENT_LOG, |event| {
        let Ok(entry) = serde_json::from_str::<LogEntry>(event.payload()) else {
            return;
        };
        let summary = format_summary(&entry.message, &entry.fields);
        let target = entry.target.as_str();
        match entry.level.as_str() {
            "trace" => log::trace!(target: target, "{summary}"),
            "debug" => log::debug!(target: target, "{summary}"),
            "info" => log::info!(target: target, "{summary}"),
            "warn" => log::warn!(target: target, "{summary}"),
            "error" => log::error!(target: target, "{summary}"),
            _ => log::info!(target: target, "{summary}"),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_fields_yield_just_message() {
        assert_eq!(format_summary("hi", &json!({})), "hi");
        assert_eq!(format_summary("hi", &serde_json::Value::Null), "hi");
    }

    #[test]
    fn non_empty_fields_are_appended() {
        let summary = format_summary("changed", &json!({"count": 5}));
        assert!(summary.starts_with("changed "));
        assert!(summary.contains("\"count\":5"));
    }
}
