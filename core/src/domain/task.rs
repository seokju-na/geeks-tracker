use std::fmt::{Display, Formatter};

use chrono::Utc;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typed_builder::TypedBuilder;
use typeshare::typeshare;

use crate::eventsourcing::{Aggregate, AggregateRoot, Command, Event, Timestamp};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
#[typeshare(serialized_as = "String")]
pub struct TaskId {
  pub no: i32,
}

impl TaskId {
  pub fn new(no: i32) -> Self {
    Self { no }
  }
}

impl Default for TaskId {
  fn default() -> Self {
    Self::new(1)
  }
}

impl Display for TaskId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "#{}", self.no)
  }
}

impl TryFrom<String> for TaskId {
  type Error = crate::domain::Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let no_str = &value[1..];
    let no: i32 = no_str.parse().map_err(|_| Self::Error::InvalidTaskId)?;
    Ok(Self { no })
  }
}

impl Serialize for TaskId {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let str = format!("#{}", self.no);
    s.serialize_str(&str)
  }
}

impl<'de> Deserialize<'de> for TaskId {
  fn deserialize<D>(d: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let str = String::deserialize(d)?;
    let id = TaskId::try_from(str).map_err(|_| Error::custom("invalid task id"))?;
    Ok(id)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Task {
  pub id: TaskId,
  pub title: String,
  #[builder(default)]
  pub body: Option<String>,
  #[builder(default)]
  pub status: TaskStatus,
  #[builder(default = Utc::now().timestamp())]
  pub created_at: Timestamp,
  #[builder(default = Utc::now().timestamp())]
  pub updated_at: Timestamp,
  #[builder(default)]
  pub backlog_at: Option<Timestamp>,
  #[builder(default)]
  pub in_progress_at: Option<Timestamp>,
  #[builder(default)]
  pub queue_at: Option<Timestamp>,
  #[builder(default)]
  pub done_at: Option<Timestamp>,
  #[builder(default)]
  pub schedule: Option<TaskSchedule>,
}

impl Task {
  pub fn schedule_available(&self) -> bool {
    if let Some(schedule) = &self.schedule {
      return schedule.at <= Utc::now().timestamp_millis();
    }
    false
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct TaskSchedule {
  pub at: Timestamp,
  pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare]
pub enum TaskStatus {
  Backlog,
  Queue,
  InProgress,
  Done,
}

impl Display for TaskStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Backlog => "BACKLOG",
        Self::Queue => "QUEUE",
        Self::InProgress => "IN_PROGRESS",
        Self::Done => "DONE",
      }
    )
  }
}

impl Default for TaskStatus {
  fn default() -> Self {
    Self::Backlog
  }
}

impl Aggregate for Task {
  type Command = TaskCommand;
  type Event = TaskEvent;
  type Error = crate::domain::Error;

  fn id(&self) -> String {
    self.id.to_string()
  }

  fn handle_command(
    this: Option<&Self>,
    command: Self::Command,
    root: &AggregateRoot<Self>,
  ) -> Result<Self::Event, Self::Error> {
    match this {
      Some(_) => match command {
        Self::Command::UpdateTitle { title, .. } => Ok(Self::Event::TitleUpdated { title }),
        Self::Command::UpdateStatus { status, .. } => Ok(Self::Event::StatusUpdated { status }),
        Self::Command::UpdateBody { body, .. } => Ok(Self::Event::BodyUpdated { body }),
        Self::Command::Delete { .. } => Ok(Self::Event::Deleted {}),
        Self::Command::UpdateSchedule { schedule, .. } => {
          Ok(Self::Event::ScheduleUpdated { schedule })
        }
        _ => Err(crate::domain::Error::TaskNotExists),
      },
      None => match command {
        Self::Command::Create {
          title,
          status,
          schedule,
        } => Ok(Self::Event::Created {
          id: get_next_task_id(&root),
          title,
          body: None,
          status: status.unwrap_or_default(),
          schedule,
        }),
        _ => Err(crate::domain::Error::TaskAlreadyExists),
      },
    }
  }

  fn apply_event(
    this: Option<Self>,
    event: Self::Event,
  ) -> Result<(String, Option<Self>), Self::Error> {
    let now = Utc::now().timestamp_millis();
    match this {
      Some(mut task) => match event {
        Self::Event::TitleUpdated { title } => {
          task.title = title;
          task.updated_at = now;
          Ok((task.id.to_string(), Some(task)))
        }
        Self::Event::StatusUpdated { status } => {
          task.status = status.to_owned();
          task.updated_at = now;
          update_task_status_timestamp(&mut task, now);
          Ok((task.id.to_string(), Some(task)))
        }
        Self::Event::BodyUpdated { body } => {
          task.body = body;
          task.updated_at = now;
          Ok((task.id.to_string(), Some(task)))
        }
        Self::Event::Deleted { .. } => Ok((task.id.to_string(), None)),
        Self::Event::ScheduleUpdated { schedule, .. } => {
          task.schedule = schedule;
          task.updated_at = now;
          Ok((task.id.to_string(), Some(task)))
        }
        _ => Err(crate::domain::Error::TaskAlreadyExists),
      },
      None => match event {
        Self::Event::Created {
          id,
          title,
          body,
          status,
          schedule,
        } => {
          let mut task = Task::builder()
            .id(id)
            .title(title)
            .body(body)
            .status(status)
            .schedule(schedule)
            .build();
          update_task_status_timestamp(&mut task, now);
          Ok((task.id.to_string(), Some(task)))
        }
        _ => Err(crate::domain::Error::TaskNotExists),
      },
    }
  }
}

fn update_task_status_timestamp(task: &mut Task, at: Timestamp) {
  match task.status {
    TaskStatus::Backlog => {
      task.backlog_at = Some(at);
    }
    TaskStatus::Queue => {
      task.queue_at = Some(at);
    }
    TaskStatus::InProgress => {
      task.in_progress_at = Some(at);
    }
    TaskStatus::Done => {
      task.done_at = Some(at);
    }
  };
}

fn get_next_task_id(root: &AggregateRoot<Task>) -> TaskId {
  let mut no_list = root
    .ids()
    .iter()
    .map(|x| x.to_string())
    .filter_map(|x| TaskId::try_from(x).ok())
    .map(|x| x.no)
    .collect::<Vec<_>>();
  no_list.sort();
  let last_no = no_list.last().cloned().unwrap_or(0);
  TaskId::new(last_no + 1)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[typeshare]
#[serde(tag = "name", content = "data")]
pub enum TaskEvent {
  #[serde(rename = "task.created", rename_all = "camelCase")]
  Created {
    id: TaskId,
    title: String,
    body: Option<String>,
    status: TaskStatus,
    schedule: Option<TaskSchedule>,
  },
  #[serde(rename = "task.titleUpdated", rename_all = "camelCase")]
  TitleUpdated { title: String },
  #[serde(rename = "task.statusUpdated", rename_all = "camelCase")]
  StatusUpdated { status: TaskStatus },
  #[serde(rename = "task.bodyUpdated", rename_all = "camelCase")]
  BodyUpdated { body: Option<String> },
  #[serde(rename = "task.deleted", rename_all = "camelCase")]
  Deleted {},
  #[serde(rename = "task.scheduleUpdated", rename_all = "camelCase")]
  ScheduleUpdated { schedule: Option<TaskSchedule> },
}

impl Event for TaskEvent {
  fn name(&self) -> &'static str {
    match self {
      Self::Created { .. } => "task.created",
      Self::TitleUpdated { .. } => "task.titleUpdated",
      Self::StatusUpdated { .. } => "task.statusUpdated",
      Self::BodyUpdated { .. } => "task.bodyUpdated",
      Self::Deleted { .. } => "task.deleted",
      Self::ScheduleUpdated { .. } => "task.scheduleUpdated",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "name", content = "data")]
#[typeshare]
pub enum TaskCommand {
  #[serde(rename = "task.create", rename_all = "camelCase")]
  Create {
    title: String,
    status: Option<TaskStatus>,
    schedule: Option<TaskSchedule>,
  },
  #[serde(rename = "task.updateTitle", rename_all = "camelCase")]
  UpdateTitle { id: TaskId, title: String },
  #[serde(rename = "task.updateStatus", rename_all = "camelCase")]
  UpdateStatus { id: TaskId, status: TaskStatus },
  #[serde(rename = "task.updateBody", rename_all = "camelCase")]
  UpdateBody { id: TaskId, body: Option<String> },
  #[serde(rename = "task.delete", rename_all = "camelCase")]
  Delete { id: TaskId },
  #[serde(rename = "task.updateSchedule", rename_all = "camelCase")]
  UpdateSchedule {
    id: TaskId,
    schedule: Option<TaskSchedule>,
  },
}

impl Command for TaskCommand {
  fn name(&self) -> &'static str {
    match self {
      Self::Create { .. } => "task.create",
      Self::UpdateTitle { .. } => "task.updateTitle",
      Self::UpdateStatus { .. } => "task.updateStatus",
      Self::UpdateBody { .. } => "task.updateBody",
      Self::Delete { .. } => "task.delete",
      Self::UpdateSchedule { .. } => "task.updateSchedule",
    }
  }

  fn aggregate_id(&self) -> Option<String> {
    match self {
      Self::Create { .. } => None,
      Self::UpdateTitle { id, .. } => Some(id),
      Self::UpdateStatus { id, .. } => Some(id),
      Self::UpdateBody { id, .. } => Some(id),
      Self::Delete { id } => Some(id),
      Self::UpdateSchedule { id, .. } => Some(id),
    }
    .map(|x| x.to_string())
  }
}
