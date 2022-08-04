import { invoke } from '@tauri-apps/api';
import { CategoryCommand } from '../types/CategoryCommand';

export function executeCategoryCommand(command: CategoryCommand) {
  return invoke<void>('execute_category_command', { command });
}
