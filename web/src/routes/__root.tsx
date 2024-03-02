import { Outlet, createRootRoute } from '@tanstack/react-router';
import { useSubscription } from 'observable-hooks';
import { Suspense, lazy } from 'react';
import { hideApp } from '../bridges';
import { escKeydown$ } from '../events';

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
  return (
    <>
      <Outlet />
      <Suspense>
        <Devtools />
      </Suspense>
    </>
  );
}

export const Route = createRootRoute({
  component: Root,
});
