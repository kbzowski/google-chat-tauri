// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;

use features::user_agent::USER_AGENT;
use features::window;

const INJECTION_SCRIPT: &str = include_str!("../injection.js");

fn chat_url(account_index: u32) -> String {
    format!("https://mail.google.com/chat/u/{account_index}")
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            window::toggle_main_window(app);
        }))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--start-hidden"]),
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            features::online::check_if_online,
            features::config::get_settings,
            features::config::set_settings,
            features::config::get_custom_css,
            features::debug_info::get_debug_info,
            features::zoom::get_zoom_level,
            features::zoom::set_zoom_level,
            features::windows::open_settings,
            features::windows::open_about,
            features::windows::open_offline,
            features::windows::open_shortcuts,
            features::focus_mode::enable_focus_mode,
            features::focus_mode::disable_focus_mode,
            features::focus_mode::is_focus_mode_active,
            features::updater::check_for_updates_command,
        ])
        .setup(|app| {
            let settings = features::config::load(app.handle());
            let url = chat_url(settings.account_index)
                .parse()
                .expect("invalid Google Chat URL");
            let app_handle = app.handle().clone();
            let window = tauri::WebviewWindowBuilder::new(
                app,
                window::MAIN_WINDOW_LABEL,
                tauri::WebviewUrl::External(url),
            )
            .title("Google Chat")
            .inner_size(1280.0, 900.0)
            .center()
            .user_agent(USER_AGENT)
            .initialization_script(INJECTION_SCRIPT)
            .on_navigation(move |url| {
                if features::external_links::is_whitelisted(url) {
                    log::debug!(target: "navigation", "ALLOW {}", url.as_str());
                    true
                } else {
                    log::info!(target: "navigation", "EXTERNAL {}", url.as_str());
                    use tauri_plugin_opener::OpenerExt;
                    let _ = app_handle.opener().open_url(url.as_str(), None::<&str>);
                    false
                }
            })
            .on_page_load(|window, payload| {
                if matches!(payload.event(), tauri::webview::PageLoadEvent::Finished) {
                    features::taskbar::show_normal(&window);
                }
            })
            .build()?;

            window::attach_close_to_tray(&window);
            window::apply_initial_state(&window, &settings);
            window::attach_minimize_to_tray(&window, app.handle().clone());
            features::taskbar::show_loading(&window);
            features::tray::build_tray(app)?;
            features::notifications::setup_click_handler(app.handle());
            features::badge::setup_listener(app.handle());
            features::injection_log::setup_listener(app.handle());
            features::crash_report::install_hook(app.handle().clone());
            features::shortcuts::re_register_from_config(app.handle());
            features::theme::apply_from_config(app.handle());
            features::watchdog::spawn_online_watchdog(app.handle().clone());
            features::watchdog::setup_heartbeat_listener(app.handle());
            features::watchdog::spawn_stuck_watchdog(app.handle().clone());
            features::updater::spawn_startup_check(app.handle().clone());

            {
                use tauri::{Listener, Manager};
                let handle = app.handle().clone();
                app.listen("apply-shortcut", move |event| {
                    if let Ok(shortcut) = serde_json::from_str::<String>(event.payload()) {
                        if let Err(err) = features::shortcuts::register(&handle, &shortcut) {
                            log::warn!(target: "shortcuts", "re-register failed: {err}");
                        }
                    }
                });
                let handle = app.handle().clone();
                app.listen("apply-theme", move |event| {
                    if let Ok(theme) =
                        serde_json::from_str::<features::config::Theme>(event.payload())
                    {
                        features::theme::apply(&handle, theme);
                    }
                });
                let handle = app.handle().clone();
                app.listen("apply-always-on-top", move |event| {
                    if let Ok(value) = serde_json::from_str::<bool>(event.payload()) {
                        if let Some(window) =
                            handle.get_webview_window(features::window::MAIN_WINDOW_LABEL)
                        {
                            let _ = window.set_always_on_top(value);
                        }
                    }
                });
            }

            let menu = features::menu::build(app.handle())?;
            app.set_menu(menu)?;

            if window::should_start_hidden(std::env::args()) {
                let _ = window.hide();
            }

            Ok(())
        })
        .on_menu_event(|app, event| {
            features::menu::handle_event(app, event.id.as_ref());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_url_uses_account_index() {
        assert_eq!(chat_url(0), "https://mail.google.com/chat/u/0");
        assert_eq!(chat_url(1), "https://mail.google.com/chat/u/1");
        assert_eq!(chat_url(9), "https://mail.google.com/chat/u/9");
    }
}
