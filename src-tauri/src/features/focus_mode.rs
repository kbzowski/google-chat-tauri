use std::sync::Mutex;
use std::time::{Duration, Instant};

static STATE: Mutex<Option<Instant>> = Mutex::new(None);

pub fn is_active_now(now: Instant) -> bool {
    let mut guard = STATE.lock().expect("focus_mode mutex poisoned");
    match *guard {
        Some(until) if now < until => true,
        Some(_) => {
            *guard = None;
            false
        }
        None => false,
    }
}

pub fn enable_for(now: Instant, duration_minutes: u32) {
    let until = now + Duration::from_secs(60 * u64::from(duration_minutes));
    *STATE.lock().expect("focus_mode mutex poisoned") = Some(until);
}

pub fn disable() {
    *STATE.lock().expect("focus_mode mutex poisoned") = None;
}

#[tauri::command]
pub fn enable_focus_mode(duration_minutes: u32) {
    enable_for(Instant::now(), duration_minutes);
}

#[tauri::command]
pub fn disable_focus_mode() {
    disable();
}

#[tauri::command]
pub fn is_focus_mode_active() -> bool {
    is_active_now(Instant::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reset() {
        disable();
    }

    #[test]
    fn inactive_by_default() {
        reset();
        assert!(!is_active_now(Instant::now()));
    }

    #[test]
    fn enable_then_query_within_window_returns_true() {
        reset();
        let now = Instant::now();
        enable_for(now, 25);
        assert!(is_active_now(now + Duration::from_secs(60)));
    }

    #[test]
    fn elapsed_window_clears_state() {
        reset();
        let now = Instant::now();
        enable_for(now, 1);
        assert!(!is_active_now(now + Duration::from_secs(120)));
        assert!(!is_active_now(Instant::now()));
    }

    #[test]
    fn disable_clears_state() {
        let now = Instant::now();
        enable_for(now, 60);
        disable();
        assert!(!is_active_now(now + Duration::from_secs(1)));
    }
}
