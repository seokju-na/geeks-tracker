use serde::{Deserialize, Serialize};

use crate::eventsourcing::Version;

pub trait Event: Send + Sync + Clone {
  fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedEvent<T>
where
  T: Event,
{
  pub aggregate_id: String,
  pub version: Version,
  pub event: T,
}
