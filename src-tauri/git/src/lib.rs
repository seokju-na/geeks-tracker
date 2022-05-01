pub use crate::commit::*;
pub use crate::commit_info::*;
pub use crate::commit_message::*;
pub use crate::commit_reader::*;
pub use crate::error::*;
pub use crate::repository::*;

pub type GitResult<T> = Result<T, GitError>;

mod commit;
mod commit_info;
mod commit_message;
mod commit_reader;
mod error;
mod repository;
