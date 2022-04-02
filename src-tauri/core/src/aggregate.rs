use std::collections::HashMap;
use std::error::Error;

use crate::command::Command;
use crate::event::Event;
use crate::EventData;

pub trait AggregateState: Default + Clone {}

pub struct Aggregate<S>
where
  S: AggregateState,
{
  states: HashMap<String, S>,
  versions: HashMap<String, i64>,
}

impl<S> Aggregate<S>
where
  S: AggregateState,
{
  pub fn new() -> Self {
    Self {
      states: HashMap::new(),
      versions: HashMap::new(),
    }
  }

  pub fn get_state(&self, aggregate_id: &str) -> Option<&S> {
    self.states.get(aggregate_id)
  }

  pub fn get_version(&self, aggregate_id: &str) -> Option<&i64> {
    self.versions.get(aggregate_id)
  }

  pub fn execute_command<C: Command<S>>(
    &mut self,
    command: C,
  ) -> Result<EventData, Box<dyn Error>> {
    let state = self
      .states
      .entry(String::from(command.aggregate_id()))
      .or_default();

    let version = self
      .versions
      .entry(String::from(command.aggregate_id()))
      .or_default();

    let event = command.handle(state, *version)?;
    event.handle(state);
    *version += 1;

    Ok(event.into())
  }
}

#[cfg(test)]
mod aggregate_tests {
  use serde_json::json;

  use crate::testing::{CreateItemPayload, ItemCommand, ItemState, UpdateItemTitlePayload};
  use crate::Aggregate;

  #[test]
  fn execute_command_returns_event_data() {
    let mut aggregate = Aggregate::<ItemState>::new();
    let command = ItemCommand::Create(CreateItemPayload {
      id: "item_0".to_string(),
      title: "Hello".to_string(),
    });

    let event_data = aggregate.execute_command(command).unwrap();
    assert_eq!(event_data.name, "ItemCreated");
    assert_eq!(event_data.aggregate_id, "item_0");
    assert_eq!(event_data.payload, json!({ "title": "Hello" }))
  }

  #[test]
  fn execute_command_mutates_aggregate_state() {
    let mut aggregate = Aggregate::<ItemState>::new();
    assert!(matches!(aggregate.get_state("item_0"), None));

    let command = ItemCommand::Create(CreateItemPayload {
      id: "item_0".to_string(),
      title: "Hello".to_string(),
    });
    aggregate.execute_command(command).unwrap();

    let state = aggregate.get_state("item_0");
    let _expected = ItemState {
      exists: true,
      title: Some("Hello".to_string()),
    };
    assert!(matches!(state, Some(_expected)));
  }

  #[test]
  fn execute_command_increase_state_version() {
    let mut aggregate = Aggregate::<ItemState>::new();
    assert_eq!(aggregate.get_version("item_0"), None);

    let command1 = ItemCommand::Create(CreateItemPayload {
      id: "item_0".to_string(),
      title: "Hello".to_string(),
    });
    aggregate.execute_command(command1).unwrap();
    assert_eq!(aggregate.get_version("item_0"), Some(&1));

    let command2 = ItemCommand::UpdateTitle(UpdateItemTitlePayload {
      id: "item_0".to_string(),
      title: "Hello Again".to_string(),
    });
    aggregate.execute_command(command2).unwrap();
    assert_eq!(aggregate.get_version("item_0"), Some(&2));
  }
}
