use geeks_tracker_core::dispatch::DispatchMessage;
use tauri::api::notification::Notification;
use tauri::async_runtime::spawn;
use tauri::{App, Manager, Runtime};
use tokio_schedule::{every, Job};

use geeks_tracker_core::domain::task::TaskCommand;

use crate::application::{Application, CommandHandler};
use crate::dispatcher::Dispatcher;

pub fn setup_schedule<R: Runtime>(app: &mut App<R>) {
  let handle = app.handle();
  spawn(async move {
    every(1)
      .second()
      .perform(|| async {
        let application = handle.state::<Application>();
        let dispatcher = handle.state::<Dispatcher>();
        let tasks = application
          .lock()
          .await
          .tasks
          .states
          .values()
          .filter(|&x| x.schedule_available())
          .cloned()
          .collect::<Vec<_>>();
        for task in tasks {
          if let Some(schedule) = task.schedule {
            let prev = task.status.to_owned();
            let next = schedule.status.to_owned();
            let e1 = application
              .lock()
              .await
              .handle_command(TaskCommand::UpdateStatus {
                id: task.id.to_owned(),
                status: schedule.status,
              })
              .await;
            let e2 = application
              .lock()
              .await
              .handle_command(TaskCommand::UpdateSchedule {
                id: task.id,
                schedule: None,
              })
              .await;
            if let (Ok(e1), Ok(e2)) = (e1, e2) {
              let _ = Notification::new("me.seokju.geeks-tracker")
                .title(format!("{} {} -> {}", task.id, prev, next))
                .body(task.title)
                .show();
              let _ = dispatcher
                .send(DispatchMessage::TaskPersisted {
                  events: vec![e1, e2],
                })
                .await;
              log::trace!("notify: {:?}", task.id);
            } else {
              log::error!("schedule error");
            }
          }
        }
      })
      .await;
  });
}
