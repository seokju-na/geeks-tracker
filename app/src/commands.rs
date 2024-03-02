use tauri::State;

use geeks_tracker_core::dispatch::DispatchMessage;
use geeks_tracker_core::domain::task::{Task, TaskCommand};

use crate::application::{Application, CommandHandler};
use crate::dispatcher::Dispatcher;

#[tauri::command]
pub async fn list_tasks(application: State<'_, Application>) -> Result<Vec<Task>, ()> {
  log::trace!("tauri command: list_tasks");
  let tasks: Vec<_> = application
    .lock()
    .await
    .tasks
    .states
    .values()
    .cloned()
    .collect();
  Ok(tasks)
}

#[tauri::command]
pub async fn run_task_command(
  application: State<'_, Application>,
  dispatcher: State<'_, Dispatcher>,
  command: TaskCommand,
) -> Result<(), crate::error::Error> {
  log::trace!("tauri command: run_task_command");
  let persisted = application.lock().await.handle_command(command).await?;
  let _ = dispatcher
    .send(DispatchMessage::TaskPersisted {
      events: vec![persisted],
    })
    .await;
  Ok(())
}
