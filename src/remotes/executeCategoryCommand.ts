import { invoke } from '@tauri-apps/api';
import { CategoryCommand } from '../types/CategoryCommand';

export function executeCategoryCommand(command: CategoryCommand) {
  return invoke<void>('geeks-tracker://categories/executeCommand', { command });
}
