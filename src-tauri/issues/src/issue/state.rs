use serde::{Deserialize, Serialize};

use geeks_tracker_core::AggregateState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueState {
  pub exists: bool,
  pub title: Option<String>,
  pub status_id: Option<String>,
  pub created_at: Option<i64>,
  pub updated_at: Option<i64>,
}

impl Default for IssueState {
  fn default() -> Self {
    Self {
      exists: false,
      title: None,
      status_id: None,
      created_at: None,
      updated_at: None,
    }
  }
}

impl AggregateState for IssueState {}
