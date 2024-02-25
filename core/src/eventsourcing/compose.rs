use crate::eventsourcing::{
  Aggregate, AggregateRoot, Eventstore, PersistedEvent, Snapshot, VersionSelect,
};

pub async fn get_unsaved_events<T, E>(
  root: &AggregateRoot<T>,
  eventstore: &E,
) -> Result<Vec<PersistedEvent<T::Event>>, E::Error>
where
  T: Aggregate,
  E: Eventstore<Event = T::Event>,
{
  let versions = root.versions.clone();
  let mut unsaved_events = Vec::new();
  let read_events = versions.iter().map(|(id, version)| async {
    eventstore
      .read(id.to_owned(), VersionSelect::From(*version + 1))
      .await
  });

  for events in read_events {
    let events = events.await?;
    unsaved_events.append(&mut events.clone());
  }

  Ok(unsaved_events)
}

#[derive(thiserror::Error, Debug)]
pub enum LoadAggregateError<E, EE, SE> {
  #[error("aggregate error: {0}")]
  Aggregate(#[source] E),

  #[error("eventstore error: {0}")]
  Eventstore(#[source] EE),

  #[error("snapshot error: {0}")]
  Snapshot(#[source] SE),
}

pub async fn load_aggregate<T, E, S>(
  eventstore: E,
  snapshot: S,
) -> Result<AggregateRoot<T>, LoadAggregateError<T::Error, E::Error, S::Error>>
where
  T: Aggregate,
  E: Eventstore<Event = T::Event>,
  S: Snapshot<T>,
{
  let mut root = snapshot
    .load()
    .await
    .map_err(LoadAggregateError::Snapshot)?;
  let unsaved_events = get_unsaved_events(&root, &eventstore)
    .await
    .map_err(LoadAggregateError::Eventstore)?;
  root
    .save_events(unsaved_events)
    .map_err(LoadAggregateError::Aggregate)?;
  snapshot
    .save(&root)
    .await
    .map_err(LoadAggregateError::Snapshot)?;
  Ok(root)
}
