import { type Task, type TaskStatus, formatTaskStatus } from '@geeks-tracker/core';
import { useSuspenseQuery } from '@tanstack/react-query';
import { groupBy, objectEntries } from '@toss/utils';
import cx from 'classnames';
import { useSubscription } from 'observable-hooks';
import { useMemo, useState } from 'react';
import { keyDowns$ } from '../global';
import { ChevronDownIcon } from '../icons/ChevronDownIcon';
import { ChevronUpIcon } from '../icons/ChevronUpIcon';
import { ClockIcon } from '../icons/ClockIcon';
import { taskQueries } from '../queries';
import { useSettings, useUpdateSettings } from '../settings';

export function Tasks() {
  // TODO: refactor this stuff
  const { data: visibility } = useSettings('ui.tasks.visibility');
  const visibleStatus = objectEntries(visibility)
    .filter(([, visible]) => visible)
    .map(([status]) => status as TaskStatus);
  const { data: tasks } = useSuspenseQuery(taskQueries.list({ status: visibleStatus }));
  const grouped = useMemo(() => groupBy(tasks, x => x.status), [tasks]);
  const { data: expansion } = useSettings('ui.tasks.expansion');
  const { mutate: updateSettings } = useUpdateSettings();
  const [selected, setSelected] = useState<string>();
  useSubscription(keyDowns$, e => {
    switch (e.key) {
      case 'ArrowDown': {
        e.stopPropagation();
        e.preventDefault();
        const t = Object.values(grouped).flat();
        let current = t.findIndex(x => x.id === selected) + 1;
        while (0 <= current && current <= tasks.length) {
          const task = t[current];
          if (task != null && visibility[task.status] && expansion[task.status]) {
            setSelected(task.id);
            break;
          }
          current += 1;
        }
        break;
      }
      case 'ArrowUp': {
        e.stopPropagation();
        e.preventDefault();
        const t = Object.values(grouped).flat();
        let current = t.findIndex(x => x.id === selected) - 1;
        while (0 <= current && current <= t.length) {
          const task = t[current];
          if (task != null && visibility[task.status] && expansion[task.status]) {
            setSelected(task.id);
            break;
          }
          current += 1;
        }
        break;
      }
      case 'Escape':
        if (selected != null) {
          e.stopPropagation();
          e.preventDefault();
          setSelected(undefined);
        }
        break;
    }
  });
  return (
    <>
      {visibleStatus.map(status => {
        return (
          <TaskGroup
            key={status}
            expanded={expansion[status]}
            status={status}
            tasks={grouped[status] ?? []}
            selected={selected}
            onExpandChange={x => {
              updateSettings({
                key: 'ui.tasks.expansion',
                value: { ...expansion, [status]: x },
              });
            }}
          />
        );
      })}
    </>
  );
}

interface TaskGroupProps {
  expanded: boolean;
  status: TaskStatus;
  tasks: Task[];
  selected: string | undefined;
  onExpandChange: (expanded: boolean) => void;
}

function TaskGroup({ expanded, status, selected, onExpandChange, tasks }: TaskGroupProps) {
  return (
    <section>
      <header
        aria-expanded={expanded}
        tabIndex={0}
        onKeyDown={e => {
          if (e.key === 'Enter') {
            e.preventDefault();
            e.stopPropagation();
            onExpandChange(!expanded);
          }
        }}
        className="flex items-center bg-zinc-700 justify-between px-2 py-1 focus:outline-none text-zinc-400 focus-visible:bg-blue-700 focus-visible:text-white"
      >
        <span className="text-sm font-semibold">
          {formatTaskStatus(status)} ({tasks.length})
        </span>
        {expanded ? <ChevronUpIcon size={16} /> : <ChevronDownIcon size={16} />}
      </header>
      {expanded ? (
        <ul>
          {tasks.map(task => {
            const isSelected = task.id === selected;
            return <TaskItem key={task.id} task={task} selected={isSelected} />;
          })}
        </ul>
      ) : null}
    </section>
  );
}

interface TaskItemProps {
  task: Task;
  selected?: boolean;
  className?: string;
}

function TaskItem({ task, selected, className }: TaskItemProps) {
  return (
    <li
      data-selected={selected}
      className={cx('flex items-center text-sm text-white py-1 px-2 gap-2 border-b border-b-zinc-700', className)}
    >
      <span className="w-8">{task.id}</span>
      <span className="flex-1 text-ellipsis overflow-hidden min-w-0">{task.title}</span>
      {task.schedule != null ? <ClockIcon size={12} /> : null}
    </li>
  );
}
