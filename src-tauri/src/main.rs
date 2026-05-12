// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod features;

use features::user_agent::USER_AGENT;

const GOOGLE_CHAT_URL: &str = "https://mail.google.com/chat/u/0";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let url = GOOGLE_CHAT_URL.parse().expect("invalid Google Chat URL");
            tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::External(url))
                .title("Google Chat")
                .inner_size(1280.0, 900.0)
                .center()
                .user_agent(USER_AGENT)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
