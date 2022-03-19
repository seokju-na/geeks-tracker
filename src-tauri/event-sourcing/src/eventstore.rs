use crate::eventstore_error::EventstoreError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  pub name: String,
  pub data: String,
}

impl Event {
  pub fn new<B: Serialize>(name: &str, data: &B) -> Self {
    Self {
      name: name.to_string(),
      data: serde_json::to_string(data).unwrap(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordedEvent {
  pub id: String,
  pub name: String,
  pub data: String,
}

pub trait Eventstore {
  fn append(&mut self, event: Event) -> Result<RecordedEvent, EventstoreError>;
  fn read(&mut self) -> Result<Vec<RecordedEvent>, EventstoreError>;
}
