import { useMutation, useQuery } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { runCommand } from '../bridges';
import { queryClient, taskQueries } from '../queries';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const { data: tasks } = useQuery(taskQueries.list());
  const [title, setTitle] = useState('');
  const { mutate, isPending } = useMutation({
    mutationFn: async () => {
      await runCommand({
        name: 'task.create',
        data: {
          id: '#1',
          title,
        },
      });
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: taskQueries.list().queryKey });
    },
  });
  console.log(tasks);
  return (
    <div className="p-2">
      <h3>Welcome Home!</h3>
      <input value={title} onChange={e => setTitle(e.target.value)} />
      <button type="button" disabled={isPending} onClick={() => mutate()}>
        Submit
      </button>
    </div>
  );
}
