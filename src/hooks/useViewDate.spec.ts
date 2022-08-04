import { act, waitFor } from '@testing-library/react';
import { isToday, subDays } from 'date-fns';
import { expect, it } from 'vitest';
import { mockStore } from '../testing/mockStore';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { useViewDate } from './useViewDate';

it('get "viewDate" from ".history" store', async () => {
  const date = new Date(2022, 7, 4, 18, 0);
  mockStore('.history').set('viewDate', date.toISOString());

  const { result } = renderHookWithTestBed(() => useViewDate());
  await waitFor(() => {
    expect(result.current.viewDate).toEqual(date);
  });
});

it('default "viewDate" is today.', async () => {
  mockStore('.history');

  const { result } = renderHookWithTestBed(() => useViewDate());
  await waitFor(() => {
    expect(isToday(result.current.viewDate)).toBe(true);
  });
});

it('update "viewDate"', async () => {
  const date = new Date(2022, 7, 4, 18, 0);
  mockStore('.history').set('viewDate', date.toISOString());

  const { result } = renderHookWithTestBed(() => useViewDate());
  await waitFor(() => {
    expect(result.current.viewDate).toEqual(date);
  });

  const updated = subDays(date, 1);
  await act(async () => {
    await result.current.updateViewDate(updated);
  });
  await waitFor(() => {
    expect(result.current.viewDate).toEqual(updated);
  });
});
