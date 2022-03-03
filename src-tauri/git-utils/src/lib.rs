use git2::{Commit, Error as Git2Error, Oid, Repository};

use constants::EOL;

const NO_PARENTS: [&Commit<'_>; 0] = [];

pub struct GitUtils {}

impl GitUtils {
  pub fn commit_on_head(repo: &Repository, message: &str) -> Result<Oid, Git2Error> {
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = repo.signature()?;

    match repo.refname_to_id("HEAD") {
      Ok(head_id) => {
        let parent = repo.find_commit(head_id)?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])
      }
      Err(_) => repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &NO_PARENTS),
    }
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

  pub fn parse_commit_message(message: &str) -> (String, String) {
    let lines: Vec<_> = message.split(EOL).collect();
    let subject = lines.get(0).unwrap_or(&"");
    let body = if lines.len() > 2 {
      lines[2..].join(EOL)
    } else {
      "".to_string()
    };

    (subject.to_string(), body)
  }

  pub fn format_commit_message(subject: &str, body: &str) -> String {
    format!("{}{}{}{}", subject, EOL, EOL, body)
  }
}

#[cfg(test)]
mod git_utils_tests {
  use testing::git::FixtureRepository;

  use super::*;

  #[test]
  fn should_commit_and_read_on_head() {
    let fixture = FixtureRepository::setup();
    let repo = Repository::open(&fixture.path).unwrap();
    let commit_id = GitUtils::commit_on_head(&repo, "latest").unwrap();

    let commits = GitUtils::read_commits_from_head(&repo).unwrap();
    assert_eq!(commits.len(), 1);

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
    assert_eq!(commits.len(), 5);
  }

  #[test]
  fn should_parse_commit_message() {
    let subject = "subject";
    let body = format!("line1{}line2{}line3", EOL, EOL);
    let message = format!("{}{}{}{}", subject, EOL, EOL, body);

    let result = GitUtils::parse_commit_message(&message);

    assert_eq!(result.0, subject);
    assert_eq!(result.1, body);
  }

  #[test]
  fn should_parse_subject_only_commit_message() {
    let subject = "subject";

    let result = GitUtils::parse_commit_message(&subject);

    assert_eq!(result.0, subject);
    assert_eq!(result.1, "");
  }
}
