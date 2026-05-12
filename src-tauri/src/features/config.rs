use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

pub const STORE_FILE: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub auto_check_for_updates: bool,
    pub auto_launch_at_login: bool,
    pub start_hidden: bool,
    pub hide_menu_bar: bool,
    pub disable_spell_checker: bool,
    pub show_on_message: bool,

    pub theme: Theme,
    pub zoom_level: f64,
    pub always_on_top: bool,
    pub notification_sound: bool,
    pub global_shortcut: String,
    pub focus_mode: bool,
    pub focus_mode_duration: u32,
    pub custom_css: String,
    pub show_unread_in_title: bool,
    pub show_unread_in_tray: bool,

    pub external_links_guard_enabled: bool,
    pub external_links_guard_disabled_until: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_check_for_updates: true,
            auto_launch_at_login: false,
            start_hidden: false,
            hide_menu_bar: false,
            disable_spell_checker: false,
            show_on_message: true,
            theme: Theme::System,
            zoom_level: 1.0,
            always_on_top: false,
            notification_sound: true,
            global_shortcut: "CmdOrCtrl+Shift+G".into(),
            focus_mode: false,
            focus_mode_duration: 30,
            custom_css: String::new(),
            show_unread_in_title: true,
            show_unread_in_tray: true,
            external_links_guard_enabled: true,
            external_links_guard_disabled_until: None,
        }
    }
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> AppSettings {
    let Ok(store) = app.store(STORE_FILE) else {
        return AppSettings::default();
    };
    store
        .get(SETTINGS_KEY)
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

pub fn save<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> Result<(), String> {
    let store = app.store(STORE_FILE).map_err(|e| e.to_string())?;
    let value = serde_json::to_value(settings).map_err(|e| e.to_string())?;
    store.set(SETTINGS_KEY, value);
    Ok(())
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> AppSettings {
    load(&app)
}

#[tauri::command]
pub fn set_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    save(&app, &settings)
}

#[tauri::command]
pub fn get_custom_css(app: AppHandle) -> String {
    load(&app).custom_css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_zoom_in_safe_range() {
        let s = AppSettings::default();
        assert!(s.zoom_level >= 0.5 && s.zoom_level <= 3.0);
    }

    #[test]
    fn default_has_global_shortcut() {
        assert!(!AppSettings::default().global_shortcut.is_empty());
    }

    #[test]
    fn serde_roundtrip_preserves_values() {
        let orig = AppSettings::default();
        let json = serde_json::to_string(&orig).expect("serialize");
        let parsed: AppSettings = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(orig, parsed);
    }

    #[test]
    fn theme_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&Theme::System).unwrap(), "\"system\"");
        assert_eq!(serde_json::to_string(&Theme::Light).unwrap(), "\"light\"");
        assert_eq!(serde_json::to_string(&Theme::Dark).unwrap(), "\"dark\"");
    }

    #[test]
    fn camel_case_serialization() {
        let s = AppSettings::default();
        let json = serde_json::to_string(&s).expect("serialize");
        assert!(json.contains("\"autoCheckForUpdates\""));
        assert!(json.contains("\"externalLinksGuardEnabled\""));
        assert!(!json.contains("\"auto_check_for_updates\""));
    }
}
