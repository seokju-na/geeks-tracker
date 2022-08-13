import { useCallback } from 'react';
import { useQuery } from 'react-query';
import { queryClient } from '../queryClient';
import { executeCommand } from '../remotes/executeCommand';
import { listCategories } from '../remotes/listCategories';
import { uid } from '../utils/uid';
import { useStore } from './useStore';

const queryKey = ['categories'];

function invalidateQueries() {
  return queryClient.invalidateQueries(queryKey);
}

export function useCategories() {
  const { data: categories = [] } = useQuery(queryKey, () => listCategories());

  const addCategory = useCallback(async (payload: { title: string; template: string }) => {
    await executeCommand({
      name: 'CategoryCommand.Create',
      id: uid(),
      ...payload,
    });
    await invalidateQueries();
  }, []);

  const { value: lastSelectedCategoryId, setValue: setSelectedCategoryId } = useStore<string>(
    '.history',
    'lastSelectedCategoryId'
  );
  const selectedCategoryId = lastSelectedCategoryId ?? categories[0]?.id;

  const selectCategory = useCallback(
    (categoryId: string) => {
      return setSelectedCategoryId(categoryId);
    },
    [setSelectedCategoryId]
  );

  return {
    categories,
    selectedCategoryId,
    selectedCategory: categories.find(x => x.id === selectedCategoryId),
    addCategory,
    selectCategory,
  } as const;
}

useCategories.invalidateQueries = invalidateQueries;
