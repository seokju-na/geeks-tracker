import type { TaskCommand } from './type.gen';

export {
  type TaskId,
  type Task,
  TaskStatus,
  type DispatchMessage,
  type Persisted,
  type Version,
  type Timestamp,
} from './type.gen';

type AllCommand = TaskCommand;
export type CommandName = AllCommand['name'];
export type Command<T extends CommandName = CommandName> = Extract<AllCommand, { name: T }>;
