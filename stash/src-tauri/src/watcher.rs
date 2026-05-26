use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::file_events::FileEvent;

const PARTIAL_EXTENSIONS: &[&str] = &["crdownload", "download", "part", "tmp"];

fn downloads_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join("Downloads"))
}

fn is_partial_download(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| PARTIAL_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn should_emit(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    if is_partial_download(path) {
        return false;
    }
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| !name.starts_with('.'))
        .unwrap_or(false)
}

fn build_file_event(path: &Path, source_folder: &str) -> FileEvent {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    FileEvent {
        id: Uuid::new_v4().to_string(),
        file_name,
        full_path: path.to_string_lossy().to_string(),
        source_folder: source_folder.to_string(),
        detected_at: chrono::Utc::now().to_rfc3339(),
    }
}

pub fn start_downloads_watcher(app: AppHandle) -> Result<RecommendedWatcher, Box<dyn std::error::Error>> {
    let downloads = downloads_dir().ok_or("could not resolve home/Downloads")?;

    if !downloads.exists() {
        return Err(format!("Downloads folder not found: {}", downloads.display()).into());
    }

    println!("Watching Downloads at: {}", downloads.display());

    let recent_events: Arc<Mutex<HashMap<PathBuf, Instant>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let debounce_window = Duration::from_millis(500);

    let app_handle = app.clone();
    let watch_path = downloads.clone();

    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<notify::Event>| {
            let Ok(event) = result else {
                eprintln!("watch error: {:?}", result.err());
                return;
            };

            let is_relevant = matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_)
            );
            if !is_relevant {
                return;
            }

            for path in event.paths {
                if !should_emit(&path) {
                    continue;
                }

                let now = Instant::now();
                let mut recent = recent_events.lock().unwrap();
                recent.retain(|_, instant| now.duration_since(*instant) < debounce_window);

                if recent.contains_key(&path) {
                    continue;
                }
                recent.insert(path.clone(), now);
                drop(recent);

                let payload = build_file_event(&path, "Downloads");
                println!("file detected: {}", payload.file_name);

                if let Err(e) = app_handle.emit("file_detected", &payload) {
                    eprintln!("emit file_detected failed: {e}");
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

    Ok(watcher)
}
