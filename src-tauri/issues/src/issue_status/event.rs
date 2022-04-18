use serde::{Deserialize, Serialize};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::Event;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum IssueStatusEvent {
  IssueStatusCreated {
    id: String,
    title: String,
    color: RGB,
  },
}

impl Event for IssueStatusEvent {
  fn name(&self) -> &'static str {
    match self {
      IssueStatusEvent::IssueStatusCreated { .. } => "IssueStatusCreated",
    }
  }
}
