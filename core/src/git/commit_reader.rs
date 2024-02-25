use git2::{Error, Oid, Repository, Revwalk};

use crate::git;
use crate::git::CommitInfo;

pub enum CommitReadStartOn {
  Head,
  Oid(Oid),
}

pub type CommitReaderEndWhen = fn(&CommitInfo) -> bool;

fn keep_reading(_: &CommitInfo) -> bool {
  false
}

pub struct CommitReader<'a> {
  repo: &'a Repository,
  revwalk: Revwalk<'a>,
  start_on: CommitReadStartOn,
  end_when: Option<CommitReaderEndWhen>,
  started: bool,
}

impl<'a> CommitReader<'a> {
  pub fn new(repo: &'a Repository) -> Result<Self, git::Error> {
    let revwalk = repo.revwalk()?;

    Ok(Self {
      repo,
      revwalk,
      start_on: CommitReadStartOn::Head,
      end_when: None,
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
    Self {
      start_on: start,
      ..self
    }
  }

  #[must_use]
  pub fn end_when(self, end_when: CommitReaderEndWhen) -> Self {
    Self {
      end_when: Some(end_when),
      ..self
    }
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
  type Item = Result<CommitInfo, git::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    // returns err when push start fails.
    if let Err(e) = self.push_start() {
      return Some(Err(git::Error::from(e)));
    }

    let item = self.revwalk.next().map(|x| match x {
      Ok(oid) => self.repo.find_commit(oid).map(CommitInfo::from),
      Err(e) => Err(e),
    });

    if let Some(Ok(commit)) = &item {
      let end_when = self.end_when.unwrap_or(keep_reading);
      if end_when(commit) {
        return None;
      }
    }

    item.map(|x| x.map_err(git::Error::from))
  }
}

#[cfg(test)]
mod tests {
  use geeks_tracker_testing::git::FixtureRepository;

  use crate::git::commit;

  use super::*;

  #[test]
  fn should_read_commits_from_head() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "1").unwrap();
    commit(&fixture.repo(), "2").unwrap();
    commit(&fixture.repo(), "3").unwrap();
    let repo = fixture.repo();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 3);
    assert_eq!(commits[0].message, "3".into());
    assert_eq!(commits[1].message, "2".into());
    assert_eq!(commits[2].message, "1".into());
  }

  #[test]
  fn should_read_commits_from_oid() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "1").unwrap();
    commit(&fixture.repo(), "2").unwrap();
    commit(&fixture.repo(), "3").unwrap();
    let repo = fixture.repo();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    let reader = CommitReader::new(&repo)
      .unwrap()
      .start_on_oid(commits[1].id);
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].message, "2".into());
    assert_eq!(commits[1].message, "1".into());
  }

  #[test]
  fn should_read_commits_by_limited() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "1").unwrap();
    commit(&fixture.repo(), "2").unwrap();
    commit(&fixture.repo(), "3").unwrap();
    let repo = fixture.repo();
    let reader = CommitReader::new(&repo).unwrap().start_on_head();
    let commits: Vec<_> = reader.take(2).map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].message, "3".into());
    assert_eq!(commits[1].message, "2".into());
  }

  #[test]
  fn should_read_commits_until_end() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "1").unwrap();
    commit(&fixture.repo(), "2").unwrap();
    commit(&fixture.repo(), "3").unwrap();
    commit(&fixture.repo(), "4").unwrap();
    commit(&fixture.repo(), "5").unwrap();
    let repo = fixture.repo();
    let reader = CommitReader::new(&repo)
      .unwrap()
      .start_on_head()
      .end_when(|commit: &CommitInfo| commit.message.subject == "3");
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 2);
    assert_eq!(commits[0].message, "5".into());
    assert_eq!(commits[1].message, "4".into());

    let reader = CommitReader::new(&repo)
      .unwrap()
      .start_on_head()
      .end_when(|commit: &CommitInfo| commit.message.subject == "5");
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 0);
  }
}
