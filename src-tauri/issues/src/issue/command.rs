use serde::{Deserialize, Serialize};

use geeks_tracker_core::Command;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum IssueCommand {
  CreateIssue {
    id: String,
    title: String,
    status_id: Option<String>,
  },
}

impl Command for IssueCommand {
  fn name(&self) -> &'static str {
    match self {
      IssueCommand::CreateIssue { .. } => "CreateIssue",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      IssueCommand::CreateIssue { id, .. } => id,
    }
  }
}
