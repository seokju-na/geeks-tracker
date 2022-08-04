import { waitFor } from '@testing-library/react';
import { expect, it, vitest } from 'vitest';
import { DummyCategory } from '../testing/DummyCategory';
import { mockIPC } from '../testing/mockIPC';
import { renderHookWithTestBed } from '../testing/renderWithTestBed';
import { useCategories } from './useCategories';

it('query categories', async () => {
  const categories = DummyCategory.buildList(10);
  mockIPC('list_categories', () => categories);

  const { result } = renderHookWithTestBed(() => useCategories());
  await waitFor(() => {
    expect(result.current.categories).toEqual(categories);
  });
});

it('add category', async () => {
  const executeCommand = vitest.fn();

  mockIPC('list_categories', () => DummyCategory.buildList(5));
  mockIPC('execute_category_command', executeCommand);

  const { result } = renderHookWithTestBed(() => useCategories());
  await waitFor(async () => {
    await result.current.addCategory({
      title: 'My New Category',
      template: 'this is template',
    });
  });
  expect(executeCommand).toHaveBeenCalledWith({
    command: expect.objectContaining({
      name: 'CategoryCommand.Create',
      title: 'My New Category',
      template: 'this is template',
    }),
  });
});
