#![feature(assert_matches)]

pub use crate::aggregate::{Aggregate, AggregateState};
pub use crate::application::Application;
pub use crate::command::Command;
pub use crate::event::{Event, EventData, EventParseError};
pub use crate::eventstore::Eventstore;
pub use crate::saga::Saga;

mod aggregate;
mod application;
mod command;
mod event;
mod eventstore;
mod saga;
pub mod testing;
