use geeks_git::GitError;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app_state::AppState;
use crate::application::{ApplicationError, CommandHandler, FileSnapshotError, QueryHandler};
use crate::domain::{Category, CategoryCommand, CategoryError};

#[tauri::command]
pub async fn execute_category_command(
  command: CategoryCommand,
  app_state: State<'_, AppState>,
) -> Result<(), CommandError> {
  app_state
    .application
    .lock()
    .await
    .handle_command(command)
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn list_categories(
  app_state: State<'_, AppState>,
) -> Result<Vec<Category>, CommandError> {
  let result = app_state.application.lock().await.list_categories();
  Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
  kind: String,
  name: String,
  message: Option<String>,
}

impl From<ApplicationError> for CommandError {
  fn from(e: ApplicationError) -> Self {
    match e {
      ApplicationError::FileSnapshot(e) => match e {
        FileSnapshotError::Io(e) => CommandError {
          kind: "FileSnapshotError".to_string(),
          name: "Io".to_string(),
          message: Some(e.to_string()),
        },
        FileSnapshotError::JsonParse(e) => CommandError {
          kind: "FileSnapshotError".to_string(),
          name: "JsonParse".to_string(),
          message: Some(e.to_string()),
        },
      },
      ApplicationError::Category(e) => match e {
        CategoryError::AlreadyExists => CommandError {
          kind: "CategoryError".to_string(),
          name: "AlreadyExists".to_string(),
          message: None,
        },
        CategoryError::NotExists => CommandError {
          kind: "CategoryError".to_string(),
          name: "NotExists".to_string(),
          message: None,
        },
      },
      ApplicationError::Git(e) => match e {
        GitError::Io(e) => CommandError {
          kind: "GitError".to_string(),
          name: "Io".to_string(),
          message: Some(e.to_string()),
        },
        GitError::Git2(e) => CommandError {
          kind: "GitError".to_string(),
          name: "Git2".to_string(),
          message: Some(e.message().to_string()),
        },
        GitError::NoHead => CommandError {
          kind: "GitError".to_string(),
          name: "NoHead".to_string(),
          message: None,
        },
        GitError::Generic(e) => CommandError {
          kind: "GitError".to_string(),
          name: "Generic".to_string(),
          message: Some(e),
        },
      },
    }
  }
}
