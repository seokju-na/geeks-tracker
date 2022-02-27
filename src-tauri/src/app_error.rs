use git2::Error as Git2Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("parse record event error")]
  ParseRecordEventError,
  #[error("git error")]
  GitError(#[from] Git2Error),
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}
