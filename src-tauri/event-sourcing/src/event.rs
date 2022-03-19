use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  pub name: String,
  pub data: String,
}

impl Event {
  pub fn new<B: Serialize>(name: &str, data: &B) -> Self {
    Self {
      name: name.to_string(),
      data: serde_json::to_string(data).unwrap(),
    }
  }
}
