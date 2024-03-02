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

  #[cfg(target_os = "macos")]
  window_vibrancy::apply_vibrancy(
    &get_main_window(app),
    window_vibrancy::NSVisualEffectMaterial::HudWindow,
    None,
    None,
  )
  .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

  #[cfg(target_os = "windows")]
  window_vibrancy::apply_blur(&get_main_window(app), Some((18, 18, 18, 125)))
    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

  let win = get_main_window(app);
  win.clone().listen("hide_app", move |_| {
    let _ = win.hide();
  });

  // register window event.
  let win = get_main_window(app);
  win.clone().on_window_event(move |event| {
    if let WindowEvent::Focused(ref focused) = event {
      match focused {
        true => {
          let _ = win.emit("app_focused", true);
        }
        false => {
          // hide window when looses focuses (production only).
          #[cfg(not(debug_assertions))]
          {
            if win.is_visible().unwrap() {
              let _ = win.hide();
            }
          }
        }
      }
    }
  });

  Ok(())
}
