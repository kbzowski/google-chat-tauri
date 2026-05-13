use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn check_and_prompt(app: AppHandle, manual: bool) {
    let updater = match app.updater() {
        Ok(u) => u,
        Err(err) => {
            log::warn!(target: "updater", "Updater not available: {err}");
            return;
        }
    };
    let update = match updater.check().await {
        Ok(Some(u)) => u,
        Ok(None) => {
            log::info!(target: "updater", "No update available");
            if manual {
                let _ = app
                    .dialog()
                    .message("You're up to date.")
                    .title("Check for Updates")
                    .blocking_show();
            }
            return;
        }
        Err(err) => {
            log::warn!(target: "updater", "Check failed: {err}");
            if manual {
                let _ = app
                    .dialog()
                    .message(format!("Update check failed: {err}"))
                    .title("Check for Updates")
                    .blocking_show();
            }
            return;
        }
    };

    let version = update.version.clone();
    log::info!(target: "updater", "Update available: {version}");
    let body = format!("Version {version} is available. Install now?");
    let confirmed = app
        .dialog()
        .message(body)
        .title("Update available")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Install".into(),
            "Remind Later".into(),
        ))
        .blocking_show();
    if !confirmed {
        return;
    }

    if let Err(err) = update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
    {
        log::error!(target: "updater", "Install failed: {err}");
        let _ = app
            .dialog()
            .message(format!("Update failed: {err}"))
            .title("Update")
            .blocking_show();
        return;
    }
    log::info!(target: "updater", "Update installed, restarting");
    app.restart();
}

#[tauri::command]
pub async fn check_for_updates_command(app: AppHandle) {
    check_and_prompt(app, true).await;
}
