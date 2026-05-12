// Some items here (TrayState variants, set_state) are wired up in subsequent
// commits (badge listener, online detection). Allow dead_code at module scope
// to keep clippy happy until those land.
#![allow(dead_code, unused_imports)]

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager};

use crate::features::window;

const ICON_NORMAL: &[u8] = include_bytes!("../../icons/tray/normal.png");
const ICON_BADGE: &[u8] = include_bytes!("../../icons/tray/badge.png");
const ICON_OFFLINE: &[u8] = include_bytes!("../../icons/tray/offline.png");

pub const TRAY_ID: &str = "main-tray";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayState {
    Normal,
    Badge,
    Offline,
}

impl TrayState {
    fn bytes(&self) -> &'static [u8] {
        match self {
            Self::Normal => ICON_NORMAL,
            Self::Badge => ICON_BADGE,
            Self::Offline => ICON_OFFLINE,
        }
    }

    pub fn from_unread(count: i64) -> Self {
        if count > 0 {
            Self::Badge
        } else {
            Self::Normal
        }
    }
}

pub fn build_tray(app: &App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "tray-show", "Show Google Chat", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "tray-quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;
    let icon = Image::from_bytes(ICON_NORMAL)?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .tooltip("Google Chat")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "tray-show" => window::toggle_main_window(app),
            "tray-quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                window::toggle_main_window(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

pub fn set_state(app: &AppHandle, state: TrayState) -> tauri::Result<()> {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let icon = Image::from_bytes(state.bytes())?;
        tray.set_icon(Some(icon))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unread_count_maps_to_state() {
        assert_eq!(TrayState::from_unread(0), TrayState::Normal);
        assert_eq!(TrayState::from_unread(1), TrayState::Badge);
        assert_eq!(TrayState::from_unread(99), TrayState::Badge);
    }

    #[test]
    fn negative_unread_treated_as_normal() {
        assert_eq!(TrayState::from_unread(-1), TrayState::Normal);
        assert_eq!(TrayState::from_unread(-100), TrayState::Normal);
    }

    #[test]
    fn each_state_has_non_empty_bytes() {
        assert!(!TrayState::Normal.bytes().is_empty());
        assert!(!TrayState::Badge.bytes().is_empty());
        assert!(!TrayState::Offline.bytes().is_empty());
    }
}
