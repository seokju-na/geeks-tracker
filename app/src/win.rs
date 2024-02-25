use tauri::{App, Manager, Window, WindowEvent};

#[cfg(target_os = "macos")]
use crate::patches::TransparentTitlebar;

pub fn get_window(app: &App, label: &str) -> Window {
  app.get_window(label).expect("cannot get window")
}

pub fn get_main_window(app: &App) -> Window {
  get_window(app, "main")
}

pub fn setup_main_window(app: &mut App) -> Result<(), crate::error::Error> {
  // https://github.com/tauri-apps/tauri/discussions/2684#discussioncomment-1433069
  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Accessory);

  // [macOS] set main window transparent titlebar
  #[cfg(target_os = "macos")]
  {
    let win = get_main_window(app);
    win.set_transparent_titlebar(true, true);
  }

  let win = get_main_window(app);
  win.clone().listen("hide_app", move |_| {
    let _ = win.hide();
  });

  // register window event.
  let win = get_main_window(app);
  win.clone().on_window_event(move |event| {
    // hide window when looses focuses (production only).
    if let WindowEvent::Focused(focused) = event {
      if win.is_visible().unwrap() && !(*focused) {
        #[cfg(not(debug_assertions))]
        win.hide().unwrap();
      }
    }
  });

  Ok(())
}
