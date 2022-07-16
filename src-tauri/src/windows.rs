use tauri::{App, AppHandle, Manager, Result, Runtime, Window, WindowEvent};

#[cfg(target_os = "macos")]
use crate::macos_titlebar_patch::TransparentTitlebar;

const MAIN_WIN: &str = "main";

pub fn setup_windows<R: Runtime>(app: &mut App<R>) {
  // [macOS] set main window transparent titlebar
  #[cfg(target_os = "macos")]
  {
    let win = app.get_main_window();
    win.set_transparent_titlebar(true, true);
  }

  // register window event.
  let win = app.get_main_window();
  win.clone().on_window_event(move |event| {
    // hide window when looses focuses (production only).
    if let WindowEvent::Focused(focused) = event {
      if win.is_visible().unwrap() && !(*focused) {
        #[cfg(not(debug_assertions))]
        win.hide().unwrap();
      }
    }
  });
}

pub trait AppExtra<R: Runtime> {
  fn get_main_window(&self) -> Window<R>;
}

impl<R> AppExtra<R> for App<R>
where
  R: Runtime,
{
  fn get_main_window(&self) -> Window<R> {
    self.get_window(MAIN_WIN).expect("cannot get main window")
  }
}

impl<R> AppExtra<R> for AppHandle<R>
where
  R: Runtime,
{
  fn get_main_window(&self) -> Window<R> {
    self.get_window(MAIN_WIN).expect("cannot get main window")
  }
}

pub trait WindowExtra {
  fn toggle(&self) -> Result<()>;
}

impl<R> WindowExtra for Window<R>
where
  R: Runtime,
{
  fn toggle(&self) -> Result<()> {
    if self.is_visible()? {
      self.hide()?;
    } else {
      self.show()?;
      self.set_focus()?;
    }

    Ok(())
  }
}
