#![allow(non_snake_case)]

use std::str::from_utf8;

use git_utils::GitUtils;

use crate::app_error::AppError;
use crate::Workspace;

#[derive(serde::Serialize)]
pub struct ReadGitIndexEntryItem {
  uid: u32,
  fileSize: u32,
  path: String,
}

#[tauri::command]
pub fn read_git_index_entries(
  workspace: tauri::State<Workspace>,
) -> Result<Vec<ReadGitIndexEntryItem>, AppError> {
  let repo = workspace.open_repo()?;
  let index_entries = GitUtils::read_index_entries(&repo)?;

  let items: Vec<_> = index_entries
    .iter()
    .map(|entry| ReadGitIndexEntryItem {
      uid: entry.uid,
      fileSize: entry.file_size,
      path: String::from(from_utf8(&entry.path).expect("cannot parse entry path.")),
    })
    .collect();

  Ok(items)
}
