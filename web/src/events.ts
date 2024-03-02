import { listen } from '@tauri-apps/api/event';
import { Observable, filter, from, fromEvent, share } from 'rxjs';

export const escKeydown$ = fromEvent<KeyboardEvent>(document, 'keydown').pipe(
  filter(e => e.key === 'Escape' && !e.defaultPrevented),
  share()
);

export const appFocused$ = from(
  new Observable<void>(subscribe => {
    const unlisten = listen('app_focused', () => {
      subscribe.next();
    });
    return () => unlisten.then(fn => fn());
  })
).pipe(share());
