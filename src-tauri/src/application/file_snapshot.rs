use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use geeks_event_sourcing::{Aggregate, AggregateRoot, Snapshot, Version};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::fs::{create_dir_all, read_to_string, write};
use tokio::io;

pub struct FileSnapshot {
  path: PathBuf,
}

impl FileSnapshot {
  pub fn new(path: &Path) -> Self {
    Self {
      path: path.to_path_buf(),
    }
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

    if !self.path.as_path().exists() {
      if let Some(parent) = self.path.as_path().parent() {
        create_dir_all(parent).await?;
      }
    }

    write(&self.path, data_raw.as_bytes()).await?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use tempfile::TempDir;
  use tokio::fs::create_dir_all;

  use crate::domain::Category;

  use super::*;

  #[tokio::test]
  async fn load() {
    let data = r#"
    {
      "states": {
        "category_1": {
          "id": "category_1",
          "title": "My Category",
          "template": "...",
          "createdAt": 1658565790687,
          "updatedAt": 1658565790687
        }
      },
      "versions": {
        "category_1": 1
      }
    }
    "#;
    let tempdir = TempDir::new().unwrap();
    create_dir_all(&tempdir.path().join(".geeks"))
      .await
      .unwrap();
    write(
      &tempdir.path().join(".geeks/categories.json"),
      data.as_bytes(),
    )
    .await
    .unwrap();

    let snapshot = FileSnapshot::new(&tempdir.path().join(".geeks/categories.json"));
    let root: AggregateRoot<Category> = snapshot.load().await.unwrap();

    assert!(root.get_state("category_1").is_some());

    tempdir.close().unwrap();
  }

  #[tokio::test]
  async fn file_not_exists_err() {
    let tempdir = TempDir::new().unwrap();
    let snapshot = FileSnapshot::new(&tempdir.path().join(".geeks/categories.json"));

    let res: Result<AggregateRoot<Category>, _> = snapshot.load().await;
    let err = res.unwrap_err();

    if let FileSnapshotError::Io(e) = err {
      assert_eq!(e.to_string(), "No such file or directory (os error 2)");
    } else {
      panic!("error was not FileSnapshotError::Io");
    }

    tempdir.close().unwrap();
  }

  #[tokio::test]
  async fn unwrap_or_default() {
    let tempdir = TempDir::new().unwrap();
    let snapshot = FileSnapshot::new(&tempdir.path().join(".geeks/categories.json"));

    let res: Result<AggregateRoot<Category>, _> = snapshot.load().await;
    let root = res.unwrap_or_default();

    assert_eq!(root.states.len(), 0);

    tempdir.close().unwrap();
  }

  #[tokio::test]
  async fn save() {
    let tempdir = TempDir::new().unwrap();

    let category1 = Category {
      id: "category_1".to_string(),
      title: "My Category".to_string(),
      template: "...".to_string(),
      order: 1,
      created_at: 1658565790687,
      updated_at: 1658565790687,
    };
    let mut root = AggregateRoot::<Category>::default();
    root
      .states
      .insert("category_1".to_string(), category1.clone());

    let snapshot = FileSnapshot::new(&tempdir.path().join(".geeks/categories.json"));
    snapshot.save(root).await.unwrap();

    let loaded: AggregateRoot<Category> = snapshot.load().await.unwrap();
    assert_eq!(loaded.states.len(), 1);
    assert_eq!(loaded.get_state("category_1").cloned().unwrap(), category1);

    tempdir.close().unwrap();
  }
}
