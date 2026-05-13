use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};

use crate::features::{online, taskbar, windows};

const POLL_INTERVAL: Duration = Duration::from_secs(30);

pub fn spawn_online_watchdog(app: AppHandle) {
    let was_online = Arc::new(AtomicBool::new(true));
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(POLL_INTERVAL).await;
            let now_online = online::check_connectivity().await;
            let prev = was_online.swap(now_online, Ordering::SeqCst);
            if prev == now_online {
                continue;
            }
            let event = if now_online {
                "network-online"
            } else {
                "network-offline"
            };
            log::info!(target: "watchdog", "Connectivity changed: {event}");
            let _ = app.emit(event, ());
            if now_online {
                if let Some(window) = app.get_webview_window("offline") {
                    let _ = window.close();
                }
                if let Some(main) =
                    app.get_webview_window(crate::features::window::MAIN_WINDOW_LABEL)
                {
                    taskbar::show_normal(&main);
                }
            } else {
                let _ = windows::open_offline(app.clone());
                if let Some(main) =
                    app.get_webview_window(crate::features::window::MAIN_WINDOW_LABEL)
                {
                    taskbar::show_offline(&main);
                }
            }
        }
    });
}
