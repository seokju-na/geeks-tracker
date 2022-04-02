use std::error::Error;

use crate::aggregate::{Aggregate, AggregateState};
use crate::command::Command;
use crate::eventstore::Eventstore;
use crate::saga::Saga;
use crate::EventData;

pub struct Application {
  eventstore: Box<dyn Eventstore>,
  sagas: Vec<Box<dyn Saga>>,
}

impl Application {
  pub fn new(eventstore: Box<dyn Eventstore>, sagas: Vec<Box<dyn Saga>>) -> Self {
    Self { eventstore, sagas }
  }

  pub fn get_state<'a, S>(&self, aggregate: &'a Aggregate<S>, aggregate_id: &str) -> Option<&'a S>
  where
    S: AggregateState,
  {
    aggregate.get_state(aggregate_id)
  }

  pub fn execute_command<S, C>(
    &mut self,
    aggregate: &mut Aggregate<S>,
    command: C,
  ) -> Result<EventData, Box<dyn Error>>
  where
    S: AggregateState,
    C: Command<S>,
  {
    let event_data = aggregate.execute_command(command)?;

    self.eventstore.append(&event_data);
    self.sagas.iter().for_each(|saga| {
      saga.effect(&event_data);
    });

    Ok(event_data)
  }
}
