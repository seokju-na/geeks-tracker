use std::error::Error;

use crate::EventData;

pub trait Saga {
  fn effect(&self, event_data: &EventData) -> Result<(), Box<dyn Error>>;
}
