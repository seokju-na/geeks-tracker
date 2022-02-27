use async_trait::async_trait;

use crate::eventstore_error::EventstoreError;

#[async_trait]
pub trait Eventstore<Event, RecordedEvent> {
  async fn append(
    &mut self,
    stream_name: &str,
    events: Vec<Event>,
  ) -> Result<Vec<RecordedEvent>, EventstoreError>;
}
