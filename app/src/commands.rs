use tauri::State;

use geeks_tracker_core::domain::task::{Task, TaskCommand};

use crate::application::{ApplicationState, CommandHandler};

#[tauri::command]
pub async fn list_tasks(application: State<'_, ApplicationState>) -> Result<Vec<Task>, ()> {
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
  application: State<'_, ApplicationState>,
  command: TaskCommand,
) -> Result<(), crate::error::Error> {
  log::trace!("tauri command: run_task_command");
  application.lock().await.handle_command(command).await?;
  Ok(())
}
