import { type TaskCommand, TaskStatus } from './type.gen';

export {
  type TaskId,
  type Task,
  TaskStatus,
  type DispatchMessage,
  type Persisted,
  type Version,
  type Timestamp,
  type TaskSchedule,
} from './type.gen';

type AllCommand = TaskCommand;
export type CommandName = AllCommand['name'];
export type Command<T extends CommandName = CommandName> = Extract<AllCommand, { name: T }>;

export function formatTaskStatus(status: TaskStatus): string {
  switch (status) {
    case TaskStatus.Backlog:
      return 'BACKLOG';
    case TaskStatus.Queue:
      return 'QUEUE';
    case TaskStatus.InProgress:
      return 'IN PROGRESS';
    case TaskStatus.Done:
      return 'DONE';
  }
}
