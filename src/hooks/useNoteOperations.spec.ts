import { it, vi } from 'vitest';
import { mockIPC } from '../testing/mockIPC';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { useNoteOperations } from './useNoteOperations';

it('create or update note', async () => {
  const callback = vi.fn();
  mockIPC('execute_note_command', callback);

  const { result } = renderHookWithTestBed(() => useNoteOperations());
  await result.current.createOrUpdateNote({
    id: 'noteId',
    categoryId: 'categoryId',
    body: 'body',
  });
  expect(callback).toHaveBeenCalledWith({
    command: {
      name: 'NoteCommand.CreateOrUpdate',
      id: 'noteId',
      categoryId: 'categoryId',
      body: 'body',
    },
  });
});
