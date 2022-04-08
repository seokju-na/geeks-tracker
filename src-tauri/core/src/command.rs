use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::aggregate::AggregateState;
use crate::event::Event;

pub trait Command<S>: Serialize + DeserializeOwned
where
  S: AggregateState,
{
  type Event: Event<S>;

  fn aggregate_id(&self) -> &str;
  fn handle(&self, state: &S, version: i64) -> Result<Self::Event, Box<dyn Error>>;
}
