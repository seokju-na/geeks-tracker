import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { expect, it, vitest } from 'vitest';
import { mockOSType } from '../../testing/mockOSType';
import { renderWithTestBed } from '../../testing/renderWithTestBed';
import { Accelerator } from './Accelerator';

it('(macOS) append hotkey label on "aria-label" attribute', async () => {
  mockOSType('macos');
  renderWithTestBed(
    <Accelerator shortcut={['CmdOrCtrl', 'A']}>
      <button aria-label="Add" />
    </Accelerator>
  );
  expect(await screen.findByRole('button')).toHaveAttribute('aria-label', 'Add (⌘+A)');
});

it('(linux) append hotkey label on "aria-label" attribute', async () => {
  mockOSType('linux');
  renderWithTestBed(
    <Accelerator shortcut={['CmdOrCtrl', 'A']}>
      <button aria-label="Add" />
    </Accelerator>
  );
  expect(await screen.findByRole('button')).toHaveAttribute('aria-label', 'Add (Ctrl+A)');
});

it('(windows) append hotkey label on "aria-label" attribute', async () => {
  mockOSType('windows');
  renderWithTestBed(
    <Accelerator shortcut={['CmdOrCtrl', 'A']}>
      <button aria-label="Add" />
    </Accelerator>
  );
  expect(await screen.findByRole('button')).toHaveAttribute('aria-label', 'Add (Ctrl+A)');
});

it('call "onKeyPress" on click', async () => {
  const onKeyPress = vitest.fn();

  mockOSType('macos');
  renderWithTestBed(
    <Accelerator shortcut={['CmdOrCtrl', 'K']} onKeyPress={onKeyPress}>
      <button aria-label="Click me" />
    </Accelerator>
  );
  const button = await screen.findByRole('button');
  await userEvent.click(button);

  expect(onKeyPress).toHaveBeenCalled();
});
