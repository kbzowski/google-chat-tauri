use serde::Serialize;

use crate::features::user_agent;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugInfo {
    app_version: &'static str,
    firefox_version: &'static str,
    user_agent: &'static str,
    build_date: &'static str,
    platform: String,
    webview_version: String,
}

#[tauri::command]
pub fn get_debug_info() -> DebugInfo {
    DebugInfo {
        app_version: env!("CARGO_PKG_VERSION"),
        firefox_version: user_agent::FIREFOX_VERSION,
        user_agent: user_agent::USER_AGENT,
        build_date: user_agent::BUILD_DATE,
        platform: std::env::consts::OS.to_string(),
        webview_version: tauri::webview_version().unwrap_or_else(|_| "unknown".into()),
    }
}
