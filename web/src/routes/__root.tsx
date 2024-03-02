import { Provider, defaultTheme } from '@adobe/react-spectrum';
import { QueryClientProvider } from '@tanstack/react-query';
import { Outlet, createRootRoute } from '@tanstack/react-router';
import { useSubscription } from 'observable-hooks';
import { Suspense, lazy } from 'react';
import { hideApp } from '../bridges';
import { dispatcherMessages$, escKeydown$ } from '../events';
import { queryClient, taskQueries } from '../queries';

const Devtools = PRODUCTION
  ? () => null
  : lazy(() =>
      import('@tanstack/router-devtools').then(x => ({
        default: x.TanStackRouterDevtools,
      }))
    );

function Root() {
  useSubscription(escKeydown$, e => {
    const { activeElement } = document;
    // Hide window when focus lost.
    if (
      activeElement === null ||
      activeElement === document.body ||
      (activeElement as HTMLInputElement)?.value === ''
    ) {
      e.preventDefault();
      hideApp();
    } else {
      (activeElement as HTMLElement)?.blur();
    }
  });
  useSubscription(dispatcherMessages$, e => {
    console.log(e);
    switch (e.payload.name) {
      case 'task.persisted':
        queryClient.invalidateQueries({ queryKey: taskQueries.all });
        break;
    }
  });
  return (
    <QueryClientProvider client={queryClient}>
      <Provider theme={defaultTheme} UNSAFE_style={{ background: 'transparent' }}>
        <Outlet />
        <Suspense>
          <Devtools />
        </Suspense>
      </Provider>
    </QueryClientProvider>
  );
}

export const Route = createRootRoute({
  component: Root,
});
