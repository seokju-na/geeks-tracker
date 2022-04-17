#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum IssueStatusError {
  #[error("Issue status already exists")]
  AlreadyExists,
  #[error("Issue not exists")]
  NotExists,
}
