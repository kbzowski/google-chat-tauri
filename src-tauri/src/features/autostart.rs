use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

/// Reconcile the OS launch-at-login registration with the desired state.
/// Idempotent: only touches the registration when it differs.
pub fn apply(app: &AppHandle, enabled: bool) {
    let manager = app.autolaunch();
    let is_enabled = manager.is_enabled().unwrap_or(false);
    let result = match (enabled, is_enabled) {
        (true, false) => manager.enable(),
        (false, true) => manager.disable(),
        _ => return,
    };
    if let Err(err) = result {
        log::warn!(target: "autostart", "failed to set autostart to {enabled}: {err}");
    }
}

pub fn apply_from_config(app: &AppHandle) {
    apply(app, crate::features::config::load(app).auto_launch_at_login);
}
