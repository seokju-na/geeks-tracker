use tauri::async_runtime::spawn;
use tauri::{App, Manager};
use tokio::sync::mpsc;

use geeks_tracker_core::dispatch::DispatchMessage;

use crate::win::get_main_window;

#[non_exhaustive]
#[derive(Debug)]
pub struct Dispatcher {
  tx: mpsc::Sender<DispatchMessage>,
}

impl Dispatcher {
  pub fn new(tx: mpsc::Sender<DispatchMessage>) -> Self {
    Self { tx }
  }

  pub async fn send(
    &self,
    message: DispatchMessage,
  ) -> Result<(), mpsc::error::SendError<DispatchMessage>> {
    let tx = self.tx.clone();
    tx.send(message).await
  }
}

pub fn setup_dispatcher(app: &mut App) {
  let (tx, mut rx) = mpsc::channel::<DispatchMessage>(100);
  let dispatcher = Dispatcher::new(tx);
  let win = get_main_window(app);
  spawn(async move {
    while let Some(message) = rx.recv().await {
      let _ = win.emit("dispatcher_message", message);
    }
  });
  app.manage(dispatcher);
}
