use std::sync::Mutex;
use std::time::Instant;

use serde::Deserialize;
use tauri::{AppHandle, Listener, Manager};

use crate::features::tray::{self, TrayState};
use crate::features::window::{self, MAIN_WINDOW_LABEL};
use crate::features::{config, focus_mode, notifications};

pub const EVENT_UNREAD: &str = "unread-count";

/// Last seen unread count. `None` until the first event so the initial load
/// (count jumping 0 -> N for already-existing unread chats) does not fire a
/// spurious notification.
static PREV_COUNT: Mutex<Option<i64>> = Mutex::new(None);

#[derive(Deserialize)]
struct UnreadPayload {
    count: i64,
}

pub fn parse_count(payload: &str) -> i64 {
    serde_json::from_str::<UnreadPayload>(payload)
        .map(|p| p.count)
        .unwrap_or(0)
}

/// A toast is warranted only when the count strictly increased over a known
/// previous value. The first event (`None`) just establishes the baseline.
fn should_notify(prev: Option<i64>, new: i64) -> bool {
    matches!(prev, Some(p) if new > p)
}

pub fn setup_listener(app: &AppHandle) {
    let handle = app.clone();
    app.listen(EVENT_UNREAD, move |event| {
        let count = parse_count(event.payload());
        let settings = config::load(&handle);
        if let Some(window) = handle.get_webview_window(MAIN_WINDOW_LABEL) {
            let badge = if count > 0 { Some(count) } else { None };
            let _ = window.set_badge_count(badge);
            if settings.show_unread_in_title {
                window::update_title(&window, count);
            }
        }
        if settings.show_unread_in_tray {
            let _ = tray::set_state(&handle, TrayState::from_unread(count));
        }

        let prev = PREV_COUNT
            .lock()
            .expect("badge prev-count mutex poisoned")
            .replace(count);
        if should_notify(prev, count)
            && settings.show_on_message
            && !focus_mode::is_active_now(Instant::now())
        {
            notifications::show_new_message(&handle, count);
        }
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

    #[test]
    fn first_event_only_sets_baseline() {
        assert!(!should_notify(None, 0));
        assert!(!should_notify(None, 5));
    }

    #[test]
    fn notifies_only_on_increase() {
        assert!(should_notify(Some(0), 1));
        assert!(should_notify(Some(3), 4));
    }

    #[test]
    fn no_notify_on_equal_or_decrease() {
        assert!(!should_notify(Some(5), 5));
        assert!(!should_notify(Some(5), 2));
        assert!(!should_notify(Some(5), 0));
    }
}
