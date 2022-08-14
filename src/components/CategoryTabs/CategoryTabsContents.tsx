import { CSS } from '@stitches/react';
import { ReactNode, Suspense } from 'react';
import { useCategories } from '../../hooks/useCategories';
import { Category } from '../../types/Category';
import { FlatTabs } from '../FlatTabs';

interface Props {
  children: (category: Category) => ReactNode;
  css?: CSS;
}

export function CategoryTabsContent({ children, ...props }: Props) {
  const { categories, selectedCategoryId } = useCategories();

  return (
    <Suspense fallback={null}>
      {categories.map(category => (
        <FlatTabs.Content key={category.id} value={category.id} {...props}>
          {category.id === selectedCategoryId ? <>{children(category)}</> : null}
        </FlatTabs.Content>
      ))}
    </Suspense>
  );
}
