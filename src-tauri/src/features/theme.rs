use tauri::{AppHandle, Manager, Theme as TauriTheme};

use crate::features::config::{load, Theme};
use crate::features::window::MAIN_WINDOW_LABEL;

pub fn theme_to_tauri(theme: Theme) -> Option<TauriTheme> {
    match theme {
        Theme::System => None,
        Theme::Light => Some(TauriTheme::Light),
        Theme::Dark => Some(TauriTheme::Dark),
    }
}

pub fn apply(app: &AppHandle, theme: Theme) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.set_theme(theme_to_tauri(theme));
    }
}

pub fn apply_from_config(app: &AppHandle) {
    let settings = load(app);
    apply(app, settings.theme);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_mapping() {
        assert_eq!(theme_to_tauri(Theme::System), None);
        assert_eq!(theme_to_tauri(Theme::Light), Some(TauriTheme::Light));
        assert_eq!(theme_to_tauri(Theme::Dark), Some(TauriTheme::Dark));
    }
}
