import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      refetchInterval: false,
      refetchOnReconnect: false,
      keepPreviousData: true,
      suspense: true,
    },
  },
});
