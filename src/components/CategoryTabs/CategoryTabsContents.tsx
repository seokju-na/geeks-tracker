import { CSS } from '@stitches/react';
import { ReactNode, Suspense } from 'react';
import { useCategories } from '../../hooks/useCategories';
import { FlatTabs } from '../FlatTabs';

interface Props {
  children: ReactNode;
  css?: CSS;
}

export function CategoryTabsContent({ children, ...props }: Props) {
  const { categories, selectedCategoryId } = useCategories();

  return (
    <Suspense fallback={null}>
      {categories.map(category => (
        <FlatTabs.Content key={category.id} value={category.id} {...props}>
          {category.id === selectedCategoryId ? <>{children}</> : null}
        </FlatTabs.Content>
      ))}
    </Suspense>
  );
}
