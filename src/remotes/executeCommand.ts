import { invoke } from '@tauri-apps/api';
import { CategoryCommand, isCategoryCommand } from '../types/CategoryCommand';
import { isNoteCommand, NoteCommand } from '../types/NoteCommand';

type Command = CategoryCommand | NoteCommand;

export function executeCommand(command: Command) {
  if (isCategoryCommand(command)) {
    return invoke<void>('execute_category_command', { command });
  }
  if (isNoteCommand(command)) {
    return invoke<void>('execute_note_command', { command });
  }
}
