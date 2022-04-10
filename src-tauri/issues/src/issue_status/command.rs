use chrono::Utc;
use serde::{Deserialize, Serialize};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::Command;

use crate::issue_status::{
  IssueStatusCreatedPayload, IssueStatusError, IssueStatusEvent, IssueStatusState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum IssueStatusCommand {
  Create {
    id: String,
    title: String,
    color: Option<RGB>,
  },
}

impl Command<IssueStatusState> for IssueStatusCommand {
  type Event = IssueStatusEvent;
  type Error = IssueStatusError;

  fn aggregate_id(&self) -> &str {
    match self {
      IssueStatusCommand::Create { id, .. } => id,
    }
  }

  fn handle(&self, state: &IssueStatusState, version: i64) -> Result<Self::Event, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match self {
      IssueStatusCommand::Create { id, title, color } => {
        if state.exists {
          return Err(IssueStatusError::IssueStatusAlreadyExists);
        }

        Ok(IssueStatusEvent::Created {
          id: id.to_owned(),
          version,
          timestamp,
          payload: IssueStatusCreatedPayload {
            title: title.to_owned(),
            color: color.to_owned(),
          },
        })
      }
    }
  }
}
