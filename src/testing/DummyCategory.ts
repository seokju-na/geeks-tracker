import { each, Sync } from 'factory.ts';
import { Category } from '../types/Category';

export const DummyCategory = Sync.makeFactory<Category>({
  id: each(x => `category-${x}`),
  title: each(x => `category-title-${x}`),
  template: each(x => `category-template-${x}`),
  order: 0,
  createdAt: Date.now(),
  updatedAt: Date.now(),
});
