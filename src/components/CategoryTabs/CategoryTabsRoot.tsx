import { CSS } from '@stitches/react';
import { ReactNode, Suspense } from 'react';
import { useCategories } from '../../hooks/useCategories';
import { FlatTabs } from '../FlatTabs';

interface Props {
  children: ReactNode;
  css?: CSS;
}

export function CategoryTabsRoot(props: Props) {
  return (
    <Suspense fallback={<Fallback {...props} />}>
      <Resolved {...props} />
    </Suspense>
  );
}

function Fallback(props: Props) {
  return <FlatTabs.Root {...props} />;
}

function Resolved({ children, ...props }: Props) {
  const { selectedCategoryId, selectCategory } = useCategories();

  return (
    <FlatTabs.Root value={selectedCategoryId} onValueChange={selectCategory} {...props}>
      {children}
    </FlatTabs.Root>
  );
}
