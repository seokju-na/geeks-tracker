#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;

use crate::app_state::setup_app_state;
use tauri::{App, Runtime};

use crate::global_shortcuts::setup_global_shortcuts;
use crate::tray::{handle_tray, tray};
use crate::windows::setup_windows;

mod app_state;
mod application;
mod domain;
mod global_shortcuts;
mod macos_titlebar_patch;
mod tray;
mod windows;
mod workspace;

fn setup<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn Error>> {
  // https://github.com/tauri-apps/tauri/discussions/2684#discussioncomment-1433069
  #[cfg(target_os = "macos")]
  {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
  }

  setup_app_state(app);
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
