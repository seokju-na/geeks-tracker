use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::eventsourcing::Version;

pub trait Event: Send + Sync + Clone {
  fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Persisted<T>
where
  T: Event,
{
  pub aggregate_id: String,
  pub version: Version,
  pub event: T,
}
