use git2::{Error as Git2Error, Repository};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
  #[error("data store disconnected")]
  Original(#[from] Git2Error),
  #[error("is not branch")]
  IsNotBranch,
  #[error("invalid branch ref")]
  InvalidBranchRef,
}

pub struct GitUtils {}

impl GitUtils {
  pub fn get_current_branch_name(repo: &Repository) -> Result<String, GitError> {
    let reference = repo.head()?;
    if !reference.is_branch() {
      return Err(GitError::IsNotBranch);
    }

    Self::parse_branch_name(reference.name().unwrap_or(""))
  }

  pub fn branch_name_to_ref_name(branch_name: &str) -> String {
    format!("refs/heads/{}", branch_name)
  }

  pub fn parse_branch_name(ref_name: &str) -> Result<String, GitError> {
    let re = Regex::new(r"^refs/heads/(.+)$").unwrap();
    let cap = re.captures(ref_name);

    match cap {
      Some(g) => Ok(g[1].to_string()),
      None => Err(GitError::InvalidBranchRef),
    }
  }
}

#[cfg(test)]
mod git_utils_tests {
  use git2::Repository;

  use crate::testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn should_get_current_branch_name() {
    let fixture = FixtureRepository::open(
      "should_get_current_branch_name",
      r#"
        git commit --allow-empty -m "1"
        "#,
    );
    let repo = Repository::open_bare(&fixture.git_path).unwrap();

    assert_eq!(GitUtils::get_current_branch_name(&repo).unwrap(), "main");
  }

  #[test]
  fn should_convert_ref_name_from_branch_name() {
    assert_eq!(GitUtils::branch_name_to_ref_name("main"), "refs/heads/main");
    assert_eq!(GitUtils::branch_name_to_ref_name("test"), "refs/heads/test");
  }

  #[test]
  fn should_parse_branch_name() {
    assert_eq!(
      GitUtils::parse_branch_name("refs/heads/main").unwrap(),
      "main"
    );
    assert_eq!(
      GitUtils::parse_branch_name("refs/heads/master").unwrap(),
      "master"
    );
    assert_eq!(
      GitUtils::parse_branch_name("refs/heads/test").unwrap(),
      "test"
    );
  }

  #[test]
  fn should_parse_error_when_ref_name_is_incorrect() {
    assert!(matches!(
      GitUtils::parse_branch_name("not_ref_name").unwrap_err(),
      GitError::InvalidBranchRef,
    ));
    assert!(matches!(
      GitUtils::parse_branch_name("refs/is_head/hello").unwrap_err(),
      GitError::InvalidBranchRef,
    ));
  }
}
