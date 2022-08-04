use crate::application::Application;
use crate::domain::Category;

pub trait QueryHandler {
  fn list_categories(&self) -> Vec<Category>;
}

impl QueryHandler for Application {
  fn list_categories(&self) -> Vec<Category> {
    self.categories.states.values().cloned().collect()
  }
}
