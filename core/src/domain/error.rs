#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("invalid task id")]
  InvalidTaskId,
  #[error("task not exists")]
  TaskNotExists,
  #[error("task already exists")]
  TaskAlreadyExists,
}
