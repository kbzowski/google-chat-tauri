use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Deserialize;
use tauri::{AppHandle, Listener, Manager};

use crate::features::config;
use crate::features::window::MAIN_WINDOW_LABEL;

pub const EVENT_MESSAGE: &str = "notification-message";

/// AppUserModelID for toast notifications. Must match the bundle identifier the
/// installers stamp on the Start Menu shortcut, otherwise toasts don't display.
#[cfg(all(target_os = "windows", not(debug_assertions)))]
const APP_ID: &str = "com.google-chat-tauri.app";

/// Friendly name Windows shows as the toast's app attribution. Without this,
/// the attribution defaults to the Start Menu shortcut name (`productName`,
/// i.e. the "google-chat-tauri" slug).
#[cfg(all(target_os = "windows", not(debug_assertions)))]
const APP_DISPLAY_NAME: &str = "Google Chat";

/// Timestamp of the last content-driven toast. The count-driven fallback in
/// `badge.rs` checks this so it stays quiet when real message content already
/// produced a toast.
static LAST_CONTENT_TOAST: Mutex<Option<Instant>> = Mutex::new(None);

#[derive(Deserialize)]
struct MessagePayload {
    title: String,
    #[serde(default)]
    body: String,
}

/// Bring the main Google Chat window to the foreground. A notification click
/// should always surface it, so unlike `window::toggle_main_window` this never
/// hides.
pub fn focus_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn mark_content_notified(now: Instant) {
    *LAST_CONTENT_TOAST
        .lock()
        .expect("content-toast mutex poisoned") = Some(now);
}

/// Whether a content-driven toast fired within `window` before `now`.
pub fn content_notified_within(window: Duration, now: Instant) -> bool {
    LAST_CONTENT_TOAST
        .lock()
        .expect("content-toast mutex poisoned")
        .map(|t| now.duration_since(t) < window)
        .unwrap_or(false)
}

/// Listen for `notification-message` emitted by the injection's Notification
/// patch (the real sender + body Google Chat hands to `new Notification`).
pub fn setup_message_listener(app: &AppHandle) {
    let handle = app.clone();
    app.listen(EVENT_MESSAGE, move |event| {
        let Ok(payload) = serde_json::from_str::<MessagePayload>(event.payload()) else {
            log::warn!(target: "notifications", "invalid message payload: {}", event.payload());
            return;
        };
        if !config::load(&handle).show_on_message {
            return;
        }
        mark_content_notified(Instant::now());
        let sound = config::load(&handle).notification_sound;
        show_message(&handle, &payload.title, &payload.body, sound);
    });
}

/// AppUserModelID used for the WinRT toast. A toast only displays and routes
/// activation when its app id matches a Start Menu shortcut's AUMID, which the
/// installers register with the bundle identifier. During `tauri dev` no such
/// shortcut exists, so fall back to the always-present PowerShell AUMID.
#[cfg(target_os = "windows")]
fn app_id() -> &'static str {
    #[cfg(debug_assertions)]
    {
        tauri_winrt_notification::Toast::POWERSHELL_APP_ID
    }
    #[cfg(not(debug_assertions))]
    {
        APP_ID
    }
}

/// Register the AUMID's friendly display name so Windows shows "Google Chat" as
/// the toast attribution instead of the raw `productName` slug. Writes to
/// `HKCU\Software\Classes\AppUserModelId\{APP_ID}` — the location the toast
/// platform reads for a custom-AUMID desktop app. Best-effort: a failure only
/// means the attribution falls back to the shortcut name.
///
/// Only in release builds: debug builds toast under the shared PowerShell AUMID
/// (see `app_id`), which must not be relabeled.
#[cfg(all(target_os = "windows", not(debug_assertions)))]
pub fn register_display_name() {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let path = format!("Software\\Classes\\AppUserModelId\\{APP_ID}");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.create_subkey(&path) {
        Ok((key, _)) => {
            if let Err(err) = key.set_value("DisplayName", &APP_DISPLAY_NAME) {
                log::warn!(target: "notifications", "failed to set toast DisplayName: {err}");
            }
        }
        Err(err) => {
            log::warn!(target: "notifications", "failed to open AUMID registry key: {err}");
        }
    }
}

#[cfg(not(all(target_os = "windows", not(debug_assertions))))]
pub fn register_display_name() {}

/// Show a native toast carrying the real sender (`title`) and message preview
/// (`body`). Falls back to a generic title when the sender is empty.
#[cfg(target_os = "windows")]
pub fn show_message(app: &AppHandle, title: &str, body: &str, with_sound: bool) {
    use tauri_winrt_notification::{Sound, Toast};

    let handle = app.clone();
    let title = if title.is_empty() {
        "Google Chat"
    } else {
        title
    };

    let toast = Toast::new(app_id())
        .title(title)
        .text1(body)
        .sound(if with_sound {
            Some(Sound::Default)
        } else {
            None
        })
        .on_activated(move |_action| {
            focus_main_window(&handle);
            Ok(())
        });

    if let Err(err) = toast.show() {
        log::warn!(target: "notifications", "failed to show toast: {err}");
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_message(_app: &AppHandle, _title: &str, _body: &str, _with_sound: bool) {}

/// Generic fallback body when only an unread count is known (no message content
/// was captured).
fn body_for_count(count: i64) -> String {
    if count <= 1 {
        "New message".to_string()
    } else {
        format!("{count} new messages")
    }
}

/// Count-driven fallback toast, used only when no content-driven toast fired.
#[cfg(target_os = "windows")]
pub fn show_new_message(app: &AppHandle, count: i64) {
    let with_sound = config::load(app).notification_sound;
    show_message(app, "Google Chat", &body_for_count(count), with_sound);
}

#[cfg(not(target_os = "windows"))]
pub fn show_new_message(_app: &AppHandle, _count: i64) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_singular_for_one_or_fewer() {
        assert_eq!(body_for_count(1), "New message");
        assert_eq!(body_for_count(0), "New message");
    }

    #[test]
    fn body_plural_includes_count() {
        assert_eq!(body_for_count(2), "2 new messages");
        assert_eq!(body_for_count(99), "99 new messages");
    }

    #[test]
    fn content_window_tracks_recent_marks() {
        let base = Instant::now();
        mark_content_notified(base);
        assert!(content_notified_within(
            Duration::from_secs(3),
            base + Duration::from_secs(1)
        ));
        assert!(!content_notified_within(
            Duration::from_secs(3),
            base + Duration::from_secs(4)
        ));
    }
}
