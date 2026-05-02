use tauri::Manager;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |app_handle, shortcut, event| {
                        println!("{:?}", shortcut);
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
                                            println!("test_overlay is being hidden");
                                            if let Err(e) = window.hide() {
                                                eprintln!("shortcut: hide failed: {e}");
                                            }
                                        }
                                        Ok(false) => {
                                            println!("test_overlay is being shown");
                                            if let Err(e) = window.show() {
                                                eprintln!("shortcut: show failed: {e}");
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("shortcut: is_visible failed: {e}");
                                        }
                                    }
                                    println!("Ctrl-N Pressed!");
                                }
                                ShortcutState::Released => {
                                    println!("Ctrl-N Released!");
                                }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
