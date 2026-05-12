use serde::Deserialize;
use tauri::{AppHandle, Listener, Manager};

use crate::features::tray::{self, TrayState};
use crate::features::window::MAIN_WINDOW_LABEL;

pub const EVENT_UNREAD: &str = "unread-count";

#[derive(Deserialize)]
struct UnreadPayload {
    count: i64,
}

pub fn parse_count(payload: &str) -> i64 {
    serde_json::from_str::<UnreadPayload>(payload)
        .map(|p| p.count)
        .unwrap_or(0)
}

pub fn setup_listener(app: &AppHandle) {
    let handle = app.clone();
    app.listen(EVENT_UNREAD, move |event| {
        let count = parse_count(event.payload());
        if let Some(window) = handle.get_webview_window(MAIN_WINDOW_LABEL) {
            let badge = if count > 0 { Some(count) } else { None };
            let _ = window.set_badge_count(badge);
        }
        let _ = tray::set_state(&handle, TrayState::from_unread(count));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_payload() {
        assert_eq!(parse_count(r#"{"count":5}"#), 5);
        assert_eq!(parse_count(r#"{"count":0}"#), 0);
        assert_eq!(parse_count(r#"{"count":99}"#), 99);
    }

    #[test]
    fn defaults_to_zero_on_invalid_payload() {
        assert_eq!(parse_count(""), 0);
        assert_eq!(parse_count("not json"), 0);
        assert_eq!(parse_count(r#"{"other":1}"#), 0);
    }

    #[test]
    fn handles_negative_count() {
        assert_eq!(parse_count(r#"{"count":-1}"#), -1);
    }
}
