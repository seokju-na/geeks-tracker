use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
  #[error("git: no head found")]
  NoHead,

  #[error("git2 error:{0}")]
  Git2(#[from] git2::Error),

  #[error("io error:{0}")]
  Io(#[from] std::io::Error),
}
