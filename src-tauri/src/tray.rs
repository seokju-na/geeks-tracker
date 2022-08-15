use tauri::{
  AppHandle, CustomMenuItem, Runtime, SystemTray, SystemTrayEvent, SystemTrayMenu,
  SystemTrayMenuItem,
};

use crate::constants::{ACCELERATOR_OPEN, ACCELERATOR_PREFERENCES};
use crate::os_type::os_type;
use crate::windows::{AppExtra, WindowExtra};

pub fn tray() -> SystemTray {
  SystemTray::new().with_menu(menu())
}

pub fn handle_tray<R: Runtime>(app: &AppHandle<R>, event: SystemTrayEvent) {
  if let SystemTrayEvent::MenuItemClick { id, .. } = event {
    match id.as_str() {
      "open" => {
        app.get_main_window().show().unwrap();
      }
      "preferences" => {
        let win = app.get_main_window();
        win.navigate("/preferences").unwrap();
        win.show().unwrap();
      }
      "quit" => {
        std::process::exit(0);
      }
      _ => {}
    }
  }
}

fn menu() -> SystemTrayMenu {
  let open =
    CustomMenuItem::new("open".to_string(), "Open Geek's Tracker").accelerator(ACCELERATOR_OPEN);
  let preferences = CustomMenuItem::new(
    "preferences".to_string(),
    match os_type() {
      "macos" => "Preferences...",
      _ => "Preferences",
    },
  )
  .accelerator(ACCELERATOR_PREFERENCES);
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  SystemTrayMenu::new()
    .add_item(open)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(preferences)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit)
}
