use serde::Serialize;
use tauri::{
  App, AppHandle, Manager, Result, Runtime, Window, WindowBuilder, WindowEvent, WindowUrl,
};
use url::Url;

#[cfg(target_os = "macos")]
use crate::macos_titlebar_patch::TransparentTitlebar;
use crate::os_type::os_type;

const MAIN_WIN: &str = "main";

pub fn setup_windows<R: Runtime>(app: &mut App<R>) -> Result<()> {
  setup_main_window(app)?;

  Ok(())
}

fn setup_main_window<R: Runtime>(app: &mut App<R>) -> Result<()> {
  let url = WindowUrl::External(Url::parse("http://localhost:5173").unwrap());
  #[cfg(not(debug_assertions))]
  let url = WindowUrl::App("../dist/index.html".into());

  let builder = WindowBuilder::new(app, MAIN_WIN, url)
    .title("Geek's Tracker")
    .inner_size(600_f64, 400_f64)
    .resizable(false)
    .fullscreen(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(false)
    .initialization_script(&format!(
      r#"
    window.__TAURI_OS_TYPE__ = '{os_type}';
    "#,
      os_type = os_type()
    ));

  let win = builder.build()?;
  // [macOS] set main window transparent titlebar
  #[cfg(target_os = "macos")]
  {
    win.set_transparent_titlebar(true, true);
  }

  // register "hide" event for main window.
  let win = app.get_main_window();
  win.clone().listen("geeks-tracker://hide", move |_| {
    win.hide().unwrap();
  });

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

  Ok(())
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
  fn navigate<T: Into<String>>(&self, to: T) -> Result<()>;
}

#[derive(Clone, Serialize)]
struct NavigatePayload {
  to: String,
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

  fn navigate<T: Into<String>>(&self, to: T) -> Result<()> {
    self.emit(
      "geeks-tracker://navigate",
      NavigatePayload { to: to.into() },
    )
  }
}
