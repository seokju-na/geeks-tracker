use git2::{Error, ErrorCode, Oid, Repository, Signature};

use crate::{GitError, GitResult};

pub fn get_head(repo: &Repository) -> GitResult<Oid> {
  let head = repo.head()?.target();
  match head {
    Some(x) => Ok(x),
    None => Err(GitError::NoHead),
  }
}

pub(crate) fn get_signature(repo: &Repository) -> Result<Signature<'_>, Error> {
  let sig = repo.signature();

  if let Err(e) = &sig {
    if e.code() == ErrorCode::NotFound {
      let config = repo.config()?;

      if let (Err(_), Ok(email_entry)) = (
        config.get_entry("user.name"),
        config.get_entry("user.email"),
      ) {
        if let Some(email) = email_entry.value() {
          return Signature::now("unknown", email);
        }
      };
    }
  }

  sig
}
