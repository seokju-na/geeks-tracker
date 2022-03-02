#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app_error;
mod git_eventstore;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      use tauri::GlobalShortcutManager;

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
