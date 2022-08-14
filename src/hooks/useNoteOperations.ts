import { useCallback } from 'react';
import { executeCommand } from '../remotes/executeCommand';
import { useNote } from './useNote';

export function useNoteOperations() {
  const createOrUpdateNote = useCallback(async (payload: { id: string; categoryId: string; body: string }) => {
    await executeCommand({
      name: 'NoteCommand.CreateOrUpdate',
      ...payload,
    });
    await useNote.invalidate(payload.id);
  }, []);

  return {
    createOrUpdateNote,
  };
}
