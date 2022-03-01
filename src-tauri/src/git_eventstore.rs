use git2::{Commit, Repository};
use serde::{Deserialize, Serialize};

use git_utils::GitUtils;

use crate::app_error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
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
struct RecordedEvent {
  pub id: String,
  pub name: String,
  pub data: String,
}

impl RecordedEvent {
  pub fn create(repo: &Repository, event: &Event) -> Result<Self, AppError> {
    let commit_id = GitUtils::commit_on_head(&repo, &RecordedEvent::stringify_event(event))?;

    Ok(Self {
      id: commit_id.to_string(),
      name: event.name.to_owned(),
      data: event.data.to_owned(),
    })
  }

  pub fn from_commit(commit: &Commit) -> Result<Self, AppError> {
    let (name, data) = RecordedEvent::parse_event_message(commit.message().unwrap_or(""))?;

    Ok(Self {
      id: commit.id().to_string(),
      name,
      data,
    })
  }

  fn stringify_event(event: &Event) -> String {
    GitUtils::format_commit_message(&format!("[event] {}", event.name), &event.data)
  }

  fn parse_event_message(event_message: &str) -> Result<(String, String), AppError> {
    let (subject, body) = GitUtils::parse_commit_message(event_message);

    if !subject.starts_with("[event] ") {
      return Err(AppError::ParseRecordEventError);
    }

    let subject_splits: Vec<_> = subject.split(" ").collect();
    let event_name = subject_splits[1..].join("");

    Ok((event_name, body))
  }
}

pub struct GitEventstore<'repo> {
  repo: &'repo Repository,
}

impl<'repo> GitEventstore<'repo> {
  pub fn new(repo: &'repo Repository) -> Self {
    Self { repo }
  }

  pub fn append(&self, events: Vec<Event>) -> Result<Vec<RecordedEvent>, AppError> {
    let mut recorded_events = Vec::new();

    for event in events.iter() {
      let recorded = RecordedEvent::create(&self.repo, event)?;
      recorded_events.push(recorded);
    }

    Ok(recorded_events)
  }

  pub fn read(&self) -> Result<Vec<RecordedEvent>, AppError> {
    let commits = GitUtils::read_commits_from_head(&self.repo)?;
    let mut recorded_events = Vec::new();

    for commit in commits.iter() {
      if let Ok(recorded_event) = RecordedEvent::from_commit(commit) {
        recorded_events.push(recorded_event);
      }
    }

    Ok(recorded_events)
  }
}

#[cfg(test)]
mod git_eventstore_tests {
  use nanoid::nanoid;
  use serde_json::json;

  use testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn should_append_events() {
    let fixture = FixtureRepository::setup();
    let repo = Repository::open(&fixture.path).unwrap();

    let eventstore = GitEventstore::new(&repo);
    let event1 = Event::new("a", &json!({ "text": "123" }));
    let event2 = Event::new("b", &json!({ "flag": true }));

    eventstore.append(vec![event1, event2]).unwrap();

    let commits = GitUtils::read_commits_from_head(&repo).unwrap();
    let event1_commit = commits.get(1).unwrap();
    let event2_commit = commits.get(0).unwrap();

    assert_eq!(
      event1_commit.message().unwrap(),
      GitUtils::format_commit_message("[event] a", r#"{"text":"123"}"#)
    );
    assert_eq!(
      event2_commit.message().unwrap(),
      GitUtils::format_commit_message("[event] b", r#"{"flag":true}"#)
    );
  }

  #[test]
  fn should_read_recorded_events() {
    let fixture = FixtureRepository::setup_with_script(
      r#"
    git commit --allow-empty -m '[event] a

{"text":"123"}'
    git commit --allow-empty -m '[event] b

{"flag":true}'
    "#,
    );
    let repo = Repository::open(&fixture.path).unwrap();

    let eventstore = GitEventstore::new(&repo);
    let recorded_events = eventstore.read().unwrap();
    assert_eq!(recorded_events.len(), 2);

    let event1 = recorded_events.get(0).unwrap();
    let event2 = recorded_events.get(1).unwrap();

    assert_eq!(event1.name, "b");
    assert_eq!(event2.name, "a");
  }

  #[test]
  fn should_read_recorded_event_with_long_data() {
    let fixture = FixtureRepository::setup();
    let repo = Repository::open(&fixture.path).unwrap();

    let eventstore = GitEventstore::new(&repo);

    let long_data = json!({
      "some_very_long_data": "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
    });

    eventstore
      .append(vec![Event::new("LongDataEvent", &long_data)])
      .unwrap();

    let recorded_events = eventstore.read().unwrap();
    let recorded_event = recorded_events.get(0).unwrap();

    assert_eq!(recorded_event.data, long_data.to_string());
  }
}
