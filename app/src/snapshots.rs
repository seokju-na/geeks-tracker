use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs;

use geeks_tracker_core::domain::task::Task;
use geeks_tracker_core::eventsourcing::{AggregateRoot, Snapshot, Version};

use crate::utils::parse_frontmatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData<T> {
  pub id: String,
  pub version: Version,
  pub state: T,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct TaskSnapshot {
  dir: PathBuf,
}

impl TaskSnapshot {
  pub async fn new(basedir: &Path) -> Result<Self, crate::error::Error> {
    let dir = basedir.join("tasks");
    fs::create_dir_all(&dir).await?;
    Ok(Self { dir })
  }
}

#[async_trait]
impl Snapshot<Task> for TaskSnapshot {
  type Error = crate::error::Error;

  async fn load(&self) -> Result<AggregateRoot<Task>, Self::Error> {
    let files = TaskFile::find_all(&self.dir).await?;
    let mut states: HashMap<String, Task> = HashMap::with_capacity(100);
    let mut versions: HashMap<String, Version> = HashMap::with_capacity(100);

    for file in files {
      let data = file.load().await?;
      states.insert(data.id.to_owned(), data.state);
      versions.insert(data.id.to_owned(), data.version);
    }

    Ok(AggregateRoot::new(states, versions))
  }

  async fn save(&self, root: &AggregateRoot<Task>) -> Result<(), Self::Error> {
    for (id, state) in root.states.iter() {
      let file = TaskFile::new(state, &self.dir);
      let version = root.versions.get(id).cloned().unwrap();
      file.save(state, version).await?;
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq)]
struct TaskFile {
  filepath: PathBuf,
}

impl TaskFile {
  pub fn new(state: &Task, dir: &Path) -> Self {
    let filepath = dir.join(format!("{}.md", state.id));
    Self { filepath }
  }

  pub async fn find_all(dir: &Path) -> Result<Vec<Self>, crate::error::Error> {
    let mut files = vec![];
    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
      let metadata = entry.metadata().await?;
      if metadata.is_file() {
        files.push(Self {
          filepath: entry.path(),
        })
      }
    }
    Ok(files)
  }

  pub async fn load(&self) -> Result<SnapshotData<Task>, crate::error::Error> {
    let raw = fs::read_to_string(&self.filepath).await?;
    let data = parse_frontmatter::<SnapshotData<Task>>(&raw)?;
    Ok(data)
  }

  pub async fn save(&self, state: &Task, version: Version) -> Result<(), crate::error::Error> {
    let data = SnapshotData {
      id: state.id.to_owned(),
      version,
      state,
    };
    let contents = format!(
      r#"---
{frontmatter}---

## Title
{title}

## Status
{status}
"#,
      frontmatter = serde_yaml::to_string(&data)?,
      title = state.title,
      status = serde_json::to_string(&state.status)?,
    );
    if let Some(dirname) = self.filepath.parent() {
      fs::create_dir_all(dirname).await?;
    }
    fs::write(&self.filepath, contents).await?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use tokio::fs;

  use geeks_tracker_testing::tempdir;

  use super::*;

  #[tokio::test]
  async fn read_files_from_dir() {
    let dir = tempdir::TempDir::new("workspace").unwrap();
    fs::create_dir_all(dir.path().join("tasks")).await.unwrap();
    fs::write(dir.path().join("tasks/#1.md"), "content")
      .await
      .unwrap();
    fs::write(dir.path().join("tasks/#2.md"), "content")
      .await
      .unwrap();
    let files = TaskFile::find_all(&dir.path().join("tasks")).await.unwrap();
    assert_eq!(files.len(), 2);
  }

  #[tokio::test]
  async fn save_and_parse_file() {
    let dir = tempdir::TempDir::new("workspace").unwrap();
    let task = Task::builder()
      .id("#1".to_string())
      .title("Hello".to_string())
      .build();
    let file = TaskFile::new(&task, dir.path());
    file.save(&task, 1).await.unwrap();
    let parsed = file.load().await.unwrap();
    assert_eq!(parsed.id, "#1");
    assert_eq!(parsed.version, 1);
    assert_eq!(parsed.state, task);
  }
}
