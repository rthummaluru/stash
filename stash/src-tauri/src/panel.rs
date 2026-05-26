use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, PhysicalPosition, WebviewWindow};

pub const PANEL_LABEL: &str = "panel";
const POSITION_FILE: &str = "panel-position.json";

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct PanelPosition {
    x: i32,
    y: i32,
}

pub struct PanelState {
    saved_position: Mutex<Option<PanelPosition>>,
}

impl PanelState {
    pub fn new(initial: Option<PanelPosition>) -> Self {
        Self {
            saved_position: Mutex::new(initial),
        }
    }
}

fn position_file_path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_local_data_dir()
        .ok()
        .map(|dir| dir.join(POSITION_FILE))
}

pub fn load_saved_position(app: &AppHandle) -> Option<PanelPosition> {
    if let Some(state) = app.try_state::<PanelState>() {
        if let Ok(saved) = state.saved_position.lock() {
            if let Some(position) = *saved {
                return Some(position);
            }
        }
    }

    load_position_from_file(app)
}

fn load_position_from_file(app: &AppHandle) -> Option<PanelPosition> {
    let path = position_file_path(app)?;
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn initial_panel_state(app: &AppHandle) -> PanelState {
    PanelState::new(load_position_from_file(app))
}

fn persist_position(app: &AppHandle, position: PanelPosition) {
    if let Some(state) = app.try_state::<PanelState>() {
        if let Ok(mut saved) = state.saved_position.lock() {
            *saved = Some(position);
        }
    }

    let Some(path) = position_file_path(app) else {
        return;
    };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(json) = serde_json::to_string(&position) {
        let _ = fs::write(path, json);
    }
}

pub fn save_current_position(app: &AppHandle, window: &WebviewWindow) {
    let Ok(position) = window.outer_position() else {
        return;
    };

    persist_position(
        app,
        PanelPosition {
            x: position.x,
            y: position.y,
        },
    );
}

pub fn apply_panel_vibrancy(window: &WebviewWindow) {
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

        if let Err(e) = apply_vibrancy(
            window,
            NSVisualEffectMaterial::Sidebar,
            Some(NSVisualEffectState::Active),
            Some(12.0),
        ) {
            eprintln!("panel: apply_vibrancy failed: {e}");
        }
    }
}

pub fn position_panel_right_centered(window: &WebviewWindow) -> Result<(), String> {
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or_else(|| window.primary_monitor().ok().flatten())
        .ok_or_else(|| "no monitor available".to_string())?;

    let monitor_size = monitor.size();
    let monitor_pos = monitor.position();
    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    let x = monitor_pos.x + monitor_size.width as i32 - window_size.width as i32;
    let y = monitor_pos.y + (monitor_size.height as i32 - window_size.height as i32) / 2;

    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string())
}

fn restore_or_default_position(app: &AppHandle, window: &WebviewWindow) {
    if let Some(saved) = load_saved_position(app) {
        if let Err(e) = window.set_position(PhysicalPosition::new(saved.x, saved.y)) {
            eprintln!("panel: restore position failed: {e}");
        }
        return;
    }

    if let Err(e) = position_panel_right_centered(window) {
        eprintln!("panel: position failed: {e}");
    }

    save_current_position(app, window);
}

pub fn setup_panel_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window(PANEL_LABEL) else {
        eprintln!("panel: no window with label {PANEL_LABEL}");
        return;
    };

    let app_handle = app.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Moved(position) = event {
            persist_position(
                &app_handle,
                PanelPosition {
                    x: position.x,
                    y: position.y,
                },
            );
        }
    });
}

pub fn show_panel(app: &AppHandle) {
    let Some(window) = app.get_webview_window(PANEL_LABEL) else {
        eprintln!("panel: no window with label {PANEL_LABEL}");
        return;
    };

    restore_or_default_position(app, &window);

    if let Err(e) = window.show() {
        eprintln!("panel: show failed: {e}");
    }
}

pub fn hide_panel(app: &AppHandle) {
    let Some(window) = app.get_webview_window(PANEL_LABEL) else {
        return;
    };

    save_current_position(app, &window);

    if let Err(e) = window.hide() {
        eprintln!("panel: hide failed: {e}");
    }
}

pub fn toggle_panel(app: &AppHandle) {
    let Some(window) = app.get_webview_window(PANEL_LABEL) else {
        eprintln!("panel: no window with label {PANEL_LABEL}");
        return;
    };

    match window.is_visible() {
        Ok(true) => hide_panel(app),
        Ok(false) => show_panel(app),
        Err(e) => eprintln!("panel: is_visible failed: {e}"),
    }
}

#[tauri::command]
pub fn hide_panel_command(app: AppHandle) {
    hide_panel(&app);
}
