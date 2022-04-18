#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum IssueError {
  #[error("Issue already exists")]
  AlreadyExists,
  #[error("Issue not exists")]
  NotExists,
}
