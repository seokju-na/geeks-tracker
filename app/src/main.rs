// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, CustomMenuItem, SystemTray, SystemTrayMenu};
use tauri_plugin_log::LogTarget;

use crate::application::setup_application;
use crate::commands::{list_tasks, run_task_command};
use crate::dispatcher::setup_dispatcher;
use crate::global_shortcut::setup_global_shortcut;
use crate::win::setup_main_window;
use crate::workspace::setup_workspace;

mod application;
mod commands;
mod dispatcher;
mod error;
mod global_shortcut;
mod patches;
mod snapshots;
mod utils;
mod win;
mod workspace;

fn main() {
  tauri::Builder::default()
    .system_tray(
      SystemTray::new()
        .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit"))),
    )
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(
      tauri_plugin_log::Builder::default()
        .targets([LogTarget::Stdout])
        .build(),
    )
    .setup(|app| {
      setup_dispatcher(app);
      setup_workspace(app).expect("fail to setup workspace");
      setup_application(app).expect("fail to setup application");
      setup_main_window(app).expect("fail to setup main window");
      setup_global_shortcut(app).expect("fail to setup global shortcut");
      Ok(())
    })
    .invoke_handler(generate_handler![list_tasks, run_task_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
