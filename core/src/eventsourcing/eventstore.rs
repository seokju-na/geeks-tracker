use async_trait::async_trait;

use crate::eventsourcing::{Event, Persisted, Version};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionSelect {
  All,
  From(Version),
}

#[async_trait]
pub trait Eventstore: Send + Sync {
  type Event: Event;
  type Error: Send + Sync;

  async fn read(
    &self,
    aggregate_id: String,
    select: VersionSelect,
  ) -> Result<Vec<Persisted<Self::Event>>, Self::Error>;

  async fn append(&self, events: Vec<Persisted<Self::Event>>) -> Result<(), Self::Error>;
}
