import { invoke } from '@tauri-apps/api';
import { Category } from '../types/Category';

export function listCategories() {
  return invoke<Category[]>('geeks-tracker://categories/list');
}
