use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::{Event, EventData, EventParseError};

use crate::issue_status::state::IssueStatusState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatusCreatedPayload {
  pub title: String,
  pub color: Option<RGB>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum IssueStatusEvent {
  Created {
    id: String,
    version: i64,
    timestamp: i64,
    payload: IssueStatusCreatedPayload,
  },
}

impl Event<IssueStatusState> for IssueStatusEvent {
  fn from_event_data(event_data: EventData) -> Result<Self, EventParseError> {
    let id = event_data.aggregate_id;
    let version = event_data.aggregate_version;
    let timestamp = event_data.timestamp;

    match event_data.name.as_str() {
      "IssueStatusCreated" => {
        let payload = from_value::<IssueStatusCreatedPayload>(event_data.payload)
          .map_err(|_| EventParseError::Fail)?;
        Ok(IssueStatusEvent::Created {
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
      IssueStatusEvent::Created {
        id,
        version,
        timestamp,
        payload,
      } => EventData {
        name: "IssueStatusCreated".to_string(),
        aggregate_id: id,
        aggregate_version: version,
        timestamp,
        payload: to_value(payload).unwrap(),
      },
    }
  }

  fn handle(&self, state: &mut IssueStatusState) -> () {
    match self {
      IssueStatusEvent::Created {
        timestamp, payload, ..
      } => {
        state.exists = true;
        state.title = Some(payload.title.to_owned());
        if let Some(x) = &payload.color {
          state.color = x.to_owned();
        }
        state.created_at = Some(*timestamp);
        state.updated_at = Some(*timestamp);
      }
    }
  }
}
