import type { AnyObject } from './types';

export class AssertionError extends Error {
  readonly name = 'AssertionError';

  static instanceOf(e: unknown): e is AssertionError {
    return e != null && (e as AnyObject)?.name === 'AssertionError';
  }

  constructor(message?: string) {
    super(message);
  }
}

export function assert(condition: unknown, messageOrError: string | Error = new AssertionError()): asserts condition {
  if (!condition) {
    if (typeof messageOrError === 'string') {
      throw new AssertionError(messageOrError);
    } else {
      throw messageOrError;
    }
  }
}
