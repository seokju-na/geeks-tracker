use std::collections::HashMap;

use async_trait::async_trait;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::eventstore::Eventstore;
use crate::eventstore_error::EventstoreError;

#[derive(Serialize, Deserialize)]
struct Event {
  name: String,
  message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecordedEvent {
  id: String,
  name: String,
  message: String,
}

impl RecordedEvent {
  fn from(event: &Event) -> Self {
    let id = nanoid!();

    Self {
      id,
      name: event.name.to_owned(),
      message: event.message.to_owned(),
    }
  }
}

pub struct MemoryEventstore {
  records: HashMap<String, Vec<RecordedEvent>>,
}

impl MemoryEventstore {
  fn new() -> Self {
    Self {
      records: HashMap::new(),
    }
  }

  fn add_stream(&mut self, stream_name: &str) {
    self.records.insert(stream_name.to_string(), Vec::new());
  }

  fn get_stream(
    &mut self,
    stream_name: &str,
  ) -> Result<&mut Vec<RecordedEvent>, EventstoreError> {
    match self.records.get_mut(stream_name) {
      Some(x) => Ok(x),
      None => Err(EventstoreError::StreamNotExists),
    }
  }
}

#[async_trait]
impl Eventstore<Event, RecordedEvent> for MemoryEventstore {
  async fn append(
    &mut self,
    stream_name: &str,
    events: Vec<Event>,
  ) -> Result<Vec<RecordedEvent>, EventstoreError> {
    if events.len() == 0 {
      return Err(EventstoreError::EventsAreEmpty);
    }

    let stream = self.get_stream(stream_name)?;
    let recorded = events
      .into_iter()
      .map(|event| RecordedEvent::from(&event))
      .collect::<Vec<RecordedEvent>>();

    stream.append(&mut recorded.clone());

    Ok(recorded)
  }
}

#[cfg(test)]
mod memory_eventstore_tests {
    use crate::eventstore::Eventstore;
    use crate::eventstore_error::EventstoreError;
    use crate::memory_eventstore::{Event, MemoryEventstore};

    #[tokio::test]
  async fn should_error_when_events_are_empty() {
    let mut eventstore = MemoryEventstore::new();

    let err = eventstore.append("stream", Vec::new()).await.unwrap_err();
    assert!(matches!(err, EventstoreError::EventsAreEmpty));
  }

  #[tokio::test]
  async fn should_error_when_stream_not_exists() {
    let mut eventstore = MemoryEventstore::new();
    let mut events = Vec::<Event>::new();
    events.push(Event {
      name: "name".to_string(),
      message: "message".to_string(),
    });

    let err = eventstore
      .append("not found stream", events)
      .await
      .unwrap_err();
    assert!(matches!(err, EventstoreError::StreamNotExists));
  }

  #[tokio::test]
  async fn should_append_events() {
    let mut eventstore = MemoryEventstore::new();
    eventstore.add_stream("stream");

    let mut events = Vec::<Event>::new();
    events.push(Event {
      name: "pizza".to_string(),
      message: "1".to_string(),
    });
    events.push(Event {
      name: "chicken".to_string(),
      message: "2".to_string(),
    });

    let recorded = eventstore.append("stream", events).await.unwrap();
    assert_eq!(recorded.len(), 2);
  }
}
