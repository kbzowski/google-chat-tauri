// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;

use features::user_agent::USER_AGENT;
use features::window;

const GOOGLE_CHAT_URL: &str = "https://mail.google.com/chat/u/0";

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
        .setup(|app| {
            let url = GOOGLE_CHAT_URL.parse().expect("invalid Google Chat URL");
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
            .on_navigation(move |url| {
                if features::external_links::is_whitelisted(url) {
                    true
                } else {
                    use tauri_plugin_opener::OpenerExt;
                    let _ = app_handle.opener().open_url(url.as_str(), None::<&str>);
                    false
                }
            })
            .build()?;

            window::attach_close_to_tray(&window);

            if window::should_start_hidden(std::env::args()) {
                let _ = window.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
