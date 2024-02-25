use tauri::Window;

pub fn toggle_win(
  win: &Window,
) -> Result<(), tauri::Error> {
  if win.is_visible()? {
    win.hide()?;
  } else {
    win.show()?;
    win.set_focus()?;
  }
  Ok(())
}
