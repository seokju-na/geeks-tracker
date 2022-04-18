#![feature(assert_matches)]

extern crate core;

pub use crate::aggregate::{Aggregate, AggregateRoot};
pub use crate::command::Command;
pub use crate::event::{Event, PersistedEvent};

mod aggregate;
mod command;
mod event;
pub mod testing;

pub type Version = u64;
pub type Timestamp = i64;
