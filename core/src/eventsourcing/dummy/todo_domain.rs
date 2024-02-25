use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::str::{from_utf8, Utf8Error};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_value};
use tokio::fs::{read, write};

use crate::eventsourcing::{
  Aggregate, AggregateRoot, Command, Event, Snapshot, Timestamp, Version,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum TodoEvent {
  TodoCreated {
    id: String,
    title: String,
    status: TodoStatus,
  },
  TodoTitleUpdated {
    title: String,
  },
  TodoStatusUpdated {
    status: TodoStatus,
  },
}

impl Event for TodoEvent {
  fn name(&self) -> &'static str {
    match self {
      TodoEvent::TodoCreated { .. } => "TodoCreated",
      TodoEvent::TodoTitleUpdated { .. } => "TodoTitleUpdated",
      TodoEvent::TodoStatusUpdated { .. } => "TodoStatusUpdated",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum TodoCommand {
  CreateTodo {
    id: String,
    title: String,
    status: Option<TodoStatus>,
  },
  UpdateTodoTitle {
    id: String,
    title: String,
  },
  UpdateTodoStatus {
    id: String,
    status: TodoStatus,
  },
}

impl Command for TodoCommand {
  fn name(&self) -> &'static str {
    match self {
      TodoCommand::CreateTodo { .. } => "CreateTodo",
      TodoCommand::UpdateTodoTitle { .. } => "UpdateTodoTitle",
      TodoCommand::UpdateTodoStatus { .. } => "UpdateTodoStatus",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      TodoCommand::CreateTodo { id, .. } => id,
      TodoCommand::UpdateTodoTitle { id, .. } => id,
      TodoCommand::UpdateTodoStatus { id, .. } => id,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TodoStatus {
  #[serde(rename = "todo")]
  Todo,
  #[serde(rename = "in-progress")]
  InProgress,
  #[serde(rename = "done")]
  Done,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum TodoError {
  #[error("Todo already exists")]
  AlreadyExists,
  #[error("Todo not exists")]
  NotExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
  pub id: String,
  pub title: String,
  pub status: TodoStatus,
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

impl Aggregate for Todo {
  type Command = TodoCommand;
  type Event = TodoEvent;
  type Error = TodoError;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match command {
      TodoCommand::CreateTodo { id, title, status } => {
        if this.is_some() {
          return Err(TodoError::AlreadyExists);
        }

        Ok(TodoEvent::TodoCreated {
          id,
          title,
          status: status.unwrap_or(TodoStatus::Todo),
        })
      }
      TodoCommand::UpdateTodoTitle { title, .. } => Ok(TodoEvent::TodoTitleUpdated { title }),
      TodoCommand::UpdateTodoStatus { status, .. } => Ok(TodoEvent::TodoStatusUpdated { status }),
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match event {
      TodoEvent::TodoCreated { id, title, status } => {
        if this.is_some() {
          return Err(TodoError::AlreadyExists);
        }
        Ok(Todo {
          id,
          title,
          status,
          created_at: timestamp,
          updated_at: timestamp,
        })
      }
      TodoEvent::TodoTitleUpdated { title } => match this {
        Some(mut todo) => {
          todo.title = title;
          Ok(todo)
        }
        None => Err(TodoError::NotExists),
      },
      TodoEvent::TodoStatusUpdated { status } => match this {
        Some(mut todo) => {
          todo.status = status;
          Ok(todo)
        }
        None => Err(TodoError::NotExists),
      },
    }
  }
}

pub struct TodoSnapshot {
  file_path: PathBuf,
}

impl TodoSnapshot {
  pub fn new(dir: &Path) -> Self {
    Self {
      file_path: dir.join("todo"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoSnapshotData {
  states: HashMap<String, Todo>,
  versions: HashMap<String, Version>,
}

impl From<AggregateRoot<Todo>> for TodoSnapshotData {
  fn from(root: AggregateRoot<Todo>) -> Self {
    Self {
      states: root.states,
      versions: root.versions,
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum TodoSnapshotError {
  #[error(transparent)]
  Io(#[from] io::Error),

  #[error(transparent)]
  Utf8(#[from] Utf8Error),

  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
}

#[async_trait]
impl Snapshot<Todo> for TodoSnapshot {
  type Error = TodoSnapshotError;

  async fn load(&self) -> Result<AggregateRoot<Todo>, Self::Error> {
    let raw = read(&self.file_path).await?;
    let raw_str = from_utf8(&raw)?;
    let data = from_str::<TodoSnapshotData>(raw_str)?;

    Ok(AggregateRoot::new(data.states, data.versions))
  }

  async fn save(&self, root: &AggregateRoot<Todo>) -> Result<(), Self::Error> {
    let data: TodoSnapshotData = root.clone().into();
    let raw_data = to_value(data)?;
    write(&self.file_path, raw_data.to_string().as_bytes()).await?;

    Ok(())
  }
}
