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
  Created {
    id: String,
    version: i64,
    timestamp: i64,
    payload: TodoCreatedPayload,
  },
  TitleUpdated {
    id: String,
    version: i64,
    timestamp: i64,
    payload: TodoTitleUpdatedPayload,
  },
  StatusUpdated {
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
        Ok(TodoEvent::Created {
          id,
          version,
          timestamp,
          payload,
        })
      }
      "TodoTitleUpdated" => {
        let payload = from_value::<TodoTitleUpdatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(TodoEvent::TitleUpdated {
          id,
          version,
          timestamp,
          payload,
        })
      }
      "TodoStatusUpdated" => {
        let payload = from_value::<TodoStatusUpdatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(TodoEvent::StatusUpdated {
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
      TodoEvent::Created {
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
      TodoEvent::TitleUpdated {
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
      TodoEvent::StatusUpdated {
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
      TodoEvent::Created {
        timestamp, payload, ..
      } => {
        state.exists = true;
        state.title = Some(payload.title.to_owned());
        state.created_at = Some(timestamp.to_owned());
        state.updated_at = Some(timestamp.to_owned());
      }
      TodoEvent::TitleUpdated {
        timestamp, payload, ..
      } => {
        state.title = Some(payload.title.to_owned());
        state.updated_at = Some(timestamp.to_owned());
      }
      TodoEvent::StatusUpdated {
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
  Created { id: String, title: String },
  UpdateTitle { id: String, title: String },
  UpdateStatus { id: String, status: TodoStatus },
}

impl Command<TodoState> for TodoCommand {
  type Event = TodoEvent;
  type Error = TodoError;

  fn aggregate_id(&self) -> &str {
    match self {
      TodoCommand::Created { id, .. } => id,
      TodoCommand::UpdateTitle { id, .. } => id,
      TodoCommand::UpdateStatus { id, .. } => id,
    }
  }

  fn handle(&self, state: &TodoState, version: i64) -> Result<Self::Event, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match self {
      TodoCommand::Created { id, title } => {
        if state.exists {
          return Err(TodoError::TodoAlreadyExists);
        }

        Ok(TodoEvent::Created {
          id: id.to_owned(),
          version,
          timestamp,
          payload: TodoCreatedPayload {
            title: title.to_owned(),
          },
        })
      }
      TodoCommand::UpdateTitle { id, title } => Ok(TodoEvent::TitleUpdated {
        id: id.to_owned(),
        version,
        timestamp,
        payload: TodoTitleUpdatedPayload {
          title: title.to_owned(),
        },
      }),
      TodoCommand::UpdateStatus { id, status } => Ok(TodoEvent::StatusUpdated {
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
