import { assert, AssertionError } from '~/utils/assert';

describe('assert', () => {
  it('should throw "AssertionError" when condition falsy', () => {
    const a: number = 10;

    expect(() => assert(a === 20)).toThrowError(new AssertionError());
  });

  it('should throw custom error', () => {
    const a: number = 15;

    class MyCustomError extends Error {
      constructor() {
        super('error!');
      }
    }

    expect(() => assert(a === 20, new MyCustomError())).toThrowError(new MyCustomError());
  });
});
