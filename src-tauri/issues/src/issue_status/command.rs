use serde::{Deserialize, Serialize};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::Command;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum IssueStatusCommand {
  CreateIssueStatus {
    id: String,
    title: String,
    color: Option<RGB>,
  },
}

impl Command for IssueStatusCommand {
  fn name(&self) -> &'static str {
    match self {
      IssueStatusCommand::CreateIssueStatus { .. } => "CreateIssueStatus",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      IssueStatusCommand::CreateIssueStatus { id, .. } => id,
    }
  }
}
