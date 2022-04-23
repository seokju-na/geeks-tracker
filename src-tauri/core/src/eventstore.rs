use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::{Event, PersistedEvent, Version};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionSelect {
  All,
  From(Version),
}

pub type Stream<'a, Item, Err> = BoxStream<'a, Result<Item, Err>>;

#[async_trait]
pub trait Eventstore: Send + Sync {
  type Event: Event + Send + Sync;
  type Error: Send + Sync;

  fn stream(
    &self,
    id: &str,
    select: VersionSelect,
  ) -> Stream<PersistedEvent<Self::Event>, Self::Error>;

  async fn append(
    &self,
    id: String,
    events: Vec<PersistedEvent<Self::Event>>,
  ) -> Result<(), Self::Error>;
}
