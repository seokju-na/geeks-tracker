use async_trait::async_trait;
use geeks_event_sourcing::{Command, Eventstore};
use geeks_event_sourcing_git::GitEventstore;

use crate::application::{Application, ApplicationError};
use crate::domain::CategoryCommand;

#[async_trait]
pub trait CommandHandler<T>
where
  T: Command,
{
  async fn handle_command(&mut self, command: T) -> Result<(), ApplicationError>;
}

#[async_trait]
impl CommandHandler<CategoryCommand> for Application {
  async fn handle_command(&mut self, command: CategoryCommand) -> Result<(), ApplicationError> {
    let event = self.categories.execute_command(command)?;
    let eventstore = GitEventstore::new(&self.workspace_dir);
    eventstore.append(vec![event]).await?;

    Ok(())
  }
}
