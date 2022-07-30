import { emit } from '@tauri-apps/api/event';
import { useKeydown } from './useKeydown';

export function useGlobalEscKeydown() {
  useKeydown(document, event => {
    if (event.defaultPrevented || event.key !== 'Escape') {
      return;
    }

    const { activeElement } = document;

    // Hide window when focus lost
    if (activeElement == null || activeElement === document.body) {
      event.preventDefault();
      emit('geeks-tracker://hide');
    } else {
      (activeElement as HTMLElement)?.blur();
    }
  });
}
