use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::aggregate::AggregateState;

#[derive(thiserror::Error, Debug)]
pub enum EventParseError {
  #[error("Fail")]
  Fail,
  #[error("No Matches")]
  NoMatches,
}

pub trait Event<S>: Serialize + DeserializeOwned
where
  S: AggregateState,
{
  fn from_event_data(event_data: EventData) -> Result<Self, EventParseError>;
  fn to_event_data(self) -> EventData;
  fn handle(&self, state: &mut S) -> ();
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventData {
  pub name: String,
  pub aggregate_id: String,
  pub aggregate_version: i64,
  pub timestamp: i64,
  pub payload: Value,
}
