import { emit } from '@tauri-apps/api/event';
import { location } from '../location';
import { useKeydown } from './useKeydown';

export function useGlobalEscKeydown() {
  useKeydown(document, event => {
    if (event.defaultPrevented || event.key !== 'Escape') {
      return;
    }

    const { activeElement } = document;

    if (activeElement != null && activeElement !== document.body) {
      (activeElement as HTMLElement)?.blur();
      return;
    }

    if (location.current.pathname !== '/') {
      event.preventDefault();
      location.history.back();
      return;
    }

    // Hide window when focus lost
    event.preventDefault();
    emit('geeks-tracker://hide');
  });
}
