use std::error::Error;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use geeks_tracker_core::Command;

use crate::issue::error::IssueError;
use crate::issue::event::IssueCreatedPayload;
use crate::issue::{IssueEvent, IssueState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum IssueCommand {
  CreateIssue {
    id: String,
    title: String,
    status_id: Option<String>,
  },
}

impl Command<IssueState> for IssueCommand {
  type Event = IssueEvent;

  fn aggregate_id(&self) -> &str {
    match self {
      IssueCommand::CreateIssue { id, .. } => id,
    }
  }

  fn handle(&self, state: &IssueState, version: i64) -> Result<Self::Event, Box<dyn Error>> {
    let timestamp = Utc::now().timestamp();

    match self {
      IssueCommand::CreateIssue {
        id,
        title,
        status_id,
      } => {
        if state.exists {
          return Err(Box::new(IssueError::IssueAlreadyExists));
        }

        Ok(IssueEvent::IssueCreated {
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
