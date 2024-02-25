use std::{fs, path};

use git2::Repository;
use tauri::{App, Manager, Runtime};

use geeks_tracker_core::git;
use geeks_tracker_core::git::commit;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Workspace {
  dir: path::PathBuf,
}

impl Workspace {
  pub fn new(basedir: &path::Path) -> Result<Self, crate::error::Error> {
    let dir = basedir.join("workspace");
    if fs::read_dir(&dir).is_err() {
      fs::create_dir_all(&dir)?;
    }
    if Repository::open(&dir).is_err() {
      let repo = Repository::init(&dir).map_err(git::Error::from)?;
      commit(&repo, "initial")?;
    }
    Ok(Self { dir })
  }

  pub fn path(&self) -> &path::Path {
    &self.dir
  }

  pub fn repo(&self) -> Result<Repository, crate::error::Error> {
    let repo = Repository::open(self.path()).map_err(git::Error::from)?;
    Ok(repo)
  }
}

pub fn setup_workspace<R: Runtime>(app: &mut App<R>) -> Result<(), crate::error::Error> {
  let basedir = app
    .path_resolver()
    .app_data_dir()
    .expect("fail to get app data dir");
  let workspace = Workspace::new(&basedir)?;
  app.manage(workspace);
  Ok(())
}
