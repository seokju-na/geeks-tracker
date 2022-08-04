import { act, waitFor } from '@testing-library/react';
import { expect, it, vitest } from 'vitest';
import { DummyCategory } from '../testing/DummyCategory';
import { mockIPC } from '../testing/mockIPC';
import { mockStore } from '../testing/mockStore';
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

it('get last selected id from ".history" store', async () => {
  const categories = DummyCategory.buildList(5);
  mockStore('.history').set('lastSelectedCategoryId', categories[2]!.id);

  const { result } = renderHookWithTestBed(() => useCategories());
  await waitFor(() => {
    expect(result.current.selectedCategory).toEqual(categories[2]);
  });
});

it('get select id from first category if last selected id is not exists', async () => {
  const categories = DummyCategory.buildList(5);
  mockIPC('list_categories', () => categories);

  const { result } = renderHookWithTestBed(() => useCategories());
  await waitFor(() => {
    expect(result.current.selectedCategory).toEqual(categories[0]);
  });
});

it('select category id', async () => {
  const categories = DummyCategory.buildList(5);
  mockIPC('list_categories', () => categories);
  mockStore('.history').set('lastSelectedCategoryId', categories[0]!.id);

  const { result } = renderHookWithTestBed(() => useCategories());
  await waitFor(() => {
    expect(result.current.selectedCategory).toEqual(categories[0]);
  });
  await act(async () => {
    await result.current.selectCategory(categories[2]!.id);
  });
  await waitFor(() => {
    expect(result.current.selectedCategory).toEqual(categories[2]);
  });
});
