#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

struct WindowState {
    can_click_through: bool,
}
struct WindowStateMutex(Mutex<WindowState>);

fn toggle_click_through(
    app: &tauri::AppHandle,
    state: tauri::State<WindowStateMutex>,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "click_through" => {
                    let mut state = state.0.lock().unwrap();
                    let window = app.get_window("main").unwrap();
                    match state.can_click_through {
                        true => {
                            window.set_ignore_cursor_events(false).unwrap();
                            item_handle.set_selected(true).unwrap();
                            *state = WindowState {
                                can_click_through: false,
                            };
                        }
                        false => {
                            window.set_ignore_cursor_events(true).unwrap();
                            item_handle.set_selected(false).unwrap();
                            *state = WindowState {
                                can_click_through: true,
                            };
                        }
                    }
                }
                "quit" => std::process::exit(0),
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit", "Quit");
    let click_through = CustomMenuItem::new("click_through", "Click Trough");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_item(click_through);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .manage(WindowStateMutex(Mutex::new(WindowState {
            can_click_through: false,
        })))
        .on_system_tray_event(|app, event| {
            let state = app.state::<WindowStateMutex>();
            toggle_click_through(app, state, event);
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
