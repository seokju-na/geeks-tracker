import { parseISO } from 'date-fns';
import { useCallback } from 'react';
import { useStore } from './useStore';

export function useViewDate() {
  const { value, setValue } = useStore<string>('.history', 'viewDate');
  const viewDate = safedateParse(value) ?? new Date();

  const updateViewDate = useCallback(
    (viewDate: Date) => {
      return setValue(viewDate.toISOString());
    },
    [setValue]
  );

  return {
    viewDate,
    updateViewDate,
  } as const;
}

function safedateParse(dateStr: string | null | undefined): Date | null {
  if (dateStr == null) {
    return null;
  }

  try {
    return parseISO(dateStr);
  } catch {
    return null;
  }
}
