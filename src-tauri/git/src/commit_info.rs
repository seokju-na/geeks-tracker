use git2::{Commit, Oid};

use crate::CommitMessage;

#[derive(Debug, PartialEq, Eq)]
pub struct CommitInfo {
  pub message: CommitMessage,
  pub time: i64,
  pub author_name: String,
  pub author_email: String,
  pub id: Oid,
}

impl<'a> From<Commit<'a>> for CommitInfo {
  fn from(commit: Commit<'a>) -> Self {
    let message = CommitMessage::from(commit.message().unwrap_or(""));
    let author = commit.author();

    Self {
      message,
      time: commit.time().seconds(),
      author_name: author.name().unwrap_or("").to_string(),
      author_email: author.email().unwrap_or("").to_string(),
      id: commit.id(),
    }
  }
}
