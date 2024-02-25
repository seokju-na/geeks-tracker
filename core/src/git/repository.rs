use git2::{ErrorCode, Oid, Repository, Signature};

use crate::git;
use crate::git::CommitInfo;

pub fn get_head(repo: &Repository) -> Result<Oid, git::Error> {
  let head = repo.head()?.target();
  match head {
    Some(head) => Ok(head),
    None => Err(git::Error::NoHead),
  }
}

pub fn get_head_commit(repo: &Repository) -> Result<CommitInfo, git::Error> {
  let head = get_head(repo)?;
  let commit = repo.find_commit(head).map(CommitInfo::from)?;
  Ok(commit)
}

pub fn get_signature(repo: &Repository) -> Result<Signature<'_>, git::Error> {
  match repo.signature() {
    Ok(sig) => Ok(sig),
    Err(e) => {
      if e.code() == ErrorCode::NotFound {
        let config = repo.config()?;
        if let (Err(_), Ok(email_entry)) = (
          config.get_entry("user.name"),
          config.get_entry("user.email"),
        ) {
          if let Some(email) = email_entry.value() {
            let sig = Signature::now("unknown", email)?;
            return Ok(sig);
          }
        };
      }
      Err(crate::git::Error::from(e))
    }
  }
}

#[cfg(test)]
mod tests {
  use git2::Repository;

  use geeks_tracker_testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn no_head() {
    let fixture = FixtureRepository::default();
    let repo = Repository::open(fixture.path()).unwrap();
    let head_commit = get_head_commit(&repo);
    assert!(head_commit.is_err());
  }
}
