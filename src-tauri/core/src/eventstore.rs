use crate::EventData;

pub trait Eventstore {
  fn append(&mut self, event: &EventData) -> ();
}
