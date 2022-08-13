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
#[serde(tag = "name")]
pub enum CategoryEvent {
  #[serde(rename = "CategoryEvent.Created", rename_all = "camelCase")]
  Created {
    id: String,
    title: String,
    template: String,
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
        } => Ok(CategoryEvent::Created {
          id,
          title,
          template,
        }),
        _ => Err(CategoryError::AlreadyExists),
      },
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

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
