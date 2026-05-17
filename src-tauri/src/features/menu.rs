use tauri::menu::{Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
use tauri::{AppHandle, Manager, Wry};

use crate::features::window::MAIN_WINDOW_LABEL;

const LOGOUT_URL: &str = "https://accounts.google.com/Logout";

pub fn build(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let file = SubmenuBuilder::new(app, "File")
        .item(&MenuItem::with_id(
            app,
            "close-to-tray",
            "Close to Tray",
            true,
            Some("CmdOrCtrl+W"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "relaunch",
            "Relaunch",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::minimize(app, None)?)
        .item(&MenuItem::with_id(
            app,
            "sign-out",
            "Sign Out",
            true,
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "preferences",
            "Preferences",
            true,
            Some("CmdOrCtrl+,"),
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "quit",
            "Quit",
            true,
            Some("CmdOrCtrl+Q"),
        )?)
        .build()?;

    let help = SubmenuBuilder::new(app, "Help")
        .item(&MenuItem::with_id(
            app,
            "help-check-updates",
            "Check For Updates",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-shortcuts",
            "Keyboard Shortcuts",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-devtools",
            "Open DevTools",
            true,
            Some("F12"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-show-logs",
            "Show Logs in File Manager",
            true,
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "help-about",
            "About",
            true,
            None::<&str>,
        )?)
        .build()?;

    MenuBuilder::new(app).items(&[&file, &help]).build()
}

#[tauri::command]
pub fn toggle_main_menu(app: AppHandle) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let visible = w.is_menu_visible().unwrap_or(false);
        if visible {
            let _ = w.hide_menu();
        } else {
            let _ = w.show_menu();
        }
    }
}

pub fn handle_event(app: &AppHandle, event_id: &str) {
    let window = app.get_webview_window(MAIN_WINDOW_LABEL);
    match event_id {
        "close-to-tray" => {
            if let Some(w) = window {
                let _ = w.hide();
            }
        }
        "relaunch" => {
            app.restart();
        }
        "sign-out" => {
            if let Some(w) = window {
                let _ = w.eval(format!("window.location.href='{LOGOUT_URL}'"));
            }
        }
        "quit" => {
            app.exit(0);
        }
        "preferences" => {
            let _ = crate::features::windows::open_settings(app.clone());
        }
        "help-about" => {
            let _ = crate::features::windows::open_about(app.clone());
        }
        "help-shortcuts" => {
            let _ = crate::features::windows::open_shortcuts(app.clone());
        }
        "help-devtools" => {
            if let Some(w) = window {
                w.open_devtools();
            }
        }
        "help-show-logs" => {
            if let Ok(dir) = app.path().app_log_dir() {
                use tauri_plugin_opener::OpenerExt;
                let _ = app.opener().open_path(dir.to_string_lossy(), None::<&str>);
            }
        }
        "help-check-updates" => {
            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                crate::features::updater::check_and_prompt(handle, true).await;
            });
        }
        _ => {}
    }
}
