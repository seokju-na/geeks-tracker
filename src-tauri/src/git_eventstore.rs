use std::marker::PhantomData;
use std::path::PathBuf;

use async_trait::async_trait;
use git2::Repository;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

use geeks_tracker_core::{Event, Eventstore, PersistedEvent, VersionSelect};
use geeks_tracker_git::{commit, CommitInfo, CommitMessage, CommitReader, GitError};

pub struct GitEventstore<T>
where
  T: Event,
{
  repo_path: PathBuf,
  _event: PhantomData<T>,
}

impl<T> GitEventstore<T>
where
  T: Event,
{
  pub fn new(repo_path: PathBuf) -> Self {
    Self {
      repo_path,
      _event: PhantomData::default(),
    }
  }
}

#[async_trait]
impl<T> Eventstore for GitEventstore<T>
where
  T: Event + Clone + Serialize + DeserializeOwned,
{
  type Event = T;
  type Error = GitError;

  fn stream(
    &self,
    id: &str,
    select: VersionSelect,
  ) -> Result<Vec<PersistedEvent<Self::Event>>, Self::Error> {
    println!("id: {}", id); // TODO: Treat git branch as stream id.

    let repo = Repository::open(&self.repo_path)?;
    let reader = CommitReader::new(&repo)?.start_on_head();
    let events: Vec<_> = reader
      .flat_map(|x| x.map(commit_message_to_event))
      .filter(|event| match select {
        VersionSelect::All => true,
        VersionSelect::From(v) => event.version >= v,
      })
      .collect();

    Ok(events)
  }

  async fn append(
    &self,
    id: String,
    events: Vec<PersistedEvent<Self::Event>>,
  ) -> Result<(), Self::Error> {
    println!("id: {}", id); // TODO: Treat git branch as stream id.

    let commit_messages = events.into_iter().map(event_to_commit_message);

    for message in commit_messages {
      commit(&self.repo_path, message)?;
    }

    Ok(())
  }
}

pub(crate) fn event_to_commit_message<T>(persisted: PersistedEvent<T>) -> CommitMessage
where
  T: Event + Clone + Serialize,
{
  CommitMessage {
    subject: format!("[event] {}", persisted.event.name()),
    body: to_string(&persisted).unwrap(),
  }
}

pub(crate) fn commit_message_to_event<T>(commit: CommitInfo) -> PersistedEvent<T>
where
  T: Event + Clone + DeserializeOwned,
{
  from_str(&commit.message.body).unwrap()
}
