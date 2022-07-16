use std::path::{Path, PathBuf};

use geeks_event_sourcing::{AggregateRoot, Snapshot};
use geeks_event_sourcing_git::{commit_snapshot, GitEventstore};
use geeks_git::GitError;

use crate::application::{FileSnapshot, FileSnapshotError};
use crate::domain::{Category, CategoryError};

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
  #[error("category error: {0}")]
  Category(#[from] CategoryError),

  #[error("file snapshot error: {0}")]
  FileSnapshot(#[from] FileSnapshotError),

  #[error("git error: {0}")]
  Git(#[from] GitError),
}

#[derive(Debug, Clone)]
pub struct Application {
  pub workspace_dir: PathBuf,
  pub categories: AggregateRoot<Category>,
}

impl Application {
  pub async fn init(workspace_dir: &Path) -> Result<Self, ApplicationError> {
    // load aggregates
    let categories = Application::load_categories(workspace_dir).await?;

    // commit snapshot (if updated)
    commit_snapshot(workspace_dir)?;

    Ok(Self {
      workspace_dir: workspace_dir.to_path_buf(),
      categories,
    })
  }

  async fn load_categories(
    workspace_dir: &Path,
  ) -> Result<AggregateRoot<Category>, ApplicationError> {
    let eventstore = GitEventstore::new(workspace_dir);
    let snapshot = FileSnapshot::new(workspace_dir.join(".geeks/categories.json"));

    let mut root = snapshot.load().await.unwrap_or_default();
    let unsaved_events = eventstore.read_until_snapshot().await?;

    root.save_events(unsaved_events)?;
    snapshot.save(root.clone()).await?;

    Ok(root)
  }
}
