use std::error::Error;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};

use crate::{AggregateState, Command, CommandData, Event, EventData};

pub const ITEM_CREATED: &'static str = "ItemCreated";
pub const ITEM_TITLE_UPDATED: &'static str = "ItemTitleUpdated";

pub const CREATE_ITEM: &'static str = "CreateItem";
pub const UPDATE_ITEM_TITLE: &'static str = "UpdateItemTitle";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemState {
  pub exists: bool,
  pub title: Option<String>,
}

impl AggregateState for ItemState {}

#[derive(thiserror::Error, Debug)]
pub enum ItemError {
  #[error("Item already exists")]
  ItemAlreadyExists,
  #[error("Item not exists")]
  ItemNotExists,
  #[error("No event matches")]
  NoEventMatches,
  #[error("No command matches")]
  NoCommandMatches,
  #[error("Parse event fail")]
  ParseEventFail,
  #[error("Parse command fail")]
  ParseCommandFail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCreatedPayload {
  pub id: String,
  pub version: i64,
  pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTitleUpdatedPayload {
  pub id: String,
  pub version: i64,
  pub title: String,
}

#[derive(Debug)]
pub enum ItemEvent {
  Created(ItemCreatedPayload),
  TitleUpdated(ItemTitleUpdatedPayload),
}

impl TryFrom<EventData> for ItemEvent {
  type Error = ItemError;

  fn try_from(data: EventData) -> Result<Self, Self::Error> {
    match data.name.as_str() {
      ITEM_CREATED => {
        let payload =
          from_value::<ItemCreatedPayload>(data.payload).map_err(|_| ItemError::ParseEventFail)?;
        Ok(ItemEvent::Created(payload))
      }
      ITEM_TITLE_UPDATED => {
        let payload = from_value::<ItemTitleUpdatedPayload>(data.payload)
          .map_err(|_| ItemError::ParseEventFail)?;
        Ok(ItemEvent::TitleUpdated(payload))
      }
      _ => Err(ItemError::NoEventMatches),
    }
  }
}

impl Into<EventData> for ItemEvent {
  fn into(self) -> EventData {
    let timestamp = Utc::now().timestamp();

    match self {
      ItemEvent::Created(payload) => EventData {
        name: ITEM_CREATED.to_string(),
        aggregate_id: payload.id.to_owned(),
        aggregate_version: payload.version,
        timestamp,
        payload: json!({
          "title": payload.title
        }),
      },
      ItemEvent::TitleUpdated(payload) => EventData {
        name: ITEM_TITLE_UPDATED.to_string(),
        aggregate_id: payload.id.to_owned(),
        aggregate_version: payload.version,
        timestamp,
        payload: json!({
          "title": payload.title
        }),
      },
    }
  }
}

impl Event<ItemState> for ItemEvent {
  fn handle(&self, state: &mut ItemState) -> () {
    match self {
      ItemEvent::Created(payload) => {
        state.exists = true;
        state.title = Some(payload.title.to_owned());
      }
      ItemEvent::TitleUpdated(payload) => {
        state.title = Some(payload.title.to_owned());
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemPayload {
  pub id: String,
  pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemTitlePayload {
  pub id: String,
  pub title: String,
}

pub enum ItemCommand {
  Create(CreateItemPayload),
  UpdateTitle(UpdateItemTitlePayload),
}

impl TryFrom<CommandData> for ItemCommand {
  type Error = ItemError;

  fn try_from(data: CommandData) -> Result<Self, Self::Error> {
    match data.name.as_str() {
      CREATE_ITEM => {
        let payload = if let Some(x) = data.payload {
          from_value::<CreateItemPayload>(x).map_err(|_| ItemError::ParseEventFail)?
        } else {
          return Err(ItemError::ParseCommandFail);
        };
        Ok(ItemCommand::Create(payload))
      }
      UPDATE_ITEM_TITLE => {
        let payload = if let Some(x) = data.payload {
          from_value::<UpdateItemTitlePayload>(x).map_err(|_| ItemError::ParseEventFail)?
        } else {
          return Err(ItemError::ParseCommandFail);
        };
        Ok(ItemCommand::UpdateTitle(payload))
      }
      _ => Err(ItemError::NoCommandMatches),
    }
  }
}

impl Command<ItemState> for ItemCommand {
  type Event = ItemEvent;

  fn aggregate_id(&self) -> &str {
    match self {
      ItemCommand::Create(payload) => &payload.id,
      ItemCommand::UpdateTitle(payload) => &payload.id,
    }
  }

  fn handle(&self, state: &ItemState, version: i64) -> Result<ItemEvent, Box<dyn Error>> {
    match self {
      ItemCommand::Create(payload) => {
        if state.exists {
          return Err(Box::new(ItemError::ItemAlreadyExists));
        }

        Ok(ItemEvent::Created(ItemCreatedPayload {
          id: payload.id.to_owned(),
          version,
          title: payload.title.to_owned(),
        }))
      }
      ItemCommand::UpdateTitle(payload) => {
        if !state.exists {
          return Err(Box::new(ItemError::ItemNotExists));
        }

        Ok(ItemEvent::TitleUpdated(ItemTitleUpdatedPayload {
          id: payload.id.to_owned(),
          version,
          title: payload.title.to_owned(),
        }))
      }
    }
  }
}
