#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

use crate::commands::read_git_index_entries;
use crate::workspace::Workspace;

mod app_error;
mod commands;
mod workspace;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_dir = app
        .path_resolver()
        .app_dir()
        .expect("Cannot resolve app dir");

      let workspace = Workspace::initialize(app_dir).expect("fail to initialize workspace");
      app.manage(workspace);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![read_git_index_entries])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
