#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
  #[error(transparent)]
  Git(#[from] geeks_tracker_core::git::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
  #[error(transparent)]
  SerdeYaml(#[from] serde_yaml::Error),
  #[error(transparent)]
  Domain(#[from] geeks_tracker_core::domain::Error),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
