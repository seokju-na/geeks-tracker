use std::path::PathBuf;

use git2::Repository;

use crate::app_error::AppError;

pub struct Workspace {
  workspace_dir: PathBuf,
}

impl Workspace {
  pub fn initialize(app_dir: PathBuf) -> Result<Self, AppError> {
    let mut workspace_dir = PathBuf::from(app_dir);
    workspace_dir.push("workspace");

    let workspace = Self { workspace_dir };
    workspace.open_repo()?;

    Ok(workspace)
  }

  pub fn open_repo(&self) -> Result<Repository, AppError> {
    let repo = Repository::init(&self.workspace_dir)?;
    Ok(repo)
  }
}
