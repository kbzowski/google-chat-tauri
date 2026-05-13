use tauri::menu::{Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
use tauri::{AppHandle, Emitter, Manager, Wry};

use crate::features::config::{load, save};
use crate::features::window::MAIN_WINDOW_LABEL;
use crate::features::zoom::{clamp, ZOOM_STEP};

const LOGOUT_URL: &str = "https://accounts.google.com/Logout";
const HOME_URL: &str = "https://mail.google.com/chat/u/0";

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
            "quit",
            "Quit",
            true,
            Some("CmdOrCtrl+Q"),
        )?)
        .build()?;

    let edit = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let view = SubmenuBuilder::new(app, "View")
        .item(&MenuItem::with_id(
            app,
            "reload",
            "Reload",
            true,
            Some("CmdOrCtrl+R"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "search",
            "Search",
            true,
            Some("CmdOrCtrl+F"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "copy-url",
            "Copy Current URL",
            true,
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "toggle-fullscreen",
            "Toggle Fullscreen",
            true,
            Some("F11"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "reset-zoom",
            "Reset Zoom",
            true,
            Some("CmdOrCtrl+0"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "zoom-in",
            "Zoom In",
            true,
            Some("CmdOrCtrl+Plus"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "zoom-out",
            "Zoom Out",
            true,
            Some("CmdOrCtrl+-"),
        )?)
        .build()?;

    let history = SubmenuBuilder::new(app, "History")
        .item(&MenuItem::with_id(
            app,
            "back",
            "Back",
            true,
            Some("Alt+Left"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "forward",
            "Forward",
            true,
            Some("Alt+Right"),
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "home",
            "Navigate to Home",
            true,
            Some("Alt+Home"),
        )?)
        .build()?;

    let prefs = SubmenuBuilder::new(app, "Preferences")
        .item(&MenuItem::with_id(
            app,
            "pref-auto-update",
            "Auto check for Updates",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "pref-autostart",
            "Auto Launch at Login",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "pref-start-hidden",
            "Start Hidden",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "pref-show-on-message",
            "Show window on message",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "pref-hide-menu-bar",
            "Hide Menu Bar",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "pref-disable-spellcheck",
            "Disable Spell Checker",
            true,
            None::<&str>,
        )?)
        .build()?;

    let help = SubmenuBuilder::new(app, "Help")
        .item(&MenuItem::with_id(
            app,
            "help-thanks",
            "Say Thanks to Developer",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-check-updates",
            "Check For Updates",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-report-issue",
            "Report Issue",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-toggle-guard",
            "Toggle External Links Guard",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-demo-badge",
            "Demo Badge Count",
            true,
            None::<&str>,
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "help-shortcuts",
            "Keyboard Shortcuts",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-offline",
            "Show Offline Page",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-show-logs",
            "Show Logs in File Manager",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-devtools",
            "Open DevTools",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-reset",
            "Reset and Relaunch App",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            "help-about",
            "About",
            true,
            None::<&str>,
        )?)
        .build()?;

    MenuBuilder::new(app)
        .items(&[&file, &edit, &view, &history, &prefs, &help])
        .build()
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
        "reload" => {
            if let Some(w) = window {
                let _ = w.eval("window.location.reload()");
            }
        }
        "toggle-fullscreen" => {
            if let Some(w) = window {
                let is_fs = w.is_fullscreen().unwrap_or(false);
                let _ = w.set_fullscreen(!is_fs);
            }
        }
        "back" => {
            if let Some(w) = window {
                let _ = w.eval("window.history.back()");
            }
        }
        "forward" => {
            if let Some(w) = window {
                let _ = w.eval("window.history.forward()");
            }
        }
        "home" => {
            if let Some(w) = window {
                let _ = w.eval(format!("window.location.href='{HOME_URL}'"));
            }
        }
        "search" => {
            let _ = app.emit("search-shortcut", ());
        }
        "pref-auto-update"
        | "pref-autostart"
        | "pref-start-hidden"
        | "pref-show-on-message"
        | "pref-hide-menu-bar"
        | "pref-disable-spellcheck" => {
            let _ = crate::features::windows::open_settings(app.clone());
        }
        "help-about" => {
            let _ = crate::features::windows::open_about(app.clone());
        }
        "help-offline" => {
            let _ = crate::features::windows::open_offline(app.clone());
        }
        "help-shortcuts" => {
            let _ = crate::features::windows::open_shortcuts(app.clone());
        }
        "help-check-updates" => {
            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                crate::features::updater::check_and_prompt(handle, true).await;
            });
        }
        "zoom-in" | "zoom-out" | "reset-zoom" => {
            let mut settings = load(app);
            settings.zoom_level = match event_id {
                "zoom-in" => clamp(settings.zoom_level + ZOOM_STEP),
                "zoom-out" => clamp(settings.zoom_level - ZOOM_STEP),
                _ => 1.0,
            };
            let _ = save(app, &settings);
            let _ = app.emit("apply-zoom", settings.zoom_level);
        }
        // Wired up in later phases: Preferences (Faza 4), Help dialogs (Faza 4/5),
        // updater (Faza 6).
        _ => {}
    }
}
