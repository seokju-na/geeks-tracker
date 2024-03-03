import { createFileRoute } from '@tanstack/react-router';
import { Suspense } from 'react';
import { CommandInput } from '../components/CommandInput';
import { Tasks } from '../components/Tasks';
import { Titlebar } from '../components/Titlebar';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <>
      <Titlebar />
      <div className="flex py-1.5 px-1 items-center border-b border-b-zinc-700">
        <CommandInput />
      </div>
      <Suspense>
        <Tasks />
      </Suspense>
    </>
  );
}
