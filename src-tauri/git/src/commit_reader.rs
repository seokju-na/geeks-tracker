use git2::{Error, Oid, Repository, Revwalk};

use crate::commit_info::CommitInfo;
use crate::GitResult;

pub enum CommitReadStartOn {
  Head,
  Oid(Oid),
}

pub struct CommitReader<'a> {
  repo: &'a Repository,
  revwalk: Revwalk<'a>,
  start_on: CommitReadStartOn,
  started: bool,
}

impl<'a> CommitReader<'a> {
  pub fn new(repo: &'a Repository) -> GitResult<Self> {
    let revwalk = repo.revwalk()?;

    Ok(Self {
      repo,
      revwalk,
      start_on: CommitReadStartOn::Head,
      started: false,
    })
  }

  #[must_use]
  pub fn start_on_head(self) -> Self {
    self.start_on(CommitReadStartOn::Head)
  }

  #[must_use]
  pub fn start_on_oid(self, oid: Oid) -> Self {
    self.start_on(CommitReadStartOn::Oid(oid))
  }

  #[must_use]
  pub fn start_on(self, start: CommitReadStartOn) -> Self {
    Self { start_on: start, ..self }
  }

  fn push_start(&mut self) -> Result<(), Error> {
    if self.started {
      return Ok(());
    }

    match self.start_on {
      CommitReadStartOn::Head => {
        self.revwalk.push_head()?;
      }
      CommitReadStartOn::Oid(oid) => {
        self.revwalk.push(oid)?;
      }
    }
    self.started = true;
    Ok(())
  }
}

impl<'a> Iterator for CommitReader<'a> {
  type Item = Result<CommitInfo, Error>;

  fn next(&mut self) -> Option<Self::Item> {
    // returns err when push start fails.
    if let Err(e) = self.push_start() {
      return Some(Err(e));
    }

    self.revwalk.next().map(|x| match x {
      Ok(oid) => self.repo.find_commit(oid).map(CommitInfo::from),
      Err(e) => Err(e),
    })
  }
}

#[cfg(test)]
mod tests {
  use git2::Repository;

  use geeks_tracker_testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn should_read_commits_from_head() {
    let fixture = FixtureRepository::setup_with_script(
      r#"
      git commit --allow-empty -m "1"
      git commit --allow-empty -m "2"
      git commit --allow-empty -m "3"
      "#,
    );
    let repo = Repository::open(&fixture.path).unwrap();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 3);
    assert_eq!(commits[0].message, "3".into());
    assert_eq!(commits[1].message, "2".into());
    assert_eq!(commits[2].message, "1".into());
  }

  #[test]
  fn should_read_commits_from_oid() {
    let fixture = FixtureRepository::setup_with_script(
      r#"
      git commit --allow-empty -m "1"
      git commit --allow-empty -m "2"
      git commit --allow-empty -m "3"
      "#,
    );
    let repo = Repository::open(&fixture.path).unwrap();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    let reader = CommitReader::new(&repo).unwrap().start_on_oid(commits[1].id);
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].message, "2".into());
    assert_eq!(commits[1].message, "1".into());
  }

  #[test]
  fn should_read_commits_by_limited() {
    let fixture = FixtureRepository::setup_with_script(
      r#"
      git commit --allow-empty -m "1"
      git commit --allow-empty -m "2"
      git commit --allow-empty -m "3"
      "#,
    );
    let repo = Repository::open(&fixture.path).unwrap();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.take(2).map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].message, "3".into());
    assert_eq!(commits[1].message, "2".into());
  }
}
