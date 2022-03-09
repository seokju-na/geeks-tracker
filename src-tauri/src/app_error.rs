use std::io;

use git2::Error as Git2Error;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use thiserror::Error;

struct AppErrorData {
  pub name: String,
  pub message: String,
}

#[derive(Error, Debug)]
pub enum AppError {
  #[error("error while parsing record event")]
  ParseRecordEventError,
  #[error("error when initializing workspace")]
  WorkspaceInitializeError,
  #[error("git error")]
  GitError(#[from] Git2Error),
  #[error("io error")]
  IoError(#[from] io::Error),
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}

impl AppError {
  pub fn name(&self) -> String {
    let name = match self {
      Self::ParseRecordEventError => "ParseRecordEventError",
      Self::WorkspaceInitializeError => "WorkspaceInitializeError",
      Self::GitError(_) => "GitError",
      Self::IoError(_) => "IoError",
      Self::Other(_) => "Other",
    };

    String::from(name)
  }

  pub fn message(&self) -> String {
    let message = match self {
      Self::ParseRecordEventError => "error while parsing record event".to_string(),
      Self::WorkspaceInitializeError => "error when initializing workspace".to_string(),
      Self::GitError(x) => x.to_string(),
      Self::IoError(x) => x.to_string(),
      Self::Other(x) => x.to_string(),
    };

    message
  }
}

impl Serialize for AppError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut data = serializer.serialize_struct("AppErrorData", 2)?;
    data.serialize_field::<String>("name", &self.name())?;
    data.serialize_field::<String>("message", &self.message())?;

    data.end()
  }
}
