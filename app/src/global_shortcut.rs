use tauri::{App, GlobalShortcutManager};

use crate::utils::toggle_win;
use crate::win::get_main_window;

pub fn setup_global_shortcut(app: &App) -> Result<(), crate::error::Error> {
  let win = get_main_window(app);
  app
    .global_shortcut_manager()
    .register("CmdOrCtrl+Shift+T", move || {
      let _ = toggle_win(&win);
    })
    .expect("fail to register global shortcut");

  Ok(())
}
