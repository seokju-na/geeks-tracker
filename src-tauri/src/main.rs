#![feature(async_stream)]
#![feature(associated_type_defaults)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::GlobalShortcutManager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut shortcut = app.global_shortcut_manager();
            shortcut
                .register("Cmd+G", || {
                    println!("hello");
                })
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
