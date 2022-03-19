use eventstore::eventstore::{Event, Eventstore, RecordedEvent};
use eventstore::eventstore_error::EventstoreError;
use nanoid::nanoid;

pub struct MockEventstore {
  store: Vec<RecordedEvent>,
}

impl MockEventstore {
  pub fn new() -> Self {
    Self { store: Vec::new() }
  }

  pub fn record(event: &Event) -> RecordedEvent {
    RecordedEvent {
      id: nanoid!(),
      name: event.name.to_owned(),
      data: event.data.to_owned(),
    }
  }

  pub fn add(&mut self, event: RecordedEvent) {
    self.store.push(event)
  }

  pub fn remove(&mut self, index: usize) -> RecordedEvent {
    self.store.remove(index)
  }

  pub fn clear(&mut self) {
    self.store.clear()
  }
}

impl Eventstore for MockEventstore {
  fn append(&mut self, event: Event) -> Result<RecordedEvent, EventstoreError> {
    let recorded_event = MockEventstore::record(&event);
    self.store.push(recorded_event.clone());

    Ok(recorded_event)
  }

  fn read(&mut self) -> Result<Vec<RecordedEvent>, EventstoreError> {
    Ok(self.store.clone())
  }
}
