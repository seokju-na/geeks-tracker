use std::path::{Path, PathBuf};

use geeks_git::commit;
use git2::Repository;

pub fn init_workspace(app_dir: &Path) -> PathBuf {
  let workspace_dir = app_dir.join("workspace");

  if Repository::open(&workspace_dir).is_err() {
    Repository::init(&workspace_dir).expect("fail to initialize git");
    commit(&workspace_dir, "initial").expect("fail to initial commit");
  }

  workspace_dir
}
