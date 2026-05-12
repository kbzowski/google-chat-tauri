// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(
                    "https://mail.google.com/chat/u/0"
                        .parse()
                        .expect("invalid URL"),
                ),
            )
            .title("Google Chat")
            .inner_size(1280.0, 900.0)
            .center()
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
