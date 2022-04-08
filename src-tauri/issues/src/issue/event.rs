use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};

use geeks_tracker_core::{Event, EventData, EventParseError};

use crate::issue::IssueState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCreatedPayload {
  pub title: String,
  pub status_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum IssueEvent {
  IssueCreated {
    id: String,
    version: i64,
    timestamp: i64,
    payload: IssueCreatedPayload,
  },
}

impl Event<IssueState> for IssueEvent {
  fn from_event_data(event_data: EventData) -> Result<Self, EventParseError> {
    match event_data.name.as_str() {
      "IssueCreated" => {
        let payload = from_value::<IssueCreatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;

        Ok(IssueEvent::IssueCreated {
          id: event_data.aggregate_id,
          version: event_data.aggregate_version,
          timestamp: event_data.timestamp,
          payload,
        })
      }
      _ => Err(EventParseError::NoMatches),
    }
  }

  fn to_event_data(self) -> EventData {
    match self {
      IssueEvent::IssueCreated {
        id,
        version,
        timestamp,
        payload,
      } => EventData {
        name: "IssueCreated".to_string(),
        aggregate_id: id,
        aggregate_version: version,
        timestamp,
        payload: to_value(payload).unwrap(),
      },
    }
  }

  fn handle(&self, state: &mut IssueState) -> () {
    match self {
      IssueEvent::IssueCreated {
        timestamp, payload, ..
      } => {
        state.exists = true;
        state.title = Some(payload.title.to_owned());
        state.status_id = match &payload.status_id {
          Some(x) => Some(x.to_string()),
          None => None,
        };
        state.created_at = Some(*timestamp);
        state.updated_at = Some(*timestamp);
      }
    }
  }
}
