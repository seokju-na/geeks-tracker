use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::domain::task::TaskEvent;
use crate::eventsourcing::Persisted;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", content = "data")]
#[typeshare]
pub enum DispatchMessage {
  #[serde(rename = "task.persisted", rename_all = "camelCase")]
  TaskPersisted { events: Vec<Persisted<TaskEvent>> },
}
