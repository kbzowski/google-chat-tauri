use tauri::AppHandle;

use crate::features::config::{load, save};

pub const ZOOM_MIN: f64 = 0.5;
pub const ZOOM_MAX: f64 = 3.0;
pub const ZOOM_STEP: f64 = 0.1;

pub fn clamp(value: f64) -> f64 {
    value.clamp(ZOOM_MIN, ZOOM_MAX)
}

#[tauri::command]
pub fn get_zoom_level(app: AppHandle) -> f64 {
    load(&app).zoom_level
}

#[tauri::command]
pub fn set_zoom_level(app: AppHandle, level: f64) -> Result<(), String> {
    let mut settings = load(&app);
    settings.zoom_level = clamp(level);
    save(&app, &settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_below_minimum() {
        assert_eq!(clamp(0.1), ZOOM_MIN);
    }

    #[test]
    fn clamps_above_maximum() {
        assert_eq!(clamp(10.0), ZOOM_MAX);
    }

    #[test]
    fn passes_through_in_range() {
        assert!((clamp(1.5) - 1.5).abs() < f64::EPSILON);
    }
}
