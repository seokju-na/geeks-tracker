import { Cell, Column, Divider, Flex, Row, TableBody, TableHeader, TableView } from '@adobe/react-spectrum';
import { useSuspenseQuery } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router';
import { Suspense } from 'react';
import { CommandInput } from '../components/CommandInput';
import { taskQueries } from '../queries';

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

function Tasks() {
  const { data: tasks } = useSuspenseQuery(taskQueries.list());
  return (
    <TableView aria-label="Example table with static contents">
      <TableHeader>
        <Column>#</Column>
        <Column>Title</Column>
        <Column>Status</Column>
      </TableHeader>
      <TableBody>
        {tasks.map(task => {
          return (
            <Row key={task.id}>
              <Cell>{task.id}</Cell>
              <Cell>{task.title}</Cell>
              <Cell>{task.status}</Cell>
            </Row>
          );
        })}
      </TableBody>
    </TableView>
  );
}
