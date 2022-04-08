#[derive(thiserror::Error, Debug)]
pub enum IssueError {
  #[error("Issue already exists")]
  IssueAlreadyExists,
}
