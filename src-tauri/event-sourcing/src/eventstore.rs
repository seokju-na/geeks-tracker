use crate::event::Event;
use crate::eventstore_error::EventstoreError;
use crate::recorded_event::RecordedEvent;

pub trait Eventstore {
  fn append(&mut self, event: Event) -> Result<RecordedEvent, EventstoreError>;
  fn read(&mut self) -> Result<Vec<RecordedEvent>, EventstoreError>;
}
