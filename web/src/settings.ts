import { TaskStatus } from '@geeks-tracker/core';
import { useMutation, useQueryClient, useSuspenseQuery } from '@tanstack/react-query';
import { useSubscription } from 'observable-hooks';
import { Observable, from, share } from 'rxjs';
import { Store } from 'tauri-plugin-store-api';

export interface Settings {
  'ui.tasks.visibility': Record<TaskStatus, boolean>;
  'ui.tasks.expansion': Record<TaskStatus, boolean>;
}

export type SettingsKey = keyof Settings;

export const defaultSettings: Settings = {
  'ui.tasks.visibility': {
    [TaskStatus.Backlog]: true,
    [TaskStatus.Queue]: true,
    [TaskStatus.InProgress]: true,
    [TaskStatus.Done]: false,
  },
  'ui.tasks.expansion': {
    [TaskStatus.Backlog]: true,
    [TaskStatus.Queue]: true,
    [TaskStatus.InProgress]: true,
    [TaskStatus.Done]: true,
  },
};

const store = new Store('.settings');

type Changed<T = SettingsKey> = T extends SettingsKey ? { key: T; value: Settings[T] | null } : never;

const storeChanges$ = from(
  new Observable<Changed>(subscriber => {
    const unlisten = store.onChange((key, value: any) => {
      subscriber.next({ key, value } as Changed);
    });
    return () => unlisten.then(fn => fn());
  })
).pipe(share());

const queryKey = (key: SettingsKey) => ['settings', key];

export function useSettings<T extends SettingsKey>(key: T) {
  const queryClient = useQueryClient();
  const query = useSuspenseQuery({
    queryKey: queryKey(key),
    queryFn: async () => {
      const value = await store.get<Settings[T]>(key);
      return value ?? defaultSettings[key];
    },
  });
  useSubscription(storeChanges$, changed => {
    if (changed.key === key) {
      queryClient.setQueryData(queryKey(key), changed.value ?? defaultSettings[key]);
    }
  });
  return query;
}

export function useUpdateSettings() {
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationFn: async (changed: Changed) => {
      await store.set(changed.key, changed.value);
      return changed;
    },
    onSuccess: async changed => {
      queryClient.setQueryData(queryKey(changed.key), changed.value);
      await store.save();
    },
  });
  return mutation;
}
