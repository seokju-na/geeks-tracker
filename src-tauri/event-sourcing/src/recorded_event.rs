use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordedEvent {
  pub id: String,
  pub name: String,
  pub data: String,
}
