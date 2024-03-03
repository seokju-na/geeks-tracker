import { type ListTasksFilter, getTask, listTasks } from './bridges';

export const taskQueries = {
  all: ['tasks'],
  list: (filter: ListTasksFilter = {}) => ({
    queryKey: ['tasks', 'list', { filter }],
    queryFn: () => listTasks(filter),
  }),
  detail: (id: string) => ({
    queryKey: ['tasks', 'detail', id],
    queryFn: () => getTask(id),
  }),
};
