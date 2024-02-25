import { invoke } from '@tauri-apps/api';
import type { Command, Task, TaskStatus } from '@geeks-tracker/core';
import { emit } from '@tauri-apps/api/event';

export function hideApp() {
  return emit('hide_app');
}

export interface ListTasksFilter {
  keyword?: string;
  status?: TaskStatus[];
  from?: string;
  to?: string;
}

export function listTasks(filter?: ListTasksFilter) {
  return invoke<Task[]>('list_tasks', { filter });
}

export function getTask(id: string) {
  return invoke<Task>('get_task', { id });
}

export function runCommand(command: Command) {
  const [category] = command.name.split('.');
  return invoke<void>(`run_${category}_command`, { command });
}
