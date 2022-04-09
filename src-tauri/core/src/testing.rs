use std::error::Error;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};

use crate::{AggregateState, Command, Event, EventData, EventParseError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
  #[serde(rename = "todo")]
  Todo,
  #[serde(rename = "in-progress")]
  InProgress,
  #[serde(rename = "done")]
  Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoState {
  pub exists: bool,
  pub title: Option<String>,
  pub status: TodoStatus,
  pub created_at: Option<i64>,
  pub updated_at: Option<i64>,
}

impl Default for TodoState {
  fn default() -> Self {
    Self {
      exists: false,
      title: None,
      status: TodoStatus::Todo,
      created_at: None,
      updated_at: None,
    }
  }
}

impl AggregateState for TodoState {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoCreatedPayload {
  pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoTitleUpdatedPayload {
  pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStatusUpdatedPayload {
  pub status: TodoStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum TodoEvent {
  TodoCreated {
    id: String,
    version: i64,
    timestamp: i64,
    payload: TodoCreatedPayload,
  },
  TodoTitleUpdated {
    id: String,
    version: i64,
    timestamp: i64,
    payload: TodoTitleUpdatedPayload,
  },
  TodoStatusUpdated {
    id: String,
    version: i64,
    timestamp: i64,
    payload: TodoStatusUpdatedPayload,
  },
}

impl Event<TodoState> for TodoEvent {
  fn from_event_data(event_data: EventData) -> Result<Self, EventParseError> {
    let id = event_data.aggregate_id;
    let version = event_data.aggregate_version;
    let timestamp = event_data.timestamp;

    match event_data.name.as_str() {
      "TodoCreated" => {
        let payload = from_value::<TodoCreatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(TodoEvent::TodoCreated {
          id,
          version,
          timestamp,
          payload,
        })
      }
      "TodoTitleUpdated" => {
        let payload = from_value::<TodoTitleUpdatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(TodoEvent::TodoTitleUpdated {
          id,
          version,
          timestamp,
          payload,
        })
      }
      "TodoStatusUpdated" => {
        let payload = from_value::<TodoStatusUpdatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(TodoEvent::TodoStatusUpdated {
          id,
          version,
          timestamp,
          payload,
        })
      }
      _ => Err(EventParseError::NoMatches),
    }
  }

  fn to_event_data(self) -> EventData {
    match self {
      TodoEvent::TodoCreated {
        id,
        version,
        timestamp,
        payload,
      } => EventData {
        name: "TodoCreated".to_string(),
        aggregate_id: id,
        aggregate_version: version,
        timestamp,
        payload: to_value(payload).unwrap(),
      },
      TodoEvent::TodoTitleUpdated {
        id,
        version,
        timestamp,
        payload,
      } => EventData {
        name: "TodoTitleUpdated".to_string(),
        aggregate_id: id,
        aggregate_version: version,
        timestamp,
        payload: to_value(payload).unwrap(),
      },
      TodoEvent::TodoStatusUpdated {
        id,
        version,
        timestamp,
        payload,
      } => EventData {
        name: "TodoStatusUpdated".to_string(),
        aggregate_id: id,
        aggregate_version: version,
        timestamp,
        payload: to_value(payload).unwrap(),
      },
    }
  }

  fn handle(&self, state: &mut TodoState) -> () {
    match self {
      TodoEvent::TodoCreated {
        timestamp, payload, ..
      } => {
        state.exists = true;
        state.title = Some(payload.title.to_owned());
        state.created_at = Some(timestamp.to_owned());
        state.updated_at = Some(timestamp.to_owned());
      }
      TodoEvent::TodoTitleUpdated {
        timestamp, payload, ..
      } => {
        state.title = Some(payload.title.to_owned());
        state.updated_at = Some(timestamp.to_owned());
      }
      TodoEvent::TodoStatusUpdated {
        timestamp, payload, ..
      } => {
        state.status = payload.status.to_owned();
        state.updated_at = Some(timestamp.to_owned());
      }
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum TodoError {
  #[error("Todo already exists")]
  TodoAlreadyExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum TodoCommand {
  CreateTodo { id: String, title: String },
  UpdateTodoTitle { id: String, title: String },
  UpdateTodoStatus { id: String, status: TodoStatus },
}

impl Command<TodoState> for TodoCommand {
  type Event = TodoEvent;

  fn aggregate_id(&self) -> &str {
    match self {
      TodoCommand::CreateTodo { id, .. } => id,
      TodoCommand::UpdateTodoTitle { id, .. } => id,
      TodoCommand::UpdateTodoStatus { id, .. } => id,
    }
  }

  fn handle(&self, state: &TodoState, version: i64) -> Result<Self::Event, Box<dyn Error>> {
    let timestamp = Utc::now().timestamp();

    match self {
      TodoCommand::CreateTodo { id, title } => {
        if state.exists {
          return Err(Box::new(TodoError::TodoAlreadyExists));
        }

        Ok(TodoEvent::TodoCreated {
          id: id.to_owned(),
          version,
          timestamp,
          payload: TodoCreatedPayload {
            title: title.to_owned(),
          },
        })
      }
      TodoCommand::UpdateTodoTitle { id, title } => Ok(TodoEvent::TodoTitleUpdated {
        id: id.to_owned(),
        version,
        timestamp,
        payload: TodoTitleUpdatedPayload {
          title: title.to_owned(),
        },
      }),
      TodoCommand::UpdateTodoStatus { id, status } => Ok(TodoEvent::TodoStatusUpdated {
        id: id.to_owned(),
        version,
        timestamp,
        payload: TodoStatusUpdatedPayload {
          status: status.to_owned(),
        },
      }),
    }
  }
}
