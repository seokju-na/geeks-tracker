use crate::issue::{IssueCommand, IssueError, IssueEvent};
use chrono::Utc;
use geeks_tracker_core::{Aggregate, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
  pub id: String,
  pub title: String,
  pub status_id: Option<String>,
  pub created_at: Timestamp,
  pub updated_at: Timestamp,
}

impl Aggregate for Issue {
  type Command = IssueCommand;
  type Event = IssueEvent;
  type Error = IssueError;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match command {
      IssueCommand::CreateIssue {
        id,
        title,
        status_id,
      } => {
        if this.is_some() {
          return Err(IssueError::AlreadyExists);
        }
        Ok(IssueEvent::IssueCreated {
          id,
          title,
          status_id,
        })
      }
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match event {
      IssueEvent::IssueCreated {
        id,
        title,
        status_id,
      } => match this {
        Some(_) => Err(IssueError::AlreadyExists),
        None => Ok(Issue {
          id,
          title,
          status_id,
          created_at: timestamp,
          updated_at: timestamp,
        }),
      },
    }
  }
}

#[cfg(test)]
mod test {
  use crate::issue::{Issue, IssueCommand, IssueEvent};
  use geeks_tracker_core::{AggregateRoot, PersistedEvent};
  use std::assert_matches::assert_matches;

  #[test]
  fn create_issue_command() {
    let mut issue_root: AggregateRoot<Issue> = AggregateRoot::new();
    let command = IssueCommand::CreateIssue {
      id: "issue_0".to_string(),
      title: "My Issue".to_string(),
      status_id: None,
    };

    let persisted = issue_root.execute_command(command).unwrap();
    assert_eq!(
      persisted,
      PersistedEvent {
        stream_id: "issue_0".to_string(),
        version: 1,
        event: IssueEvent::IssueCreated {
          id: "issue_0".to_string(),
          title: "My Issue".to_string(),
          status_id: None,
        }
      }
    );
    assert_matches!(
      issue_root.get_state("issue_0"),
      Some(x) if
      x.id == "issue_0"
      && x.title == "My Issue"
      && x.status_id == None
    );
  }
}
