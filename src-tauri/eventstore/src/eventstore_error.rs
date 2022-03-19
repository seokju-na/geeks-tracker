use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventstoreError {
  #[error("error while recording event")]
  RecordEventError,
  #[error("error while parsing record event")]
  ParseRecordedEventError,
  #[error(transparent)]
  Unknown(#[from] anyhow::Error),
}
