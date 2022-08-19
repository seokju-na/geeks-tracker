import { emit } from '@tauri-apps/api/event';
import { history } from '../location';
import { useKeydown } from './useKeydown';

export function useGlobalEscKeydown() {
  useKeydown(document, event => {
    if (event.defaultPrevented || event.key !== 'Escape') {
      return;
    }

    const { activeElement } = document;
    event.preventDefault();

    if (activeElement != null && activeElement !== document.body) {
      (activeElement as HTMLElement)?.blur();
      return;
    }

    if (event.target === document.body) {
      event.preventDefault();
      if (history.index > 0) {
        history.back();
      } else {
        emit('geeks-tracker://hide');
      }
    }
  });
}
