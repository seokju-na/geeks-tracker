import { Divider, Flex } from '@adobe/react-spectrum';
import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { CommandInput } from '../components/CommandInput';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const [value, setValue] = useState('');
  return (
    <>
      <Flex data-tauri-drag-region alignItems="center" height="size-550">
        <CommandInput />
      </Flex>
      <Divider size="S" />
    </>
  );
}
