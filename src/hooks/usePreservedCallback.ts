import { useCallback, useEffect, useRef } from 'react';
import { Any } from '../utils/types';

export function usePreservedCallback<Callback extends (...args: Any[]) => Any>(callback: Callback) {
  const callbackRef = useRef<Callback>(callback);

  useEffect(() => {
    callbackRef.current = callback;
  }, [callback]);

  return useCallback(
    (...args: Any[]) => {
      return callbackRef.current(...args);
    },
    [callbackRef]
  ) as Callback;
}
