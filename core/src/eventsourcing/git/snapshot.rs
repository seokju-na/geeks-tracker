use git2::{IndexAddOption, Oid, Repository};

use crate::git::{commit, get_head_commit, get_status, StatusType};

pub const SNAPSHOT_MSG: &str = "[snapshot]";

pub fn commit_snapshot(repo: &Repository) -> Result<Option<Oid>, crate::git::Error> {
  let is_head_snapshot = get_head_commit(repo)?
    .message
    .subject
    .contains(SNAPSHOT_MSG);
  let is_working_dir_clean = get_status(repo, StatusType::WorkingDir)?.is_empty();

  if is_head_snapshot || is_working_dir_clean {
    return Ok(None);
  }

  let mut index = repo.index()?;
  index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
  index.write()?;
  let oid = commit(repo, SNAPSHOT_MSG)?;

  Ok(Some(oid))
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::Path;

  use git2::Repository;

  use geeks_tracker_testing::git::FixtureRepository;

  use crate::git::CommitInfo;

  use super::*;

  #[test]
  fn should_write_all_files_with_snapshot_commit() {
    let fixture = FixtureRepository::default();
    let repo = Repository::open(fixture.path()).unwrap();
    commit(&repo, "initial").unwrap();
    fs::write(fixture.path().join("a.txt"), "A").unwrap();
    fs::write(fixture.path().join("b.txt"), "A").unwrap();
    fs::create_dir_all(fixture.path().join("foo")).unwrap();
    fs::write(fixture.path().join("foo/bar.txt"), "foo/bar").unwrap();
    let oid = commit_snapshot(&repo).unwrap().unwrap();
    let commit = repo.find_commit(oid).unwrap();
    assert_eq!(CommitInfo::from(commit).message, SNAPSHOT_MSG.into());

    let status = get_status(&repo, StatusType::Both).unwrap();
    assert!(status.is_empty());
  }

  #[test]
  fn should_not_create_snapshot_commit_when_workdir_clean() {
    let fixture = FixtureRepository::default();
    let repo = Repository::open(fixture.path()).unwrap();
    commit(&repo, "initial").unwrap();
    fs::write(fixture.path().join("a.txt"), "A").unwrap();
    let mut index = repo.index().unwrap();
    index.add_path(Path::new("a.txt")).unwrap();
    index.write().unwrap();
    commit(&repo, "secondary").unwrap();
    let result = commit_snapshot(&repo).unwrap();
    assert!(result.is_none());
  }

  #[test]
  fn should_not_create_snapshot_commit_when_head_is_snapshot_commit() {
    let fixture = FixtureRepository::default();
    let repo = Repository::open(fixture.path()).unwrap();
    commit(&repo, "initial").unwrap();
    fs::write(fixture.path().join("a.txt"), "A").unwrap();
    let result = commit_snapshot(&repo).unwrap();
    assert!(result.is_some());
    let result = commit_snapshot(&repo).unwrap();
    assert!(result.is_none());
  }
}
