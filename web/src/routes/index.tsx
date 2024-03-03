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
      <CommandInput />
      <Suspense>
        <Tasks />
      </Suspense>
    </>
  );
}
