#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;

use tauri::{App, Runtime};

use crate::global_shortcuts::setup_global_shortcuts;
use crate::tray::{handle_tray, tray};
use crate::windows::setup_windows;

mod global_shortcuts;
mod tray;
mod windows;

fn setup<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn Error>> {
  setup_windows(app);
  setup_global_shortcuts(app);

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .system_tray(tray())
    .on_system_tray_event(handle_tray)
    .setup(setup)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
