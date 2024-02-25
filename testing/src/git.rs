use std::fs::remove_dir_all;
use std::path::Path;

use git2::Repository;
use tempdir::TempDir;

pub struct FixtureRepository {
  dir: TempDir,
}

impl Default for FixtureRepository {
  fn default() -> Self {
    let prefix = rand::random::<u32>().to_string();
    let dir = TempDir::new(&prefix).expect("fail to create temp dir");
    let repo = Repository::init(dir.path()).expect("fail to initialize git");
    let mut config = repo.config().unwrap();
    config.set_str("user.email", "test@test.com").unwrap();
    config.set_str("user.name", "Test").unwrap();
    Self { dir }
  }
}

impl FixtureRepository {
  pub fn path(&self) -> &Path {
    self.dir.path()
  }

  pub fn repo(&self) -> Repository {
    Repository::open(self.path()).unwrap()
  }
}

impl Drop for FixtureRepository {
  fn drop(&mut self) {
    let _ = remove_dir_all(self.path());
  }
}
