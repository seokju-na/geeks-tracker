use chrono::Utc;
use serde::{Deserialize, Serialize};

use geeks_tracker_core::Command;

use crate::issue::error::IssueError;
use crate::issue::event::IssueCreatedPayload;
use crate::issue::{IssueEvent, IssueState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum IssueCommand {
  Create {
    id: String,
    title: String,
    status_id: Option<String>,
  },
}

impl Command<IssueState> for IssueCommand {
  type Event = IssueEvent;
  type Error = IssueError;

  fn aggregate_id(&self) -> &str {
    match self {
      IssueCommand::Create { id, .. } => id,
    }
  }

  fn handle(&self, state: &IssueState, version: i64) -> Result<Self::Event, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match self {
      IssueCommand::Create {
        id,
        title,
        status_id,
      } => {
        if state.exists {
          return Err(IssueError::IssueAlreadyExists);
        }

        Ok(IssueEvent::Created {
          id: id.to_owned(),
          version,
          timestamp,
          payload: IssueCreatedPayload {
            title: title.to_owned(),
            status_id: status_id.to_owned(),
          },
        })
      }
    }
  }
}
