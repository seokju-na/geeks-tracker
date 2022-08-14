import { invoke } from '@tauri-apps/api';
import { Note } from '../types/Note';

export function getNote(id: string) {
  return invoke<Note | null>('get_note', { id });
}
