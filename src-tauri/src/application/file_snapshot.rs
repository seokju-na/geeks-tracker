use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;
use geeks_event_sourcing::{Aggregate, AggregateRoot, Snapshot, Version};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::fs::{read_to_string, write};
use tokio::io;

pub struct FileSnapshot {
  path: PathBuf,
}

impl FileSnapshot {
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum FileSnapshotError {
  #[error("io error: {0}")]
  Io(#[from] io::Error),

  #[error("json parse error: {0}")]
  JsonParse(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
struct FileSnapshotData<T>
where
  T: Aggregate,
{
  states: HashMap<String, T>,
  versions: HashMap<String, Version>,
}

#[async_trait]
impl<T> Snapshot<T> for FileSnapshot
where
  T: 'static + Aggregate + Serialize + DeserializeOwned,
{
  type Error = FileSnapshotError;

  async fn load(&self) -> Result<AggregateRoot<T>, Self::Error> {
    let raw = read_to_string(&self.path).await?;
    let data = from_str::<FileSnapshotData<T>>(&raw)?;
    let aggregate_root = AggregateRoot::new(data.states, data.versions);

    Ok(aggregate_root)
  }

  async fn save(&self, root: AggregateRoot<T>) -> Result<(), Self::Error> {
    let data = FileSnapshotData {
      states: root.states,
      versions: root.versions,
    };
    let data_raw = to_string(&data)?;
    write(&self.path, data_raw.as_bytes()).await?;

    Ok(())
  }
}
