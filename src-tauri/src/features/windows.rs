use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

fn ensure_window(app: &AppHandle, label: &str, title: &str, size: (f64, f64)) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }
    WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .title(title)
        .inner_size(size.0, size.1)
        .resizable(true)
        .center()
        .build()?;
    Ok(())
}

#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    ensure_window(&app, "settings", "Settings", (640.0, 720.0)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_about(app: AppHandle) -> Result<(), String> {
    ensure_window(&app, "about", "About google-chat-tauri", (480.0, 480.0))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_offline(app: AppHandle) -> Result<(), String> {
    ensure_window(&app, "offline", "Offline", (480.0, 360.0)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_shortcuts(app: AppHandle) -> Result<(), String> {
    ensure_window(&app, "shortcuts", "Keyboard Shortcuts", (480.0, 540.0))
        .map_err(|e| e.to_string())
}
