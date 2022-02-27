use git2::{Commit, Error as Git2Error, Oid, Repository};

pub struct GitUtils {}

impl GitUtils {
  pub fn commit_on_head(repo: &Repository, message: &str) -> Result<Oid, Git2Error> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let head_id = repo.refname_to_id("HEAD")?;
    let parent = repo.find_commit(head_id)?;

    let sig = repo.signature()?;

    let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?;

    Ok(commit_id)
  }

  pub fn read_commits_from_head(repo: &Repository) -> Result<Vec<Commit<'_>>, Git2Error> {
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
}

#[cfg(test)]
mod git_utils_tests {
  use std::str::from_utf8;

  use testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn should_commit_and_read_on_head() {
    let fixture = FixtureRepository::setup();
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
    let fixture = FixtureRepository::setup();
    let repo = Repository::open(&fixture.path).unwrap();

    for message in vec!["1", "2", "3", "4", "5"].into_iter() {
      GitUtils::commit_on_head(&repo, message).unwrap();
    }

    let commits = GitUtils::read_commits_from_head(&repo).unwrap();
    assert_eq!(commits.len(), 6);
  }

  #[test]
  fn should_aaa() {
    let fixture = FixtureRepository::setup_with_script(
      r#"
    echo -e "a" >> a.txt
    mkdir -p A/ && echo -e "b" >> A/b.txt
    git add -A
    "#,
    );
    let repo = Repository::open(&fixture.path).unwrap();

    let index = repo.index().unwrap();

    for entry in index.iter() {
      println!("{} {}", from_utf8(&entry.path).unwrap(), entry.id);
    }
  }
}
