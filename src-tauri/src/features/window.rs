use tauri::{AppHandle, Manager, WebviewWindow, WindowEvent};

use crate::features::config::{self, AppSettings};

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const WINDOW_TITLE: &str = "Google Chat";

pub fn title_with_count(count: i64) -> String {
    if count > 0 {
        format!("({count}) {WINDOW_TITLE}")
    } else {
        WINDOW_TITLE.to_string()
    }
}

pub fn update_title(window: &WebviewWindow, count: i64) {
    let _ = window.set_title(&title_with_count(count));
}

pub fn attach_close_to_tray(window: &WebviewWindow) {
    let win = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = win.hide();
        }
    });
}

pub fn apply_initial_state(window: &WebviewWindow, settings: &AppSettings) {
    let _ = window.set_always_on_top(settings.always_on_top);
}

pub fn attach_minimize_to_tray(window: &WebviewWindow, app: AppHandle) {
    let win = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Resized(_) = event {
            if win.is_minimized().unwrap_or(false) && config::load(&app).minimize_to_tray {
                let _ = win.hide();
            }
        }
    });
}

pub fn toggle_main_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) else {
        return;
    };
    match window.is_visible() {
        Ok(true) => match window.is_focused() {
            Ok(true) => {
                let _ = window.hide();
            }
            _ => {
                let _ = window.set_focus();
            }
        },
        _ => {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

pub fn should_start_hidden(args: impl IntoIterator<Item = String>) -> bool {
    args.into_iter()
        .any(|a| a == "--start-hidden" || a == "--hidden")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_start_hidden_long_flag() {
        let args = vec!["app.exe".into(), "--start-hidden".into()];
        assert!(should_start_hidden(args));
    }

    #[test]
    fn detects_legacy_hidden_flag() {
        let args = vec!["app.exe".into(), "--hidden".into()];
        assert!(should_start_hidden(args));
    }

    #[test]
    fn no_flag_means_visible() {
        let args = vec!["app.exe".into()];
        assert!(!should_start_hidden(args));
    }

    #[test]
    fn unrelated_flag_does_not_match() {
        let args = vec!["app.exe".into(), "--start-something-else".into()];
        assert!(!should_start_hidden(args));
    }

    #[test]
    fn title_zero_count_omits_prefix() {
        assert_eq!(title_with_count(0), "Google Chat");
        assert_eq!(title_with_count(-1), "Google Chat");
    }

    #[test]
    fn title_with_positive_count_uses_parenthesised_prefix() {
        assert_eq!(title_with_count(1), "(1) Google Chat");
        assert_eq!(title_with_count(99), "(99) Google Chat");
    }
}
