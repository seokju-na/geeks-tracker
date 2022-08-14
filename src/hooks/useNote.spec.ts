import { waitFor } from '@testing-library/react';
import { expect, it, vi } from 'vitest';
import { DummyNote } from '../testing/DummyNote';
import { mockIPC } from '../testing/mockIPC';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { useNote } from './useNote';

it('query note by id', async () => {
  const note = DummyNote.build();
  const callback = vi.fn().mockResolvedValue(note);
  mockIPC('get_note', callback);

  const { result } = renderHookWithTestBed(() => useNote(note.id));
  await waitFor(() => {
    expect(result.current.note).toEqual(note);
  });
  expect(callback).toHaveBeenCalledWith({ id: note.id });
});
