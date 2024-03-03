import { Divider, Flex } from '@adobe/react-spectrum';
import { createFileRoute } from '@tanstack/react-router';
import { Suspense } from 'react';
import { CommandInput } from '../components/CommandInput';
import { Tasks } from '../components/Tasks';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <>
      <Flex data-tauri-drag-region alignItems="center" height="size-550">
        <CommandInput />
      </Flex>
      <Divider size="S" />
      <Suspense>
        <Tasks />
      </Suspense>
    </>
  );
}
