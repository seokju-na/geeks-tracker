pub trait Command: Send + Sync + Clone {
  fn name(&self) -> &'static str;
  fn aggregate_id(&self) -> Option<String>;
}
