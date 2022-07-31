import { act, waitFor } from '@testing-library/react';
import { it } from 'vitest';
import { mockStore } from '../testing/mockStore';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { useStore } from './useStore';

it('get value', async () => {
  mockStore('store').set('key', 'hello');

  const { result } = renderHookWithTestBed(() => useStore('store', 'key'));
  await waitFor(() => {
    expect(result.current.value).toEqual('hello');
  });
});

it('set value', async () => {
  mockStore('store').set('key', 'A');

  const { result } = renderHookWithTestBed(() => useStore('store', 'key'));
  await waitFor(() => expect(result.current.value).toEqual('A'));

  act(() => {
    result.current.setValue('B');
  });
  await waitFor(() => expect(result.current.value).toEqual('B'));
});

it('delete value', async () => {
  mockStore('store').set('key', 'A');

  const { result } = renderHookWithTestBed(() => useStore('store', 'key'));
  await waitFor(() => expect(result.current.value).toEqual('A'));

  act(() => {
    result.current.deleteValue();
  });
  await waitFor(() => expect(result.current.value).toEqual(null));
});
