use tauri::{AppHandle, Listener, Manager};

use crate::features::window::MAIN_WINDOW_LABEL;

pub const EVENT_CLICKED: &str = "notification-clicked";

/// Bring the main Google Chat window to the foreground.
///
/// Shared by the legacy `notification-clicked` web bridge and the native
/// Windows toast activation (`on_activated`). Unlike `window::toggle_main_window`
/// this never hides the window — a notification click should always surface it.
pub fn focus_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Listen for the `notification-clicked` event emitted by the injection script.
///
/// Google Chat currently delivers notifications via a Service Worker, so this
/// path rarely fires; it is kept as a harmless fallback. New-message toasts are
/// produced natively by [`show_new_message`].
pub fn setup_click_handler(app: &AppHandle) {
    let handle = app.clone();
    app.listen(EVENT_CLICKED, move |_event| {
        focus_main_window(&handle);
    });
}

/// Notification body derived from the unread count.
fn body_for_count(count: i64) -> String {
    if count <= 1 {
        "New message".to_string()
    } else {
        format!("{count} new messages")
    }
}

/// AppUserModelID used for the WinRT toast.
///
/// A toast only displays and routes activation when its app id matches a Start
/// Menu shortcut's AUMID. Installers (NSIS/MSI) register that shortcut with the
/// bundle identifier. During `tauri dev` no such shortcut exists, so fall back
/// to the always-present PowerShell AUMID.
#[cfg(target_os = "windows")]
fn app_id() -> &'static str {
    #[cfg(debug_assertions)]
    {
        tauri_winrt_notification::Toast::POWERSHELL_APP_ID
    }
    #[cfg(not(debug_assertions))]
    {
        "com.google-chat-tauri.app"
    }
}

/// Show a native Windows toast for a newly arrived message. Clicking it brings
/// the main window forward. Audio respects the `notification_sound` setting.
#[cfg(target_os = "windows")]
pub fn show_new_message(app: &AppHandle, count: i64) {
    use tauri_winrt_notification::{Sound, Toast};

    let handle = app.clone();
    let with_sound = crate::features::config::load(app).notification_sound;

    let toast = Toast::new(app_id())
        .title("Google Chat")
        .text1(&body_for_count(count))
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

/// No-op on non-Windows targets (the crate is Windows-only); the app itself is
/// Windows-only, this keeps the module portable for tooling.
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
}
