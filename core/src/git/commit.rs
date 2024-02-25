use git2::{Oid, Repository};

use crate::git;
use crate::git::{get_head, get_signature};

pub fn commit<Message>(repo: &Repository, message: Message) -> Result<Oid, git::Error>
where
  Message: ToString,
{
  let sig = get_signature(repo)?;
  let mut index = repo.index()?;
  let tree_id = index.write_tree()?;
  let tree = repo.find_tree(tree_id)?;

  let parents = if let Ok(id) = get_head(repo) {
    vec![repo.find_commit(id)?]
  } else {
    Vec::new()
  };
  let parents = parents.iter().collect::<Vec<_>>();
  let oid = repo.commit(
    Some("HEAD"),
    &sig,
    &sig,
    &message.to_string(),
    &tree,
    &parents,
  )?;

  Ok(oid)
}

#[cfg(test)]
mod tests {
  use geeks_tracker_testing::git::FixtureRepository;

  use crate::git::CommitReader;

  use super::*;

  #[test]
  fn should_able_to_commit_on_head() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "hello").unwrap();
  }

  #[test]
  fn should_commit() {
    let fixture = FixtureRepository::default();
    commit(&fixture.repo(), "1").unwrap();
    commit(&fixture.repo(), "2").unwrap();
    commit(&fixture.repo(), "3").unwrap();

    let repo = fixture.repo();
    let reader = CommitReader::new(&repo).unwrap();
    let commits: Vec<_> = reader.map(|x| x.unwrap()).collect();

    assert_eq!(commits.len(), 3);
    assert_eq!(commits[0].message, "3".into());
    assert_eq!(commits[1].message, "2".into());
    assert_eq!(commits[2].message, "1".into());
  }
}
