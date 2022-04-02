use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::aggregate::AggregateState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
  pub name: String,
  pub timestamp: i64,
  pub aggregate_id: String,
  pub aggregate_version: i64,
  pub payload: Value,
}

pub trait Event<S>: TryFrom<EventData> + Into<EventData>
where
  S: AggregateState,
{
  fn handle(&self, state: &mut S) -> ();
}
