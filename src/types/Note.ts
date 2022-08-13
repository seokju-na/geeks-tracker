import { format } from 'date-fns';
import { Category } from './Category';

export interface Note {
  id: string;
  categoryId: string;
  body: string;
  createdAt: number;
  updatedAt: number;
}

export function makeNoteId({ category, date }: { category: Category; date: Date }) {
  const dateStr = format(date, 'yyyy-MM-dd');
  return `${category.id}/${dateStr}`;
}
