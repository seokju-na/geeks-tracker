import { createRootRoute, Link, Outlet } from '@tanstack/react-router';
import { lazy, Suspense } from 'react';

const Devtools = PRODUCTION
  ? () => null
  : lazy(() =>
      import('@tanstack/router-devtools').then(x => ({
        default: x.TanStackRouterDevtools,
      }))
    );

export const Route = createRootRoute({
  component: () => (
    <>
      <div data-tauri-drag-region className="p-2 flex gap-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>{' '}
        <Link to="/about" className="[&.active]:font-bold">
          About
        </Link>
      </div>
      <hr />
      <Outlet />
      <Suspense>
        <Devtools />
      </Suspense>
    </>
  ),
});
