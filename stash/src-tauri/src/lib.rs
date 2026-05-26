mod file_events;
mod panel;
mod watcher;

use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![panel::hide_panel_command])
        .setup(|app| {
            app.manage(panel::initial_panel_state(app.handle()));

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

                let open_panel =
                    MenuItem::with_id(app, "open_panel", "Open Panel", true, None::<&str>)?;
                let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let tray_menu = Menu::with_items(app, &[&open_panel, &quit])?;

                let _tray = TrayIconBuilder::with_id("main-tray")
                    .icon(
                        app.default_window_icon()
                            .expect("missing default window icon")
                            .clone(),
                    )
                    .tooltip("Stash")
                    .menu(&tray_menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(move |app, event| match event.id.as_ref() {
                        "open_panel" => panel::show_panel(app),
                        "quit" => app.exit(0),
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button,
                            button_state,
                            ..
                        } = event
                        {
                            if button == MouseButton::Left && button_state == MouseButtonState::Up
                            {
                                panel::toggle_panel(tray.app_handle());
                            }
                        }
                    })
                    .build(app)?;

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, shortcut, event| {
                            if shortcut == &ctrl_n_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        panel::toggle_panel(app_handle);
                                    }
                                    ShortcutState::Released => {}
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }

            if let Some(window) = app.get_webview_window(panel::PANEL_LABEL) {
                panel::apply_panel_vibrancy(&window);
            }

            panel::setup_panel_window(app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
