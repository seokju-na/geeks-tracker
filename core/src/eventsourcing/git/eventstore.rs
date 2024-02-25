use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use git2::Repository;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

use crate::eventsourcing::git::SNAPSHOT_MSG;
use crate::eventsourcing::{Event, Eventstore, PersistedEvent, VersionSelect};
use crate::git;
use crate::git::{commit, CommitInfo, CommitMessage, CommitReader};

pub const EVENT_MSG: &str = "[event]";

pub struct GitEventstore<T>
where
  T: Event,
{
  repo_path: PathBuf,
  _event: PhantomData<T>,
}

impl<T> GitEventstore<T>
where
  T: Event + Serialize + DeserializeOwned,
{
  pub fn new(repo_path: &Path) -> Self {
    Self {
      repo_path: repo_path.to_path_buf(),
      _event: PhantomData,
    }
  }

  fn event_to_commit_message(persisted: PersistedEvent<T>) -> CommitMessage {
    CommitMessage {
      subject: format!(
        "{prefix} {event_name}",
        prefix = EVENT_MSG,
        event_name = persisted.event.name()
      ),
      body: to_string(&persisted).unwrap(),
    }
  }

  fn commit_to_event(commit: CommitInfo) -> Option<PersistedEvent<T>> {
    if !commit.message.subject.contains(EVENT_MSG) {
      return None;
    }

    match from_str(commit.message.body.trim()) {
      Ok(x) => Some(x),
      Err(_) => None,
    }
  }

  pub async fn read_until_snapshot(&self) -> Result<Vec<PersistedEvent<T>>, git::Error> {
    let repo = Repository::open(&self.repo_path)?;
    let mut events: Vec<_> = CommitReader::new(&repo)?
      .start_on_head()
      .end_when(|x| x.message.subject.contains(SNAPSHOT_MSG))
      .flatten()
      .filter_map(GitEventstore::commit_to_event)
      .collect();

    events.reverse();
    Ok(events)
  }
}

#[async_trait]
impl<T> Eventstore for GitEventstore<T>
where
  T: Event + Serialize + DeserializeOwned,
{
  type Event = T;
  type Error = git::Error;

  async fn read(
    &self,
    aggregate_id: String,
    select: VersionSelect,
  ) -> Result<Vec<PersistedEvent<Self::Event>>, Self::Error> {
    let repo = Repository::open(&self.repo_path)?;
    let mut events: Vec<_> = CommitReader::new(&repo)?
      .start_on_head()
      .flatten()
      .filter_map(GitEventstore::commit_to_event)
      .filter(|event| event.aggregate_id == aggregate_id)
      .filter(|event| match select {
        VersionSelect::All => true,
        VersionSelect::From(v) => event.version >= v,
      })
      .collect();

    events.reverse();
    Ok(events)
  }

  async fn append(&self, events: Vec<PersistedEvent<Self::Event>>) -> Result<(), Self::Error> {
    let repo = Repository::open(&self.repo_path)?;
    let commit_messages = events
      .into_iter()
      .map(GitEventstore::event_to_commit_message);

    for message in commit_messages {
      commit(&repo, message)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use geeks_tracker_testing::git::FixtureRepository;

  use crate::eventsourcing::dummy::{TodoEvent, TodoStatus};
  use crate::eventsourcing::git::GitEventstore;
  use crate::eventsourcing::{Event, Eventstore, PersistedEvent, VersionSelect};

  #[tokio::test]
  async fn should_read_events() {
    let event1 = TodoEvent::TodoCreated {
      id: "todo1".to_string(),
      title: "Drink coffee".to_string(),
      status: TodoStatus::InProgress,
    };
    let event2 = TodoEvent::TodoTitleUpdated {
      title: "Eat pizza".to_string(),
    };

    let fixture = FixtureRepository::default();
    let eventstore = GitEventstore::new(fixture.path());
    eventstore
      .append(vec![
        PersistedEvent {
          aggregate_id: "todo1".to_string(),
          version: 2,
          event: event2,
        },
        PersistedEvent {
          aggregate_id: "todo1".to_string(),
          version: 1,
          event: event1,
        },
      ])
      .await
      .unwrap();

    let events = eventstore
      .read("todo1".to_string(), VersionSelect::All)
      .await
      .unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event.name(), "TodoTitleUpdated");
    assert_eq!(events[1].event.name(), "TodoCreated");
  }
}
