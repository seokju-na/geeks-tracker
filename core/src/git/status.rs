/// codes from https://github.com/extrawurst/gitui/blob/master/asyncgit/src/sync/status.rs
use std::path::Path;

use git2::{Delta, Repository, Status, StatusOptions, StatusShow};

use crate::git;

#[derive(Copy, Clone, Hash, PartialEq, Debug)]
pub enum StatusItemType {
  New,
  Modified,
  Deleted,
  Renamed,
  Typechange,
  Conflicted,
}

impl From<Status> for StatusItemType {
  fn from(s: Status) -> Self {
    if s.is_index_new() || s.is_wt_new() {
      Self::New
    } else if s.is_index_deleted() || s.is_wt_deleted() {
      Self::Deleted
    } else if s.is_index_renamed() || s.is_wt_renamed() {
      Self::Renamed
    } else if s.is_index_typechange() || s.is_wt_typechange() {
      Self::Typechange
    } else if s.is_conflicted() {
      Self::Conflicted
    } else {
      Self::Modified
    }
  }
}

impl From<Delta> for StatusItemType {
  fn from(d: Delta) -> Self {
    match d {
      Delta::Added => Self::New,
      Delta::Deleted => Self::Deleted,
      Delta::Renamed => Self::Renamed,
      Delta::Typechange => Self::Typechange,
      _ => Self::Modified,
    }
  }
}

#[derive(Clone, Hash, PartialEq, Debug)]
pub struct StatusItem {
  pub path: String,
  pub status: StatusItemType,
}

#[derive(Copy, Clone, Hash, PartialEq, Debug)]
pub enum StatusType {
  WorkingDir,
  Stage,
  Both,
}

impl Default for StatusType {
  fn default() -> Self {
    Self::WorkingDir
  }
}

impl From<StatusType> for StatusShow {
  fn from(s: StatusType) -> Self {
    match s {
      StatusType::WorkingDir => Self::Workdir,
      StatusType::Stage => Self::Index,
      StatusType::Both => Self::IndexAndWorkdir,
    }
  }
}

pub fn get_status(
  repo: &Repository,
  status_type: StatusType,
) -> Result<Vec<StatusItem>, git::Error> {
  if repo.is_bare() && !repo.is_worktree() {
    return Ok(Vec::new());
  }

  let mut options = StatusOptions::default();
  options
    .show(status_type.into())
    .update_index(true)
    .include_untracked(true)
    .renames_head_to_index(true)
    .recurse_untracked_dirs(true);

  let statuses = repo.statuses(Some(&mut options))?;
  let mut res = Vec::with_capacity(statuses.len());

  for entry in statuses.iter() {
    let status: Status = entry.status();
    let path = match entry.head_to_index() {
      Some(diff) => diff
        .new_file()
        .path()
        .and_then(Path::to_str)
        .map(String::from)
        .ok_or_else(|| git::Error::Generic("failed to get path to diff's new file.".to_string()))?,
      None => entry.path().map(String::from).ok_or_else(|| {
        git::Error::Generic("failed to get the path to indexed file.".to_string())
      })?,
    };

    res.push(StatusItem {
      path,
      status: StatusItemType::from(status),
    });
  }

  res.sort_by(|a, b| {
    let a_path = a.path.as_str();
    let b_path = b.path.as_str();

    Path::new(a_path).cmp(Path::new(b_path))
  });

  Ok(res)
}
