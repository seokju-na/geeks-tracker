import { useCallback } from 'react';
import { useQuery } from 'react-query';
import { queryClient } from '../queryClient';
import { executeCategoryCommand } from '../remotes/executeCategoryCommand';
import { listCategories } from '../remotes/listCategories';
import { uid } from '../utils/uid';

const queryKey = ['categories'];

interface AddCategory {
  title: string;
  template: string;
}

function invalidateQueries() {
  return queryClient.invalidateQueries(queryKey);
}

export function useCategories() {
  const { data: categories = [] } = useQuery(queryKey, () => listCategories());

  const addCategory = useCallback(async ({ title, template }: AddCategory) => {
    await executeCategoryCommand({
      name: 'CategoryCommand.Create',
      id: uid(),
      title,
      template,
    });
    await invalidateQueries();
  }, []);

  return {
    categories,
    addCategory,
  } as const;
}

useCategories.invalidateQueries = invalidateQueries;
