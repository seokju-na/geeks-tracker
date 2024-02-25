#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("git no head")]
  NoHead,
  #[error("generic: {0}")]
  Generic(String),
  #[error(transparent)]
  Git(#[from] git2::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}
