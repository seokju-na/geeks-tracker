use std::{
  collections::HashMap,
  convert::Infallible,
  sync::{Arc, RwLock},
};

use async_trait::async_trait;
use futures::stream::{iter, StreamExt};

use crate::{Event, Eventstore, PersistedEvent, Stream, VersionSelect};

#[derive(Debug)]
struct InMemoryBackend<T>
where
  T: Event,
{
  streams: HashMap<String, Vec<PersistedEvent<T>>>,
}

impl<T> Default for InMemoryBackend<T>
where
  T: Event,
{
  fn default() -> Self {
    Self {
      streams: HashMap::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct InMemory<T>
where
  T: Event,
{
  backend: Arc<RwLock<InMemoryBackend<T>>>,
}

impl<T> Default for InMemory<T>
where
  T: Event,
{
  fn default() -> Self {
    Self {
      backend: Arc::default(),
    }
  }
}

#[async_trait]
impl<T> Eventstore for InMemory<T>
where
  T: Event + Clone + Send + Sync,
{
  type Event = T;
  type Error = Infallible;

  fn stream(
    &self,
    id: &str,
    select: VersionSelect,
  ) -> Stream<PersistedEvent<Self::Event>, Self::Error> {
    let backend = self.backend.read().expect("locked");
    let events = backend
      .streams
      .get(id)
      .cloned()
      .unwrap_or_default()
      .into_iter()
      .filter(move |event| match select {
        VersionSelect::All => true,
        VersionSelect::From(v) => event.version >= v,
      });

    iter(events).map(Ok).boxed()
  }

  async fn append(
    &self,
    id: String,
    events: Vec<PersistedEvent<Self::Event>>,
  ) -> Result<(), Self::Error> {
    let mut events_to_append = events.into_iter().collect();
    let mut backend = self
      .backend
      .write()
      .expect("acquire write lock on event store backend");

    backend
      .streams
      .entry(id)
      .and_modify(|x| {
        x.append(&mut events_to_append);
      })
      .or_insert_with(|| events_to_append);

    Ok(())
  }
}
