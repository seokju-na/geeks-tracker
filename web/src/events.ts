import type { DispatchMessage } from '@geeks-tracker/core';
import { type Event, type EventName, listen } from '@tauri-apps/api/event';
import { Observable, filter, fromEvent, share } from 'rxjs';

export const escKeydown$ = fromEvent<KeyboardEvent>(document, 'keydown').pipe(
  filter(e => e.key === 'Escape' && !e.defaultPrevented),
  share()
);

const fromTauriEvent = <T>(event: EventName): Observable<Event<T>> => {
  return new Observable(subscriber => {
    const unlisten = listen<T>(event, event => {
      subscriber.next(event);
    });
    return () => unlisten.then(x => x());
  });
};

export const appFocused$ = fromTauriEvent<void>('app_focused').pipe(share());
export const dispatcherMessages$ = fromTauriEvent<DispatchMessage>('dispatcher_message').pipe(share());
