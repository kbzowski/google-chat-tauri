use std::str::FromStr;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::features::config;
use crate::features::window;

pub fn register(app: &AppHandle, shortcut_str: &str) -> Result<(), String> {
    let shortcut = Shortcut::from_str(shortcut_str).map_err(|e| e.to_string())?;
    let handle = app.clone();
    let _ = app.global_shortcut().unregister_all();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _sc, event| {
            if event.state() == ShortcutState::Pressed {
                window::toggle_main_window(&handle);
            }
        })
        .map_err(|e| e.to_string())
}

pub fn re_register_from_config(app: &AppHandle) {
    let settings = config::load(app);
    if settings.global_shortcut.is_empty() {
        return;
    }
    if let Err(err) = register(app, &settings.global_shortcut) {
        log::warn!(target: "shortcuts", "Failed to register {}: {err}", settings.global_shortcut);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_well_known_shortcut() {
        assert!(Shortcut::from_str("CmdOrCtrl+Shift+G").is_ok());
        assert!(Shortcut::from_str("Alt+F4").is_ok());
    }

    #[test]
    fn rejects_garbage_shortcut() {
        assert!(Shortcut::from_str("not a shortcut !!").is_err());
    }
}
