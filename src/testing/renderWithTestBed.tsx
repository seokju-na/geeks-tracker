import { QueryClientProvider } from '@tanstack/react-query';
import { queries, Queries, render, renderHook, RenderHookOptions, RenderOptions } from '@testing-library/react';
import { ReactElement, ReactNode, Suspense } from 'react';
import { queryClient } from '../queryClient';

export function renderWithTestBed<
  Q extends Queries = typeof queries,
  C extends Element | DocumentFragment = HTMLElement
>(ui: ReactElement, options: RenderOptions<Q, C> & Omit<Props, 'children'> = {}) {
  return render<Q, C>(ui, {
    ...options,
    wrapper: ({ children }) => {
      const Wrapper = options.wrapper as any;

      return <Provider>{Wrapper != null ? <Wrapper>{children}</Wrapper> : children}</Provider>;
    },
  });
}

export function renderHookWithTestBed<P, R>(callback: (props: P) => R, options: RenderHookOptions<P> = {}) {
  return renderHook(callback, {
    ...options,
    wrapper: ({ children }) => {
      const Wrapper = options.wrapper as any;

      return <Provider>{Wrapper != null ? <Wrapper>{children}</Wrapper> : children}</Provider>;
    },
  });
}

interface Props {
  children?: ReactNode;
}

function Provider({ children }: Props) {
  return (
    <QueryClientProvider client={queryClient}>
      <Suspense fallback={null}>{children}</Suspense>
    </QueryClientProvider>
  );
}
