use serde::{Deserialize, Serialize};

use geeks_tracker_core::Event;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum IssueEvent {
  IssueCreated {
    id: String,
    title: String,
    status_id: Option<String>,
  },
}

impl Event for IssueEvent {
  fn name(&self) -> &'static str {
    match self {
      IssueEvent::IssueCreated { .. } => "IssueCreated",
    }
  }
}
