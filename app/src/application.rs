use std::ops::Deref;
use std::path::PathBuf;

use async_trait::async_trait;
use tauri::async_runtime::{block_on, Mutex};
use tauri::{App, Manager, Runtime, State};

use geeks_tracker_core::domain::task::{Task, TaskCommand};
use geeks_tracker_core::eventsourcing::git::{commit_snapshot, GitEventstore};
use geeks_tracker_core::eventsourcing::{AggregateRoot, Command, Eventstore, Snapshot};

use crate::snapshots::TaskSnapshot;
use crate::workspace::Workspace;

pub struct ApplicationState(Mutex<Application>);

impl Deref for ApplicationState {
  type Target = Mutex<Application>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Application {
  pub workspace_dir: PathBuf,
  pub tasks: AggregateRoot<Task>,
}

impl Application {
  pub async fn new(workspace: &Workspace) -> Result<Self, crate::error::Error> {
    let tasks = Application::init_tasks(workspace).await?;
    commit_snapshot(&workspace.repo()?)?;

    Ok(Self {
      workspace_dir: workspace.path().to_path_buf(),
      tasks,
    })
  }

  async fn init_tasks(workspace: &Workspace) -> Result<AggregateRoot<Task>, crate::error::Error> {
    let eventstore = GitEventstore::new(workspace.path());
    let snapshot = TaskSnapshot::new(workspace.path()).await?;

    let mut root = snapshot.load().await?;
    let unsaved_events = eventstore.read_until_snapshot().await?;

    root.save_events(unsaved_events)?;
    snapshot.save(&root).await?;

    Ok(root)
  }
}

pub fn setup_application<R: Runtime>(app: &mut App<R>) -> Result<(), crate::error::Error> {
  let workspace: State<Workspace> = app.state();
  let application = block_on(Application::new(&workspace))?;
  app.manage(ApplicationState(Mutex::new(application)));
  Ok(())
}

#[async_trait]
pub trait CommandHandler<T>
where
  T: Command,
{
  async fn handle_command(&mut self, command: T) -> Result<(), crate::error::Error>;
}

#[async_trait]
impl CommandHandler<TaskCommand> for Application {
  async fn handle_command(&mut self, command: TaskCommand) -> Result<(), crate::error::Error> {
    let event = self.tasks.execute_command(command)?;
    let eventstore = GitEventstore::new(&self.workspace_dir);
    eventstore.append(vec![event]).await?;
    Ok(())
  }
}
