import { QueryClient } from '@tanstack/react-query';
import { type ListTasksFilter, getTask, listTasks } from './bridges';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      networkMode: 'always',
      retry: false,
      staleTime: Number.POSITIVE_INFINITY,
    },
    mutations: {
      retry: false,
    },
  },
});

export const taskQueries = {
  list: (filter: ListTasksFilter = {}) => ({
    queryKey: ['tasks', 'list', { filter }],
    queryFn: () => listTasks(filter),
  }),
  detail: (id: string) => ({
    queryKey: ['tasks', 'detail', { id }],
    queryFn: () => getTask(id),
  }),
};
