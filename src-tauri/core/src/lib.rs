#![feature(assert_matches)]

pub use crate::aggregate::{Aggregate, AggregateState};
pub use crate::command::Command;
pub use crate::event::{Event, EventData, EventParseError};
pub use crate::eventstore::Eventstore;

mod aggregate;
mod command;
mod event;
mod eventstore;
pub mod testing;
