#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("task not exists")]
  TaskNotExists,
  #[error("task already exists")]
  TaskAlreadyExists,
}
