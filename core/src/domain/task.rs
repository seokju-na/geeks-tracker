use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use typeshare::typeshare;

use crate::eventsourcing::{Aggregate, Command, Event};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Task {
  pub id: String,
  pub title: String,
  #[builder(default)]
  pub body: Option<String>,
  #[builder(default)]
  pub status: TaskStatus,
  #[builder(default = Utc::now())]
  pub created_at: DateTime<Utc>,
  #[builder(default = Utc::now())]
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare]
pub enum TaskStatus {
  Backlog,
  Queue,
  InProgress,
  Done,
}

impl Default for TaskStatus {
  fn default() -> Self {
    Self::Backlog
  }
}

impl Aggregate for Task {
  type Command = TaskCommand;
  type Event = TaskEvent;
  type Error = crate::domain::Error;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match this {
      Some(_) => match command {
        Self::Command::UpdateTitle { title, .. } => Ok(Self::Event::TitleUpdated { title }),
        Self::Command::UpdateStatus { status, .. } => Ok(Self::Event::StatusUpdated { status }),
        Self::Command::UpdateBody { body, .. } => Ok(Self::Event::BodyUpdated { body }),
        Self::Command::Delete { .. } => Ok(Self::Event::Deleted {}),
        _ => Err(crate::domain::Error::TaskNotExists),
      },
      None => match command {
        Self::Command::Create { id, title, status } => Ok(Self::Event::Created {
          id,
          title,
          body: None,
          status: status.unwrap_or_default(),
        }),
        _ => Err(crate::domain::Error::TaskAlreadyExists),
      },
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let now = Utc::now();
    match this {
      Some(mut task) => match event {
        Self::Event::TitleUpdated { title } => {
          task.title = title;
          task.updated_at = now;
          Ok(task)
        }
        Self::Event::StatusUpdated { status } => {
          task.status = status;
          task.updated_at = now;
          Ok(task)
        }
        Self::Event::BodyUpdated { body } => {
          task.body = body;
          task.updated_at = now;
          Ok(task)
        }
        // TODO: how to delete?
        _ => Err(crate::domain::Error::TaskNotExists),
      },
      None => match event {
        Self::Event::Created {
          id,
          title,
          body,
          status,
        } => Ok(Task {
          id,
          title,
          body,
          status,
          created_at: now,
          updated_at: now,
        }),
        _ => Err(crate::domain::Error::TaskAlreadyExists),
      },
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum TaskEvent {
  #[serde(rename = "task.created")]
  Created {
    id: String,
    title: String,
    body: Option<String>,
    status: TaskStatus,
  },
  #[serde(rename = "task.titleUpdated")]
  TitleUpdated { title: String },
  #[serde(rename = "task.statusUpdated")]
  StatusUpdated { status: TaskStatus },
  #[serde(rename = "task.bodyUpdated")]
  BodyUpdated { body: Option<String> },
  #[serde(rename = "task.deleted")]
  Deleted {},
}

impl Event for TaskEvent {
  fn name(&self) -> &'static str {
    match self {
      Self::Created { .. } => "task.created",
      Self::TitleUpdated { .. } => "task.titleUpdated",
      Self::StatusUpdated { .. } => "task.statusUpdated",
      Self::BodyUpdated { .. } => "task.bodyUpdated",
      Self::Deleted { .. } => "task.deleted",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name", content = "data")]
#[typeshare]
pub enum TaskCommand {
  #[serde(rename = "task.create", rename_all = "camelCase")]
  Create {
    id: String,
    title: String,
    status: Option<TaskStatus>,
  },
  #[serde(rename = "task.updateTitle", rename_all = "camelCase")]
  UpdateTitle { id: String, title: String },
  #[serde(rename = "task.updateStatus", rename_all = "camelCase")]
  UpdateStatus { id: String, status: TaskStatus },
  #[serde(rename = "task.updateBody", rename_all = "camelCase")]
  UpdateBody { id: String, body: Option<String> },
  #[serde(rename = "task.delete", rename_all = "camelCase")]
  Delete { id: String },
}

impl Command for TaskCommand {
  fn name(&self) -> &'static str {
    match self {
      Self::Create { .. } => "task.create",
      Self::UpdateTitle { .. } => "task.updateTitle",
      Self::UpdateStatus { .. } => "task.updateStatus",
      Self::UpdateBody { .. } => "task.updateBody",
      Self::Delete { .. } => "task.delete",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      Self::Create { id, .. } => id,
      Self::UpdateTitle { id, .. } => id,
      Self::UpdateStatus { id, .. } => id,
      Self::UpdateBody { id, .. } => id,
      Self::Delete { id } => id,
    }
  }
}
