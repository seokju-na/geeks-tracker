use serde::{Deserialize, Serialize};

use geeks_tracker_common::colors::RGB;
use geeks_tracker_core::AggregateState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatusState {
  pub exists: bool,
  pub title: Option<String>,
  pub color: RGB,
  pub created_at: Option<i64>,
  pub updated_at: Option<i64>,
}

impl Default for IssueStatusState {
  fn default() -> Self {
    Self {
      exists: false,
      title: None,
      color: RGB::white(),
      created_at: None,
      updated_at: None,
    }
  }
}

impl AggregateState for IssueStatusState {}
