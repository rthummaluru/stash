mod file_events;
mod watcher;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            match watcher::start_downloads_watcher(app.handle().clone()) {
                Ok(watcher) => {
                    app.manage(watcher);
                }
                Err(e) => {
                    eprintln!("failed to start downloads watcher: {e}");
                }
            }

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, shortcut, event| {
                            if shortcut == &ctrl_n_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        let Some(window) =
                                            app_handle.get_webview_window("test_overlay")
                                        else {
                                            eprintln!("shortcut: no window with label test_overlay");
                                            return;
                                        };

                                        match window.is_visible() {
                                            Ok(true) => {
                                                if let Err(e) = window.hide() {
                                                    eprintln!("shortcut: hide failed: {e}");
                                                }
                                            }
                                            Ok(false) => {
                                                if let Err(e) = window.show() {
                                                    eprintln!("shortcut: show failed: {e}");
                                                }
                                                if let Err(e) = window.set_focus() {
                                                    eprintln!("shortcut: set_focus failed: {e}");
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("shortcut: is_visible failed: {e}");
                                            }
                                        }
                                    }
                                    ShortcutState::Released => {}
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
