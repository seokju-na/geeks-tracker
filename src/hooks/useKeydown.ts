import { useEffect } from 'react';
import { usePreservedCallback } from './usePreservedCallback';

export function useKeydown(elem: HTMLElement | Document | null, listener: (event: KeyboardEvent) => void) {
  const callback = usePreservedCallback(listener);

  useEffect(() => {
    if (elem == null) {
      return;
    }

    const eventListener = (e: Event) => {
      callback(e as KeyboardEvent);
    };
    elem.addEventListener('keydown', eventListener);

    return () => {
      elem.removeEventListener('keydown', eventListener);
    };
  }, [elem, callback]);
}
