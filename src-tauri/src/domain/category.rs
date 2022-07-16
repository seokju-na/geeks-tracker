use chrono::Utc;
use geeks_event_sourcing::{Aggregate, Command, Event};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
  pub id: String,
  pub title: String,
  pub template: String,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum CategoryEvent {
  #[serde(rename = "CategoryEvent.Created")]
  Created {
    id: String,
    title: String,
    template: String,
  },
  #[serde(rename = "CategoryEvent.Updated")]
  Updated {
    title: Option<String>,
    template: Option<String>,
  },
}

impl Event for CategoryEvent {
  fn name(&self) -> &'static str {
    match self {
      CategoryEvent::Created { .. } => "CategoryEvent.Created",
      CategoryEvent::Updated { .. } => "CategoryEvent.Updated",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum CategoryCommand {
  #[serde(rename = "CategoryCommand.Create")]
  Create {
    id: String,
    title: String,
    template: String,
  },
  #[serde(rename = "CategoryCommand.Update")]
  Update {
    id: String,
    title: Option<String>,
    template: Option<String>,
  },
}

impl Command for CategoryCommand {
  fn name(&self) -> &'static str {
    match self {
      CategoryCommand::Create { .. } => "CategoryCommand.Create",
      CategoryCommand::Update { .. } => "CategoryCommand.Update",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      CategoryCommand::Create { id, .. } => &id,
      CategoryCommand::Update { id, .. } => &id,
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
        CategoryCommand::Update {
          title, template, ..
        } => Ok(CategoryEvent::Updated { title, template }),
        _ => Err(CategoryError::AlreadyExists),
      },
      None => match command {
        CategoryCommand::Create {
          id,
          title,
          template,
        } => Ok(CategoryEvent::Created {
          id,
          title,
          template,
        }),
        _ => Err(CategoryError::NotExists),
      },
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match this {
      Some(mut category) => match event {
        CategoryEvent::Updated { title, template } => {
          if let Some(_title) = title {
            category.title = _title;
          }
          if let Some(_template) = template {
            category.template = _template;
          }
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
        } => Ok(Category {
          id,
          title,
          template,
          created_at: timestamp,
          updated_at: timestamp,
        }),
        _ => Err(CategoryError::AlreadyExists),
      },
    }
  }
}
