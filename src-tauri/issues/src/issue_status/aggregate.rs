use chrono::Utc;
use serde::{Deserialize, Serialize};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::{Aggregate, Timestamp};

use crate::issue_status::{IssueStatusCommand, IssueStatusError, IssueStatusEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatus {
  pub id: String,
  pub title: String,
  pub color: RGB,
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

impl Aggregate for IssueStatus {
  type Command = IssueStatusCommand;
  type Event = IssueStatusEvent;
  type Error = IssueStatusError;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match command {
      IssueStatusCommand::CreateIssueStatus { id, title, color } => {
        if this.is_some() {
          return Err(IssueStatusError::AlreadyExists);
        }
        Ok(IssueStatusEvent::IssueStatusCreated {
          id,
          title,
          color: color.unwrap_or(RGB::white()),
        })
      }
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match event {
      IssueStatusEvent::IssueStatusCreated { id, title, color } => match this {
        Some(_) => Err(IssueStatusError::AlreadyExists),
        None => Ok(IssueStatus {
          id,
          title,
          color,
          created_at: timestamp,
          updated_at: timestamp,
        }),
      },
    }
  }
}
