use std::collections::HashMap;

use crate::{EventData, Eventstore};

pub struct MockEventstore {
  record: HashMap<String, Vec<EventData>>,
}

impl Eventstore for MockEventstore {
  fn append(&mut self, event_data: EventData) -> () {
    self
      .record
      .entry(event_data.aggregate_id.to_owned())
      .or_default()
      .push(event_data);
  }
}

impl MockEventstore {
  fn new() -> Self {
    Self {
      record: HashMap::new(),
    }
  }
}
