use std::panic;
use std::sync::OnceLock;

use tauri::{AppHandle, Wry};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_opener::OpenerExt;

const REPO: &str = "kbzowski/google-chat-tauri";

static APP_HANDLE: OnceLock<AppHandle<Wry>> = OnceLock::new();

fn url_encode(input: &str) -> String {
    url::form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

fn build_issue_url(payload: &str, location: &str) -> String {
    let title = format!("Crash: {payload}");
    let body = format!(
        "**Error:** {payload}\n**Location:** {location}\n**App version:** {}\n**OS:** {}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
    );
    format!(
        "https://github.com/{REPO}/issues/new?title={}&body={}",
        url_encode(&title),
        url_encode(&body),
    )
}

pub fn install_hook(app: AppHandle<Wry>) {
    let _ = APP_HANDLE.set(app);
    panic::set_hook(Box::new(|info| {
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or("(unknown panic payload)")
            .to_string();
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "(unknown location)".into());
        log::error!("PANIC at {location}: {payload}");
        let Some(app) = APP_HANDLE.get() else {
            return;
        };
        let report = app.clone();
        let url = build_issue_url(&payload, &location);
        app.dialog()
            .message(format!(
                "google-chat-tauri encountered an unexpected error.\n\n{payload}\nLocation: {location}\n\nClick 'Report Issue' to open GitHub Issues."
            ))
            .title("Unexpected error")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Report Issue".into(),
                "Close".into(),
            ))
            .show(move |confirmed| {
                if confirmed {
                    let _ = report.opener().open_url(&url, None::<&str>);
                }
            });
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encoding_escapes_spaces_and_specials() {
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn issue_url_contains_payload_and_location() {
        let url = build_issue_url("oops", "main.rs:1:2");
        assert!(url.starts_with("https://github.com/kbzowski/google-chat-tauri/issues/new"));
        assert!(url.contains("Crash%3A+oops"));
        assert!(url.contains("main.rs%3A1%3A2"));
    }
}
