import { useCallback } from 'react';
import { defaultAllSettings, Settings, SettingsName } from '../types/Settings';
import { useStore } from './useStore';

export function useSettings<T extends SettingsName>(name: T) {
  const { value: _settings, setValue } = useStore<Settings<T>>('.settings', name);
  const settings = _settings ?? defaultAllSettings[name];

  const updateSettings = useCallback(
    (settings: Settings<T>) => {
      return setValue(settings);
    },
    [setValue]
  );

  return {
    settings,
    updateSettings,
  } as const;
}
