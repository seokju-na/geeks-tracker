use async_trait::async_trait;

use crate::{Event, PersistedEvent, Version};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionSelect {
  All,
  From(Version),
}

#[async_trait]
pub trait Eventstore: Send + Sync {
  type Event: Event;
  type Error: Send + Sync;

  fn stream(
    &self,
    id: &str,
    select: VersionSelect,
  ) -> Result<Vec<PersistedEvent<Self::Event>>, Self::Error>;

  async fn append(
    &self,
    id: String,
    events: Vec<PersistedEvent<Self::Event>>,
  ) -> Result<(), Self::Error>;
}
