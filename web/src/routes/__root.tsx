import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { createRootRoute, Outlet } from '@tanstack/react-router';
import { useSubscription } from 'observable-hooks';
import { lazy, Suspense, useState } from 'react';
import { filter } from 'rxjs';
import { hideApp } from '../bridges';
import { dispatcherMessages$, keyDowns$ } from '../global';
import { taskQueries } from '../queries';

const Devtools = PRODUCTION
  ? () => null
  : lazy(() =>
      import('@tanstack/router-devtools').then(x => ({
        default: x.TanStackRouterDevtools,
      }))
    );

function Root() {
  const [queryClient] = useState(
    () =>
      new QueryClient({
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
      })
  );
  useSubscription(keyDowns$.pipe(filter(e => e.key === 'Escape')), e => {
    const { activeElement } = document;
    // Hide window when focus lost.
    if (
      activeElement === null ||
      activeElement === document.body ||
      (activeElement as HTMLInputElement)?.value === ''
    ) {
      e.stopPropagation();
      e.preventDefault();
      hideApp();
    } else {
      // (activeElement as HTMLElement)?.blur();
    }
  });
  useSubscription(dispatcherMessages$, e => {
    switch (e.payload.name) {
      case 'task.persisted':
        queryClient.invalidateQueries({ queryKey: taskQueries.all });
        break;
    }
  });
  return (
    <QueryClientProvider client={queryClient}>
      <Outlet />
      <Suspense>
        <Devtools />
      </Suspense>
    </QueryClientProvider>
  );
}

export const Route = createRootRoute({
  component: Root,
});
