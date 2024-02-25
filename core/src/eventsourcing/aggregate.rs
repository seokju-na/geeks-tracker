use std::collections::HashMap;

use crate::eventsourcing::{Command, Event, PersistedEvent, Version};

pub trait Aggregate: Sized + Send + Sync + Clone {
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

#[derive(Debug, Clone)]
pub struct AggregateRoot<T>
where
  T: Aggregate,
{
  pub states: HashMap<String, T>,
  pub versions: HashMap<String, Version>,
}

impl<T> Default for AggregateRoot<T>
where
  T: Aggregate,
{
  fn default() -> Self {
    Self {
      states: HashMap::new(),
      versions: HashMap::new(),
    }
  }
}

impl<T> AggregateRoot<T>
where
  T: Aggregate,
{
  pub fn new(states: HashMap<String, T>, versions: HashMap<String, Version>) -> Self {
    Self { states, versions }
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
      aggregate_id: id,
      version: *version,
      event,
    };

    Ok(persisted)
  }

  pub fn save_events(&mut self, events: Vec<PersistedEvent<T::Event>>) -> Result<(), T::Error> {
    for persisted in events {
      let id = persisted.aggregate_id.to_owned();
      let state = T::apply_event(self.states.get(&id).cloned(), persisted.event)?;
      self.states.insert(id.to_owned(), state);
      self.versions.insert(id.to_owned(), persisted.version);
    }

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::eventsourcing::dummy::{Todo, TodoCommand, TodoError, TodoEvent, TodoStatus};
  use crate::eventsourcing::{AggregateRoot, PersistedEvent};

  #[test]
  fn execute_command_and_returns_persisted_event() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::default();
    let command = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Drink soda".to_string(),
      status: Some(TodoStatus::InProgress),
    };

    let persisted = todo_root.execute_command(command).unwrap();
    assert_eq!(
      persisted,
      PersistedEvent {
        aggregate_id: "todo_0".to_string(),
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
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::default();
    assert!(todo_root.get_state("todo_0").is_none());

    let command1 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
      status: None,
    };
    todo_root.execute_command(command1).unwrap();

    let todo = todo_root.get_state("todo_0").unwrap();
    assert_eq!(todo.title, "Eat rice");
    assert_eq!(todo.status, TodoStatus::Todo);

    let command2 = TodoCommand::UpdateTodoTitle {
      id: "todo_0".to_string(),
      title: "Eat pizza".to_string(),
    };
    todo_root.execute_command(command2).unwrap();

    let todo = todo_root.get_state("todo_0").unwrap();
    assert_eq!(todo.title, "Eat pizza");
    assert_eq!(todo.status, TodoStatus::Todo);

    let command3 = TodoCommand::UpdateTodoStatus {
      id: "todo_0".to_string(),
      status: TodoStatus::Done,
    };
    todo_root.execute_command(command3).unwrap();

    let todo = todo_root.get_state("todo_0").unwrap();
    assert_eq!(todo.title, "Eat pizza");
    assert_eq!(todo.status, TodoStatus::Done);
  }

  #[test]
  fn execute_command_should_increase_state_version() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::default();
    assert!(todo_root.get_version("todo_0").is_none());

    let command1 = TodoCommand::CreateTodo {
      id: "todo_0".to_string(),
      title: "Eat rice".to_string(),
      status: None,
    };
    todo_root.execute_command(command1).unwrap();
    assert_eq!(todo_root.get_version("todo_0").unwrap(), &1);

    let command2 = TodoCommand::UpdateTodoTitle {
      id: "todo_0".to_string(),
      title: "More rice".to_string(),
    };
    todo_root.execute_command(command2).unwrap();
    assert_eq!(todo_root.get_version("todo_0").unwrap(), &2);
  }

  #[test]
  fn error_when_execute_command_fail() {
    let mut todo_root: AggregateRoot<Todo> = AggregateRoot::default();

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

    assert_eq!(err, TodoError::AlreadyExists);
  }

  #[test]
  fn save_events() {
    let events = vec![
      PersistedEvent {
        aggregate_id: "todo_0".to_string(),
        version: 1,
        event: TodoEvent::TodoCreated {
          id: "todo_0".to_string(),
          title: "Drink soda".to_string(),
          status: TodoStatus::InProgress,
        },
      },
      PersistedEvent {
        aggregate_id: "todo_0".to_string(),
        version: 2,
        event: TodoEvent::TodoTitleUpdated {
          title: "Coding".to_string(),
        },
      },
    ];
    let mut root: AggregateRoot<Todo> = AggregateRoot::default();
    root.save_events(events).unwrap();

    let state = root.get_state("todo_0").unwrap();
    let version = root.get_version("todo_0").unwrap();

    assert_eq!(state.title, "Coding");
    assert_eq!(state.status, TodoStatus::InProgress);
    assert_eq!(*version, 2);
  }
}
