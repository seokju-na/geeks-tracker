use std::collections::HashMap;

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

  pub fn execute_command<C: Command<S>>(&mut self, command: C) -> Result<EventData, C::Error> {
    let state = self
      .states
      .entry(String::from(command.aggregate_id()))
      .or_default();

    let version = self
      .versions
      .entry(String::from(command.aggregate_id()))
      .or_default();

    *version += 1;
    let event = command.handle(state, *version)?;
    event.handle(state);

    Ok(event.to_event_data())
  }
}

#[cfg(test)]
mod aggregate_tests {
  use std::assert_matches::assert_matches;

  use serde_json::json;

  use crate::testing::{TodoCommand, TodoError, TodoState, TodoStatus};
  use crate::{Aggregate, EventData};

  #[test]
  fn execute_command_and_returns_event_data() {
    let mut aggregate = Aggregate::<TodoState>::new();
    let command = TodoCommand::Created {
      id: "todo_0".to_string(),
      title: "Eat pizza".to_string(),
    };

    let event_data = aggregate.execute_command(command).unwrap();
    assert_matches!(
      event_data,
      EventData { name, aggregate_id, aggregate_version, payload, .. }
      if name == "TodoCreated"
      && aggregate_id == "todo_0"
      && aggregate_version == 1
      && payload == json!({
        "title": "Eat pizza"
      })
    );
  }

  #[test]
  fn execute_command_can_mutates_state() {
    let mut aggregate = Aggregate::<TodoState>::new();
    assert_matches!(aggregate.get_state("todo_0"), None);

    let command1 = TodoCommand::Created {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
    };
    aggregate.execute_command(command1).unwrap();

    let state = aggregate.get_state("todo_0").unwrap();
    assert_matches!(&state.title, Some(x) if x == "Eat rice");

    let command2 = TodoCommand::UpdateTitle {
      id: "todo_0".to_string(),
      title: "More rice".to_string(),
    };
    aggregate.execute_command(command2).unwrap();

    let state = aggregate.get_state("todo_0").unwrap();
    assert_matches!(&state.title, Some(x) if x == "More rice");

    let command3 = TodoCommand::UpdateStatus {
      id: "todo_0".to_string(),
      status: TodoStatus::InProgress,
    };
    aggregate.execute_command(command3).unwrap();

    let state = aggregate.get_state("todo_0").unwrap();
    assert_matches!(&state.status, TodoStatus::InProgress);
  }

  #[test]
  fn execute_command_should_increase_state_version() {
    let mut aggregate = Aggregate::<TodoState>::new();
    assert_matches!(aggregate.get_version("todo_0"), None);

    let command1 = TodoCommand::Created {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
    };
    aggregate.execute_command(command1).unwrap();
    assert_matches!(aggregate.get_version("todo_0"), Some(1));

    let command2 = TodoCommand::UpdateTitle {
      id: "todo_0".to_string(),
      title: "More rice".to_string(),
    };
    aggregate.execute_command(command2).unwrap();
    assert_matches!(aggregate.get_version("todo_0"), Some(2));
  }

  #[test]
  fn error_when_execute_command_fail() {
    let mut aggregate = Aggregate::<TodoState>::new();

    let command1 = TodoCommand::Created {
      id: "todo_0".to_string(),
      title: "Watch movie".to_string(),
    };
    aggregate.execute_command(command1).unwrap();

    let command2 = TodoCommand::Created {
      id: "todo_0".to_string(),
      title: "Again watch movie".to_string(),
    };
    let err = aggregate
      .execute_command(command2)
      .expect_err("should error");

    assert_matches!(err, TodoError::TodoAlreadyExists);
  }
}
