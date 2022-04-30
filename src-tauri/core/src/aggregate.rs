use std::collections::HashMap;

use crate::{Command, Event, PersistedEvent, Version};

pub trait Aggregate: Sized + Send + Sync {
  type Command: Command;
  type Event: Event;
  type Error: Send + Sync;

  fn id(&self) -> &str;

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error>;

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub struct AggregateRoot<T>
where
  T: Aggregate,
{
  states: HashMap<String, T>,
  versions: HashMap<String, Version>,
}

impl<T> AggregateRoot<T>
where
  T: Aggregate + Clone,
  T::Event: Clone,
{
  pub fn new() -> Self {
    Self {
      states: HashMap::new(),
      versions: HashMap::new(),
    }
  }

  pub fn get_state<K: AsRef<str>>(&self, id: K) -> Option<&T> {
    self.states.get(id.as_ref())
  }

  pub fn get_version<K: AsRef<str>>(&self, id: K) -> Option<&Version> {
    self.versions.get(id.as_ref())
  }

  pub fn execute_command(
    &mut self,
    command: T::Command,
  ) -> Result<PersistedEvent<T::Event>, T::Error> {
    let id = command.aggregate_id().to_owned();
    let event = T::handle_command(self.states.get(&id), command)?;
    let state = T::apply_event(self.states.get(&id).cloned(), event.clone())?;
    self.states.insert(id.to_owned(), state);

    let version = self.versions.entry(id.to_owned()).or_insert(0);
    *version += 1;

    let persisted = PersistedEvent {
      stream_id: id,
      version: *version,
      event,
    };

    Ok(persisted)
  }
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::testing::{Todo, TodoCommand, TodoError, TodoEvent, TodoStatus};
  use crate::{AggregateRoot, PersistedEvent};

  #[test]
  fn execute_command_and_returns_persisted_event() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::new();
    let command = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Drink soda".to_string(),
      status: Some(TodoStatus::InProgress),
    };

    let persisted = todo_root.execute_command(command).unwrap();
    assert_eq!(
      persisted,
      PersistedEvent {
        stream_id: "todo_0".to_string(),
        version: 1,
        event: TodoEvent::TodoCreated {
          id: "todo_0".to_string(),
          title: "Drink soda".to_string(),
          status: TodoStatus::InProgress,
        },
      }
    );
  }

  #[test]
  fn execute_command_can_mutates_state() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::new();
    assert_matches!(todo_root.get_state("todo_0"), None);

    let command1 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
      status: None,
    };
    todo_root.execute_command(command1).unwrap();

    let todo = todo_root.get_state("todo_0");
    assert_matches!(
      todo,
      Some(x) if
      x.title == "Eat rice"
      && x.status == TodoStatus::Todo
    );

    let command2 = TodoCommand::UpdateTodoTitle {
      id: "todo_0".to_string(),
      title: "Eat pizza".to_string(),
    };
    todo_root.execute_command(command2).unwrap();

    let todo = todo_root.get_state("todo_0");
    assert_matches!(
      todo,
      Some(x) if
      x.title == "Eat pizza"
      && x.status == TodoStatus::Todo
    );

    let command3 = TodoCommand::UpdateTodoStatus {
      id: "todo_0".to_string(),
      status: TodoStatus::Done,
    };
    todo_root.execute_command(command3).unwrap();

    let todo = todo_root.get_state("todo_0");
    assert_matches!(
      todo,
      Some(x) if
      x.title == "Eat pizza"
      && x.status == TodoStatus::Done
    );
  }

  #[test]
  fn execute_command_should_increase_state_version() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::new();
    assert_matches!(todo_root.get_version("todo_0"), None);

    let command1 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
      status: None,
    };
    todo_root.execute_command(command1).unwrap();
    assert_matches!(todo_root.get_version("todo_0"), Some(1));

    let command2 = TodoCommand::UpdateTodoTitle {
      id: "todo_0".to_string(),
      title: "More rice".to_string(),
    };
    todo_root.execute_command(command2).unwrap();
    assert_matches!(todo_root.get_version("todo_0"), Some(2));
  }

  #[test]
  fn error_when_execute_command_fail() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::new();

    let command1 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
      status: None,
    };
    todo_root.execute_command(command1).unwrap();

    let command2 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat pizza".to_string(),
      status: Some(TodoStatus::Done),
    };
    let err = todo_root.execute_command(command2).unwrap_err();

    assert_matches!(err, TodoError::AlreadyExists);
  }
}
