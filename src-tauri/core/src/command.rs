pub trait Command: Send + Sync {
  fn name(&self) -> &'static str;
  fn aggregate_id(&self) -> &str;
}
