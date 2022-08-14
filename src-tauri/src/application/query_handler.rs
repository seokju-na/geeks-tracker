use crate::application::Application;
use crate::domain::{Category, Note};

pub trait QueryHandler {
  fn list_categories(&self) -> Vec<Category>;
  fn get_note(&self, id: String) -> Option<Note>;
}

impl QueryHandler for Application {
  fn list_categories(&self) -> Vec<Category> {
    let mut categories: Vec<_> = self.categories.states.values().cloned().collect();
    categories.sort();

    categories
  }

  fn get_note(&self, id: String) -> Option<Note> {
    self.notes.get_state(id).cloned()
  }
}
