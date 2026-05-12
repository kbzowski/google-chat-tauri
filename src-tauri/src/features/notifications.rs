use tauri::{AppHandle, Listener, Manager};

use crate::features::window::MAIN_WINDOW_LABEL;

pub const EVENT_CLICKED: &str = "notification-clicked";

pub fn setup_click_handler(app: &AppHandle) {
    let handle = app.clone();
    app.listen(EVENT_CLICKED, move |_event| {
        if let Some(window) = handle.get_webview_window(MAIN_WINDOW_LABEL) {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    });
}
