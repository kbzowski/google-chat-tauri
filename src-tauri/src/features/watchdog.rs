use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Listener, Manager};

use crate::features::{online, taskbar, windows};

const POLL_INTERVAL: Duration = Duration::from_secs(30);
const HEARTBEAT_EVENT: &str = "webview-heartbeat";
const STUCK_THRESHOLD: Duration = Duration::from_secs(30);
const STUCK_CHECK_INTERVAL: Duration = Duration::from_secs(10);

static LAST_HEARTBEAT: Mutex<Option<Instant>> = Mutex::new(None);

pub fn setup_heartbeat_listener(app: &AppHandle) {
    app.listen(HEARTBEAT_EVENT, |_event| {
        *LAST_HEARTBEAT.lock().expect("heartbeat mutex") = Some(Instant::now());
    });
}

pub fn spawn_stuck_watchdog(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(STUCK_CHECK_INTERVAL).await;
            let stuck = {
                let guard = LAST_HEARTBEAT.lock().expect("heartbeat mutex");
                guard
                    .map(|t| t.elapsed() > STUCK_THRESHOLD)
                    .unwrap_or(false)
            };
            if !stuck {
                continue;
            }
            if let Some(window) = app.get_webview_window(crate::features::window::MAIN_WINDOW_LABEL)
            {
                log::warn!(target: "watchdog", "Webview heartbeat stalled, reloading");
                let _ = window.eval("window.location.reload()");
                *LAST_HEARTBEAT.lock().expect("heartbeat mutex") = Some(Instant::now());
            }
        }
    });
}

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
