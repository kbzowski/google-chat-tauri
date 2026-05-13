use tauri::window::{ProgressBarState, ProgressBarStatus};
use tauri::WebviewWindow;

fn set(window: &WebviewWindow, status: ProgressBarStatus, progress: Option<u64>) {
    let _ = window.set_progress_bar(ProgressBarState {
        status: Some(status),
        progress,
    });
}

pub fn show_loading(window: &WebviewWindow) {
    set(window, ProgressBarStatus::Indeterminate, None);
}

pub fn show_normal(window: &WebviewWindow) {
    set(window, ProgressBarStatus::None, None);
}

#[allow(dead_code)] // wired up in F5-C8 (online watchdog)
pub fn show_offline(window: &WebviewWindow) {
    set(window, ProgressBarStatus::Paused, Some(100));
}
