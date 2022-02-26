#[derive(Debug)]
pub enum EventstoreError {
  StreamNotExists,
  EventsAreEmpty,
  RecordError,
}
