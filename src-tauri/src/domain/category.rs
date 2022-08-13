use std::cmp::Ordering;

use chrono::Utc;
use geeks_event_sourcing::{Aggregate, Command, Event};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
  pub id: String,
  pub title: String,
  pub template: String,
  pub order: usize,
  pub created_at: i64,
  pub updated_at: i64,
}

impl PartialEq<Self> for Category {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for Category {}

impl PartialOrd for Category {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Category {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.order != other.order {
      return self.order.cmp(&other.order);
    }
    self.created_at.cmp(&other.created_at)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum CategoryEvent {
  #[serde(rename = "CategoryEvent.Created", rename_all = "camelCase")]
  Created {
    id: String,
    title: String,
    template: String,
    order: usize,
  },
  #[serde(rename = "CategoryEvent.TitleUpdated", rename_all = "camelCase")]
  TitleUpdated { title: String },
  #[serde(rename = "CategoryEvent.TemplateUpdated", rename_all = "camelCase")]
  TemplatedUpdated { template: String },
}

impl Event for CategoryEvent {
  fn name(&self) -> &'static str {
    match self {
      CategoryEvent::Created { .. } => "CategoryEvent.Created",
      CategoryEvent::TitleUpdated { .. } => "CategoryEvent.TitleUpdated",
      CategoryEvent::TemplatedUpdated { .. } => "CategoryEvent.TemplatedUpdated",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum CategoryCommand {
  #[serde(rename = "CategoryCommand.Create", rename_all = "camelCase")]
  Create {
    id: String,
    title: String,
    template: String,
    order: usize,
  },
  #[serde(rename = "CategoryCommand.UpdateTitle", rename_all = "camelCase")]
  UpdateTitle { id: String, title: String },
  #[serde(rename = "CategoryCommand.UpdateTemplate", rename_all = "camelCase")]
  UpdateTemplate { id: String, template: String },
}

impl Command for CategoryCommand {
  fn name(&self) -> &'static str {
    match self {
      CategoryCommand::Create { .. } => "CategoryCommand.Create",
      CategoryCommand::UpdateTitle { .. } => "CategoryCommand.UpdateTitle",
      CategoryCommand::UpdateTemplate { .. } => "CategoryCommand.UpdateTemplate",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      CategoryCommand::Create { id, .. } => id,
      CategoryCommand::UpdateTitle { id, .. } => id,
      CategoryCommand::UpdateTemplate { id, .. } => id,
    }
  }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum CategoryError {
  #[error("category already exists")]
  AlreadyExists,
  #[error("category not exists")]
  NotExists,
}

impl Aggregate for Category {
  type Command = CategoryCommand;
  type Event = CategoryEvent;
  type Error = CategoryError;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match this {
      Some(_) => match command {
        CategoryCommand::UpdateTitle { title, .. } => Ok(CategoryEvent::TitleUpdated { title }),
        CategoryCommand::UpdateTemplate { template, .. } => {
          Ok(CategoryEvent::TemplatedUpdated { template })
        }
        _ => Err(CategoryError::NotExists),
      },
      None => match command {
        CategoryCommand::Create {
          id,
          title,
          template,
          order,
        } => Ok(CategoryEvent::Created {
          id,
          title,
          template,
          order,
        }),
        _ => Err(CategoryError::AlreadyExists),
      },
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp_millis();

    match this {
      Some(mut category) => match event {
        CategoryEvent::TitleUpdated { title } => {
          category.title = title;
          category.updated_at = timestamp;
          Ok(category)
        }
        CategoryEvent::TemplatedUpdated { template } => {
          category.template = template;
          category.updated_at = timestamp;
          Ok(category)
        }
        _ => Err(CategoryError::NotExists),
      },
      None => match event {
        CategoryEvent::Created {
          id,
          title,
          template,
          order,
        } => Ok(Category {
          id,
          title,
          template,
          order,
          created_at: timestamp,
          updated_at: timestamp,
        }),
        _ => Err(CategoryError::AlreadyExists),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ord() {
    let category1 = Category {
      id: "category1".to_string(),
      order: 1,
      title: "category1_title".to_string(),
      template: "category1_template".to_string(),
      created_at: 1659846351697,
      updated_at: 1659846351697,
    };
    let category2 = Category {
      id: "category2".to_string(),
      order: 2,
      title: "category2_title".to_string(),
      template: "category2_template".to_string(),
      created_at: 1659846351697,
      updated_at: 1659846351697,
    };
    let category3 = Category {
      id: "category3".to_string(),
      order: 2,
      title: "category3_title".to_string(),
      template: "category3_template".to_string(),
      created_at: 1659846351698,
      updated_at: 1659846351697,
    };
    let mut categories = vec![category3.clone(), category2.clone(), category1.clone()];
    categories.sort();

    assert_eq!(categories[0], category1);
    assert_eq!(categories[1], category2);
    assert_eq!(categories[2], category3);
  }
}
