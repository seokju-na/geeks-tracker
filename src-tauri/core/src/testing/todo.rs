use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{Aggregate, Command, Event, Timestamp};

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
