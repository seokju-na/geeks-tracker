use chrono::Utc;
use geeks_event_sourcing::{Aggregate, Command, Event};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
  pub id: String,
  pub category_id: String,
  pub body: String,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name")]
pub enum NoteEvent {
  #[serde(rename = "NoteEvent.Created", rename_all = "camelCase")]
  Created {
    id: String,
    category_id: String,
    body: String,
  },
  #[serde(rename = "NoteEvent.BodyUpdated", rename_all = "camelCase")]
  BodyUpdated { body: String },
}

impl Event for NoteEvent {
  fn name(&self) -> &'static str {
    match self {
      NoteEvent::Created { .. } => "NoteEvent.Created",
      NoteEvent::BodyUpdated { .. } => "NoteEvent.BodyUpdated",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum NoteCommand {
  #[serde(rename = "NoteEvent.CreateOrUpdate", rename_all = "camelCase")]
  CreateOrUpdate {
    id: String,
    category_id: String,
    body: String,
  },
}

impl Command for NoteCommand {
  fn name(&self) -> &'static str {
    match self {
      NoteCommand::CreateOrUpdate { .. } => "NoteEvent.CreateOrUpdate",
    }
  }

  fn aggregate_id(&self) -> &str {
    match self {
      NoteCommand::CreateOrUpdate { id, .. } => &id,
    }
  }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum NoteError {
  #[error("note already exists")]
  AlreadyExists,
  #[error("note not exists")]
  NotExists,
}

impl Aggregate for Note {
  type Command = NoteCommand;
  type Event = NoteEvent;
  type Error = NoteError;

  fn id(&self) -> &str {
    &self.id
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
  ) -> Result<Self::Event, Self::Error> {
    match this {
      Some(_) => match command {
        NoteCommand::CreateOrUpdate { body, .. } => Ok(NoteEvent::BodyUpdated { body }),
      },
      None => match command {
        NoteCommand::CreateOrUpdate {
          id,
          category_id,
          body,
        } => Ok(NoteEvent::Created {
          id,
          category_id,
          body,
        }),
      },
    }
  }

  fn apply_event(this: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
    let timestamp = Utc::now().timestamp();

    match this {
      Some(mut note) => match event {
        NoteEvent::BodyUpdated { body } => {
          note.body = body;
          note.updated_at = timestamp;
          Ok(note)
        }
        _ => Err(NoteError::NotExists),
      },
      None => match event {
        NoteEvent::Created {
          id,
          category_id,
          body,
        } => Ok(Note {
          id,
          category_id,
          body,
          created_at: timestamp,
          updated_at: timestamp,
        }),
        _ => Err(NoteError::AlreadyExists),
      },
    }
  }
}
