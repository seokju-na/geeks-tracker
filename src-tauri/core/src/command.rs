use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::aggregate::AggregateState;
use crate::event::Event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandData {
  pub name: String,
  pub aggregate_id: String,
  pub payload: Option<Value>,
}

pub trait Command<S>: TryFrom<CommandData>
where
  S: AggregateState,
{
  type Event: Event<S>;

  fn aggregate_id(&self) -> &str;
  fn handle(&self, state: &S, version: i64) -> Result<Self::Event, Box<dyn Error>>;
}
