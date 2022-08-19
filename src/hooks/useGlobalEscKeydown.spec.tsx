import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { expect, it, vitest } from 'vitest';
import { history } from '../location';
import { mockTauriEvent } from '../testing/mockIPC';
import { useGlobalEscKeydown } from './useGlobalEscKeydown';

it('emit hide event when press "ESC" key if event target is document', async () => {
  const hide = vitest.fn();
  mockTauriEvent('geeks-tracker://hide', hide);

  function Test() {
    useGlobalEscKeydown();
    return <div />;
  }
  render(<Test />);
  await userEvent.keyboard('{Escape}');

  expect(hide).toHaveBeenCalled();
});

it('history back when press "ESC" key if event target is document and has history', async () => {
  const hide = vitest.fn();
  mockTauriEvent('geeks-tracker://hide', hide);
  history.push('/path');

  function Test() {
    useGlobalEscKeydown();
    return <div />;
  }
  render(<Test />);
  await userEvent.keyboard('{Escape}');
  expect(history.index).toEqual(0);
  await userEvent.keyboard('{Escape}');
  expect(hide).toHaveBeenCalled();
});

it('blur active element when press "ESC" key if active element is not document', async () => {
  function Test() {
    useGlobalEscKeydown();
    return (
      <div>
        <button data-testid="test-btn">btn</button>
      </div>
    );
  }
  render(<Test />);

  // 1. focus and blur button
  await userEvent.keyboard('{Tab}');
  expect(document.activeElement).toBe(screen.getByTestId('test-btn'));
  await userEvent.keyboard('{Escape}');
  expect(document.activeElement).toBe(document.body);

  // 2. hit ESC key once again
  const callback = vitest.fn();
  mockTauriEvent('geeks-tracker://hide', callback);
  await userEvent.keyboard('{Escape}');
  expect(callback).toHaveBeenCalled();
});
