import { act, waitFor } from '@testing-library/react';
import { it } from 'vitest';
import { mockStore } from '../testing/mockStore';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { defaultAllSettings, Settings } from '../types/Settings';
import { useSettings } from './useSettings';

it('default "appearance" settings', async () => {
  mockStore('.settings');

  const { result } = renderHookWithTestBed(() => useSettings('appearance'));
  await waitFor(() => {
    expect(result.current.settings).toEqual(defaultAllSettings.appearance);
  });
});

it('get "appearance" settings from store', async () => {
  const settings: Settings<'appearance'> = {
    viewDateFormat: 'yyyy/MM/dd',
  };
  mockStore('.settings').set('appearance', settings);

  const { result } = renderHookWithTestBed(() => useSettings('appearance'));
  await waitFor(() => {
    expect(result.current.settings).toEqual(settings);
  });
});

it('update "appearance" settings', async () => {
  mockStore('.settings');
  const { result } = renderHookWithTestBed(() => useSettings('appearance'));
  await waitFor(() => {
    expect(result.current.settings).toEqual(defaultAllSettings.appearance);
  });
  const updated: Settings<'appearance'> = {
    viewDateFormat: 'sky is blue',
  };
  await act(async () => {
    await result.current.updateSettings(updated);
  });
  await waitFor(() => {
    expect(result.current.settings).toEqual(updated);
  });
});
