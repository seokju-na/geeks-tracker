use git2::{Commit, Error as Git2Error, Oid, Repository};
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

  pub fn commit_on_head(repo: &Repository, message: &str) -> Result<Oid, GitError> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let head_id = repo.refname_to_id("HEAD")?;
    let parent = repo.find_commit(head_id)?;

    let sig = repo.signature()?;

    let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?;

    Ok(commit_id)
  }

  pub fn read_commits_from_head(repo: &Repository) -> Result<Vec<Commit<'_>>, GitError> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    macro_rules! filter_try {
      ($e:expr) => {
        match $e {
          Ok(t) => t,
          Err(e) => return Some(Err(e)),
        }
      };
    }

    let revwalk = revwalk.filter_map(|id| {
      let id = filter_try!(id);
      let commit = filter_try!(repo.find_commit(id));

      Some(Ok(commit))
    });
    let mut commits = Vec::<Commit<'_>>::new();

    for commit in revwalk {
      commits.push(commit?);
    }

    Ok(commits)
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
  use testing::git::FixtureRepository;

  use super::*;

  const FIXTURE_REPO_BASE_PATH: &str = "test-fixtures";

  fn get_fixture_repo_path(name: &str) -> String {
    format!("{}/{}", FIXTURE_REPO_BASE_PATH, name)
  }

  #[test]
  fn should_get_current_branch_name() {
    let path = get_fixture_repo_path("should_get_current_branch_name");
    let fixture = FixtureRepository::open(&path, "");
    let repo = Repository::open(&fixture.path).unwrap();

    assert_eq!(GitUtils::get_current_branch_name(&repo).unwrap(), "main");
  }

  #[test]
  fn should_commit_and_read_on_head() {
    let path = get_fixture_repo_path("should_commit_and_read_on_head");
    let fixture = FixtureRepository::open(&path, "");
    let repo = Repository::open(&fixture.path).unwrap();
    let commit_id = GitUtils::commit_on_head(&repo, "latest").unwrap();

    let commits = GitUtils::read_commits_from_head(&repo).unwrap();
    assert_eq!(commits.len(), 2);

    let latest_commit = commits.get(0).unwrap();
    assert_eq!(latest_commit.id(), commit_id);
    assert_eq!(latest_commit.message().unwrap(), "latest");
  }

  #[test]
  fn should_commit_many() {
    let path = get_fixture_repo_path("should_commit_many");
    let fixture = FixtureRepository::open(&path, "");
    let repo = Repository::open(&fixture.path).unwrap();
    let messages = vec!["1", "2", "3", "4", "5"];

    for message in messages.into_iter() {
      GitUtils::commit_on_head(&repo, message).unwrap();
    }

    let commits = GitUtils::read_commits_from_head(&repo).unwrap();
    assert_eq!(commits.len(), 6);
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
