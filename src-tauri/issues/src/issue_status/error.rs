#[derive(thiserror::Error, Debug)]
pub enum IssueStatusError {
  #[error("Issue status already exists")]
  IssueStatusAlreadyExists,
}
