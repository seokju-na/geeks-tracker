use async_trait::async_trait;

use crate::eventsourcing::{Event, PersistedEvent, Version};

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
  ) -> Result<Vec<PersistedEvent<Self::Event>>, Self::Error>;

  async fn append(&self, events: Vec<PersistedEvent<Self::Event>>) -> Result<(), Self::Error>;
}
