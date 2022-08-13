import { each, Sync } from 'factory.ts';
import { Note } from '../types/Note';

export const DummyNote = Sync.makeFactory<Note>({
  id: each(x => `note-${x}`),
  categoryId: each(x => `category-${x}`),
  body: each(x => `note-body-${x}`),
  createdAt: Date.now(),
  updatedAt: Date.now(),
});
